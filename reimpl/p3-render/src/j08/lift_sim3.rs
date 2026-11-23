//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00462280` popup_close_all_if_allowed
pub fn f_00462280(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6cc3bcu32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(0x6cc3b8u32);
                fl = lift::logic(esi & esi, 32);
                m.w8(esp.wrapping_add(0x7u32), (0x1u32) as u8);
                if fl.zf { pc = 12; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6cc3bcu32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = m.r32(esi.wrapping_add(0x8u32));
                esi = m.r32(esi);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(esp.wrapping_add(0xfu32), ((eax & 0xff)) as u8);
                pc = 9; continue;
            }
            8 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 9; continue;
            }
            9 => {
                fl = lift::logic(esi & esi, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6cbdb8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1f41u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x443150, ecx, &a); }
                esp = esp.wrapping_add(28);
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x7u32)))) & 0xff);
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

/// `0X004665D0` clip_stuff
pub fn f_004665d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0x10);
    eax = m.r32(ecx.wrapping_add(0x14u32));
    edx = m.r32(ecx.wrapping_add(0x18u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(ecx.wrapping_add(0x2cu32));
    m.w32(esp.wrapping_add(0x4u32), eax);
    let (r, f) = lift::add(esi, eax, 32); fl = f;
    esi = r;
    eax = m.r32(ecx.wrapping_add(0x30u32));
    m.w8(ecx.wrapping_add(0xfcu32), (0x0u32) as u8);
    ecx = esp.wrapping_add(0x4u32);
    let (r, f) = lift::add(eax, edx, 32); fl = f;
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6da5f0u32;
    m.w32(esp.wrapping_add(0xcu32), edx);
    m.w32(esp.wrapping_add(0x10u32), esi);
    m.w32(esp.wrapping_add(0x14u32), eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004DB7C0` st_town_hall_notices_refresh
pub fn f_004db7c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                m.w8(esp.wrapping_add(0x30u32), (0x0u32) as u8);
                m.w8(esp.wrapping_add(0x31u32), (0x3u32) as u8);
                m.w8(esp.wrapping_add(0x32u32), (0x2u32) as u8);
                m.w8(esp.wrapping_add(0x33u32), (0xbu32) as u8);
                m.w8(esp.wrapping_add(0x34u32), (0xeu32) as u8);
                m.w8(esp.wrapping_add(0x35u32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0x36u32), (0x7u32) as u8);
                m.w8(esp.wrapping_add(0x37u32), (0x8u32) as u8);
                eax = 0x6de531u32;
                pc = 1; continue;
            }
            1 => {
                m.w8(eax.wrapping_add(0xffffffffu32), (0xffu32) as u8);
                m.w8(eax, (0xffu32) as u8);
                let (r, f) = lift::add(eax, 0xcu32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x6de585u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(ebx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                esi = edx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x2cu32), esi);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(esi);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edi = 0x35u32;
                { let d = u64::from(edi); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esi, edx);
                eax = m.r32(0x6de4b0u32);
                fl = lift::sub(edx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), edx);
                if !fl.cf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = edx;
                edx = m.r32(0x6de4b0u32);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 31; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = m.r32(0x6de508u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                edi = esi.wrapping_add(edx.wrapping_mul(8));
                esi = ecx.wrapping_mul(4).wrapping_add(0x6de531u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2c4u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x20u32), ((edx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x20u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                pc = 6; continue;
            }
            6 => {
                ebx = 0x1u32;
                ecx = ebp;
                let (r, f) = lift::shl(ebx, (ecx & 0xff), 32, fl); fl = f;
                ebx = r;
                fl = lift::logic(edx & ebx, 32);
                if fl.zf { pc = 29; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(ebp).wrapping_add(0x30u32)))) & 0xff);
                ecx = m.r32(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 29; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xc4u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 29; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(m.r32(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x4f0u32)), 0x2710u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 29; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(m.r32(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x4f0u32)), 0x4e20u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w8(esi.wrapping_add(0xffffffffu32), ((eax & 0xff)) as u8);
                m.w8(esi, ((ebx & 0xff)) as u8);
                edx = m.r32(0x6de4a4u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(edi.wrapping_add(0x310u32));
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                edx = (edx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                m.w8(esi.wrapping_add(0x1u32), ((edx & 0xff)) as u8);
                eax = m.r32(edi.wrapping_add(0x314u32));
                edx = m.r32(edi.wrapping_add(0x2d4u32));
                let r = eax & 0xfu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0xfu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = ebx;
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi.wrapping_add(0x310u32));
                let r = edx & 0x7u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                eax = eax.wrapping_add(edx).wrapping_add(0xb4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 16; continue;
            }
            15 => {
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 16; continue;
            }
            16 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, ebx.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x672838u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = 0x1u32;
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(esi, 0x6de561u32, 32).1;
                m.w32(esi.wrapping_add(0x7u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = eax;
                edx = ebx.wrapping_add(0x4fu32);
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x29e4129fu32;
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ebp = m.r32(edx.wrapping_add(edi));
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                { let r = u64::from(eax) * u64::from(ecx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shl(ebp, 0x2u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::shr(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x7u32), ecx);
                pc = 21; continue;
            }
            20 => {
                eax = ebx.wrapping_add(0x4fu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                ebp = m.r32(eax.wrapping_add(edi));
                ecx = ebp;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ebp as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 24; continue;
            }
            23 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(ebp as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(edx, 0xfau32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = 0xfau32;
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w8(esi.wrapping_add(0x2u32), ((edx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x14u32));
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = ebx & 0x3u32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(esi, 0xcu32, 32); fl = f;
                esi = r;
                ecx = ebx.wrapping_add(eax).wrapping_add(0x9u32);
                ebx = m.r32(0x6de4b4u32);
                let (r, f) = lift::shl(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esi.wrapping_add(0xfffffff7u32), ecx);
                fl = lift::sub(esi, 0x6de585u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.sf != fl.of) { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                m.w32(esp.wrapping_add(0x10u32), 0x64u32);
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                ebp = 0x64u32;
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 30; continue;
            }
            30 => {
                ebx = m.r32(esp.wrapping_add(0x28u32));
                esi = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x17u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(0x6de4b0u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 39; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 34; continue;
            }
            34 => {
                edx = m.r32(0x6de508u32);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esi = edi.wrapping_add(edx);
                pc = 35; continue;
            }
            35 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(ecx).wrapping_add(0x30u32)))) & 0xff);
                fl = lift::sub(m.r32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x4u32)), 0x0u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                edx = (edx & !0xff) | ((0x1u32) & 0xff);
                let (r, f) = lift::shl((edx & 0xff), (ecx & 0xff), 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) | (edx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 37; continue;
            }
            37 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 35; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = (eax & !0xffff) | (((eax & 0xff)) & 0xffff);
                m.w16(esi.wrapping_add(0x2c4u32), ((eax & 0xffff)) as u16);
                eax = m.r32(0x6de4b0u32);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0x9f8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, eax, 32).1;
                if fl.cf { pc = 34; continue; }
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(ebx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = edx.wrapping_add(ecx.wrapping_mul(8));
                ecx = m.r32(0x6de508u32);
                edx = m.r32(ecx.wrapping_add(0x2d4u32));
                ecx = m.r32(eax);
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(edx, 0x7u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::shl(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(eax, ecx);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004FDF50` init
pub fn f_004fdf50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                ebx = m.r32(0x6ddb70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(0x6de4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(0x6de4b4u32);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                let r = edi & 0xffu32; fl = lift::logic(r, 32);
                edi = r;
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                eax = edi.wrapping_add(esi);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                { let d = u64::from(ebx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ebx = edi;
                m.w8(ecx.wrapping_add(0x2u32), ((edx & 0xff)) as u8);
                eax = m.r32(0x6de4a0u32);
                edx = ebp;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shr(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(0x6ddb74u32);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                { let d = u64::from(ebx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = esi;
                let r = eax & 0xfu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(ecx.wrapping_add(0x3u32), ((edx & 0xff)) as u8);
                ebx = m.r32(0x6de4a0u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x2e0u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                let r = eax & 0x1fu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(eax, 0x30u32, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(8));
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = ebx.wrapping_add(esi);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let r = eax & 0x1fu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ebx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(0x6de4b4u32);
                let (r, f) = lift::shl(ebx, 0x8u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edx, eax, 32).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(ecx.wrapping_add(0x4u32), eax);
                pc = 3; continue;
            }
            2 => {
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                pc = 3; continue;
            }
            3 => {
                eax = edi.wrapping_add(ebp);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = 0x258u32;
                { let v = lift::fild64(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x6734acu32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x24u32));
                m.w8(ecx.wrapping_add(0x9u32), ((edx & 0xff)) as u8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ebp;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let v = lift::fild64(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x6734a8u32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x24u32));
                m.w8(ecx.wrapping_add(0xau32), ((edx & 0xff)) as u8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(ecx.wrapping_add(0xau32), ((eax & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::sub(esi, ebp, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x18u32), esi);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let v = lift::fild64(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xau32)))) & 0xff);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x6734a4u32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x24u32));
                m.w8(ecx.wrapping_add(0xbu32), ((edx & 0xff)) as u8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(ecx.wrapping_add(0xbu32), ((eax & 0xff)) as u8);
                pc = 7; continue;
            }
            7 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x6734a0u32); let d = fp.st(0); fp.set(0, d * v); }
                edx = m.r32(esp.wrapping_add(0x28u32));
                m.w8(ecx.wrapping_add(0xfu32), (0xffu32) as u8);
                fl = lift::sub(edx, ebx, 32).1;
                m.w8(ecx.wrapping_add(0xeu32), ((ebx & 0xff)) as u8);
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                eax = m.r32(esp.wrapping_add(0x24u32));
                m.w8(ecx.wrapping_add(0x8u32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esi = 0xbu32;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(esi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w16(ecx.wrapping_add(0xcu32), ((ebx & 0xffff)) as u16);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w8(ecx.wrapping_add(0x8u32), ((edx & 0xff)) as u8);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xbu32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xau32)))) & 0xff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x9u32)))) & 0xff);
                edi = 0xbu32;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                eax = m.r32(esp.wrapping_add(0x24u32));
                esi = edx;
                let (r, f) = lift::sar(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                edx = esi;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shr(edx, 0x1fu32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(esi, edx, 32); fl = f;
                esi = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(edx).wrapping_add(0xau32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w16(ecx.wrapping_add(0xcu32), ((eax & 0xffff)) as u16);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0050AC40` create_pirate_band_convoy
pub fn f_0050ac40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe4u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x30u32), esi);
                m.w16(esp.wrapping_add(0x14u32), ((ebx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x16u32), ((ebx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x18u32), edi);
                m.w8(esp.wrapping_add(0x12u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x509ae0, ecx, &a); }
                ecx = 0xffffu32;
                edx = m.r32(esi.wrapping_add(0x8u32));
                let r = edi & ecx; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x34u32), edi);
                eax = edi.wrapping_add(edi.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebp = edx.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0xe4u32), ((eax & 0xffff)) as u16);
                m.w16(ebp.wrapping_add(0x8u32), ((ecx & 0xffff)) as u16);
                m.w16(ebp.wrapping_add(0xau32), ((ecx & 0xffff)) as u16);
                fl = lift::sub(u32::from(m.r16(0x6de4aau32)), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 8; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = ebx;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x19u32)))) & 0xff);
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(0x6de508u32);
                edx = m.r32(ecx.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x2c8u32));
                let r = edx & 0x40u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((edx & 0xff), 0x40u32, 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x478u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x16u32))), (eax & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x14u32)))) & 0xffff);
                fl = lift::sub((esi & 0xffff), (eax & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                m.w16(esp.wrapping_add(0x16u32), ((esi & 0xffff)) as u16);
                m.w8(esp.wrapping_add(0x13u32), ((ecx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x12u32), ((ebx & 0xff)) as u8);
                m.w16(esp.wrapping_add(0x14u32), ((eax & 0xffff)) as u16);
                pc = 7; continue;
            }
            6 => {
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                m.w16(esp.wrapping_add(0x16u32), ((eax & 0xffff)) as u16);
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if fl.cf { pc = 1; continue; }
                pc = 8; continue;
            }
            8 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x44u32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::logic((ecx & 0xffff) & (ecx & 0xffff), 16);
                if (fl.cf || fl.zf) { pc = 30; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w16(esp.wrapping_add(0x28u32), ((edx & 0xffff)) as u16);
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x6u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                let (r, f) = lift::shl(edi, 0x3u32, 32, fl); fl = f;
                edi = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), edi);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = esp.wrapping_add(ecx).wrapping_add(0x12u32);
                pc = 11; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp.wrapping_add(0x20u32));
                pc = 11; continue;
            }
            11 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xffffffffu32)))) & 0xff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ebx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                edx = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd7a0u32;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x507d60, ecx, &a); }
                esp = esp.wrapping_add(12);
                esi = eax;
                eax = m.r32(0x6de508u32);
                ecx = edi.wrapping_add(eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(eax).wrapping_add(0x2c8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(0x66e048u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                pc = 14; continue;
            }
            13 => {
                edi = m.r32(0x66e060u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52a8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = edi.wrapping_mul(8);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w8(esi.wrapping_add(0xfu32), (0x2u32) as u8);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edi;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(esi.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x14u32), ecx);
                m.w32(esi.wrapping_add(0x18u32), ecx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x10u32), edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4f9120, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x44u32)))) & 0xffff);
                m.w8(esi, ((ebx & 0xff)) as u8);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xeu32)))) & 0xffff);
                edi = esi.wrapping_add(0x160u32);
                m.w16(esi.wrapping_add(0x4u32), ((ecx & 0xffff)) as u16);
                m.w16(ebp.wrapping_add(0xeu32), ((edx & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd890u32))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd88eu32))) & 0xffff);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add((edx & 0xffff), (ecx & 0xffff), 16); fl = f;
                edx = (edx & !0xffff) | ((r) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | (((edx & 0xffff)) & 0xffff);
                ebx = 0x133u32;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6ddb78u32)), 16).1;
                if !fl.cf { pc = 15; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w16(0x6dd88eu32, ((edx & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x15eu32), ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd88eu32))) & 0xffff);
                ecx = 0x6ddaa0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x20u32, 32).1;
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w8(edi, ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w8(edi, (0x0u32) as u8);
                let r = u32::from(m.r8(esi.wrapping_add(0x3cu32))) | 0x1u32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0x3cu32), (r) as u8);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5181d0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5184f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6dd7a0u32;
                m.w16(esi.wrapping_add(0x40u32), ((eax & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x154u32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5097c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = eax;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0x42u32), ((eax & 0xffff)) as u16);
                eax = m.r32(edx);
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                { let a = []; eax = h.call_at(esp, m, 0x4fe190, ecx, &a); }
                eax = esi.wrapping_add(0x13cu32);
                ecx = 0xcu32;
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(u32::from(m.r8(eax)), 0x7u32, 8).1;
                if (fl.cf || fl.zf) { pc = 24; continue; }
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), 0x7u32, 8).1;
                if (fl.cf || fl.zf) { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                m.w8(eax, (0x3u32) as u8);
                m.w8(eax.wrapping_add(0x1u32), (0x6u32) as u8);
                pc = 24; continue;
            }
            23 => {
                m.w8(eax, (0x0u32) as u8);
                pc = 24; continue;
            }
            24 => {
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 20; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x8u32), ((edx & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x6u32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x48u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                m.w16(esi.wrapping_add(0x134u32), (0xau32) as u16);
                pc = 28; continue;
            }
            27 => {
                m.w16(esi.wrapping_add(0x134u32), (0x9u32) as u16);
                pc = 28; continue;
            }
            28 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x44u32)))) & 0xffff);
                m.w16(eax.wrapping_add(0xau32), ((ecx & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 10; continue; }
                pc = 29; continue;
            }
            29 => {
                edi = m.r32(esp.wrapping_add(0x34u32));
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                pc = 30; continue;
            }
            30 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x80u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x501680, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6dd7a8u32);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x48u32)))) & 0xff);
                eax = edi.wrapping_add(edi.wrapping_mul(2));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                ecx = ecx.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dd88cu32))) & 0xffff);
                let (r, f) = lift::add(edx, 0x9u32, 32); fl = f;
                edx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w16(ecx.wrapping_add(0x8u32), ((eax & 0xffff)) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w16(0x6dd88cu32, ((edi & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                m.w16(ecx.wrapping_add(0x12u32), ((edx & 0xffff)) as u16);
                m.w16(ecx.wrapping_add(0x16u32), ((esi & 0xffff)) as u16);
                ecx = m.r32(0x6dd894u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                edi = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                let r = u32::from(m.r8(eax.wrapping_add(0x3cu32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(eax.wrapping_add(0x3cu32), (r) as u8);
                m.w16(eax.wrapping_add(0x134u32), ((edx & 0xffff)) as u16);
                m.w16(eax.wrapping_add(0x138u32), ((esi & 0xffff)) as u16);
                eax = ecx;
                ecx = m.r32(0x6dd894u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 31; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x40u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                m.w16(ebp.wrapping_add(0x2u32), ((edx & 0xffff)) as u16);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                m.w16(ebp.wrapping_add(0x16u32), (0x900u32) as u16);
                pc = 34; continue;
            }
            34 => {
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

/// `0X0050B250` start_sea_battle
pub fn f_0050b250(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                ecx = 0x6e55d0u32;
                { let a = []; eax = h.call_at(esp, m, 0x611cd0, ecx, &a); }
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, 0xffu32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.cf { pc = 88; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x34u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0x4u32));
                ebx = ebp.wrapping_add(ebp.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(ebx)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (eax & 0xffff), 16).1;
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                if !fl.cf { pc = 47; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebx).wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 47; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(ebx).wrapping_add(0x134u32))), 0x16u32, 16).1;
                if fl.zf { pc = 47; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx.wrapping_add(edi);
                edx = (edx & !0xffff) | ((u32::from(m.r8(edx.wrapping_add(edi)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), (eax & 0xffff), 16).1;
                if !fl.cf { pc = 47; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 47; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                eax = m.r32(0x6dfc14u32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 47; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebx)))) & 0xff);
                eax = m.r32(0x6dfc14u32);
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 8; continue;
            }
            8 => {
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                if !fl.cf { pc = 47; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                pc = 10; continue;
            }
            10 => {
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x50u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 20; continue; }
                pc = 11; continue;
            }
            11 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ebp = lift::sx16(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x1eu32))));
                edi = edx.wrapping_add(eax);
                edx = m.r32(esp.wrapping_add(0x14u32));
                edx = lift::sx16(u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x1eu32))));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                ebp = lift::sx16(u32::from(m.r16(edi.wrapping_add(0x22u32))));
                edi = m.r32(esp.wrapping_add(0x14u32));
                edi = lift::sx16(u32::from(m.r16(eax.wrapping_add(edi).wrapping_add(0x22u32))));
                let (r, f) = lift::sub(edi, ebp, 32); fl = f;
                edi = r;
                ebp = edi;
                let (r, f) = lift::imul(ebp, edi); fl = f;
                ebp = r;
                edi = edx;
                let (r, f) = lift::imul(edi, edx); fl = f;
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ebp, edi, 32); fl = f;
                ebp = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xfcu32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf6u32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 18; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 19; continue;
            }
            15 => {
                edi = m.r32(esi.wrapping_add(0x8u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                if !fl.cf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32)))) & 0xffff);
                ecx = edi;
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.cf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 19; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 19; continue;
            }
            19 => {
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x52u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 30; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ebp = lift::sx16(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x1eu32))));
                edi = edx.wrapping_add(eax);
                edx = m.r32(esp.wrapping_add(0x14u32));
                edx = lift::sx16(u32::from(m.r16(eax.wrapping_add(edx).wrapping_add(0x1eu32))));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                ebp = lift::sx16(u32::from(m.r16(edi.wrapping_add(0x22u32))));
                edi = m.r32(esp.wrapping_add(0x14u32));
                edi = lift::sx16(u32::from(m.r16(eax.wrapping_add(edi).wrapping_add(0x22u32))));
                let (r, f) = lift::sub(edi, ebp, 32); fl = f;
                edi = r;
                ebp = edi;
                let (r, f) = lift::imul(ebp, edi); fl = f;
                ebp = r;
                edi = edx;
                let (r, f) = lift::imul(edi, edx); fl = f;
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ebp, edi, 32); fl = f;
                ebp = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xfcu32)))) & 0xffff);
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf == fl.of) { pc = 30; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf6u32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 28; continue; }
                pc = 23; continue;
            }
            23 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 29; continue;
            }
            25 => {
                edi = m.r32(esi.wrapping_add(0x8u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                if !fl.cf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32)))) & 0xffff);
                ecx = edi;
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.cf { pc = 26; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 29; continue;
            }
            28 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 29; continue;
            }
            29 => {
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.cf { pc = 10; continue; }
                pc = 31; continue;
            }
            31 => {
                pc = 45; continue;
            }
            32 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.cf { pc = 47; continue; }
                pc = 33; continue;
            }
            33 => {
                ebx = m.r32(esi.wrapping_add(0x4u32));
                pc = 34; continue;
            }
            34 => {
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebx).wrapping_add(0x50u32)))) & 0xffff);
                edi = eax.wrapping_add(ebx);
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x52u32)))) & 0xffff);
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.zf { pc = 44; continue; }
                pc = 36; continue;
            }
            36 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if !fl.cf { pc = 42; continue; }
                pc = 37; continue;
            }
            37 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 43; continue;
            }
            39 => {
                ebp = m.r32(esi.wrapping_add(0x8u32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebx).wrapping_add(0x6u32)))) & 0xffff);
                eax = ebp;
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 40; continue; }
                pc = 41; continue;
            }
            41 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ebp = m.r32(esp.wrapping_add(0x38u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 43; continue;
            }
            42 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 43; continue;
            }
            43 => {
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 44; continue;
            }
            44 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 34; continue; }
                pc = 45; continue;
            }
            45 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x24u32), 0x9fu32);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 47; continue;
            }
            47 => {
                ebx = m.r32(esp.wrapping_add(0x38u32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf6u32)))) & 0xffff);
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 52; continue; }
                pc = 48; continue;
            }
            48 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x18u32), 0xffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x505b60, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5062f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                eax = m.r32(esp.wrapping_add(0x14u32));
                if fl.cf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                m.w16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x12u32), (0x12u32) as u16);
                pc = 67; continue;
            }
            50 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x8u32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let r = u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x14u32))) | 0x12u32; fl = lift::logic(r, 8);
                m.w8(ecx.wrapping_add(eax).wrapping_add(0x14u32), (r) as u8);
                edx = m.r32(esi.wrapping_add(0x8u32));
                ecx = ecx.wrapping_add(eax).wrapping_add(0x14u32);
                ecx = 0x14u32;
                m.w16(edx.wrapping_add(eax).wrapping_add(0x12u32), ((ecx & 0xffff)) as u16);
                m.w16(edi.wrapping_add(0x134u32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 67; continue; }
                pc = 51; continue;
            }
            51 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let r = u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x14u32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(ecx.wrapping_add(eax).wrapping_add(0x14u32), (r) as u8);
                eax = ecx.wrapping_add(eax).wrapping_add(0x14u32);
                pc = 67; continue;
            }
            52 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xecu32)))) & 0xffff);
                ecx = esi.wrapping_add(0xecu32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = eax;
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 55; continue; }
                pc = 53; continue;
            }
            53 => {
                fl = lift::sub((edx & 0xffff), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 57; continue; }
                pc = 54; continue;
            }
            54 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32)))) & 0xffff);
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 53; continue; }
                pc = 55; continue;
            }
            55 => {
                eax = ebx.wrapping_add(ebx.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ebx = eax.wrapping_add(edx.wrapping_mul(4));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(ecx, ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                m.w16(ebx.wrapping_add(0x8u32), (0xffffu32) as u16);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 57; continue; }
                pc = 56; continue;
            }
            56 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x44f940, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 57; continue;
            }
            57 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 59; continue; }
                pc = 58; continue;
            }
            58 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx).wrapping_add(0x6u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 58; continue; }
                pc = 59; continue;
            }
            59 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 67; continue; }
                pc = 60; continue;
            }
            60 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x18u32));
                ebx = eax.wrapping_add(edx.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx | 0x20u32; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                m.w16(ebx.wrapping_add(0x12u32), (0x14u32) as u16);
                m.w32(ebx.wrapping_add(0x18u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                eax = 0xffffu32;
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                pc = 61; continue;
            }
            61 => {
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = u32::from(m.r8(edi.wrapping_add(0x3cu32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(edi.wrapping_add(0x3cu32), (r) as u8);
                fl = lift::sub(ebp, edi, 32).1;
                m.w16(edi.wrapping_add(0x134u32), (0x14u32) as u16);
                m.w16(edi.wrapping_add(0x50u32), ((eax & 0xffff)) as u16);
                m.w16(edi.wrapping_add(0x52u32), ((eax & 0xffff)) as u16);
                if fl.zf { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5176c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = 0xffffu32;
                pc = 63; continue;
            }
            63 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = edi & eax; fl = lift::logic(r, 32);
                edi = r;
                pc = 61; continue;
            }
            65 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                let r = u32::from(m.r8(ebx.wrapping_add(0x14u32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(ebx.wrapping_add(0x14u32), (r) as u8);
                pc = 67; continue;
            }
            67 => {
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edi = ebp.wrapping_add(ebp.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf6u32))), 16).1;
                if fl.cf { pc = 72; continue; }
                pc = 68; continue;
            }
            68 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x505b60, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x1cu32), 0xffffu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5062f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = (edx & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                if fl.cf { pc = 70; continue; }
                pc = 69; continue;
            }
            69 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                m.w16(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x12u32), (0x12u32) as u16);
                pc = 87; continue;
            }
            70 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let r = u32::from(m.r8(edx.wrapping_add(eax).wrapping_add(0x14u32))) | 0x12u32; fl = lift::logic(r, 8);
                m.w8(edx.wrapping_add(eax).wrapping_add(0x14u32), (r) as u8);
                ecx = edx.wrapping_add(eax).wrapping_add(0x14u32);
                edx = m.r32(esi.wrapping_add(0x8u32));
                ecx = 0x14u32;
                m.w16(edx.wrapping_add(eax).wrapping_add(0x12u32), ((ecx & 0xffff)) as u16);
                m.w16(edi.wrapping_add(0x134u32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 87; continue; }
                pc = 71; continue;
            }
            71 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let r = u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x14u32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(ecx.wrapping_add(eax).wrapping_add(0x14u32), (r) as u8);
                eax = ecx.wrapping_add(eax).wrapping_add(0x14u32);
                pc = 87; continue;
            }
            72 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xecu32)))) & 0xffff);
                ecx = esi.wrapping_add(0xecu32);
                eax = edx;
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 75; continue; }
                pc = 73; continue;
            }
            73 => {
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf6u32))), 16).1;
                if !fl.cf { pc = 77; continue; }
                pc = 74; continue;
            }
            74 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32)))) & 0xffff);
                eax = edx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 73; continue; }
                pc = 75; continue;
            }
            75 => {
                eax = ebx.wrapping_add(ebx.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ebx = eax.wrapping_add(edx.wrapping_mul(4));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(ecx, ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                m.w16(ebx.wrapping_add(0x8u32), (0xffffu32) as u16);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 77; continue; }
                pc = 76; continue;
            }
            76 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x44f940, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 77; continue;
            }
            77 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 79; continue; }
                pc = 78; continue;
            }
            78 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx).wrapping_add(0x6u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 78; continue; }
                pc = 79; continue;
            }
            79 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 87; continue; }
                pc = 80; continue;
            }
            80 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x18u32));
                ebx = eax.wrapping_add(edx.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx | 0x20u32; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x10u32)))) & 0xffff);
                m.w16(ebx.wrapping_add(0x12u32), (0x14u32) as u16);
                m.w32(ebx.wrapping_add(0x18u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                eax = 0xffffu32;
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                pc = 81; continue;
            }
            81 => {
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = u32::from(m.r8(edi.wrapping_add(0x3cu32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(edi.wrapping_add(0x3cu32), (r) as u8);
                fl = lift::sub(ebp, edi, 32).1;
                m.w16(edi.wrapping_add(0x134u32), (0x14u32) as u16);
                m.w16(edi.wrapping_add(0x50u32), ((eax & 0xffff)) as u16);
                m.w16(edi.wrapping_add(0x52u32), ((eax & 0xffff)) as u16);
                if fl.zf { pc = 83; continue; }
                pc = 82; continue;
            }
            82 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5176c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = 0xffffu32;
                pc = 83; continue;
            }
            83 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(esi.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 85; continue; }
                pc = 84; continue;
            }
            84 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = edi & eax; fl = lift::logic(r, 32);
                edi = r;
                pc = 81; continue;
            }
            85 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 87; continue; }
                pc = 86; continue;
            }
            86 => {
                let r = u32::from(m.r8(ebx.wrapping_add(0x14u32))) | 0x20u32; fl = lift::logic(r, 8);
                m.w8(ebx.wrapping_add(0x14u32), (r) as u8);
                pc = 87; continue;
            }
            87 => {
                edi = m.r32(esp.wrapping_add(0x38u32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x605420, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 88; continue;
            }
            88 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00511270` script_blob_pool_load_for_game_mode
pub fn f_00511270(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6630f2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(esi.wrapping_add(0x4u32)), ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x8u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(eax.wrapping_add(edi.wrapping_mul(8)));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(ecx.wrapping_add(edi.wrapping_mul(8)), ebx);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                pc = 6; continue;
            }
            6 => {
                m.w16(esi.wrapping_add(0x8u32), ((ebx & 0xffff)) as u16);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(0x6cc3e8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x34u32), ebx);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xdu32)))) & 0xff);
                fl = lift::sub(eax, 0x5u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x511674u32)) { 0x5112f8 => { pc = 8; continue; } 0x511308 => { pc = 9; continue; } 0x511318 => { pc = 10; continue; } 0x511328 => { pc = 11; continue; } _ => unreachable!() }
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c03c8u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c03d0u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c03dcu32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            11 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = esp.wrapping_add(0x10u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x14u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c03e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0418u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd8b9u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0434u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x34u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64f6dc, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                ebp = m.r32(edx.wrapping_add(0xfffffff8u32));
                fl = lift::sub(edi, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = ebp;
                ecx = esp.wrapping_add(0x3cu32);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x20u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d0c0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5110e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0x3cu32);
                edi = ebp.wrapping_add(0x1u32);
                m.w8(esp.wrapping_add(0x34u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 13; continue;
            }
            16 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 34; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x8u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edi.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5116f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x8u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 34; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(8)));
                eax = eax.wrapping_add(edi.wrapping_mul(8));
                fl = lift::sub(ecx, ebx, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w16(eax.wrapping_add(0x6u32), ((ebx & 0xffff)) as u16);
                pc = 33; continue;
            }
            22 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x6u32))), (ebx & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                if (fl.cf || fl.zf) { pc = 33; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(eax);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0x4u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(8)));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x2u32)))) & 0xffff);
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0x10);
                fl = lift::sub((ebx & 0xff), 0xfdu32, 8).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(ecx & eax, 32);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                if fl.zf { pc = 27; continue; }
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub((ebx & 0xff), 0xau32, 8).1;
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(8)));
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                m.w8(esp.wrapping_add(0x1cu32), ((ecx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(eax);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(ebp.wrapping_add(edx.wrapping_mul(4)), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x3cu32)))) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x3cu32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(8)));
                edx = m.r32(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x4u32));
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx)))) & 0xff);
                fl = lift::sub((ebx & 0xff), 0xfdu32, 8).1;
                let (r, f) = lift::sbb(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(edx & ecx, 32);
                if !fl.zf { pc = 24; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 35; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::sub((ebx & 0xff), 0xfdu32, 8).1;
                if !fl.zf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(8)));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx).wrapping_add(0x1u32)))) & 0xff);
                ecx = m.r32(0x6de4b4u32);
                m.w8(esp.wrapping_add(0x20u32), ((eax & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x20u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = m.r32(ebp.wrapping_add(edx.wrapping_mul(4)));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 30; continue;
            }
            30 => {
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8cf0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w16(eax.wrapping_add(0x6u32), (0x1bu32) as u16);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(8)));
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esp.wrapping_add(0x3cu32)))) & 0xffff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w16(eax.wrapping_add(0x14u32), ((ecx & 0xffff)) as u16);
                m.w32(eax.wrapping_add(0xcu32), ebp);
                m.w32(eax.wrapping_add(0x8u32), edx);
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(edi.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                m.w16(eax.wrapping_add(0x10u32), ((ecx & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(8)));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(eax.wrapping_add(0x12u32), ((edx & 0xffff)) as u16);
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edi.wrapping_mul(8)).wrapping_add(0x6u32)))) & 0xffff);
                eax = eax.wrapping_add(edi.wrapping_mul(8));
                fl = lift::sub(ecx, edx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 32; continue;
            }
            32 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 33; continue;
            }
            33 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x34u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
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
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            35 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(0x6de4b4u32);
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(8)));
                m.w32(esp.wrapping_add(0x28u32), ebx);
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx).wrapping_add(0x1u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x24u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = m.r32(ebp.wrapping_add(eax.wrapping_mul(4)));
                ebx = eax.wrapping_add(0xffffff00u32);
                fl = lift::sub(ebx, m.r32(esp.wrapping_add(0x28u32)), 32).1;
                if !fl.cf { pc = 30; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 44; continue; }
                pc = 37; continue;
            }
            37 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 44; continue; }
                pc = 38; continue;
            }
            38 => {
                fl = lift::sub(ecx, m.r32(edx), 32).1;
                if fl.zf { pc = 41; continue; }
                pc = 39; continue;
            }
            39 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edx, 0x8u32, 32); fl = f;
                edx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 38; continue; }
                pc = 40; continue;
            }
            40 => {
                pc = 44; continue;
            }
            41 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::sub(u32::from(m.r16(eax.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x6u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(eax.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x6u32), (r) as u16);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x6u32))), 0x0u32, 16).1;
                eax = ecx.wrapping_add(ebx.wrapping_mul(8));
                if !fl.zf { pc = 44; continue; }
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(eax);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_add(0x4);
                m.w32(edx.wrapping_add(ebx.wrapping_mul(8)), 0x0u32);
                pc = 44; continue;
            }
            44 => {
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 31; continue; }
                pc = 45; continue;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 31; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0052C019` trade_center_trade_with_convoy_body
pub fn f_0052c019(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0x68u32));
                eax = esi;
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x18u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = ecx.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x24u32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x6cu32), 0x0u32);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                ecx = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx))) & 0xff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                ecx = 0x6de4a0u32;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 3; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 3; continue;
            }
            3 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x74u32));
                edx = m.r32(esp.wrapping_add(0x6cu32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esp.wrapping_add(0x78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x52c6c0, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(0x6dd894u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 4; continue;
            }
            4 => {
                pc = 6; continue;
            }
            5 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6dd7a4u32);
                ebp = ebp.wrapping_add(ebp.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = m.r32(ebp.wrapping_add(0x18u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x2u32)))) & 0xff);
                eax = m.r32(ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x54u32));
                fl = lift::logic(eax & eax, 32);
                edi = ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x54u32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x78u32))), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32), ecx);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x52d4f0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(ebx.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(ebx.wrapping_add(0x20u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(ecx.wrapping_add(0x4b0u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(ecx.wrapping_add(0x4b0u32), edx);
                edx = m.r32(edi);
                edi = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x40u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32), edi);
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x40u32), ecx);
                pc = 11; continue;
            }
            10 => {
                m.w32(edi, 0x0u32);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(0x6dd894u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 14; continue;
            }
            14 => {
                esi = m.r32(esp.wrapping_add(0x68u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                edx = m.r32(esp.wrapping_add(0x78u32));
                eax = m.r32(esp.wrapping_add(0x74u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x52c6c0, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(0x6dd894u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(esi, edx, 32).1;
                m.w8(esp.wrapping_add(0x68u32), ((eax & 0xff)) as u8);
                m.w32(esp.wrapping_add(0x74u32), ebp);
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(0x6dd7a4u32);
                ecx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                eax = m.r32(ecx.wrapping_add(0x18u32));
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ecx = eax;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                edx = m.r32(esp.wrapping_add(0x74u32));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = edx.wrapping_add(ecx.wrapping_mul(8));
                m.w32(esp.wrapping_add(0x74u32), eax);
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6dd894u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 16; continue; }
                pc = 19; continue;
            }
            19 => {
                esi = m.r32(esp.wrapping_add(0x78u32));
                m.w32(esp.wrapping_add(0x7cu32), ebp);
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x74u32)), 0xc8u32, 32).1;
                if (fl.sf != fl.of) { pc = 55; continue; }
                pc = 21; continue;
            }
            21 => {
                edi = ebp;
                let r = edi & 0x80000001u32; fl = lift::logic(r, 32);
                edi = r;
                if !fl.sf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let r = edi | 0xfffffffeu32; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esp.wrapping_add(0x6cu32));
                ecx = edi;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebx).wrapping_add(0x34u32)))) & 0xff);
                edx = 0x1u32;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                fl = lift::logic((eax & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 54; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ecx)))) & 0xff);
                fl = lift::sub(edx, 0x18u32, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                if (fl.sf == fl.of) { pc = 54; continue; }
                pc = 25; continue;
            }
            25 => {
                fl = lift::logic(ebp & ebp, 32);
                if !fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                fl = lift::logic((eax & 0xff) & 0x2u32, 8);
                if fl.zf { pc = 30; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esp.wrapping_add(0x74u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esi = eax;
                let (r, f) = lift::sar(esi, 0x1u32, 32, fl); fl = f;
                esi = r;
                pc = 31; continue;
            }
            28 => {
                fl = lift::sub(ebp, 0x1u32, 32).1;
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                fl = lift::sub(ebp, 0x2u32, 32).1;
                if !fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                esi = m.r32(esp.wrapping_add(0x74u32));
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xcu32));
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf == fl.of) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                esi = eax;
                pc = 33; continue;
            }
            33 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x78u32))), 0x1u32, 16).1;
                m.w8(esp.wrapping_add(0x28u32), (u32::from(fl.zf)) as u8);
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                esi = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(esi, 0x3u32, 32, fl); fl = f;
                esi = r;
                pc = 37; continue;
            }
            36 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esi = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x52d6b0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebp = eax;
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 40; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(ebx.wrapping_add(0x20u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = eax;
                eax = ebp;
                esi = edx;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esi = eax;
                pc = 34; continue;
            }
            40 => {
                edx = m.r32(ebx.wrapping_add(0x20u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                m.w32(ebx.wrapping_add(0x20u32), edx);
                ecx = m.r32(eax.wrapping_add(0x4b4u32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(eax.wrapping_add(0x4b4u32), ecx);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x48u32));
                edx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, esi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x48u32), ecx);
                m.w32(eax.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xcu32), edx);
                eax = m.r32(esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x50u32));
                edi = esp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x50u32);
                edx = m.r32(0x6dd894u32);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(edi, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 53; continue; }
                pc = 41; continue;
            }
            41 => {
                pc = 43; continue;
            }
            42 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 43; continue;
            }
            43 => {
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 53; continue; }
                pc = 44; continue;
            }
            44 => {
                ecx = m.r32(0x6dd7a4u32);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x6u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(ebx.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 52; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                edi = edx.wrapping_add(edx.wrapping_mul(4));
                edi = edi.wrapping_add(edi.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x3u32, 32, fl); fl = f;
                edi = r;
                pc = 48; continue;
            }
            47 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                edi = edi.wrapping_add(edi.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                pc = 48; continue;
            }
            48 => {
                fl = lift::sub(edi, esi, 32).1;
                if (fl.sf != fl.of) { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x517fe0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x74u32));
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(esp.wrapping_add(0x74u32), eax);
                pc = 52; continue;
            }
            50 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                eax = edi;
                { let r = i64::from(eax as i32) * i64::from(ebp as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = eax;
                eax = esi.wrapping_add(0xffffffffu32);
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esi;
                let (r, f) = lift::adc(ebx, edx, fl.cf, 32); fl = f;
                ebx = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                ebx = eax;
                m.w32(esp.wrapping_add(0x34u32), edx);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x517fe0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x74u32));
                let (r, f) = lift::sub(ebp, ebx, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(esi, edi, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x74u32), edx);
                pc = 52; continue;
            }
            52 => {
                eax = m.r32(0x6dd894u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0x24u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 42; continue; }
                pc = 53; continue;
            }
            53 => {
                ebp = m.r32(esp.wrapping_add(0x7cu32));
                pc = 54; continue;
            }
            54 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x3u32, 32).1;
                m.w32(esp.wrapping_add(0x7cu32), ebp);
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = m.r32(0x6dd894u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 59; continue; }
                pc = 56; continue;
            }
            56 => {
                edx = m.r32(0x6dd7a4u32);
                ecx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                eax = m.r32(ecx.wrapping_add(0x18u32));
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                pc = 58; continue;
            }
            58 => {
                edx = m.r32(0x6dd894u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 56; continue; }
                pc = 59; continue;
            }
            59 => {
                eax = m.r32(esp.wrapping_add(0x38u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 63; continue; }
                pc = 60; continue;
            }
            60 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 63; continue; }
                pc = 61; continue;
            }
            61 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 81; continue; }
                pc = 63; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp.wrapping_add(0x70u32));
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x5u32))), 0x2u32, 8).1;
                if !fl.cf { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(0x6b95f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x4u32)))) & 0xff);
                ecx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6b1f20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x6cu32));
                let r = eax & 0x1u32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6b95f8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 66; continue;
            }
            65 => {
                eax = m.r32(0x6b9614u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6b95f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 66; continue;
            }
            66 => {
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 67; continue;
            }
            67 => {
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 72; continue; }
                pc = 68; continue;
            }
            68 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0x2u32)))) & 0xff);
                ebp = edx;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 70; continue; }
                pc = 69; continue;
            }
            69 => {
                eax = 0x51eb851fu32;
                ebx = (ebx & !0xff) | ((0x42u32) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 71; continue;
            }
            70 => {
                eax = 0x10624dd3u32;
                ebx = (ebx & !0xff) | ((0x4cu32) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 71; continue;
            }
            71 => {
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32), edx);
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32));
                edx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x40u32));
                eax = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = lift::sx8((ebx & 0xff));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6b9600u32);
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                eax = esp.wrapping_add(0x18u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 72; continue;
            }
            72 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 67; continue; }
                pc = 73; continue;
            }
            73 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 74; continue;
            }
            74 => {
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x48u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 80; continue; }
                pc = 75; continue;
            }
            75 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 77; continue; }
                pc = 76; continue;
            }
            76 => {
                eax = 0x51eb851fu32;
                ebx = (ebx & !0xff) | ((0x42u32) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 78; continue;
            }
            77 => {
                eax = 0x10624dd3u32;
                ebx = (ebx & !0xff) | ((0x4cu32) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 78; continue;
            }
            78 => {
                ecx = edx;
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x48u32), edx);
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 80; continue; }
                pc = 79; continue;
            }
            79 => {
                edx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x50u32));
                eax = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax)))) & 0xff);
                eax = lift::sx8((ebx & 0xff));
                edx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6b9604u32);
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                eax = esp.wrapping_add(0x18u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 80; continue;
            }
            80 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 74; continue; }
                pc = 81; continue;
            }
            81 => {
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x60u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x58u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x68u32)))) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x54);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0052C6C0` trade_center_handle_convoy_contract
pub fn f_0052c6c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66337au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x50);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(esp.wrapping_add(0x8u32), edi);
                fl = lift::sub(u32::from(m.r8(edi.wrapping_add(0x4u32))), 0x7fu32, 8).1;
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x50u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x5c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x64u32));
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x18u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                esi = ecx.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x24u32), esi);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x74u32), ebx);
                m.w8(esp.wrapping_add(0x1fu32), ((ebx & 0xff)) as u8);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                ecx = esp.wrapping_add(0x60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                ecx = 0x6de4a0u32;
                if !fl.cf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 5; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                pc = 5; continue;
            }
            5 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(edi.wrapping_add(0x24u32));
                let r = (edx & 0xff) ^ (edx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebp = m.r32(eax);
                m.w8(esp.wrapping_add(0x70u32), ((edx & 0xff)) as u8);
                fl = lift::sub(ebp, eax, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(u32::from(m.r16(ebp.wrapping_add(0x8u32))), (ecx & 0xffff), 16).1;
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(ebp);
                fl = lift::sub(ebp, eax, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 9; continue;
            }
            9 => {
                pc = 11; continue;
            }
            10 => {
                m.w8(esp.wrapping_add(0x70u32), (0x1u32) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x70u32)))) & 0xff);
                pc = 11; continue;
            }
            11 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(esp.wrapping_add(0x12u32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xbu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                m.w8(esp.wrapping_add(0x12u32), ((eax & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(esp.wrapping_add(0x11u32), (0x0u32) as u8);
                if !fl.zf { pc = 25; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x7cu32))), 0x2u32, 16).1;
                if !fl.zf { pc = 22; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(u32::from(m.r8(edi.wrapping_add(0x5u32))), 0x2u32, 8).1;
                if !fl.cf { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x80u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 22; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x11u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x46c320, ecx, &a); }
                eax = m.r32(0x6de4b4u32);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x6cu32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c340, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = esp.wrapping_add(0x28u32);
                m.w16(esp.wrapping_add(0x34u32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c380, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esp.wrapping_add(0x32u32), ((eax & 0xff)) as u8);
                eax = m.r32(0x6de4b0u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(0x6de508u32);
                edi = edx.wrapping_add(0x2d4u32);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(edi);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = m.r32(0x6de4b0u32);
                let (r, f) = lift::adc(ebx, edx, fl.cf, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edi, 0x9f8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ecx, eax, 32).1;
                if fl.cf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8000u32);
                ecx = esp.wrapping_add(0x28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c380, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::add(eax, 0x8000u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = 0x10u32;
                { let a = []; eax = h.call_at(esp, m, 0x63c610, ecx, &a); }
                let (r, f) = lift::add(eax, 0x1f4u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x38u32), eax);
                eax = m.r32(0x6de4b4u32);
                let (r, f) = lift::adc(edx, esi, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, 0xb400u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c380, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w8(esp.wrapping_add(0x33u32), ((eax & 0xff)) as u8);
                ecx = esp.wrapping_add(0x24u32);
                if fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e20u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c380, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x32u32));
                edi = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, 0x27100u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), esi);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                { let v = lift::fild64(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edi).wrapping_add(0x2u32)))) & 0xff);
                lift::fst32(m, esp.wrapping_add(0x84u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x84u32)); fp.push(v); }
                pc = 24; continue;
            }
            22 => {
                m.w8(esp.wrapping_add(0x11u32), (0x0u32) as u8);
                pc = 68; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7530u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c380, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x32u32));
                edi = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, 0x13880u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), esi);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                { let v = lift::fild64(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edi)))) & 0xff);
                lift::fst32(m, esp.wrapping_add(0x84u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x84u32)); fp.push(v); }
                pc = 24; continue;
            }
            24 => {
                { let v = lift::fld32(m, edx.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x186a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebx = m.r32(edi.wrapping_add(0x24u32));
                esi = edi.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x38u32), eax);
                ecx = esi;
                eax = m.r32(ebx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x20u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52de10, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(ebx.wrapping_add(0x4u32), eax);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(ecx, eax);
                ecx = esp.wrapping_add(0x30u32);
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52de40, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w8(esp.wrapping_add(0x68u32), (0x0u32) as u8);
                m.w32(esi.wrapping_add(0x8u32), ecx);
                let (r, f) = lift::add(u32::from(m.r16(edi.wrapping_add(0x1cu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi.wrapping_add(0x1cu32), (r) as u16);
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x46c330, ecx, &a); }
                pc = 68; continue;
            }
            25 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 37; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x84u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 37; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xau32)))) & 0xffff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edi).wrapping_add(0x2u32)))) & 0xff);
                eax = m.r32(0x6dd894u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esp.wrapping_add(0x7cu32), ((ecx & 0xff)) as u8);
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 37; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 68; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(0x6dd7a4u32);
                edi = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(edi.wrapping_add(0x18u32));
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x80u32), ebx);
                if (fl.zf || fl.sf != fl.of) { pc = 36; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esi = m.r32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x54u32));
                ecx = edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x54u32);
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 35; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(ebp.wrapping_add(0xcu32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                esi = eax;
                pc = 33; continue;
            }
            33 => {
                eax = 0x51eb851fu32;
                ebx = m.r32(esp.wrapping_add(0x24u32));
                { let r = u64::from(eax) * u64::from(esi); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                eax = m.r32(ebp.wrapping_add(0x10u32));
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(eax, edx); fl = f;
                eax = r;
                let (r, f) = lift::add(m.r32(ebx.wrapping_add(0x20u32)), eax, 32); fl = f;
                m.w32(ebx.wrapping_add(0x20u32), r);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(m.r32(ebx.wrapping_add(0x4b0u32)), eax, 32); fl = f;
                m.w32(ebx.wrapping_add(0x4b0u32), r);
                ebx = m.r32(ecx);
                let (r, f) = lift::sub(ebx, esi, 32); fl = f;
                ebx = r;
                m.w32(ecx, ebx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                ebx = m.r32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x40u32));
                ecx = esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x40u32);
                let (r, f) = lift::add(ebx, esi, 32); fl = f;
                ebx = r;
                m.w32(ecx, ebx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                ebx = m.r32(esp.wrapping_add(0x80u32));
                esi = m.r32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x48u32));
                ecx = esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x48u32);
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                m.w32(ecx, esi);
                ecx = m.r32(ebp.wrapping_add(0xcu32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                eax = ecx;
                m.w32(ebp.wrapping_add(0xcu32), ecx);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 36; continue; }
                pc = 34; continue;
            }
            34 => {
                m.w8(esp.wrapping_add(0x70u32), (0x0u32) as u8);
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 36; continue;
            }
            35 => {
                m.w32(ecx, 0x0u32);
                pc = 36; continue;
            }
            36 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                edx = m.r32(0x6dd894u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ebx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x70u32)))) & 0xff);
                if (fl.sf != fl.of) { pc = 28; continue; }
                pc = 37; continue;
            }
            37 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 68; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 68; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x84u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 68; continue; }
                pc = 40; continue;
            }
            40 => {
                edi = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(0x6dd894u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xau32)))) & 0xffff);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 44; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(0x6dd7a4u32);
                ecx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(ecx.wrapping_add(0x18u32));
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ecx = eax;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                esi = esi.wrapping_add(edx.wrapping_mul(8));
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(0x6dd894u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 41; continue; }
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx)))) & 0xff);
                ecx = esi;
                m.w8(esp.wrapping_add(0x7cu32), ((eax & 0xff)) as u8);
                eax = m.r32(ebp.wrapping_add(0xcu32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = eax;
                pc = 46; continue;
            }
            46 => {
                eax = m.r32(esp.wrapping_add(0x7cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x70u32), eax);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x84u32), ((edx & 0xff)) as u8);
                pc = 47; continue;
            }
            47 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x84u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x51eb851fu32;
                esi = edx.wrapping_add(edx.wrapping_mul(4));
                esi = esi.wrapping_add(esi.wrapping_mul(4));
                let (r, f) = lift::shl(esi, 0x3u32, 32, fl); fl = f;
                esi = r;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 50; continue;
            }
            49 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                esi = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x10624dd3u32;
                esi = esi.wrapping_add(esi.wrapping_mul(4));
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 50; continue;
            }
            50 => {
                ebx = edx;
                let (r, f) = lift::imul(ebx, m.r32(ebp.wrapping_add(0x10u32))); fl = f;
                ebx = r;
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                if fl.zf { pc = 53; continue; }
                pc = 51; continue;
            }
            51 => {
                eax = m.r32(edi.wrapping_add(0x20u32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = eax;
                eax = ebx;
                esi = edx;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                pc = 47; continue;
            }
            53 => {
                edx = m.r32(edi.wrapping_add(0x20u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(edi.wrapping_add(0x20u32), edx);
                ecx = m.r32(eax.wrapping_add(0x4b4u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(eax.wrapping_add(0x4b4u32), ecx);
                eax = 0xae147ae1u32;
                { let r = i64::from(eax as i32) * i64::from(esi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = m.r32(ebp.wrapping_add(0xcu32));
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                eax = ecx;
                m.w32(ebp.wrapping_add(0xcu32), ecx);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                pc = 55; continue;
            }
            55 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                edx = m.r32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x50u32));
                eax = esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x50u32);
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                m.w32(eax, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                ecx = m.r32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x58u32));
                eax = esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x58u32);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(eax, ecx);
                ecx = m.r32(0x6dd894u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 68; continue; }
                pc = 56; continue;
            }
            56 => {
                pc = 58; continue;
            }
            57 => {
                eax = m.r32(esp.wrapping_add(0x80u32));
                pc = 58; continue;
            }
            58 => {
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 68; continue; }
                pc = 59; continue;
            }
            59 => {
                edx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x84u32), eax);
                ecx = m.r32(eax.wrapping_add(0x18u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(ecx & ecx, 32);
                m.w32(esp.wrapping_add(0x80u32), edx);
                if (fl.zf || fl.sf != fl.of) { pc = 67; continue; }
                pc = 60; continue;
            }
            60 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x70u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                edi = edx.wrapping_add(edx.wrapping_mul(4));
                edi = edi.wrapping_add(edi.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x3u32, 32, fl); fl = f;
                edi = r;
                pc = 63; continue;
            }
            62 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                edi = edi.wrapping_add(edi.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                pc = 63; continue;
            }
            63 => {
                fl = lift::sub(edi, esi, 32).1;
                if (fl.sf != fl.of) { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(esp.wrapping_add(0x70u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x90u32));
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x517fe0, ecx, &a); }
                esp = esp.wrapping_add(12);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 67; continue;
            }
            65 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                eax = edi;
                { let r = i64::from(eax as i32) * i64::from(ebx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = eax;
                eax = esi.wrapping_add(0xffffffffu32);
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esi;
                let (r, f) = lift::adc(ebx, edx, fl.cf, 32); fl = f;
                ebx = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x84u32));
                m.w32(esp.wrapping_add(0x20u32), edx);
                edx = m.r32(esp.wrapping_add(0x70u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x517fe0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebx = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(esi, edi, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                pc = 67; continue;
            }
            67 => {
                eax = m.r32(0x6dd894u32);
                ecx = m.r32(esp.wrapping_add(0x80u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 57; continue; }
                pc = 68; continue;
            }
            68 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                ebx = m.r32(esp.wrapping_add(0x78u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 72; continue; }
                pc = 69; continue;
            }
            69 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 72; continue; }
                pc = 70; continue;
            }
            70 => {
                eax = m.r32(esp.wrapping_add(0x50u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 79; continue; }
                pc = 72; continue;
            }
            72 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x80u32), ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x68u32), (0x2u32) as u8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 74; continue; }
                pc = 73; continue;
            }
            73 => {
                eax = m.r32(0x6b1780u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edi = m.r32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x44u32));
                pc = 75; continue;
            }
            74 => {
                edx = m.r32(0x6b177cu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xau32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edi = m.r32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x54u32));
                pc = 75; continue;
            }
            75 => {
                ecx = esp.wrapping_add(0x84u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x70u32);
                ecx = esp.wrapping_add(0x78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ebp.wrapping_add(0x14u32));
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ecx = esp.wrapping_add(0x84u32);
                m.w8(esp.wrapping_add(0x68u32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                esi = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x80u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                edx = m.r32(ebp.wrapping_add(0x10u32));
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6b1f20u32));
                eax = m.r32(esp.wrapping_add(0x80u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(edi as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x42u32);
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6b1778u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(ebp.wrapping_add(0xcu32));
                esp = esp.wrapping_add(0x20);
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 77; continue; }
                pc = 76; continue;
            }
            76 => {
                ecx = m.r32(esp.wrapping_add(0x70u32));
                edx = m.r32(esp.wrapping_add(0x78u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6b1790u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x42u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 78; continue;
            }
            77 => {
                ecx = m.r32(0x6b1794u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x88u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 78; continue;
            }
            78 => {
                edx = esp.wrapping_add(0x84u32);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x18u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x84u32);
                m.w8(esp.wrapping_add(0x68u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x80u32);
                m.w8(esp.wrapping_add(0x68u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 80; continue;
            }
            79 => {
                esi = m.r32(esp.wrapping_add(0x14u32));
                pc = 80; continue;
            }
            80 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x11u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x68u32), (0x4u32) as u8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 85; continue; }
                pc = 81; continue;
            }
            81 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x33u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 83; continue; }
                pc = 82; continue;
            }
            82 => {
                edx = m.r32(esp.wrapping_add(0x32u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(esi).wrapping_add(0x2u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x84u32), ((eax & 0xff)) as u8);
                pc = 84; continue;
            }
            83 => {
                ecx = m.r32(esp.wrapping_add(0x32u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(esi)))) & 0xff);
                m.w8(esp.wrapping_add(0x84u32), ((edx & 0xff)) as u8);
                pc = 84; continue;
            }
            84 => {
                eax = esp.wrapping_add(0x80u32);
                ecx = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x40u32));
                edx = esp.wrapping_add(0x74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5311d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x80u32));
                ecx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x7cu32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x8cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                edx = m.r32(esp.wrapping_add(0x3fu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = 0x6b176cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x42u32);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6b1f20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6b1764u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x2c);
                eax = esp.wrapping_add(0x2cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 85; continue;
            }
            85 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 87; continue; }
                pc = 86; continue;
            }
            86 => {
                eax = m.r32(ebp.wrapping_add(0x4u32));
                ecx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w32(eax, ecx);
                edx = m.r32(ebp);
                eax = m.r32(ebp.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esi.wrapping_add(0x28u32), ecx);
                let (r, f) = lift::sub(u32::from(m.r16(esi.wrapping_add(0x1cu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0x1cu32), (r) as u16);
                pc = 87; continue;
            }
            87 => {
                ecx = esp.wrapping_add(0x2cu32);
                m.w8(esp.wrapping_add(0x68u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x68u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x60u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x11u32)))) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x5c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0052D6B0` trade_center_get_buy_cost
pub fn f_0052d6b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x5u32)))) & 0xff);
                edx = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub((ebx & 0xff), (edx & 0xff), 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                if (fl.cf || fl.zf) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub((ebx & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 6; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x30u32))), (eax & 0xff), 8).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x14u32), 0x4cccccccu32);
                pc = 7; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x1cu32), 0xe6666666u32);
                m.w32(esp.wrapping_add(0x14u32), 0x33333333u32);
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                { let v = lift::fild64(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx)))) & 0xff);
                ebp = eax;
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, ebp.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = lift::fild64(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                esi = eax;
                edi = edx;
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, ebp.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = esi;
                ebp = edi;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sbb(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(ebx.wrapping_mul(4).wrapping_add(0x673990u32));
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ebp = m.r32(esp.wrapping_add(0x28u32));
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sbb(edi, edx, fl.cf, 32); fl = f;
                edi = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = edx;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sar(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                eax = edx;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

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

/// `0X00542560` create_initial_ai_merchant_for_town
pub fn f_00542560(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6ddb74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                m.w32(esp.wrapping_add(0x18u32), eax);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xcu32)))) & 0xffff);
                edx = m.r32(0x6de508u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xeu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x6u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esi = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx).wrapping_add(0x2c8u32)))) & 0xff);
                edx = m.r32(esi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xff) | ((!(eax & 0xff)) & 0xff);
                edx = m.r32(edi);
                eax = lift::sx8((eax & 0xff));
                let r = eax & 0x40u32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = (eax & 0xff) | 0xa0u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shr(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x530470, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebx = (ebx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 15; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi);
                eax = m.r32(edi);
                let r = ecx & 0x3u32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(edi, eax);
                edx = m.r32(esi);
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = 0x6dd73cu32;
                m.w32(esi, edx);
                eax = m.r32(0x6de4b4u32);
                edx = eax.wrapping_add(ebx).wrapping_add(0xa00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8cf0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = (ecx & !0xff) | (((ebx & 0xff)) & 0xff);
                m.w16(eax.wrapping_add(0x6u32), (0x4u32) as u16);
                m.w16(eax.wrapping_add(0x8u32), ((ecx & 0xffff)) as u16);
                ecx = 0xffff8300u32;
                m.w16(eax.wrapping_add(0xau32), ((ecx & 0xffff)) as u16);
                m.w16(eax.wrapping_add(0xcu32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x8u32)))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x7u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0xeu32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x3u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0x10u32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::add((ecx & 0xff), 0x7u32, 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0xfu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0x11u32), ((edx & 0xff)) as u8);
                m.w8(eax.wrapping_add(0xfu32), ((ecx & 0xff)) as u8);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x10u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x12u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4f68d0, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = ebx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                eax = ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(0x6ddb70u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = esi;
                m.w8(esi.wrapping_add(0x1cu32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4f6420, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(m.r32(esp.wrapping_add(0x20u32))); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(esi.wrapping_add(0x1du32), ((edx & 0xff)) as u8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x34u32));
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(ecx))), 0x0u32, 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1du32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6ddb74u32))) & 0xff);
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::sub((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                m.w8(esi.wrapping_add(0x1du32), ((eax & 0xff)) as u8);
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(ecx))), 0x0u32, 8).1;
                if !fl.zf { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4f6450, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w8(eax.wrapping_add(ecx), (0x1u32) as u8);
                ecx = 0x6de4a0u32;
                m.w8(esi.wrapping_add(0x546u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(ebp.wrapping_add(0x8u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi, eax);
                eax = m.r32(edi);
                { let d = u64::from(m.r32(0x6de4b0u32)); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x18u32), 0x4u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                pc = 7; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x34u32));
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x507d60, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x30u32));
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(m.r32(0x6de4b0u32)); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                eax = m.r32(0x6de508u32);
                m.w32(esp.wrapping_add(0x34u32), edx);
                if !fl.cf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::logic(u32::from(m.r8(ecx.wrapping_add(0x2c8u32))) & 0x1u32, 8);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52a8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x2cu32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x2cu32), r);
                pc = 11; continue;
            }
            10 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = edx.wrapping_add(eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52a8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 11; continue;
            }
            11 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5181d0, ecx, &a); }
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                m.w8(edi, ((ebx & 0xff)) as u8);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xeu32)))) & 0xffff);
                m.w16(edi.wrapping_add(0x4u32), ((eax & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x14u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x28u32)))) & 0xff);
                m.w16(esi.wrapping_add(0xeu32), ((ecx & 0xffff)) as u16);
                edx = m.r32(edi.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6dd7a0u32;
                m.w32(edi.wrapping_add(0x18u32), edx);
                m.w8(edi.wrapping_add(0x13au32), ((eax & 0xff)) as u8);
                m.w8(edi.wrapping_add(0x13bu32), ((eax & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x509a50, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let r = u32::from(m.r8(edi.wrapping_add(0x136u32))) | 0x10u32; fl = lift::logic(r, 8);
                m.w8(edi.wrapping_add(0x136u32), (r) as u8);
                pc = 13; continue;
            }
            13 => {
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x673660u32)))) & 0xffff);
                m.w16(edi.wrapping_add(0x40u32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 6; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f7bb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
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

/// `0X00544D4F` custom_map_save_close_file_tail
pub fn f_00544d4f(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    m.w32(ebp.wrapping_add(0xfffffffcu32), 0x3u32);
    ecx = ebp.wrapping_add(0xffffffc8u32);
    m.w8(ebp.wrapping_add(0xfffffffcu32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
    { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x544d79, ecx, &a); }
    eax
}

/// `0X00544D79` custom_map_save_release_buffers_tail
pub fn f_00544d79(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                ecx = ebp.wrapping_add(0xffffffecu32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x2u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                esi = m.r32(ebp.wrapping_add(0xffffffdcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = []; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                ecx = ebp.wrapping_add(0xffffffd8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x54c400, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653b60, ecx, &a); }
                ecx = ebp.wrapping_add(0x8u32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xfffffff4u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00544E50` custom_map_queue_new_game_operations
pub fn f_00544e50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x5c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = ecx;
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(0x6cc3e8u32);
                eax = m.r32(eax);
                fl = lift::sub(eax, 0x4e2u32, 32).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = 0x4e2u32;
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(0x672d7cu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(8));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = eax.wrapping_add(ecx.wrapping_mul(8));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x20u32), 0xb7u32);
                esi = esp.wrapping_add(0x28u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edi = 0x4u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 3; continue;
            }
            3 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w16(esi, ((eax & 0xffff)) as u16);
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6cc3e8u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x10u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w8(esp.wrapping_add(0x30u32), ((ecx & 0xff)) as u8);
                if !fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x20u32)), 0xc1u32, 32).1;
                if !fl.cf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 10; continue;
            }
            7 => {
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 9; continue;
            }
            8 => {
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 9; continue;
            }
            9 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esp.wrapping_add(0x24u32), 0xafu32);
                if (fl.zf || fl.sf != fl.of) { pc = 21; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = ecx;
                eax = ecx.wrapping_add(0x16cu32);
                edi = ecx.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xa4u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, 0x194u32, 32); fl = f;
                ecx = r;
                esi = eax.wrapping_add(ebx);
                ebp = edx.wrapping_add(0x144u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::sub(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebp)))) & 0xff);
                let r = (eax & 0xff) | 0x20u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                m.w32(esp.wrapping_add(0x28u32), eax);
                fl = lift::logic((ecx & 0xff) & 0x40u32, 8);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = (eax & 0xff) | 0x80u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w32(esp.wrapping_add(0x28u32), eax);
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(edi.wrapping_add(0xffffff60u32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                ecx = m.r32(edi);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::shl(eax, 0x16u32, 32, fl); fl = f;
                eax = r;
                let r = ecx | eax; fl = lift::logic(r, 32);
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x30u32), ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                m.w8(esp.wrapping_add(0x34u32), ((edx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                m.w8(esp.wrapping_add(0x35u32), ((ecx & 0xff)) as u8);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w8(esp.wrapping_add(0x36u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x37u32), (0x1u32) as u8);
                if !fl.zf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), 0xc1u32, 32).1;
                if !fl.cf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 20; continue;
            }
            17 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 19; continue;
            }
            18 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 19; continue;
            }
            19 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 20; continue;
            }
            20 => {
                fl = lift::logic(ebx & ebx, 32);
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x24u32), 0xb0u32);
                let (r, f) = lift::add(ecx, 0x1bcu32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), 0x6ddaa8u32);
                m.w32(esp.wrapping_add(0x18u32), 0x8u32);
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub(edx, 0xa8u32, 32).1;
                ebp = m.r32(ecx.wrapping_add(0xfffffff8u32));
                eax = ebp;
                if !fl.cf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esi = m.r32(edx);
                pc = 25; continue;
            }
            24 => {
                esi = m.r32(0x6ddaa0u32);
                pc = 25; continue;
            }
            25 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(eax))), 0x0u32, 8).1;
                if !fl.zf { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 27; continue;
            }
            27 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0xffffffffu32)))) & 0xff);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0xffffffffu32)))) & 0xff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::logic(eax & eax, 32);
                if (!fl.zf && fl.sf == fl.of) { pc = 27; continue; }
                pc = 29; continue;
            }
            29 => {
                pc = 43; continue;
            }
            30 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                edi = m.r32(eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xfffffff8u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x35u32), ((edx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x36u32), ((ecx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x37u32), ((ebx & 0xff)) as u8);
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub(ebx, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                esi = ebx;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(esi)))) & 0xff);
                m.w8(esp.wrapping_add(eax).wrapping_add(0x28u32), ((ecx & 0xff)) as u8);
                pc = 35; continue;
            }
            34 => {
                m.w8(esp.wrapping_add(eax).wrapping_add(0x28u32), (0x0u32) as u8);
                pc = 35; continue;
            }
            35 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0xdu32, 32).1;
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 40; continue; }
                pc = 37; continue;
            }
            37 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), 0xc1u32, 32).1;
                if !fl.cf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 42; continue;
            }
            39 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 41; continue;
            }
            40 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 41; continue;
            }
            41 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                fl = lift::sub(ebp, ebx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 31; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                esi = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = 0x4u32;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, 0xa8u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), esi);
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 44; continue;
            }
            44 => {
                eax = m.r32(0x672d20u32);
                ecx = m.r32(0x6cc3e8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), 0xb1u32);
                m.w16(esp.wrapping_add(0x28u32), (0x0u32) as u16);
                esi = esp.wrapping_add(0x30u32);
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0xau32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                m.w16(esp.wrapping_add(0x2au32), ((edx & 0xffff)) as u16);
                edi = 0x4u32;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                pc = 45; continue;
            }
            45 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w16(esi, ((eax & 0xffff)) as u16);
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 45; continue; }
                pc = 46; continue;
            }
            46 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 50; continue; }
                pc = 47; continue;
            }
            47 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = esp.wrapping_add(0x24u32);
                fl = lift::sub(eax, 0xc1u32, 32).1;
                if !fl.cf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 52; continue;
            }
            49 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 51; continue;
            }
            50 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 51; continue;
            }
            51 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 52; continue;
            }
            52 => {
                eax = m.r32(0x6cc3e8u32);
                m.w32(esp.wrapping_add(0x24u32), 0xacu32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0xfu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x28u32), ((ecx & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x8u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                m.w16(esp.wrapping_add(0x2au32), ((edx & 0xffff)) as u16);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w8(esp.wrapping_add(0x2cu32), (0x2u32) as u8);
                if !fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 55; continue;
            }
            54 => {
                eax = esp.wrapping_add(0x24u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 55; continue;
            }
            55 => {
                m.w32(esp.wrapping_add(0x24u32), 0x7eu32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 56; continue;
            }
            56 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x28u32), eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 56; continue; }
                pc = 57; continue;
            }
            57 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 61; continue; }
                pc = 58; continue;
            }
            58 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = esp.wrapping_add(0x24u32);
                fl = lift::sub(eax, 0xc1u32, 32).1;
                if !fl.cf { pc = 60; continue; }
                pc = 59; continue;
            }
            59 => {
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 63; continue;
            }
            60 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 62; continue;
            }
            61 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 62; continue;
            }
            62 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 63; continue;
            }
            63 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x30u32), 0xadu32);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(0x673944u32);
                eax = edi.wrapping_add(edi);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                fl = lift::sub(ecx, edi, 32).1;
                m.w8(esp.wrapping_add(0x28u32), (0x0u32) as u8);
                if (fl.sf == fl.of) { pc = 66; continue; }
                pc = 65; continue;
            }
            65 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x28u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add((ebx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x28u32), ((ebx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_mul(2).wrapping_add(0x673944u32)))) & 0xff);
                fl = lift::sub(eax, edi, 32).1;
                if (fl.sf != fl.of) { pc = 65; continue; }
                pc = 66; continue;
            }
            66 => {
                ebx = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_mul(2).wrapping_add(0x673944u32)))) & 0xff);
                eax = ebx.wrapping_mul(2).wrapping_add(0x673944u32);
                fl = lift::sub(ecx, edi, 32).1;
                if !fl.zf { pc = 68; continue; }
                pc = 67; continue;
            }
            67 => {
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub(edx, edi, 32).1;
                if fl.zf { pc = 67; continue; }
                pc = 68; continue;
            }
            68 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(esi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add(u32::from(m.r8(esp.wrapping_add(0x28u32))), (edx & 0xff), 8); fl = f;
                m.w8(esp.wrapping_add(0x28u32), (r) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x29u32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(esp.wrapping_add(0x2au32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2au32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                m.w8(esp.wrapping_add(0x2bu32), ((edx & 0xff)) as u8);
                if !fl.zf { pc = 72; continue; }
                pc = 69; continue;
            }
            69 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x2bu32), ((edx & 0xff)) as u8);
                pc = 72; continue;
            }
            71 => {
                m.w8(esp.wrapping_add(0x2bu32), (0x2u32) as u8);
                pc = 72; continue;
            }
            72 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 76; continue; }
                pc = 73; continue;
            }
            73 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), 0xc1u32, 32).1;
                if !fl.cf { pc = 75; continue; }
                pc = 74; continue;
            }
            74 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 78; continue;
            }
            75 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 77; continue;
            }
            76 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 77; continue;
            }
            77 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 78; continue;
            }
            78 => {
                ebp = m.r32(esp.wrapping_add(0x28u32));
                let r = ebp & 0xffu32; fl = lift::logic(r, 32);
                ebp = r;
                pc = 79; continue;
            }
            79 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(esi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add((edx & 0xff), (ebx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x28u32), ((edx & 0xff)) as u8);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.zf { pc = 79; continue; }
                pc = 80; continue;
            }
            80 => {
                edx = edi.wrapping_add(edi).wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x34u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x29u32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(esp.wrapping_add(0x2au32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2au32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                m.w8(esp.wrapping_add(0x2bu32), ((edx & 0xff)) as u8);
                if !fl.zf { pc = 84; continue; }
                pc = 81; continue;
            }
            81 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 83; continue; }
                pc = 82; continue;
            }
            82 => {
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x2bu32), ((edx & 0xff)) as u8);
                pc = 84; continue;
            }
            83 => {
                m.w8(esp.wrapping_add(0x2bu32), (0x2u32) as u8);
                pc = 84; continue;
            }
            84 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 88; continue; }
                pc = 85; continue;
            }
            85 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), 0xc1u32, 32).1;
                if !fl.cf { pc = 87; continue; }
                pc = 86; continue;
            }
            86 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 90; continue;
            }
            87 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 89; continue;
            }
            88 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 89; continue;
            }
            89 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 90; continue;
            }
            90 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x6u32, 32).1;
                if (fl.sf != fl.of) { pc = 64; continue; }
                pc = 91; continue;
            }
            91 => {
                m.w32(esp.wrapping_add(0x24u32), 0xadu32);
                esi = 0xcu32;
                ebp = 0xc1u32;
                pc = 92; continue;
            }
            92 => {
                m.w32(esp.wrapping_add(0x34u32), esi);
                m.w8(esp.wrapping_add(0x29u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x14u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add((edx & 0xff), 0x1eu32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x28u32), ((edx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = eax;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(ecx).wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 96; continue; }
                pc = 93; continue;
            }
            93 => {
                let (r, f) = lift::add((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x32u32, 8).1;
                m.w8(esp.wrapping_add(0x28u32), ((edx & 0xff)) as u8);
                if fl.cf { pc = 95; continue; }
                pc = 94; continue;
            }
            94 => {
                edx = (edx & !0xff) | ((0x1eu32) & 0xff);
                m.w8(esp.wrapping_add(0x28u32), ((edx & 0xff)) as u8);
                pc = 95; continue;
            }
            95 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = eax;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(ecx).wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 93; continue; }
                pc = 96; continue;
            }
            96 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esp.wrapping_add(0x2au32), (0x0u32) as u8);
                m.w8(esp.wrapping_add(0x2bu32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(eax).wrapping_add(0x38u32), (0x1u32) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                if !fl.zf { pc = 100; continue; }
                pc = 97; continue;
            }
            97 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), ebp, 32).1;
                if !fl.cf { pc = 99; continue; }
                pc = 98; continue;
            }
            98 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 102; continue;
            }
            99 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 101; continue;
            }
            100 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 101; continue;
            }
            101 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 102; continue;
            }
            102 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0xfu32, 32).1;
                if (fl.sf != fl.of) { pc = 92; continue; }
                pc = 103; continue;
            }
            103 => {
                eax = m.r32(0x6cc3e8u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), 0xb4u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x13u32)))) & 0xff);
                esi = ecx.wrapping_add(ecx).wrapping_add(0x1u32);
                fl = lift::sub(esi, 0x5u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 105; continue; }
                pc = 104; continue;
            }
            104 => {
                esi = 0x5u32;
                pc = 106; continue;
            }
            105 => {
                fl = lift::sub(esi, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 114; continue; }
                pc = 106; continue;
            }
            106 => {
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w32(esp.wrapping_add(0x28u32), esi);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(0x6cc3e8u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x13u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                if !fl.zf { pc = 110; continue; }
                pc = 107; continue;
            }
            107 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = esp.wrapping_add(0x24u32);
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.cf { pc = 109; continue; }
                pc = 108; continue;
            }
            108 => {
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 112; continue;
            }
            109 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 111; continue;
            }
            110 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 111; continue;
            }
            111 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 112; continue;
            }
            112 => {
                fl = lift::sub(esi, ebx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 106; continue; }
                pc = 113; continue;
            }
            113 => {
                eax = m.r32(0x6cc3e8u32);
                pc = 114; continue;
            }
            114 => {
                m.w32(esp.wrapping_add(0x24u32), 0xb3u32);
                esi = esp.wrapping_add(0x30u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0xbu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x28u32), ((ecx & 0xffff)) as u16);
                edi = 0x4u32;
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0xau32)))) & 0xffff);
                eax = 0x3u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x672d22u32))) & 0xffff);
                m.w16(esp.wrapping_add(0x2au32), ((eax & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                pc = 115; continue;
            }
            115 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w16(esi, ((eax & 0xffff)) as u16);
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 115; continue; }
                pc = 116; continue;
            }
            116 => {
                fl = lift::logic(u32::from(m.r8(0x6df2f0u32)) & 0xcu32, 8);
                if !fl.zf { pc = 120; continue; }
                pc = 117; continue;
            }
            117 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x24u32)), ebp, 32).1;
                if !fl.cf { pc = 119; continue; }
                pc = 118; continue;
            }
            118 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535760, ecx, &a); }
                pc = 122; continue;
            }
            119 => {
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 121; continue;
            }
            120 => {
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 121; continue;
            }
            121 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 122; continue;
            }
            122 => {
                esi = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(0x6cc3e8u32);
                m.w32(esp.wrapping_add(0x24u32), 0xc4u32);
                m.w32(esp.wrapping_add(0x28u32), esi);
                eax = m.r32(ecx);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x514u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 124; continue; }
                pc = 123; continue;
            }
            123 => {
                ecx = eax.wrapping_add(0xfffffaecu32);
                eax = 0x66666667u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 125; continue;
            }
            124 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                pc = 125; continue;
            }
            125 => {
                eax = edx.wrapping_mul(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), ebx);
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                ecx = eax.wrapping_add(esi).wrapping_add(0x4600u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6df2f0u32))) & 0xff);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                ecx = 0x6df2f0u32;
                if !fl.zf { pc = 127; continue; }
                pc = 126; continue;
            }
            126 => {
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 128; continue;
            }
            127 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 128; continue;
            }
            128 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6df2f0u32);
                let r = ecx & 0xcu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 130; continue; }
                pc = 129; continue;
            }
            129 => {
                edx = esp.wrapping_add(0x18u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), 0xb8u32);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 130; continue;
            }
            130 => {
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x5c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00545650` custom_map_generate_default_layout
pub fn f_00545650(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xe0);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = ecx;
                edi = 0x673b51u32;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(esp.wrapping_add(0x10u32), 0x5u32);
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = edi;
                pc = 2; continue;
            }
            2 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xffffffffu32)))) & 0xff);
                edx = esi.wrapping_add(eax);
                let (r, f) = lift::add(ecx, 0xau32, 32); fl = f;
                ecx = r;
                m.w8(esp.wrapping_add(edx).wrapping_add(0x18u32), ((ebx & 0xff)) as u8);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xfffffff6u32)))) & 0xff);
                edx = esi.wrapping_add(eax);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x15u32, 32).1;
                m.w8(esp.wrapping_add(edx).wrapping_add(0x84u32), ((ebx & 0xff)) as u8);
                if fl.cf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(esi, 0x15u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esp.wrapping_add(0xf4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(ebp, edi);
                esi = 0x10000u32;
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(ebx, 0x12u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x545874u32)))) & 0xff);
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x545864u32)) { 0x5456c5 => { pc = 7; continue; } 0x5456c8 => { pc = 8; continue; } 0x5456d2 => { pc = 9; continue; } 0x5456dc => { pc = 10; continue; } _ => unreachable!() }
            }
            7 => {
                let r = m.r32(ebp.wrapping_add(0x44u32)) | esi; fl = lift::logic(r, 32);
                m.w32(ebp.wrapping_add(0x44u32), r);
                pc = 8; continue;
            }
            8 => {
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x144u32), (0xe0u32) as u8);
                pc = 11; continue;
            }
            9 => {
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x144u32), (0xa0u32) as u8);
                pc = 11; continue;
            }
            10 => {
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x144u32), (0x20u32) as u8);
                pc = 11; continue;
            }
            11 => {
                eax = ebx;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x14u32;
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x16cu32), ((ebx & 0xff)) as u8);
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = ebp.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x1bcu32);
                m.w8(ebx.wrapping_add(ebp).wrapping_add(0x194u32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x28u32, 32).1;
                if fl.cf { pc = 5; continue; }
                pc = 12; continue;
            }
            12 => {
                esi = 0x1u32;
                ecx = edi;
                let (r, f) = lift::shl(esi, (ecx & 0xff), 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w32(esp.wrapping_add(0x10u32), esi);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(edi); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(esi & esi, 32);
                edi = edx;
                if fl.zf { pc = 29; continue; }
                pc = 13; continue;
            }
            13 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x15u32;
                ebx = m.r32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x4u32));
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0x16cu32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_mul(4).wrapping_add(0x6ddbc0u32)))) & 0xff);
                esi = ecx;
                pc = 14; continue;
            }
            14 => {
                ecx = edx.wrapping_add(0xfffffffdu32);
                eax = 0x1u32;
                let (r, f) = lift::shl(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                fl = lift::logic(eax & ebx, 32);
                if !fl.zf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::logic(m.r32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xa4u32)) & eax, 32);
                if !fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(edx);
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(ecx).wrapping_add(0x18u32))), 0x0u32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 21; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(eax).wrapping_add(0x84u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if (!fl.cf && !fl.zf) { pc = 22; continue; }
                pc = 18; continue;
            }
            18 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edx, 0x15u32, 32).1;
                if fl.cf { pc = 14; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 23; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w32(esp.wrapping_add(0x14u32), 0x1u32);
                edx = 0x4u32;
                pc = 14; continue;
            }
            21 => {
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = edx.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(eax).wrapping_add(0x18u32)))) & 0xff);
                let (r, f) = lift::sub((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                eax = esp.wrapping_add(eax).wrapping_add(0x18u32);
                m.w8(eax, ((ecx & 0xff)) as u8);
                ecx = edx.wrapping_add(0xfffffffdu32);
                edx = 0x1u32;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                let r = edx | ebx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x4u32), edx);
                pc = 25; continue;
            }
            22 => {
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = edx.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edx.wrapping_add(0xfffffffdu32);
                edx = 0x1u32;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(eax).wrapping_add(0x84u32)))) & 0xff);
                eax = esp.wrapping_add(eax).wrapping_add(0x84u32);
                let (r, f) = lift::sub((ebx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                m.w8(eax, ((ebx & 0xff)) as u8);
                eax = m.r32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xa4u32));
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                m.w32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xa4u32), eax);
                pc = 25; continue;
            }
            23 => {
                eax = 0x1u32;
                ecx = edi;
                let (r, f) = lift::shl(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & ecx, 32);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 25; continue;
            }
            25 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.cf || fl.zf) { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 28; continue;
            }
            27 => {
                eax = m.r32(esp.wrapping_add(0xf4u32));
                edi = eax.wrapping_add(0xffffffffu32);
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 13; continue; }
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
                esp = esp.wrapping_add(0xe0);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00547534` operations_save_game_finish_main_file
pub fn f_00547534(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
    ecx = ebp.wrapping_add(0xffffffc8u32);
    m.w8(ebp.wrapping_add(0xfffffffcu32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
    { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x547566, ecx, &a); }
    eax
}

/// `0X0054762F` operations_save_game_finish_second_file
pub fn f_0054762f(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
    ecx = ebp.wrapping_add(0xffffffc8u32);
    m.w8(ebp.wrapping_add(0xfffffffcu32), (0xdu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
    { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x547650, ecx, &a); }
    eax
}

/// `0X00547650` operations_save_game_cleanup
pub fn f_00547650(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, edi);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 4; continue;
            }
            1 => {
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                ecx = ebp.wrapping_add(0xffffffc8u32);
                { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
                pc = 4; continue;
            }
            4 => {
                ecx = ebp.wrapping_add(0xffffffb8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653b60, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653b60, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffe4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x54c400, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffe8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffecu32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00547F10` operations_host_start_new_game
pub fn f_00547f10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x2cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esi.wrapping_add(0x10u32), 0x0u32);
                { let t = m.r32(0x66a574u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639bf6, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xcu32));
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x544e50, ecx, &a); }
                edi = m.r32(esi.wrapping_add(0xcu32));
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x549c40, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                m.w32(esi.wrapping_add(0xcu32), 0x0u32);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6cc3e8u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x4u32), 0xa9u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x9u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xau32)))) & 0xff);
                ecx = 0x3u32;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xeu32)))) & 0xff);
                eax = esp.wrapping_add(0x4u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00547FD0` operations_send_lobby_profile
pub fn f_00547fd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, edi);
                edi = ecx;
                fl = lift::sub(m.r32(edi), 0x2u32, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(0x6cbb94u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), 0xa4u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 3; continue;
            }
            3 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx).wrapping_add(0x204u32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = 0x1u32;
                let (r, f) = lift::shl(esi, (ecx & 0xff), 32, fl); fl = f;
                esi = r;
                let r = eax | esi; fl = lift::logic(r, 32);
                eax = r;
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 6; continue;
            }
            6 => {
                esi = 0x34u32;
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), (esi & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                if fl.cf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), (esi & 0xffff), 16).1;
                if !fl.cf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            9 => {
                esi = 0x34u32;
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(0x6cbb94u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x20cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x10u32), 0xa8u32);
                ecx = m.r32(eax.wrapping_add(0x4cb0u32));
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), (esi & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4cb0u32));
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x1d4u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                if fl.cf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), (esi & 0xffff), 16).1;
                if !fl.cf { pc = 12; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbb94u32);
                pc = 14; continue;
            }
            14 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2cu32)))) & 0xff);
                m.w32(esp.wrapping_add(0x10u32), 0xa2u32);
                m.w8(esp.wrapping_add(0x14u32), ((edx & 0xff)) as u8);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x1c4u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x15u32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x1ccu32)))) & 0xff);
                m.w8(esp.wrapping_add(0x16u32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x1fcu32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                let r = u32::from(m.r8(esp.wrapping_add(0x15u32))) | 0x80u32; fl = lift::logic(r, 8);
                m.w8(esp.wrapping_add(0x15u32), (r) as u8);
                pc = 16; continue;
            }
            16 => {
                ebx = eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x164u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                eax = m.r32(ebx);
                ebp = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w8(esp.wrapping_add(0x17u32), (0x0u32) as u8);
                pc = 17; continue;
            }
            17 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(esi, ebp, 32).1;
                if !fl.cf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(ebx);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ecx)))) & 0xff);
                m.w8(esp.wrapping_add(eax).wrapping_add(0x18u32), ((edx & 0xff)) as u8);
                pc = 21; continue;
            }
            20 => {
                m.w8(esp.wrapping_add(eax).wrapping_add(0x18u32), (0x0u32) as u8);
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0xcu32, 32).1;
                if fl.cf { pc = 18; continue; }
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                if fl.cf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                if !fl.cf { pc = 23; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x17u32)))) & 0xff);
                let (r, f) = lift::add((ecx & 0xff), 0xcu32, 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub(esi, ebp, 32).1;
                m.w8(esp.wrapping_add(0x17u32), ((ecx & 0xff)) as u8);
                if (fl.cf || fl.zf) { pc = 17; continue; }
                pc = 25; continue;
            }
            25 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005482D0` operations_receive_chat_message_chunk
pub fn f_005482d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = ecx;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x8u32, 8).1;
                if !fl.cf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = m.r32(ebx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x8e0u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1u32)))) & 0xff);
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x6ddaa0u32;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1u32)))) & 0xff);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c390cu32);
                eax = ebx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 4; continue;
            }
            4 => {
                ebp = 0xfffffffeu32;
                edi = esi.wrapping_add(0x2u32);
                let (r, f) = lift::sub(ebp, esi, 32); fl = f;
                ebp = r;
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = ebx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x8e0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                edx = edi.wrapping_add(ebp);
                fl = lift::sub(edx, 0xeu32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                edx = ebx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb40u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42b4c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = ebx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x8e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
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

/// `0X00549130` operations_send_leave_game
pub fn f_00549130(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::logic(u32::from(m.r8(esi)) & 0xcu32, 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = 0x6e55d0u32;
                { let a = []; eax = h.call_at(esp, m, 0x611fa0, ecx, &a); }
                ecx = m.r32(0x6dfc14u32);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x8u32), 0xc7u32);
                m.w32(esp.wrapping_add(0xcu32), eax);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1f4u32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00549B20` save_last_won_close_file_and_cleanup
pub fn f_00549b20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    m.w32(ebp.wrapping_add(0xfffffffcu32), 0x1u32);
    ecx = ebp.wrapping_add(0xffffffccu32);
    m.w8(ebp.wrapping_add(0xfffffffcu32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
    { let a = []; return h.call_at(esp, m, 0x549b3e, ecx, &a); }
    eax
}

/// `0X00549B3E` save_last_won_free_buffers_and_return
pub fn f_00549b3e(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, edi);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(ebp.wrapping_add(0xffffffe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 6; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 4; continue;
            }
            2 => {
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 4; continue;
            }
            4 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 5; continue;
            }
            5 => {
                ecx = ebp.wrapping_add(0xffffffccu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
                pc = 6; continue;
            }
            6 => {
                ecx = ebp.wrapping_add(0xffffffecu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x54c400, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffccu32);
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054A860` network_session_cleanup
pub fn f_0054a860(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(esi.wrapping_add(0x10u32)), ebx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x2cu32)), ebx, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2f8u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                ecx = esi.wrapping_add(0x89cu32);
                m.w32(esi.wrapping_add(0x10u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                ecx = esi.wrapping_add(0x8b8u32);
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x4u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x2cu32));
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(0x66a2fcu32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                eax = ecx.wrapping_add(edi);
                ecx = m.r32(ecx.wrapping_add(edi).wrapping_add(0x10u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(edi, 0x24u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0x120u32, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 11; continue;
            }
            11 => {
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            12 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.cf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2fcu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x44u32), ebx);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x3cu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x3cu32), ebx);
                { let t = m.r32(0x66a2f4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 17; continue;
            }
            17 => {
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x2cu32), 0xffffffffu32);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi, ebx);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x14u32), ebx);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x18u32), ebx);
                pc = 21; continue;
            }
            21 => {
                edi = m.r32(esi.wrapping_add(0xcu32));
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x549c40, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xcu32), ebx);
                pc = 23; continue;
            }
            23 => {
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

/// `0X0054A980` network_session_thread_proc
pub fn f_0054a980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::logic(u32::from(m.r8(esi)) & 0xcu32, 8);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(0x66a300u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(0x66a2bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x2cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54b500, ecx, &a); }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54b670, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54b960, ecx, &a); }
                edi = eax;
                pc = 6; continue;
            }
            6 => {
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic(u32::from(m.r8(esi)) & 0xcu32, 8);
                if !fl.zf { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54a860, ecx, &a); }
                m.w32(esi.wrapping_add(0x38u32), 0x0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                fl = lift::sub(eax, 0xfffffffdu32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54b080, ecx, &a); }
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), 0x0u32, 16).1;
                edi = eax;
                if (!fl.cf && !fl.zf) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(0x6dccf8u32);
                edx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0xbb8u32, 32).1;
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54afa0, ecx, &a); }
                pc = 6; continue;
            }
            15 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.cf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54ae50, ecx, &a); }
                pc = 7; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(0x6dccf8u32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = ecx.wrapping_add(0xffffec78u32);
                fl = lift::sub(edx, eax, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(esi.wrapping_add(0x3cu32));
                eax = esi.wrapping_add(0x4u32);
                m.w32(esi.wrapping_add(0x30u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4893u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1770u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w32(esi, 0x0u32);
                pc = 8; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054AA70` enqueue_operation
pub fn f_0054aa70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::logic(u32::from(m.r8(esi)) & 0xcu32, 8);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                edi = m.r32(0x66a2bcu32);
                if fl.cf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = edi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                m.w32(esi.wrapping_add(0x470u32), 0x1u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esi.wrapping_add(0x470u32), 0x0u32);
                { let t = edi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                ecx = esi.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x470u32), 0x0u32);
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

/// `0X0054AAE0` start_lan_game_search
pub fn f_0054aae0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x38u32));
                m.w32(esi, 0x0u32);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x2cu32), 0xffffffffu32);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(0x66a2bcu32);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                { let t = edi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x38u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x10u32), 0x0u32);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x4u32), 0x0u32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1770u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x8u32), eax);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1f4u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                { let t = m.r32(0x66a308u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 8; continue;
            }
            8 => {
                edi = esi.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a304u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = esi.wrapping_add(0x89cu32);
                { let a = []; eax = h.call_at(esp, m, 0x543950, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2f4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi, 0x0u32);
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1f4u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x54a980u32);
                m.w32(esi, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x654296, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x38u32), eax);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2f4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi, 0x0u32);
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esi, 0x0u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1f4u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            13 => {
                eax = 0x1u32;
                pc = 14; continue;
            }
            14 => {
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

/// `0X0054AE50` connect_to_lan_host
pub fn f_0054ae50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x663a23u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x44u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x8u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x544140, ecx, &a); }
                pc = 5; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 5; continue;
            }
            5 => {
                let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esi.wrapping_add(0x44u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x14u32), edi);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0x2cu32), edi);
                m.w32(esi, eax);
                m.w32(esi.wrapping_add(0x34u32), 0x2u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a318u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w32(esi.wrapping_add(0x2cu32), edi);
                m.w32(esi, 0x0u32);
                m.w32(esi.wrapping_add(0x34u32), 0x3u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x28u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x24u32));
                eax = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a314u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x44u32));
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi, 0x4u32);
                { let t = m.r32(0x66a310u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_add(0x14);
                m.w32(eax.wrapping_add(0x1cu32), 0x0u32);
                m.w32(eax.wrapping_add(0x20u32), 0x4u32);
                ecx = m.r32(esi.wrapping_add(0x44u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ecx.wrapping_add(0x14u32), 0x0u32);
                m.w32(esi.wrapping_add(0x2cu32), 0xfffffffdu32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esi.wrapping_add(0x2cu32), edi);
                m.w32(esi.wrapping_add(0x34u32), 0x4u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054AFA0` send_pending_operations_to_host
pub fn f_0054afa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x470u32));
                m.w32(esi.wrapping_add(0x46cu32), 0x1u32);
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6dccf8u32);
                ebx = m.r32(0x66a2f0u32);
                m.w32(esi.wrapping_add(0x30u32), eax);
                ecx = m.r32(0x6de4b4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x50u32), ecx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x56u32)))) & 0xffff);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edi = esi.wrapping_add(0x48u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = edx.wrapping_mul(4).wrapping_add(0x10u32);
                edx = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, 0x320u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, 0x3e7u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x44u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w16(esi.wrapping_add(0x56u32), ((ebp & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0x46cu32), ebp);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                m.w32(esi, ebp);
                m.w32(esi.wrapping_add(0x4u32), ebp);
                m.w32(esi.wrapping_add(0x2cu32), 0xffffffffu32);
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x46cu32), ebp);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1u32;
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

/// `0X0054B080` receive_operations_from_host
pub fn f_0054b080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x42c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                edi = esp.wrapping_add(0x28u32);
                ebx = 0x34u32;
                pc = 1; continue;
            }
            1 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let (r, f) = lift::add(edi, 0x14u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x420u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x20u32), edi);
                { let t = m.r32(0x66a31cu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = 0x1u32;
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 20; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(ecx.wrapping_add(0x14u32), edx);
                eax = m.r32(esp.wrapping_add(0x24u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x2cu32), eax);
                eax = m.r32(esp.wrapping_add(0x26u32));
                fl = lift::logic(((eax >> 8) & 0xff) & ((eax >> 8) & 0xff), 8);
                if !fl.sf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0xcu32);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x544300, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esi, edi);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x42c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0x18u32)), 0x3u32, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w32(esp.wrapping_add(0xcu32), ebx);
                pc = 22; continue;
            }
            8 => {
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 19; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x482u32))), 0x34u32, 16).1;
                if !fl.cf { pc = 17; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x894u32));
                m.w32(esi.wrapping_add(0x898u32), ebx);
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi.wrapping_add(0x8a0u32));
                eax = m.r32(esi.wrapping_add(0x8a8u32));
                ecx = esi.wrapping_add(0x89cu32);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(ecx.wrapping_add(0x8u32));
                edx = m.r32(ecx.wrapping_add(0x10u32));
                fl = lift::sub(eax, edx, 32).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = esi.wrapping_add(0x474u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543c70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x474u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x26u32))), (edi & 0xffff), 16).1;
                m.w32(esi.wrapping_add(0x898u32), edi);
                if (fl.cf || fl.zf) { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 18; continue;
            }
            17 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 18; continue;
            }
            18 => {
                ecx = esi.wrapping_add(0x89cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x26u32))), 0x34u32, 16).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                pc = 22; continue;
            }
            20 => {
                fl = lift::sub(eax, 0x320u32, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi, edi);
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x8a0u32));
                eax = m.r32(esi.wrapping_add(0x8a8u32));
                ecx = esi.wrapping_add(0x89cu32);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(ecx.wrapping_add(0x8u32));
                edx = m.r32(ecx.wrapping_add(0x10u32));
                fl = lift::sub(eax, edx, 32).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if fl.zf { pc = 28; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0x18u32)), 0x4u32, 32).1;
                if !fl.cf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esi.wrapping_add(0x894u32));
                m.w32(esi.wrapping_add(0x898u32), ebx);
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = esi.wrapping_add(0x474u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543c70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                m.w32(esi.wrapping_add(0x898u32), edi);
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x42c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054B230` start_game_session
pub fn f_0054b230(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x663a38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x38u32));
                m.w32(esi, ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(0x66a2bcu32);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                { let t = edi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0x38u32)), ebx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x78u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x8u32), eax);
                if fl.zf { pc = 42; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6cc3e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4cu32);
                let (r, f) = lift::add(eax, 0x14u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                edx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_add(0xc);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 7; continue;
            }
            7 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edi = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w8(edi.wrapping_add(eax).wrapping_add(0x68u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x10u32, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 9; continue;
            }
            9 => {
                pc = 11; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                m.w8(edx.wrapping_add(eax).wrapping_add(0x68u32), (0x0u32) as u8);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                edi = 0x1u32;
                m.w8(eax.wrapping_add(0x77u32), (0x0u32) as u8);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w16(ecx.wrapping_add(0x62u32), ((edi & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x8u32));
                m.w16(edx.wrapping_add(0x60u32), (0x8u32) as u16);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                fl = lift::sub(ecx, ebx, 32).1;
                ecx = m.r32(esi.wrapping_add(0x8u32));
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                m.w16(ecx.wrapping_add(0x64u32), ((eax & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                fl = lift::sub(eax, ebx, 32).1;
                m.w16(edx.wrapping_add(0x66u32), (0x85u32) as u16);
                if !fl.zf { pc = 16; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esi.wrapping_add(0x2cu32), ebx);
                fl = lift::sub(eax, edi, 32).1;
                if (!fl.cf && !fl.zf) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w32(esi, edi);
                eax = edi;
                pc = 43; continue;
            }
            15 => {
                m.w32(esi, 0x2u32);
                eax = edi;
                pc = 43; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x120u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = eax;
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                fl = lift::sub(ebp, ebx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                if fl.zf { pc = 22; continue; }
                pc = 19; continue;
            }
            19 => {
                edi = ebp;
                ebx = 0x8u32;
                pc = 20; continue;
            }
            20 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x544140, ecx, &a); }
                let (r, f) = lift::add(edi, 0x24u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 20; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = ebp;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = 0x1u32;
                pc = 23; continue;
            }
            22 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x44u32), eax);
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                pc = 42; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 26; continue;
            }
            26 => {
                ebp = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(ecx, edx, 32).1;
                m.w32(eax.wrapping_add(ebp).wrapping_add(0x10u32), ebx);
                ebp = m.r32(esi.wrapping_add(0x44u32));
                m.w32(eax.wrapping_add(ebp).wrapping_add(0x14u32), ebx);
                ebp = m.r32(esi.wrapping_add(0x44u32));
                if !fl.cf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                m.w32(eax.wrapping_add(ebp).wrapping_add(0x18u32), edi);
                pc = 29; continue;
            }
            28 => {
                m.w32(eax.wrapping_add(ebp).wrapping_add(0x18u32), ebx);
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::add(eax, 0x24u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(eax, 0x120u32, 32).1;
                if (fl.sf != fl.of) { pc = 26; continue; }
                pc = 30; continue;
            }
            30 => {
                { let t = m.r32(0x66a308u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 43; continue; }
                pc = 31; continue;
            }
            31 => {
                edi = esi.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(0x66a304u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(ebp, 0x1u32, 32).1;
                if fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = ebp;
                pc = 43; continue;
            }
            33 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4893u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4890u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                m.w16(ecx.wrapping_add(0x60u32), (0x8u32) as u16);
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = m.r32(edi);
                let (r, f) = lift::add(edx, 0x14u32, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a324u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(ebp, 0x1u32, 32).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2f4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi, ebx);
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = ebp;
                pc = 43; continue;
            }
            35 => {
                ebx = esi.wrapping_add(0x89cu32);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x543950, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 41; continue; }
                pc = 36; continue;
            }
            36 => {
                ebp = esi.wrapping_add(0x8b8u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x543950, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 40; continue; }
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(edi);
                m.w32(esi.wrapping_add(0x2cu32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x10u32), 0x1u32);
                { let t = m.r32(0x66a320u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x4);
                let r = edx | 0x4u32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x54a980u32);
                m.w32(esi, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x654296, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x38u32), eax);
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = 0x1u32;
                pc = 43; continue;
            }
            39 => {
                eax = m.r32(esi);
                ecx = ebp;
                let r = (eax & 0xff) & 0xfbu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w32(esi.wrapping_add(0x2cu32), 0xffffffffu32);
                m.w32(esi, eax);
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                pc = 40; continue;
            }
            40 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                pc = 41; continue;
            }
            41 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2f4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi, 0x0u32);
                { let t = m.r32(0x66a334u32); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 42; continue;
            }
            42 => {
                eax = 0x1f4u32;
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X0054B500` host_accept_new_client
pub fn f_0054b500(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                eax = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = m.r32(esi.wrapping_add(0x44u32));
                ebp = eax;
                m.w32(esp.wrapping_add(0x10u32), eax);
                edi = 0xffffu32;
                edx = ebx.wrapping_add(0x3cu32);
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(edx);
                fl = lift::sub(ecx, 0x1u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(edi, 0x8u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = eax;
                pc = 6; continue;
            }
            4 => {
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x24u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, 0x8u32, 32).1;
                if fl.cf { pc = 1; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(m.r32(ebx.wrapping_add(0x18u32)), 0x1u32, 32).1;
                if (fl.cf || fl.zf) { pc = 18; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54c280, ecx, &a); }
                fl = lift::sub(edi, 0x8u32, 32).1;
                if !fl.cf { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                edi = edi.wrapping_add(edi.wrapping_mul(8));
                let (r, f) = lift::shl(edi, 0x2u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a318u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x44u32));
                eax = edx.wrapping_add(edi);
                edx = m.r32(esi.wrapping_add(0x3cu32));
                ecx = eax.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a32cu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10000u32);
                m.w32(eax.wrapping_add(edi).wrapping_add(0x14u32), ebx);
                ecx = m.r32(esi.wrapping_add(0x44u32));
                edx = m.r32(esi.wrapping_add(0x3cu32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a310u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x44u32));
                m.w32(eax.wrapping_add(edi).wrapping_add(0x1cu32), ebx);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x14);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(eax.wrapping_add(0x20u32), 0x4u32);
                eax = m.r32(esi.wrapping_add(0x44u32));
                m.w32(eax.wrapping_add(edi).wrapping_add(0x18u32), 0x2u32);
                eax = m.r32(0x6cbb94u32);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c3a08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x440310, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x62u32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x60u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esp.wrapping_add(0x10u32))), 16).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w16(eax.wrapping_add(0x62u32), ((ebp & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x10u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                m.w16(edx.wrapping_add(0x60u32), ((eax & 0xffff)) as u16);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                edx = m.r32(esi.wrapping_add(0x3cu32));
                let (r, f) = lift::add(ecx, 0x14u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a328u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0xc);
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
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            18 => {
                fl = lift::sub(edi, 0x8u32, 32).1;
                if fl.cf { pc = 9; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 9; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054B960` host_broadcast_operations
pub fn f_0054b960(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x448);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                edi = esp.wrapping_add(0x48u32);
                ebx = 0x34u32;
                pc = 1; continue;
            }
            1 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let (r, f) = lift::add(edi, 0x14u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x8bcu32));
                edx = m.r32(esi.wrapping_add(0x8c4u32));
                ecx = esi.wrapping_add(0x8b8u32);
                fl = lift::sub(eax, edx, 32).1;
                m.w16(esp.wrapping_add(0x46u32), ((ebp & 0xffff)) as u16);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx.wrapping_add(0x8u32));
                edi = m.r32(ecx.wrapping_add(0x10u32));
                fl = lift::sub(edx, edi, 32).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = esp.wrapping_add(0x38u32);
                m.w32(esp.wrapping_add(0x18u32), 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543c70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            5 => {
                eax = m.r32(0x6dccf8u32);
                edi = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                ecx = 0x3e8u32;
                fl = lift::sub(ecx, eax, 32).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(0x6dccf8u32);
                m.w32(esi.wrapping_add(0x30u32), edx);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x46u32));
                m.w16(esp.wrapping_add(0x44u32), (0x1u32) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = eax.wrapping_mul(4).wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                edi = m.r32(esi.wrapping_add(0x44u32));
                edx = eax;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                ecx = edx.wrapping_add(edx.wrapping_mul(8));
                ecx = edi.wrapping_add(ecx.wrapping_mul(4));
                edi = m.r32(ecx.wrapping_add(0x18u32));
                fl = lift::sub(edi, 0x3u32, 32).1;
                if fl.cf { pc = 37; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(edi, 0x4u32, 32).1;
                if fl.cf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x544160, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = m.r32(esp.wrapping_add(0x44u32));
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x14u32), 0x1u32);
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x18u32)), ebp, 32).1;
                if fl.zf { pc = 37; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(8));
                edi = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32));
                eax = ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(eax, edi);
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = eax;
                edi = 0x5u32;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(8));
                ecx = m.r32(esi.wrapping_add(0x44u32));
                edx = m.r32(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x40u32), edx);
                pc = 15; continue;
            }
            14 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                pc = 15; continue;
            }
            15 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                edx = eax.wrapping_add(eax.wrapping_mul(8));
                eax = m.r32(esi.wrapping_add(0x44u32));
                ecx = eax.wrapping_add(edx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2f0u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, 0x320u32, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(eax, 0x3e7u32, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 37; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(edi, ebp, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 14; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 37; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esi.wrapping_add(0x44u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(8));
                eax = edx.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2fcu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_add(0x8);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(8));
                eax = m.r32(esi.wrapping_add(0x44u32));
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32), ebp);
                eax = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esi.wrapping_add(0x44u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(8));
                m.w32(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x14u32), ebp);
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esi.wrapping_add(0x44u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(8));
                m.w32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x18u32), 0x1u32);
                ecx = m.r32(0x6cbb94u32);
                { let a = []; eax = h.call_at(esp, m, 0x4402f0, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x44u32)))) & 0xff);
                edx = (edx & !0xff) | ((0x1u32) & 0xff);
                let (r, f) = lift::shl((edx & 0xff), (ecx & 0xff), 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = esp.wrapping_add(0x1cu32);
                edx = (edx & !0xff) | ((!(edx & 0xff)) & 0xff);
                let r = (eax & 0xff) & (edx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x30u32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x30u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let r = edi & 0xffu32; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x1cu32), 0xa4u32);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w32(esp.wrapping_add(0x20u32), edi);
                m.w32(esp.wrapping_add(0x24u32), ebp);
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                ebx = 0x34u32;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebx & 0xffff), 16).1;
                if fl.cf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 22; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                m.w32(esi.wrapping_add(0x470u32), 0x1u32);
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esi.wrapping_add(0x470u32), ebp);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 23; continue;
            }
            25 => {
                ebx = esi.wrapping_add(0x48u32);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x44u32)))) & 0xff);
                edx = (edx & !0xff) | ((0x1u32) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                m.w32(esi.wrapping_add(0x470u32), ebp);
                let (r, f) = lift::shl((edx & 0xff), (ecx & 0xff), 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w32(esp.wrapping_add(0x1cu32), 0xa4u32);
                m.w32(esp.wrapping_add(0x24u32), ebp);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let r = edx | edi; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w32(esp.wrapping_add(0x20u32), edx);
                if fl.zf { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                edi = 0x34u32;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (edi & 0xffff), 16).1;
                if fl.cf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (edi & 0xffff), 16).1;
                if !fl.cf { pc = 27; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                edi = 0x1u32;
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esi.wrapping_add(0x470u32), edi);
                if fl.zf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esi.wrapping_add(0x470u32), ebp);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 28; continue;
            }
            30 => {
                edi = 0x1u32;
                pc = 31; continue;
            }
            31 => {
                eax = esp.wrapping_add(0x1cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0x470u32), ebp);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w32(esp.wrapping_add(0x1cu32), 0xd3u32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                if fl.zf { pc = 36; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                if fl.cf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), 0x34u32, 16).1;
                if !fl.cf { pc = 33; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                m.w32(esi.wrapping_add(0x470u32), edi);
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esi.wrapping_add(0x470u32), ebp);
                { let t = m.r32(0x66a2bcu32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            36 => {
                edx = esp.wrapping_add(0x1cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x470u32), ebp);
                pc = 37; continue;
            }
            37 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x44u32)))) & 0xffff);
                let (r, f) = lift::add((eax & 0xffff), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xffff) | ((r) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x8u32, 16).1;
                m.w16(esp.wrapping_add(0x44u32), ((eax & 0xffff)) as u16);
                if fl.cf { pc = 8; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(esi.wrapping_add(0x894u32));
                m.w32(esi.wrapping_add(0x898u32), 0x1u32);
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.zf { pc = 44; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = m.r32(esi.wrapping_add(0x8a0u32));
                edx = m.r32(esi.wrapping_add(0x8a8u32));
                ecx = esi.wrapping_add(0x89cu32);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(ecx.wrapping_add(0x8u32));
                edi = m.r32(ecx.wrapping_add(0x10u32));
                fl = lift::sub(edx, edi, 32).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = esi.wrapping_add(0x474u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543c70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x482u32))), 0x34u32, 16).1;
                if !fl.cf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = esp.wrapping_add(0x38u32);
                ecx = esi.wrapping_add(0x474u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 44; continue;
            }
            44 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x46u32))), (ebp & 0xffff), 16).1;
                m.w32(esi.wrapping_add(0x898u32), ebp);
                if (fl.cf || fl.zf) { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x89cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 46; continue;
            }
            46 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x448);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0054BD80` release_snapshot_buffers_and_stop_listening
pub fn f_0054bd80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x14u32), edi);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x18u32), edi);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x1cu32), edi);
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esi.wrapping_add(0x20u32), edi);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a2f8u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x10u32), edi);
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

/// `0X0054BDD0` close_client_slot
pub fn f_0054bdd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(m.r32(esi.wrapping_add(0x2cu32)), ebx, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, 0x8u32, 32).1;
                if !fl.cf { pc = 14; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = 0x1u32;
                fl = lift::sub(eax, ebp, 32).1;
                if fl.cf { pc = 14; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = eax.wrapping_add(eax.wrapping_mul(8));
                eax = m.r32(esi.wrapping_add(0x44u32));
                let (r, f) = lift::shl(edi, 0x2u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(m.r32(eax.wrapping_add(0x10u32)), ebx, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x18u32)), ebp, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(eax.wrapping_add(0x18u32), ebx);
                pc = 7; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2fcu32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x44u32));
                esp = esp.wrapping_add(0x8);
                m.w32(edx.wrapping_add(edi).wrapping_add(0x10u32), ebx);
                eax = m.r32(esi.wrapping_add(0x44u32));
                m.w32(eax.wrapping_add(edi).wrapping_add(0x14u32), ebx);
                ecx = m.r32(esi.wrapping_add(0x44u32));
                m.w32(ecx.wrapping_add(edi).wrapping_add(0x18u32), ebx);
                eax = m.r32(esi.wrapping_add(0x44u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(eax.wrapping_add(0x1cu32), ebx);
                m.w32(eax.wrapping_add(0x20u32), 0x4u32);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w16(eax.wrapping_add(0x62u32), ((ebp & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = 0x24u32;
                m.w16(edx.wrapping_add(0x60u32), ((ebp & 0xffff)) as u16);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(ecx.wrapping_add(eax).wrapping_add(0x18u32)), ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(u32::from(m.r16(ecx.wrapping_add(0x60u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(ecx.wrapping_add(0x60u32), (r) as u16);
                edx = m.r32(esi.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(edx.wrapping_add(eax).wrapping_add(0x18u32)), ebp, 32).1;
                if (fl.cf || fl.zf) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(u32::from(m.r16(ecx.wrapping_add(0x62u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(ecx.wrapping_add(0x62u32), (r) as u16);
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(eax, 0x24u32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x120u32, 32).1;
                if fl.cf { pc = 9; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x3cu32));
                let (r, f) = lift::add(eax, 0x14u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a328u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 14; continue;
            }
            14 => {
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

/// `0X0054BEB0` open_client_slot
pub fn f_0054beb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                if !fl.zf { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::sub(eax, 0x8u32, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 2; continue;
            }
            2 => {
                esi = 0x1u32;
                fl = lift::sub(eax, esi, 32).1;
                if fl.cf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx.wrapping_add(0x44u32));
                eax = eax.wrapping_add(eax.wrapping_mul(8));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                ebx = m.r32(edx.wrapping_add(eax).wrapping_add(0x18u32));
                edx = edx.wrapping_add(eax).wrapping_add(0x18u32);
                fl = lift::logic(ebx & ebx, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(edx, esi);
                edx = m.r32(ecx.wrapping_add(0x44u32));
                m.w32(edx.wrapping_add(eax).wrapping_add(0x10u32), 0x0u32);
                edx = m.r32(ecx.wrapping_add(0x44u32));
                m.w32(edx.wrapping_add(eax).wrapping_add(0x14u32), 0x0u32);
                edx = m.r32(ecx.wrapping_add(0x44u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(edx.wrapping_add(0x1cu32), 0x0u32);
                m.w32(edx.wrapping_add(0x20u32), 0x4u32);
                eax = m.r32(ecx.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w16(eax.wrapping_add(0x62u32), ((esi & 0xffff)) as u16);
                eax = m.r32(ecx.wrapping_add(0x8u32));
                m.w16(eax.wrapping_add(0x60u32), ((esi & 0xffff)) as u16);
                eax = 0x24u32;
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(ecx.wrapping_add(0x44u32));
                ebx = m.r32(edx.wrapping_add(eax).wrapping_add(0x18u32));
                fl = lift::logic(ebx & ebx, 32);
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::add(u32::from(m.r16(edx.wrapping_add(0x60u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edx.wrapping_add(0x60u32), (r) as u16);
                edx = m.r32(ecx.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(edx.wrapping_add(eax).wrapping_add(0x18u32)), esi, 32).1;
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::add(u32::from(m.r16(edx.wrapping_add(0x62u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edx.wrapping_add(0x62u32), (r) as u16);
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(eax, 0x24u32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x120u32, 32).1;
                if fl.cf { pc = 6; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(ecx.wrapping_add(0x3cu32));
                let (r, f) = lift::add(eax, 0x14u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a328u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0xc);
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

/// `0X0054BF80` send_town_setup_to_client
pub fn f_0054bf80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x430);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x14u32), ebp);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esi = esp.wrapping_add(0x30u32);
                ebx = 0x34u32;
                pc = 1; continue;
            }
            1 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let (r, f) = lift::add(esi, 0x14u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x444u32));
                ecx = m.r32(ebp.wrapping_add(0x44u32));
                esi = eax.wrapping_add(eax.wrapping_mul(8));
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x18u32), esi);
                ebx = m.r32(esi.wrapping_add(ecx).wrapping_add(0x14u32));
                ecx = esi.wrapping_add(ecx).wrapping_add(0x14u32);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(ecx, ebx);
                edx = m.r32(ebp.wrapping_add(0x44u32));
                ecx = esi.wrapping_add(edx);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                edx = m.r32(ecx.wrapping_add(0x14u32));
                m.w16(esp.wrapping_add(0x2cu32), ((eax & 0xffff)) as u16);
                eax = m.r32(ebp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                m.w16(esp.wrapping_add(0x2eu32), (0x0u32) as u16);
                fl = lift::sub(m.r32(eax), 0x0u32, 32).1;
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ebp = 0xcbu32;
                pc = 4; continue;
            }
            4 => {
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x30u32), ebp);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x34u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                esi = esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x38u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x144u32)))) & 0xff);
                let (r, f) = lift::shl(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                m.w32(esi, edx);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0x194u32)))) & 0xff);
                ecx = ebx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shl(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esi, ecx);
                edx = m.r32(esp.wrapping_add(0x2eu32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(eax).wrapping_add(0x16cu32)))) & 0xff);
                edx = ebx;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::shl(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                m.w32(esi, edx);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esi, ecx);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x3cu32), ecx);
                ecx = m.r32(esp.wrapping_add(0x2eu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xa4u32));
                m.w32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x40u32), ecx);
                let (r, f) = lift::add(u32::from(m.r16(esp.wrapping_add(0x2eu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esp.wrapping_add(0x2eu32), (r) as u16);
                edx = m.r32(esp.wrapping_add(0x2eu32));
                esi = m.r32(eax);
                ecx = edx;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(ecx, esi, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 7; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x2eu32));
                pc = 7; continue;
            }
            7 => {
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                edx = edx.wrapping_add(edx.wrapping_mul(4));
                eax = edx.wrapping_mul(4).wrapping_add(0x10u32);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(ebp.wrapping_add(0x3cu32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2f0u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x410u32);
                edx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w16(esp.wrapping_add(0x3au32), (0x0u32) as u16);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(ebx, 0x28u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 22; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x2eu32))), 0x34u32, 16).1;
                if !fl.cf { pc = 22; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(ebx, 0x28u32, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esp.wrapping_add(0x2eu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x30u32), 0xccu32);
                eax = m.r32(esp.wrapping_add(0x2eu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x34u32), ebx);
                let (r, f) = lift::add(u32::from(m.r16(esp.wrapping_add(0x2eu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esp.wrapping_add(0x2eu32), (r) as u16);
                ebx = 0x29u32;
                pc = 9; continue;
            }
            13 => {
                edx = m.r32(ebp.wrapping_add(0xcu32));
                eax = m.r32(edx.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x1bcu32));
                ebp = edx.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x1bcu32);
                fl = lift::sub(edi, m.r32(eax.wrapping_add(0xfffffff8u32)), 32).1;
                if fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 9; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x2eu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x30u32), 0xccu32);
                eax = m.r32(esp.wrapping_add(0x2eu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                eax = esp.wrapping_add(ecx).wrapping_add(0x34u32);
                esi = esp.wrapping_add(ecx).wrapping_add(0x44u32);
                m.w8(eax, ((ebx & 0xff)) as u8);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w8(eax, (0x0u32) as u8);
                pc = 18; continue;
            }
            17 => {
                m.w8(eax, (0x7fu32) as u8);
                pc = 18; continue;
            }
            18 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(ebp);
                fl = lift::sub(edi, m.r32(ecx.wrapping_add(0xfffffff8u32)), 32).1;
                if !fl.cf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = edi;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx)))) & 0xff);
                fl = lift::sub(eax, esi, 32).1;
                m.w8(eax.wrapping_add(0xffffffffu32), ((edx & 0xff)) as u8);
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(u32::from(m.r16(esp.wrapping_add(0x2eu32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esp.wrapping_add(0x2eu32), (r) as u16);
                esi = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 9; continue;
            }
            22 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x2eu32))), 0x0u32, 16).1;
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                if fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(ebp.wrapping_add(0x44u32));
                ecx = m.r32(esi.wrapping_add(eax).wrapping_add(0x14u32));
                eax = esi.wrapping_add(eax).wrapping_add(0x14u32);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(eax, ecx);
                ecx = m.r32(ebp.wrapping_add(0x44u32));
                eax = m.r32(esp.wrapping_add(0x2eu32));
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(ecx.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = esp.wrapping_add(0x10u32);
                eax = eax.wrapping_mul(4).wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(ebp.wrapping_add(0x3cu32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2f0u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 24; continue;
            }
            24 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x430);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            25 => {
                eax = 0x1u32;
                pc = 26; continue;
            }
            26 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x430);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00605420` start
pub fn f_00605420(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6682b6u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x330);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x2cu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x350u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x358u32)))) & 0xff);
                edi = m.r32(esp.wrapping_add(0x354u32));
                m.w16(esp.wrapping_add(0x140u32), ((ebp & 0xffff)) as u16);
                ebp = ebp.wrapping_add(ebp.wrapping_mul(2));
                m.w8(esi.wrapping_add(0x12e8u32), ((eax & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x558u32), (0x2u32) as u8);
                ecx = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                ebx = edi.wrapping_add(edi.wrapping_mul(2));
                m.w16(esp.wrapping_add(0x142u32), ((edi & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(ebp)))) & 0xffff);
                m.w16(esi.wrapping_add(0x35au32), ((edx & 0xffff)) as u16);
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                edx = 0x15u32;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(ebx.wrapping_add(eax)))) & 0xffff);
                m.w16(esi.wrapping_add(0x35cu32), ((ecx & 0xffff)) as u16);
                m.w8(esi.wrapping_add(0x559u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0x55au32), (0x1u32) as u8);
                ecx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                eax = ecx.wrapping_add(ebp);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 251; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x134u32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 251; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r8(eax)), 0xffu32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(ecx)), 0xffu32, 8).1;
                if fl.zf { pc = 251; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = 0xffu32;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x15cu32))), 0x0u32, 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esi.wrapping_add(0xad2u32), (0x0u32) as u8);
                pc = 8; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0xad2u32), (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35cu32))), (edx & 0xffff), 16).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(0x6dd7a4u32);
                fl = lift::sub(u32::from(m.r8(ebx.wrapping_add(edx).wrapping_add(0x15cu32))), 0x0u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = u32::from(m.r8(esi.wrapping_add(0xad2u32))) | 0x2u32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0xad2u32), (r) as u8);
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, m.r32(eax.wrapping_add(0x1cu32)), 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                m.w32(esi.wrapping_add(0xad4u32), edx);
                ecx = m.r32(ecx.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, m.r32(eax.wrapping_add(0x20u32)), 32); fl = f;
                ecx = r;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0xad8u32), ecx);
                edx = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ebp).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x65eu32), ((eax & 0xffff)) as u16);
                ecx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(ecx).wrapping_add(0x8u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xad8u32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0x660u32), ((edx & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xad4u32)))) & 0xffff);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(4));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x503ac0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = 0xfeu32;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(esi.wrapping_add(0x5du32), (0x1u32) as u8);
                m.w16(esi.wrapping_add(0x35au32), ((ebx & 0xffff)) as u16);
                pc = 17; continue;
            }
            13 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 16; continue;
            }
            15 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esi.wrapping_add(0x5du32), ((ecx & 0xff)) as u8);
                pc = 17; continue;
            }
            17 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x12u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x9u32, 16).1;
                if fl.zf { pc = 22; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub((eax & 0xffff), 0x16u32, 16).1;
                if fl.zf { pc = 22; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35cu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if fl.cf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esi.wrapping_add(0x5eu32), ((ecx & 0xff)) as u8);
                pc = 23; continue;
            }
            21 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esi.wrapping_add(0x5eu32), ((ecx & 0xff)) as u8);
                pc = 23; continue;
            }
            22 => {
                m.w8(esi.wrapping_add(0x5eu32), (0x1u32) as u8);
                m.w16(esi.wrapping_add(0x35cu32), ((ebx & 0xffff)) as u16);
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(0x6dfc18u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 28; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5du32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5eu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x35cu32))), 16).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                m.w8(esi.wrapping_add(0x5eu32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x5du32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0xac8u32), (0x1u32) as u8);
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5du32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5eu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                m.w8(esi.wrapping_add(0xac8u32), (0x1u32) as u8);
                pc = 38; continue;
            }
            31 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w8(esi.wrapping_add(0xac8u32), (0x0u32) as u8);
                if !fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(esp.wrapping_add(0x350u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xadau32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xad6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xeu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x548ca0, ecx, &a); }
                esp = esp.wrapping_add(24);
                pc = 33; continue;
            }
            33 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5eu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 36; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35cu32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (eax & 0xffff), 16).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xadau32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xad6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x548ca0, ecx, &a); }
                esp = esp.wrapping_add(24);
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(0x6dfc18u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6e55d0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x612c90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 38; continue;
            }
            38 => {
                eax = 0xffu32;
                m.w8(esi.wrapping_add(0xab0u32), (0x1u32) as u8);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35cu32))), (eax & 0xffff), 16).1;
                if !fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = 0x1u32;
                pc = 41; continue;
            }
            41 => {
                m.w8(esi.wrapping_add(0xab1u32), ((eax & 0xff)) as u8);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd896u32))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if !fl.cf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x660u32))), (ecx & 0xffff), 16).1;
                if fl.cf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x35cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c7620u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x360u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x348u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 44; continue;
            }
            44 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                ebx = m.r32(0x6dd894u32);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x14u32), ebx);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                m.w8(esi.wrapping_add(0x1e92u32), (0x0u32) as u8);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.cf { pc = 51; continue; }
                pc = 45; continue;
            }
            45 => {
                edx = m.r32(0x6dd7a4u32);
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(edx)))) & 0xff);
                edx = m.r32(0x6dfc14u32);
                m.w8(esp.wrapping_add(0x1cu32), ((ecx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 49; continue; }
                pc = 46; continue;
            }
            46 => {
                edx = m.r32(0x6dfc18u32);
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 50; continue; }
                pc = 47; continue;
            }
            47 => {
                ecx = (ecx & !0xffff) | (((ecx & 0xff)) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 50; continue; }
                pc = 48; continue;
            }
            48 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                if !fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                m.w8(esi.wrapping_add(0x1e92u32), (0x1u32) as u8);
                pc = 50; continue;
            }
            50 => {
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(edx).wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 45; continue; }
                pc = 51; continue;
            }
            51 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.cf { pc = 59; continue; }
                pc = 52; continue;
            }
            52 => {
                ecx = m.r32(0x6dd7a4u32);
                pc = 53; continue;
            }
            53 => {
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(0x6dfc14u32);
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ecx)))) & 0xff);
                m.w8(esp.wrapping_add(0x1cu32), ((ecx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 57; continue; }
                pc = 54; continue;
            }
            54 => {
                edx = m.r32(0x6dfc18u32);
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 58; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = (ecx & !0xffff) | (((ecx & 0xff)) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 58; continue; }
                pc = 56; continue;
            }
            56 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                if !fl.zf { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                m.w8(esi.wrapping_add(0x1e92u32), (0x1u32) as u8);
                pc = 58; continue;
            }
            58 => {
                ecx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ecx).wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 53; continue; }
                pc = 59; continue;
            }
            59 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1e92u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 62; continue; }
                pc = 60; continue;
            }
            60 => {
                ecx = m.r32(0x6cbdd8u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                edx = lift::sx16(u32::from(m.r16(esi.wrapping_add(0xadau32))));
                eax = lift::sx16(u32::from(m.r16(esi.wrapping_add(0xad6u32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x662u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x457c90, ecx, &a); }
                esp = esp.wrapping_add(12);
                m.w8(esi.wrapping_add(0x1e92u32), (0x0u32) as u8);
                pc = 62; continue;
            }
            62 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 66; continue; }
                pc = 63; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x350u32));
                m.w32(esi.wrapping_add(0xa84u32), eax);
                ecx = m.r32(0x6dd7a4u32);
                eax = m.r32(ecx.wrapping_add(ebp).wrapping_add(0x120u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                m.w16(esi.wrapping_add(0x658u32), (0x1u32) as u16);
                m.w32(esp.wrapping_add(0x18u32), 0x1u32);
                pc = 71; continue;
            }
            65 => {
                m.w16(esi.wrapping_add(0x658u32), (0x0u32) as u16);
                m.w32(esp.wrapping_add(0x18u32), 0x1u32);
                pc = 71; continue;
            }
            66 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0x658u32), ((ecx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = m.r32(0x6dd7a8u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0xa84u32), eax);
                if !fl.cf { pc = 71; continue; }
                pc = 67; continue;
            }
            67 => {
                edx = m.r32(0x6dd7a4u32);
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0xfeu32, 16).1;
                if !fl.zf { pc = 74; continue; }
                pc = 68; continue;
            }
            68 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0x658u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0x658u32), (r) as u16);
                pc = 69; continue;
            }
            69 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x18u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x18u32), r);
                pc = 70; continue;
            }
            70 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 67; continue; }
                pc = 71; continue;
            }
            71 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 84; continue; }
                pc = 72; continue;
            }
            72 => {
                eax = m.r32(esp.wrapping_add(0x354u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esi.wrapping_add(0xa88u32), eax);
                ecx = m.r32(0x6dd7a4u32);
                eax = m.r32(edx.wrapping_add(ecx).wrapping_add(0x120u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 83; continue; }
                pc = 73; continue;
            }
            73 => {
                m.w16(esi.wrapping_add(0x65au32), (0x1u32) as u16);
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 89; continue;
            }
            74 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 77; continue; }
                pc = 75; continue;
            }
            75 => {
                eax = m.r32(ebp.wrapping_add(0x120u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 69; continue; }
                pc = 76; continue;
            }
            76 => {
                pc = 68; continue;
            }
            77 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xad2u32))), 0x2u32, 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 78; continue;
            }
            78 => {
                ecx = 0x1u32;
                edi = 0x3b6u32;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 79; continue;
            }
            79 => {
                fl = lift::logic((ecx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 81; continue; }
                pc = 80; continue;
            }
            80 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1e94u32)))) & 0xffff);
                ecx = 0x3e8u32;
                ebx = (ebx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                let r = eax & 0x3ffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = 0x3e8u32;
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = 0x3e8u32;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x10624dd3u32;
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 0x1f4u32, 32); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::shr((ebx & 0xffff), 0xau32, 16, fl); fl = f;
                ebx = (ebx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x3e8u32;
                ecx = (ecx & !0xff) | (((ebx & 0xff)) & 0xff);
                ebx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(ecx, 0xau32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w16(esi.wrapping_add(0x1e94u32), ((ecx & 0xffff)) as u16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                pc = 81; continue;
            }
            81 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                fl = lift::sub((ecx & 0xffff), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (!fl.cf && !fl.zf) { pc = 70; continue; }
                pc = 82; continue;
            }
            82 => {
                edx = edi;
                eax = 0x10624dd3u32;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, 0x1f4u32, 32); fl = f;
                edx = r;
                { let r = u64::from(eax) * u64::from(edx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                edi = edx;
                pc = 79; continue;
            }
            83 => {
                m.w16(esi.wrapping_add(0x65au32), (0x0u32) as u16);
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 89; continue;
            }
            84 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0x65au32), ((ecx & 0xffff)) as u16);
                edx = m.r32(0x6dd7a8u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0xa88u32), eax);
                if !fl.cf { pc = 89; continue; }
                pc = 85; continue;
            }
            85 => {
                edx = m.r32(0x6dd7a4u32);
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35cu32)))) & 0xffff);
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0xfeu32, 16).1;
                if !fl.zf { pc = 94; continue; }
                pc = 86; continue;
            }
            86 => {
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0x65au32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(esi.wrapping_add(0x65au32), (r) as u16);
                pc = 87; continue;
            }
            87 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x10u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x10u32), r);
                pc = 88; continue;
            }
            88 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 85; continue; }
                pc = 89; continue;
            }
            89 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x35cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 91; continue; }
                pc = 90; continue;
            }
            90 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x65au32), (0x0u32) as u16);
                let (r, f) = lift::sub(u32::from(m.r16(esp.wrapping_add(0x10u32))), (eax & 0xffff), 16); fl = f;
                m.w16(esp.wrapping_add(0x10u32), (r) as u16);
                pc = 91; continue;
            }
            91 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x10u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x658u32), ((eax & 0xffff)) as u16);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w16(esi.wrapping_add(0x65au32), ((ecx & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x14u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x4u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::add(eax, eax, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xa74u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi.wrapping_add(0xa78u32), eax);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0xcu32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::add(eax, eax, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x8u32), eax);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                esp = esp.wrapping_add(0x14);
                m.w16(esi.wrapping_add(0x10u32), (0x0u32) as u16);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 105; continue; }
                pc = 92; continue;
            }
            92 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xa74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w16(ecx, ((eax & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x164u32);
                m.w16(edx, ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x348u32), 0x1u32);
                if fl.zf { pc = 103; continue; }
                pc = 93; continue;
            }
            93 => {
                ecx = m.r32(esp.wrapping_add(0x350u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x61a4e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 104; continue;
            }
            94 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if !fl.zf { pc = 97; continue; }
                pc = 95; continue;
            }
            95 => {
                eax = m.r32(ebp.wrapping_add(0x120u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 87; continue; }
                pc = 96; continue;
            }
            96 => {
                pc = 86; continue;
            }
            97 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xad2u32))), 0x1u32, 8).1;
                if !fl.zf { pc = 88; continue; }
                pc = 98; continue;
            }
            98 => {
                ecx = 0x1u32;
                edi = 0x3b6u32;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 99; continue;
            }
            99 => {
                fl = lift::logic((ecx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 101; continue; }
                pc = 100; continue;
            }
            100 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1e94u32)))) & 0xffff);
                edx = 0x3e8u32;
                ebx = (ebx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                let r = eax & 0x3ffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x3e8u32;
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = 0x3e8u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 0x1f4u32, 32); fl = f;
                ecx = r;
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                eax = 0x3e8u32;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::shr((ebx & 0xffff), 0xau32, 16, fl); fl = f;
                ebx = (ebx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((ebx & 0xff)) & 0xff);
                let (r, f) = lift::shl(edx, 0xau32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w16(esi.wrapping_add(0x1e94u32), ((edx & 0xffff)) as u16);
                pc = 101; continue;
            }
            101 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                fl = lift::sub((ecx & 0xffff), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (!fl.cf && !fl.zf) { pc = 88; continue; }
                pc = 102; continue;
            }
            102 => {
                edx = edi;
                eax = 0x10624dd3u32;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, 0x1f4u32, 32); fl = f;
                edx = r;
                { let r = u64::from(eax) * u64::from(edx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                edi = edx;
                pc = 99; continue;
            }
            103 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 104; continue;
            }
            104 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x348u32), 0xffffffffu32);
                m.w32(edx, eax);
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w16(eax, (0xffffu32) as u16);
                ecx = m.r32(esi.wrapping_add(0xcu32));
                m.w16(ecx, (0xffffu32) as u16);
                pc = 129; continue;
            }
            105 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x658u32))), (ebx & 0xffff), 16).1;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                if (fl.cf || fl.zf) { pc = 119; continue; }
                pc = 106; continue;
            }
            106 => {
                ecx = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                ebp = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0xfeu32, 16).1;
                if fl.zf { pc = 109; continue; }
                pc = 107; continue;
            }
            107 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edx & 0xffff), 16).1;
                if fl.zf { pc = 109; continue; }
                pc = 108; continue;
            }
            108 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                ebp = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 107; continue;
            }
            109 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xa74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w16(ecx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x164u32);
                m.w16(edx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x348u32), 0x2u32);
                if fl.zf { pc = 111; continue; }
                pc = 110; continue;
            }
            110 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x61a4e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 112; continue;
            }
            111 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 112; continue;
            }
            112 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x348u32), 0xffffffffu32);
                m.w32(ecx.wrapping_add(ebx.wrapping_mul(4)), eax);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), 0xfeu32, 16).1;
                if !fl.zf { pc = 115; continue; }
                pc = 113; continue;
            }
            113 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x35cu32))), 16).1;
                if !fl.zf { pc = 115; continue; }
                pc = 114; continue;
            }
            114 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                let r = u32::from(m.r8(eax.wrapping_add(0x12au32))) | 0x1u32; fl = lift::logic(r, 8);
                m.w8(eax.wrapping_add(0x12au32), (r) as u8);
                pc = 115; continue;
            }
            115 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                ecx = ebx.wrapping_add(0xffffffffu32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edx.wrapping_add(ebx.wrapping_mul(2)), ((ecx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 117; continue; }
                pc = 116; continue;
            }
            116 => {
                edx = m.r32(esi.wrapping_add(0xcu32));
                ecx = ebx.wrapping_add(0x1u32);
                m.w16(edx.wrapping_add(ebx.wrapping_mul(2)), ((ecx & 0xffff)) as u16);
                pc = 118; continue;
            }
            117 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                m.w16(eax.wrapping_add(ebx.wrapping_mul(2)), (0xffffu32) as u16);
                pc = 118; continue;
            }
            118 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                if fl.cf { pc = 106; continue; }
                pc = 119; continue;
            }
            119 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x658u32))), (ebx & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.cf || fl.zf) { pc = 126; continue; }
                pc = 120; continue;
            }
            120 => {
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                if (fl.cf || fl.zf) { pc = 125; continue; }
                pc = 121; continue;
            }
            121 => {
                edi = eax.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xfffffffcu32);
                pc = 122; continue;
            }
            122 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x61a750, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 124; continue; }
                pc = 123; continue;
            }
            123 => {
                edx = m.r32(edi);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(edi.wrapping_add(0x4u32), edx);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::sub(edi, 0x4u32, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                if (!fl.cf && !fl.zf) { pc = 122; continue; }
                pc = 124; continue;
            }
            124 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 125; continue;
            }
            125 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xfffffffcu32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(eax.wrapping_add(ecx.wrapping_mul(4)), edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                fl = lift::sub(ebx, edx, 32).1;
                if fl.cf { pc = 120; continue; }
                pc = 126; continue;
            }
            126 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x658u32))), (ecx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 128; continue; }
                pc = 127; continue;
            }
            127 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                edi = m.r32(eax.wrapping_add(ecx.wrapping_mul(4)));
                m.w32(edx.wrapping_add(ecx.wrapping_mul(4)), edi);
                edx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                m.w16(edx.wrapping_add(0x138u32), ((ecx & 0xffff)) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 127; continue; }
                pc = 128; continue;
            }
            128 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 129; continue;
            }
            129 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                m.w16(esi.wrapping_add(0x12u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(esi.wrapping_add(0x10u32), (0x0u32) as u16);
                m.w16(ecx.wrapping_add(eax.wrapping_mul(2)), (0xffffu32) as u16);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                m.w8(esi.wrapping_add(0x12e8u32), (0x1u32) as u8);
                m.w16(esi.wrapping_add(0x65cu32), ((ebx & 0xffff)) as u16);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 134; continue; }
                pc = 130; continue;
            }
            130 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = (edx & !0xff) | (((ebx & 0xff)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xa74u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w16(edx.wrapping_add(ecx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x658u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xa78u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x164u32);
                m.w16(edx.wrapping_add(ecx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x348u32), 0x3u32);
                if fl.zf { pc = 132; continue; }
                pc = 131; continue;
            }
            131 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x354u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x61a4e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 133; continue;
            }
            132 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 133; continue;
            }
            133 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x348u32), 0xffffffffu32);
                m.w32(edx.wrapping_add(ecx.wrapping_mul(4)), eax);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(eax.wrapping_mul(2)), (0xffffu32) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                eax = m.r32(esi.wrapping_add(0xcu32));
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)), (0xffffu32) as u16);
                pc = 163; continue;
            }
            134 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6dd7a8u32);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 153; continue; }
                pc = 135; continue;
            }
            135 => {
                edx = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35cu32)))) & 0xffff);
                ebp = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0xfeu32, 16).1;
                if fl.zf { pc = 140; continue; }
                pc = 136; continue;
            }
            136 => {
                ecx = m.r32(ebp.wrapping_add(0x120u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 138; continue; }
                pc = 137; continue;
            }
            137 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x35cu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 139; continue; }
                pc = 138; continue;
            }
            138 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if fl.zf { pc = 140; continue; }
                pc = 139; continue;
            }
            139 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x358u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w8(esi.wrapping_add(0x12e8u32), ((edx & 0xff)) as u8);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                ebp = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 136; continue;
            }
            140 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xa74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w16(ecx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x619450, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x164u32);
                m.w16(edx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x348u32), 0x4u32);
                if fl.zf { pc = 142; continue; }
                pc = 141; continue;
            }
            141 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x61a4e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 143; continue;
            }
            142 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 143; continue;
            }
            143 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x348u32), 0xffffffffu32);
                m.w32(ecx.wrapping_add(ebx.wrapping_mul(4)), eax);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35cu32))), 0xfeu32, 16).1;
                if !fl.zf { pc = 146; continue; }
                pc = 144; continue;
            }
            144 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ebp))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x35au32))), 16).1;
                if !fl.zf { pc = 146; continue; }
                pc = 145; continue;
            }
            145 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                let r = u32::from(m.r8(eax.wrapping_add(0x12au32))) | 0x1u32; fl = lift::logic(r, 8);
                m.w8(eax.wrapping_add(0x12au32), (r) as u8);
                pc = 146; continue;
            }
            146 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                fl = lift::sub(ebx, ecx, 32).1;
                if (fl.cf || fl.zf) { pc = 148; continue; }
                pc = 147; continue;
            }
            147 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                edx = ebx.wrapping_add(0xffffffffu32);
                m.w16(eax.wrapping_add(ebx.wrapping_mul(2)), ((edx & 0xffff)) as u16);
                pc = 149; continue;
            }
            148 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                m.w16(ecx.wrapping_add(ebx.wrapping_mul(2)), (0xffffu32) as u16);
                pc = 149; continue;
            }
            149 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(ebx, edx, 32).1;
                if !fl.cf { pc = 151; continue; }
                pc = 150; continue;
            }
            150 => {
                ecx = m.r32(esi.wrapping_add(0xcu32));
                eax = ebx.wrapping_add(0x1u32);
                m.w16(ecx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                pc = 152; continue;
            }
            151 => {
                edx = m.r32(esi.wrapping_add(0xcu32));
                m.w16(edx.wrapping_add(ebx.wrapping_mul(2)), (0xffffu32) as u16);
                pc = 152; continue;
            }
            152 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x14u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 135; continue; }
                pc = 153; continue;
            }
            153 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x65au32))), (ebx & 0xffff), 16).1;
                edx = eax;
                m.w32(esp.wrapping_add(0x14u32), edx);
                if (fl.cf || fl.zf) { pc = 160; continue; }
                pc = 154; continue;
            }
            154 => {
                edi = edx.wrapping_add(0xfffffffcu32);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                pc = 155; continue;
            }
            155 => {
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                if (fl.cf || fl.zf) { pc = 159; continue; }
                pc = 156; continue;
            }
            156 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x61a750, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 158; continue; }
                pc = 157; continue;
            }
            157 => {
                edx = m.r32(edi);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(edi.wrapping_add(0x4u32), edx);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::sub(edi, 0x4u32, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                if (!fl.cf && !fl.zf) { pc = 156; continue; }
                pc = 158; continue;
            }
            158 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                pc = 159; continue;
            }
            159 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x1cu32), edi);
                m.w32(edx.wrapping_add(ecx.wrapping_mul(4)), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 155; continue; }
                pc = 160; continue;
            }
            160 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x65au32))), (eax & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 162; continue; }
                pc = 161; continue;
            }
            161 => {
                edi = m.r32(esi.wrapping_add(0x4u32));
                ebx = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)), ebx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                ebx = m.r32(esi.wrapping_add(0x4u32));
                edi = eax.wrapping_add(ecx);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ecx = m.r32(ebx.wrapping_add(ecx.wrapping_mul(4)));
                m.w16(ecx.wrapping_add(0x138u32), ((edi & 0xffff)) as u16);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 161; continue; }
                pc = 162; continue;
            }
            162 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 163; continue;
            }
            163 => {
                ecx = m.r32(esi.wrapping_add(0xadcu32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                eax = 0xa205b063u32;
                m.w16(esi.wrapping_add(0x12u32), (0xffffu32) as u16);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w16(esi.wrapping_add(0x65cu32), ((edx & 0xffff)) as u16);
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 165; continue; }
                pc = 164; continue;
            }
            164 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 166; continue;
            }
            165 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 166; continue;
            }
            166 => {
                ebx = eax;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                let r = ebx & 0x3u32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ebx, 0x5u32, 32); fl = f;
                ebx = r;
                edx = 0x6dd7c8u32;
                edi = 0x10u32;
                pc = 167; continue;
            }
            167 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0xfffffff8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 173; continue; }
                pc = 168; continue;
            }
            168 => {
                eax = lift::sx16(u32::from(m.r16(edx.wrapping_add(0xfffffffcu32))));
                ecx = lift::sx16(u32::from(m.r16(esi.wrapping_add(0xad6u32))));
                ebp = lift::sx16(u32::from(m.r16(esi.wrapping_add(0xadau32))));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = lift::sx16(u32::from(m.r16(edx.wrapping_add(0xfffffffeu32))));
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                ebp = ecx;
                let (r, f) = lift::imul(ebp, ecx); fl = f;
                ebp = r;
                ecx = eax;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebp, 0x384u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 173; continue; }
                pc = 169; continue;
            }
            169 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if !fl.zf { pc = 171; continue; }
                pc = 170; continue;
            }
            170 => {
                let (r, f) = lift::add(ebx, 0x3u32, 32); fl = f;
                ebx = r;
                pc = 171; continue;
            }
            171 => {
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 173; continue; }
                pc = 172; continue;
            }
            172 => {
                let (r, f) = lift::sub(ebx, 0x3u32, 32); fl = f;
                ebx = r;
                pc = 173; continue;
            }
            173 => {
                let (r, f) = lift::add(edx, 0xcu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 167; continue; }
                pc = 174; continue;
            }
            174 => {
                fl = lift::sub((ebx & 0xffff), 0x2u32, 16).1;
                if (fl.sf == fl.of) { pc = 176; continue; }
                pc = 175; continue;
            }
            175 => {
                ebx = 0x2u32;
                pc = 178; continue;
            }
            176 => {
                fl = lift::sub((ebx & 0xffff), 0xau32, 16).1;
                if (fl.zf || fl.sf != fl.of) { pc = 178; continue; }
                pc = 177; continue;
            }
            177 => {
                ebx = 0xau32;
                pc = 178; continue;
            }
            178 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = 0xa205b063u32;
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 180; continue; }
                pc = 179; continue;
            }
            179 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 181; continue;
            }
            180 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 181; continue;
            }
            181 => {
                ebp = 0xffu32;
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (ebp & 0xffff), 16).1;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                m.w8(esi.wrapping_add(0x670u32), ((edx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x671u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x5cu32), (0x0u32) as u8);
                if !fl.zf { pc = 183; continue; }
                pc = 182; continue;
            }
            182 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ecx).wrapping_add(0x42u32)))) & 0xffff);
                pc = 186; continue;
            }
            183 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35cu32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 185; continue; }
                pc = 184; continue;
            }
            184 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x42u32)))) & 0xffff);
                pc = 186; continue;
            }
            185 => {
                eax = 0xffffu32;
                pc = 186; continue;
            }
            186 => {
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 188; continue; }
                pc = 187; continue;
            }
            187 => {
                ecx = m.r32(0x6dd7a0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx).wrapping_add(0x2u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x1e90u32), ((edx & 0xff)) as u8);
                ecx = m.r32(0x6dd7a0u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx).wrapping_add(0x3u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x1e91u32), ((edx & 0xff)) as u8);
                pc = 189; continue;
            }
            188 => {
                m.w8(esi.wrapping_add(0x1e90u32), (0xffu32) as u8);
                m.w8(esi.wrapping_add(0x1e91u32), (0xffu32) as u8);
                pc = 189; continue;
            }
            189 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if (fl.cf || fl.zf) { pc = 192; continue; }
                pc = 190; continue;
            }
            190 => {
                ecx = esi.wrapping_add(0x15cu32);
                pc = 191; continue;
            }
            191 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0x5du32)))) & 0xff);
                let (r, f) = lift::sub(0, (edx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let r = (edx & 0xff) & 0x6u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::add(edx, 0xfau32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w16(ecx.wrapping_add(0xfffffffeu32), ((edx & 0xffff)) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 191; continue; }
                pc = 192; continue;
            }
            192 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xac8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 250; continue; }
                pc = 193; continue;
            }
            193 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (fl.cf || fl.zf) { pc = 258; continue; }
                pc = 194; continue;
            }
            194 => {
                ecx = esp.wrapping_add(0x140u32);
                eax = esi.wrapping_add(0x559u32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = 0xfffffaa7u32;
                edi = esi.wrapping_add(0x15cu32);
                let (r, f) = lift::sub(ecx, esi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 195; continue;
            }
            195 => {
                ecx = m.r32(0x6dfc14u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x1feu32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 248; continue; }
                pc = 196; continue;
            }
            196 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                edx = eax;
                let r = edx & ebp; fl = lift::logic(r, 32);
                edx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(esi).wrapping_add(0x5du32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 248; continue; }
                pc = 197; continue;
            }
            197 => {
                fl = lift::sub(u32::from(m.r8(0x6e59ccu32)), 0xffu32, 8).1;
                if !fl.zf { pc = 247; continue; }
                pc = 198; continue;
            }
            198 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6e59cdu32))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 247; continue; }
                pc = 199; continue;
            }
            199 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xad2u32)))) & 0xff);
                if !fl.zf { pc = 222; continue; }
                pc = 200; continue;
            }
            200 => {
                fl = lift::logic((eax & 0xff) & 0x2u32, 8);
                if fl.zf { pc = 205; continue; }
                pc = 201; continue;
            }
            201 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35cu32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 204; continue; }
                pc = 202; continue;
            }
            202 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ecx).wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 204; continue; }
                pc = 203; continue;
            }
            203 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6ddaa0u32;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1e91u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1e90u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6a16f0u32);
                pc = 209; continue;
            }
            204 => {
                ecx = m.r32(0x6a16f4u32);
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 210; continue;
            }
            205 => {
                ecx = m.r32(0x6dd7a4u32);
                edi = m.r32(esp.wrapping_add(0x24u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ecx)))) & 0xff);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1au32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 207; continue; }
                pc = 206; continue;
            }
            206 => {
                edx = m.r32(0x6dd7a4u32);
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 208; continue;
            }
            207 => {
                ecx = m.r32(0x6dd7a4u32);
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 208; continue;
            }
            208 => {
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6a16e4u32);
                pc = 209; continue;
            }
            209 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 210; continue;
            }
            210 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(0x6dd896u32))) & 0xffff);
                edx = m.r32(0x6dd894u32);
                eax = m.r32(0x6dd7a4u32);
                edi = m.r32(0x6dd7a8u32);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 217; continue; }
                pc = 211; continue;
            }
            211 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebp = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 217; continue; }
                pc = 212; continue;
            }
            212 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 215; continue; }
                pc = 213; continue;
            }
            213 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 215; continue; }
                pc = 214; continue;
            }
            214 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                ecx = m.r32(0x6a17a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 216; continue;
            }
            215 => {
                ecx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a171cu32));
                pc = 216; continue;
            }
            216 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6a1794u32);
                eax = edx.wrapping_add(eax).wrapping_add(0x160u32);
                edx = m.r32(0x6a16c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 243; continue;
            }
            217 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 220; continue; }
                pc = 218; continue;
            }
            218 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 220; continue; }
                pc = 219; continue;
            }
            219 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                ecx = m.r32(0x6a17a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 221; continue;
            }
            220 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a171cu32));
                pc = 221; continue;
            }
            221 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(0x6a16c0u32);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a1784u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 244; continue;
            }
            222 => {
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 227; continue; }
                pc = 223; continue;
            }
            223 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (ebp & 0xffff), 16).1;
                if !fl.zf { pc = 226; continue; }
                pc = 224; continue;
            }
            224 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 226; continue; }
                pc = 225; continue;
            }
            225 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x6ddaa0u32;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1e91u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1e90u32)))) & 0xff);
                ecx = 0x6ddaa0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6a16f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 231; continue;
            }
            226 => {
                eax = m.r32(0x6a16f4u32);
                ecx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 232; continue;
            }
            227 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x6de4a0u32;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1au32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 229; continue; }
                pc = 228; continue;
            }
            228 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 230; continue;
            }
            229 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x6ddaa0u32;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 230; continue;
            }
            230 => {
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6a16e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 231; continue;
            }
            231 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 232; continue;
            }
            232 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(0x6dd896u32))) & 0xffff);
                edx = m.r32(0x6dd894u32);
                eax = m.r32(0x6dd7a4u32);
                edi = m.r32(0x6dd7a8u32);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 239; continue; }
                pc = 233; continue;
            }
            233 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebp = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 239; continue; }
                pc = 234; continue;
            }
            234 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 237; continue; }
                pc = 235; continue;
            }
            235 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 237; continue; }
                pc = 236; continue;
            }
            236 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                ecx = m.r32(0x6a179cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 238; continue;
            }
            237 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a175cu32));
                pc = 238; continue;
            }
            238 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(eax).wrapping_add(0x160u32);
                eax = m.r32(0x6a1794u32);
                ecx = m.r32(0x6a16c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 244; continue;
            }
            239 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.cf { pc = 242; continue; }
                pc = 240; continue;
            }
            240 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ebx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(eax).wrapping_add(0x6u32))), (edx & 0xffff), 16).1;
                if !fl.cf { pc = 242; continue; }
                pc = 241; continue;
            }
            241 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edx.wrapping_add(eax).wrapping_add(0x160u32);
                edx = m.r32(0x6a179cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xeu32)))) & 0xff);
                eax = ecx.wrapping_add(0x160u32);
                ecx = m.r32(0x6a16c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a1784u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 244; continue;
            }
            242 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = eax.wrapping_add(ebp).wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ebp).wrapping_add(0xeu32)))) & 0xff);
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6a175cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xeu32)))) & 0xff);
                edx = ecx.wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6a16c4u32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6a1784u32));
                pc = 243; continue;
            }
            243 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 244; continue;
            }
            244 => {
                edi = esi.wrapping_add(0x1310u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                eax = esi.wrapping_add(0x12ecu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_add(0x1c);
                m.w32(eax, ecx);
                m.w8(eax.wrapping_add(0x4u32), ((ecx & 0xff)) as u8);
                m.w16(eax.wrapping_add(0x6u32), ((ecx & 0xffff)) as u16);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                m.w8(eax.wrapping_add(0xcu32), (0xffu32) as u8);
                m.w32(eax.wrapping_add(0x10u32), 0x3fb00000u32);
                m.w8(eax.wrapping_add(0x14u32), (0x1u32) as u8);
                m.w16(eax.wrapping_add(0x16u32), ((ecx & 0xffff)) as u16);
                m.w32(eax.wrapping_add(0x18u32), ecx);
                m.w32(eax.wrapping_add(0x1cu32), ecx);
                m.w8(eax.wrapping_add(0xfu32), (0xffu32) as u8);
                m.w8(eax.wrapping_add(0xdu32), ((ecx & 0xff)) as u8);
                m.w8(eax.wrapping_add(0xeu32), ((ecx & 0xff)) as u8);
                m.w16(esi.wrapping_add(0x12f2u32), (0x1u32) as u16);
                m.w32(esi.wrapping_add(0x12f4u32), 0x6b2bc0u32);
                m.w32(eax, edi);
                m.w8(esi.wrapping_add(0x12f0u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x12f8u32), (0xffu32) as u8);
                m.w32(esi.wrapping_add(0x12fcu32), 0x3f3504f4u32);
                m.w8(esi.wrapping_add(0x1300u32), ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6e59cdu32))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 246; continue; }
                pc = 245; continue;
            }
            245 => {
                m.w8(esi.wrapping_add(0x12e9u32), (0x1u32) as u8);
                ebp = 0xffu32;
                pc = 248; continue;
            }
            246 => {
                ecx = m.r32(0x6cbb10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41d9b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x12e9u32), (0x0u32) as u8);
                m.w8(0x6e59cdu32, (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x12eau32), (0x1u32) as u8);
                ebp = 0xffu32;
                pc = 248; continue;
            }
            247 => {
                m.w32(esp.wrapping_add(0x34u32), ecx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x662u32)))) & 0xffff);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x30u32), 0x96u32);
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x3cu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w16(edi, (0xffffu32) as u16);
                pc = 248; continue;
            }
            248 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edx, ecx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                if fl.cf { pc = 195; continue; }
                pc = 249; continue;
            }
            249 => {
                pc = 258; continue;
            }
            250 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 258; continue;
            }
            251 => {
                edi = 0xffffu32;
                m.w16(esi.wrapping_add(0x15cu32), (0x0u32) as u16);
                m.w8(esi.wrapping_add(0x5cu32), (0x1u32) as u8);
                m.w16(esi.wrapping_add(0x130cu32), ((edi & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x658u32), (0x0u32) as u16);
                m.w16(esi.wrapping_add(0x65au32), (0x0u32) as u16);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 253; continue; }
                pc = 252; continue;
            }
            252 => {
                m.w16(esi.wrapping_add(0x65eu32), ((edi & 0xffff)) as u16);
                pc = 254; continue;
            }
            253 => {
                eax = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ebp).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x65eu32), ((eax & 0xffff)) as u16);
                pc = 254; continue;
            }
            254 => {
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x134u32))), (edx & 0xffff), 16).1;
                if !fl.zf { pc = 256; continue; }
                pc = 255; continue;
            }
            255 => {
                m.w16(esi.wrapping_add(0x660u32), ((edi & 0xffff)) as u16);
                pc = 257; continue;
            }
            256 => {
                ecx = m.r32(0x6dd7a4u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(ecx).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x660u32), ((edx & 0xffff)) as u16);
                pc = 257; continue;
            }
            257 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 258; continue;
            }
            258 => {
                ecx = m.r32(esp.wrapping_add(0x340u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x33c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X006079D0` place_ships
pub fn f_006079d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esp.wrapping_add(0x4cu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x4u32));
                edi = m.r32(0x6dd7a4u32);
                edx = m.r32(esi.wrapping_add(0xad4u32));
                ebp = m.r32(esi.wrapping_add(0xad8u32));
                ecx = m.r32(eax);
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(eax.wrapping_add(0x1cu32));
                ebx = m.r32(eax.wrapping_add(0x20u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, ebp, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sar(edi, 0xdu32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::sar(ebx, 0xdu32, 32, fl); fl = f;
                ebx = r;
                edx = ebx;
                eax = edi;
                let (r, f) = lift::imul(edx, ebx); fl = f;
                edx = r;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x3cu32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x3cu32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x44u32), fp.st(0)); fp.pop();
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = eax;
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = 0xb5u32;
                ebx = edi;
                pc = 3; continue;
            }
            2 => {
                eax = edi;
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = eax;
                eax = ebx;
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebx = eax;
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x3cu32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x44u32), fp.st(0)); fp.pop();
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = edi.wrapping_add(edi.wrapping_mul(4));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ebx.wrapping_add(ebx.wrapping_mul(4));
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::sar(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                ebp = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::add(edx, 0x1100u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, ebp, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                let (r, f) = lift::sar(ebp, 0x8u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, 0xf02u32, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ecx = eax;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(ecx, 0x9u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x9u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                pc = 5; continue;
            }
            5 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x658u32))), (eax & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 25; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = ebx.wrapping_add(ebx.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x30u32), 0x1u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x38u32), eax);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = ebx.wrapping_add(ebx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = edi.wrapping_add(edi.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = edi.wrapping_add(edi.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = ebx.wrapping_add(ebx.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = edi.wrapping_add(edi.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), ecx);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xadcu32));
                ecx = 0xa205b063u32;
                let (r, f) = lift::imul(edx, 0x44bf19b3u32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::logic((ecx & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), ecx);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let r = ecx | 0x80000000u32; fl = lift::logic(r, 32);
                ecx = r;
                pc = 11; continue;
            }
            10 => {
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                pc = 11; continue;
            }
            11 => {
                m.w32(esi.wrapping_add(0xadcu32), ecx);
                let r = ecx & 0x7fu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(ecx, 0x40u32, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(eax.wrapping_add(edx));
                m.w16(edx.wrapping_add(0xau32), ((ecx & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0xadcu32));
                let (r, f) = lift::imul(edx, 0x44bf19b3u32); fl = f;
                edx = r;
                ecx = 0xa205b063u32;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::logic((ecx & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), ecx);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let r = ecx | 0x80000000u32; fl = lift::logic(r, 32);
                ecx = r;
                pc = 14; continue;
            }
            13 => {
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                pc = 14; continue;
            }
            14 => {
                m.w32(esi.wrapping_add(0xadcu32), ecx);
                let r = ecx & 0x7fu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(ecx, 0x40u32, 32); fl = f;
                ecx = r;
                edx = ebp;
                let (r, f) = lift::imul(ecx, edi); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(eax.wrapping_add(ecx));
                m.w16(ecx.wrapping_add(0xeu32), ((edx & 0xffff)) as u16);
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(eax.wrapping_add(edx));
                edx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x34u32), ecx);
                ecx = m.r32(ecx.wrapping_add(0x4u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx).wrapping_add(0x1u32)))) & 0xff);
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub((ecx & 0xff), 0x5u32, 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shl((ecx & 0xff), 0x4u32, 8, fl); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(edx.wrapping_add(0x11u32), ((ecx & 0xff)) as u8);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(eax.wrapping_add(ecx));
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x11u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(0x24u32), ((edx & 0xffff)) as u16);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(eax.wrapping_add(0x4u32));
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_mul(2).wrapping_add(0x67ab80u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xeu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                edx = eax.wrapping_add(edx).wrapping_add(0x168u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x40u32));
                edx = ecx.wrapping_add(edx).wrapping_add(0x168u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 0x168u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 0x168u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax)))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x6197a0, ecx, &a); }
                esp = esp.wrapping_add(20);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 24; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(edx.wrapping_add(ecx));
                { let a = []; eax = h.call_at(esp, m, 0x62125b, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(ecx.wrapping_add(eax).wrapping_add(0x4u32));
                { let a = []; eax = h.call_at(esp, m, 0x62125b, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                let (r, f) = lift::sar(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sar(ebp, 0x8u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(edx, 0x1100u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, 0xf02u32, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(0x14u32), edx);
                if (fl.cf || fl.zf) { pc = 24; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 19; continue;
            }
            19 => {
                ecx = eax;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(ecx, 0x9u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x9u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                pc = 24; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x30u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                ebp = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sar(ebp, 0x8u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(eax, 0x1100u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, 0xf02u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x18u32))), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.cf || fl.zf) { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = ecx.wrapping_add(0xffffffffu32);
                pc = 19; continue;
            }
            23 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sar(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x30u32), eax);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), ecx);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 25; continue;
            }
            25 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x44u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x44u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x44u32), fp.st(0)); fp.pop();
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ebp = 0xf02u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = edi.wrapping_add(edi.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x1100u32;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx;
                let (r, f) = lift::sar(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = ebx.wrapping_add(ebx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                edx = eax;
                m.w32(esp.wrapping_add(0x40u32), eax);
                let (r, f) = lift::sar(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub((edx & 0xffff), 0x1u32, 16).1;
                if (fl.cf || fl.zf) { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = edx.wrapping_add(0xffffffffu32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x44u32), eax);
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x9u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x9u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x65au32))), 0x0u32, 16).1;
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                if (fl.cf || fl.zf) { pc = 48; continue; }
                pc = 28; continue;
            }
            28 => {
                edx = ebx.wrapping_add(ebx.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                edx = edx.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), edx);
                edx = ebx.wrapping_add(ebx.wrapping_mul(4));
                edx = edx.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                edx = edi.wrapping_add(edi.wrapping_mul(4));
                edx = edx.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = edi.wrapping_add(edi.wrapping_mul(4));
                edx = edx.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x38u32), edx);
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(esi.wrapping_add(0xadcu32));
                eax = 0xa205b063u32;
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 32; continue;
            }
            31 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 32; continue;
            }
            32 => {
                edx = eax;
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let r = edx & 0x7fu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                let (r, f) = lift::sub(edx, 0x40u32, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::imul(edx, ebx); fl = f;
                edx = r;
                let (r, f) = lift::shr(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                m.w16(eax.wrapping_add(0xau32), ((edx & 0xffff)) as u16);
                ecx = m.r32(esi.wrapping_add(0xadcu32));
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                eax = 0xa205b063u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 35; continue;
            }
            34 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 35; continue;
            }
            35 => {
                edx = eax;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                let r = edx & 0x7fu32; fl = lift::logic(r, 32);
                edx = r;
                eax = ebp;
                let (r, f) = lift::sub(edx, 0x40u32, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::shr(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w16(ecx.wrapping_add(0xeu32), ((eax & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x4u32));
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                edx = m.r32(0x6dd7a4u32);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx).wrapping_add(0x1u32)))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub((ecx & 0xff), 0x5u32, 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shl((ecx & 0xff), 0x4u32, 8, fl); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0x11u32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x11u32)))) & 0xffff);
                m.w16(eax.wrapping_add(0x24u32), ((edx & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                let r = u32::from(m.r8(eax.wrapping_add(0x12au32))) | 0x1u32; fl = lift::logic(r, 8);
                m.w8(eax.wrapping_add(0x12au32), (r) as u8);
                pc = 37; continue;
            }
            37 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                ecx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(eax.wrapping_add(0x4u32));
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx).wrapping_add(0xeu32)))) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_mul(2).wrapping_add(0x67ab80u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xeu32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                edx = eax.wrapping_add(edx).wrapping_add(0x168u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x40u32));
                edx = ecx.wrapping_add(edx).wrapping_add(0x168u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 0x168u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 0x168u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x6197a0, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                ecx = eax.wrapping_add(0x1u32);
                fl = lift::sub(ecx, edx, 32).1;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                if !fl.cf { pc = 47; continue; }
                pc = 38; continue;
            }
            38 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                { let a = []; eax = h.call_at(esp, m, 0x62125b, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 43; continue; }
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x658u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32));
                { let a = []; eax = h.call_at(esp, m, 0x62125b, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 43; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x38u32));
                ebp = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x30u32), eax);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                edx = 0x1100u32;
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                ebp = 0xf02u32;
                let (r, f) = lift::sar(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                m.w32(esp.wrapping_add(0x14u32), edx);
                if (fl.cf || fl.zf) { pc = 47; continue; }
                pc = 41; continue;
            }
            41 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 42; continue;
            }
            42 => {
                ecx = eax;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, ebx); fl = f;
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(ecx, 0x9u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x9u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                pc = 47; continue;
            }
            43 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x3cu32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 46; continue; }
                pc = 44; continue;
            }
            44 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x30u32), eax);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                ebp = eax;
                let (r, f) = lift::sar(ebp, 0x8u32, 32, fl); fl = f;
                ebp = r;
                eax = 0x1100u32;
                m.w32(esp.wrapping_add(0x2cu32), edx);
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ebp = 0xf02u32;
                let (r, f) = lift::sar(edx, 0x8u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebp, edx, 32); fl = f;
                ebp = r;
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x18u32))), 0x1u32, 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.cf || fl.zf) { pc = 47; continue; }
                pc = 45; continue;
            }
            45 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = ecx.wrapping_add(0xffffffffu32);
                pc = 42; continue;
            }
            46 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sar(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 47; continue;
            }
            47 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65au32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 29; continue; }
                pc = 48; continue;
            }
            48 => {
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

/// `0X00633AF0` handle_start_siege
pub fn f_00633af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6684ebu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6dfc14u32);
                m.w32(0x0u32, esp);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4u32)))) & 0xff);
                edx = m.r32(0x6de508u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                esi = edx.wrapping_add(ecx.wrapping_mul(8));
                if !fl.zf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x76cu32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x34u32, 8).1;
                if fl.cf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((eax & 0xff), 0x36u32, 8).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 6; continue;
            }
            6 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edx, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(ebx, 0x3u32, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x762u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((edx & 0xff), 0x34u32, 8).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 11; continue;
            }
            11 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edx, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 9; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(ebx, 0x3u32, 32).1;
                if (fl.sf != fl.of) { pc = 27; continue; }
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x650u32);
                m.w8(edi.wrapping_add(0x101au32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x30u32), 0x0u32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x627880, ecx, &a); }
                pc = 16; continue;
            }
            15 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 16; continue;
            }
            16 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4u32)))) & 0xff);
                ebx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                m.w32(edi.wrapping_add(0x8u32), eax);
                m.w8(eax.wrapping_add(0x5d3u32), ((ecx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(edi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x629c60, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x19u32)))) & 0xff);
                eax = edx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(ebp).wrapping_add(0x39cu32))), 0x5u32, 8).1;
                if fl.cf { pc = 27; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if fl.zf { pc = 27; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x6de4b8u32)))) & 0xff);
                ecx = edx;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                edx = ecx.wrapping_add(edx.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x6de4b8u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edx.wrapping_mul(4).wrapping_add(0x6ddbc0u32)))) & 0xff);
                esi = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_add(esi.wrapping_mul(4));
                fl = lift::sub((edx & 0xff), u32::from(m.r8(ecx.wrapping_mul(4).wrapping_add(0x6ddbc0u32))), 8).1;
                if !fl.zf { pc = 27; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(8));
                eax = m.r32(0x6df270u32);
                eax = m.r32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x1020u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 26; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dfc14u32))) & 0xff);
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x5u32))), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x7u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x8u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(eax.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 22; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), 0x8bu32);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x9u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edx;
                edx = (edx & !0xff) | ((0x5u32) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x8u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x9u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edx;
                edx = (edx & !0xff) | ((0x5u32) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x8u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                m.w8(esp.wrapping_add(0x1au32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x5u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edx;
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x4u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x9u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edx;
                edx = (edx & !0xff) | ((0x5u32) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x8u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                m.w8(esp.wrapping_add(0x1bu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x5u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = edx;
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::add((eax & 0xff), 0x4u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x19u32)))) & 0xff);
                { let r = i32::from(eax as u8 as i8) * i32::from(edx as u8 as i8); eax = (eax & !0xffff) | (r as u32 & 0xffff); fl.cf = r != i32::from(r as i8); fl.of = fl.cf; }
                m.w8(esp.wrapping_add(0x19u32), ((eax & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dfc14u32))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x1eu32), ((eax & 0xff)) as u8);
                eax = m.r32(0x6de4b4u32);
                m.w8(esp.wrapping_add(0x1du32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::add(eax, 0x1500u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x14u32);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w8(esp.wrapping_add(0x23u32), ((edx & 0xff)) as u8);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x462280 => f_00462280(m, h, ecx, esp),
        0x4665d0 => f_004665d0(m, h, ecx, esp),
        0x4db7c0 => f_004db7c0(m, h, ecx, esp),
        0x4fdf50 => f_004fdf50(m, h, ecx, esp),
        0x50ac40 => f_0050ac40(m, h, ecx, esp),
        0x50b250 => f_0050b250(m, h, ecx, esp),
        0x511270 => f_00511270(m, h, ecx, esp),
        0x52c019 => f_0052c019(m, h, ecx, esp),
        0x52c6c0 => f_0052c6c0(m, h, ecx, esp),
        0x52d6b0 => f_0052d6b0(m, h, ecx, esp),
        0x52e700 => f_0052e700(m, h, ecx, esp),
        0x52e960 => f_0052e960(m, h, ecx, esp),
        0x542560 => f_00542560(m, h, ecx, esp),
        0x544d4f => f_00544d4f(m, h, ecx, esp),
        0x544d79 => f_00544d79(m, h, ecx, esp),
        0x544e50 => f_00544e50(m, h, ecx, esp),
        0x545650 => f_00545650(m, h, ecx, esp),
        0x547534 => f_00547534(m, h, ecx, esp),
        0x54762f => f_0054762f(m, h, ecx, esp),
        0x547650 => f_00547650(m, h, ecx, esp),
        0x547f10 => f_00547f10(m, h, ecx, esp),
        0x547fd0 => f_00547fd0(m, h, ecx, esp),
        0x5482d0 => f_005482d0(m, h, ecx, esp),
        0x549130 => f_00549130(m, h, ecx, esp),
        0x549b20 => f_00549b20(m, h, ecx, esp),
        0x549b3e => f_00549b3e(m, h, ecx, esp),
        0x54a860 => f_0054a860(m, h, ecx, esp),
        0x54a980 => f_0054a980(m, h, ecx, esp),
        0x54aa70 => f_0054aa70(m, h, ecx, esp),
        0x54aae0 => f_0054aae0(m, h, ecx, esp),
        0x54ae50 => f_0054ae50(m, h, ecx, esp),
        0x54afa0 => f_0054afa0(m, h, ecx, esp),
        0x54b080 => f_0054b080(m, h, ecx, esp),
        0x54b230 => f_0054b230(m, h, ecx, esp),
        0x54b500 => f_0054b500(m, h, ecx, esp),
        0x54b960 => f_0054b960(m, h, ecx, esp),
        0x54bd80 => f_0054bd80(m, h, ecx, esp),
        0x54bdd0 => f_0054bdd0(m, h, ecx, esp),
        0x54beb0 => f_0054beb0(m, h, ecx, esp),
        0x54bf80 => f_0054bf80(m, h, ecx, esp),
        0x605420 => f_00605420(m, h, ecx, esp),
        0x6079d0 => f_006079d0(m, h, ecx, esp),
        0x633af0 => f_00633af0(m, h, ecx, esp),
        _ => return None,
    }))
}
