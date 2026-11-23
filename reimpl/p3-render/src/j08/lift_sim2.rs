//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

/// `0X004DDF40` st_captain_offer
pub fn f_004ddf40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662a9eu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x2c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                ecx = m.r32(ebp);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edi = m.r32(ecx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0xcu32));
                ebx = m.r32(ecx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x8u32));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x10u32));
                eax = ecx.wrapping_add(eax.wrapping_mul(8));
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x28u32), edi);
                eax = m.r32(eax.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x51a850, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                m.w32(esp.wrapping_add(0x44u32), 0x0u32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 37; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = eax;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd892u32))) & 0xffff);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                fl = lift::sub(edi, edx, 32).1;
                if !fl.cf { pc = 37; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6dd894u32);
                esi = m.r32(0x6dd7a4u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x1cu32), 0x1u32);
                pc = 5; continue;
            }
            4 => {
                edx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(esi).wrapping_add(0x42u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, edi, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                eax = edx;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if fl.zf { pc = 14; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x24u32), esi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                pc = 6; continue;
            }
            6 => {
                eax = esi;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 13; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6dd894u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                if fl.cf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                pc = 6; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx).wrapping_add(0x42u32)))) & 0xffff);
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 12; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(ebp);
                m.w32(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x8u32), ebx);
                pc = 14; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 37; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(0x6dd7a4u32);
                esi = ebx.wrapping_add(ebx.wrapping_mul(2));
                let (r, f) = lift::shl(esi, 0x7u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, edx, 32); fl = f;
                esi = r;
                edx = m.r32(0x6dd7a0u32);
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3fu32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esp.wrapping_add(0x13u32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xbu32)))) & 0xff);
                edx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xau32)))) & 0xff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x9u32)))) & 0xff);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x51eb851fu32;
                { let r = u64::from(eax) * u64::from(edx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                eax = lift::sx8(u32::from(m.r8(esp.wrapping_add(0x13u32))));
                let (r, f) = lift::shr(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x6u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = ebx;
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax & 0x2u32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::sub((edx & 0xff), 0x3u32, 8).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 33; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::sub(edx, eax, 32).1;
                if fl.cf { pc = 33; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(edx, m.r32(ecx), 32).1;
                if (fl.cf || fl.zf) { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(0x6de4b0u32);
                m.w8(esp.wrapping_add(0x30u32), (0x4fu32) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                ebx = eax;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebx, edx, 32).1;
                if fl.cf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x13au32)))) & 0xff);
                pc = 19; continue;
            }
            19 => {
                m.w8(esp.wrapping_add(0x31u32), ((eax & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x34u32), ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x36u32), ((eax & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x1cu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x38u32), ((edx & 0xffff)) as u16);
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = (eax & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x1du32)))) & 0xffff);
                ecx = esp.wrapping_add(0x2cu32);
                m.w16(esp.wrapping_add(0x3au32), ((eax & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 36; continue;
            }
            20 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 37; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x4de4ccu32)))) & 0xff);
                match m.r32(edx.wrapping_mul(4).wrapping_add(0x4de4b0u32)) { 0x4de199 => { pc = 22; continue; } 0x4de1e4 => { pc = 25; continue; } 0x4de230 => { pc = 28; continue; } 0x4de256 => { pc = 29; continue; } 0x4de26d => { pc = 30; continue; } 0x4de2ab => { pc = 31; continue; } 0x4de48a => { pc = 37; continue; } _ => unreachable!() }
            }
            22 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 31; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 31; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = ecx.wrapping_add(eax.wrapping_mul(8));
                let (r, f) = lift::add(m.r32(eax), 0x20u32, 32); fl = f;
                m.w32(eax, r);
                pc = 32; continue;
            }
            25 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 31; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 31; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50a580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                pc = 31; continue;
            }
            28 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x138u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(ebp);
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                edx = 0x41u32;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(m.r32(eax), edx, 32); fl = f;
                m.w32(eax, r);
                pc = 32; continue;
            }
            29 => {
                ecx = m.r32(ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = ecx.wrapping_add(eax.wrapping_mul(8));
                let (r, f) = lift::add(m.r32(eax), 0x40u32, 32); fl = f;
                m.w32(eax, r);
                pc = 32; continue;
            }
            30 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                m.w16(esi.wrapping_add(0x134u32), ((edx & 0xffff)) as u16);
                ebx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ebx, 0x6u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(0x6de508u32);
                m.w16(eax.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x994u32), (0xffffu32) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                ebx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ebx, 0x6u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(0x6de508u32);
                m.w32(eax.wrapping_add(ebx.wrapping_mul(8)).wrapping_add(0x990u32), edx);
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ebx = m.r32(ecx.wrapping_add(0x4bcu32));
                edx = m.r32(ecx);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                m.w32(ecx.wrapping_add(0x4bcu32), ebx);
                ebx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(ecx, edx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w8(edi.wrapping_add(0xfu32), ((ebx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                ecx = esi.wrapping_add(0x42u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(edx.wrapping_mul(8));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x526b00, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esp.wrapping_add(0x30u32), (0x51u32) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x31u32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x34u32), ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x36u32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x1cu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x38u32), ((ecx & 0xffff)) as u16);
                ecx = 0x6dd730u32;
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xffff);
                eax = esp.wrapping_add(0x2cu32);
                m.w16(esp.wrapping_add(0x3au32), ((edx & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6530, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esp.wrapping_add(0x30u32), (0x50u32) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x31u32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x34u32), ((edx & 0xffff)) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x36u32), ((eax & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x15eu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x38u32), ((ecx & 0xffff)) as u16);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x1cu32)))) & 0xffff);
                let (r, f) = lift::shl(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                m.w16(esp.wrapping_add(0x3au32), ((ecx & 0xffff)) as u16);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x1du32)))) & 0xffff);
                let (r, f) = lift::add((ecx & 0xffff), (eax & 0xffff), 16); fl = f;
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                m.w16(esp.wrapping_add(0x3au32), ((ecx & 0xffff)) as u16);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6530, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                m.w16(ecx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x6u32), (0x29u32) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(ebp);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(8)));
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                let (r, f) = lift::add(ecx, 0xe00u32, 32); fl = f;
                ecx = r;
                m.w32(eax, ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(ebp);
                m.w32(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x8u32), ecx);
                edx = m.r32(ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x8u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x10u32), 0x1u32);
                pc = 32; continue;
            }
            32 => {
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x44u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x51a880, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1u32;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            33 => {
                edx = m.r32(0x6de4b0u32);
                m.w8(esp.wrapping_add(0x30u32), (0x4eu32) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                ebx = eax;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebx, edx, 32).1;
                if fl.cf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x13au32)))) & 0xff);
                pc = 35; continue;
            }
            35 => {
                m.w8(esp.wrapping_add(0x31u32), ((eax & 0xff)) as u8);
                edx = (edx & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x34u32), ((edx & 0xffff)) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x36u32), ((eax & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x15eu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x38u32), ((edx & 0xffff)) as u16);
                edx = esp.wrapping_add(0x2cu32);
                eax = (eax & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x1cu32)))) & 0xffff);
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                m.w16(esp.wrapping_add(0x3au32), ((eax & 0xffff)) as u16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x1du32)))) & 0xffff);
                let (r, f) = lift::add((eax & 0xffff), (ecx & 0xffff), 16); fl = f;
                eax = (eax & !0xffff) | ((r) & 0xffff);
                m.w16(esp.wrapping_add(0x3eu32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 36; continue;
            }
            36 => {
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6530, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 37; continue;
            }
            37 => {
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x44u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x51a880, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x3cu32));
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
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x38);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004E63E4` st_tavern_traveller_deadline
pub fn f_004e63e4(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662c38u32);
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
                edi = ecx;
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x51a850, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x28u32), ebx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 41; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = m.r32(0x6de4b0u32);
                esi = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(esi, 0x7u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                ebp = ecx;
                let r = ebp & 0xffu32; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(ebp, edx, 32).1;
                if fl.cf { pc = 13; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0x11u32, 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub((ecx & 0xffff), 0x12u32, 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((ecx & 0xffff), 0x17u32, 16).1;
                if !fl.cf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(eax);
                let (r, f) = lift::add(ecx, 0x20u32, 32); fl = f;
                ecx = r;
                pc = 43; continue;
            }
            6 => {
                ebx = m.r32(edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xcu32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ebp = ebx.wrapping_add(eax.wrapping_mul(8));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                edi = 0x10u32;
                pc = 8; continue;
            }
            8 => {
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x6u32))), (edi & 0xffff), 16).1;
                ecx = ebx.wrapping_add(ecx.wrapping_mul(8));
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0x8u32))), 16).1;
                if fl.zf { pc = 41; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 8; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x157u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if (fl.cf || fl.zf) { pc = 39; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(0x157u32), ((eax & 0xff)) as u8);
                pc = 40; continue;
            }
            13 => {
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(eax.wrapping_add(0xbu32))), 8).1;
                if !fl.zf { pc = 32; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0x3u32, 16).1;
                if fl.zf { pc = 29; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub((ecx & 0xffff), 0x2u32, 16).1;
                if fl.zf { pc = 29; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0xau32)))) & 0xffff);
                m.w8(esp.wrapping_add(0x14u32), (0x18u32) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x15u32), ((ecx & 0xff)) as u8);
                ecx = m.r32(0x6de4b4u32);
                m.w16(esp.wrapping_add(0x1cu32), ((edx & 0xffff)) as u16);
                edx = m.r32(eax.wrapping_add(0x10u32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.cf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xcu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x18u32), ((ebx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((eax & 0xffff)) as u16);
                pc = 21; continue;
            }
            18 => {
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x8u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, 0x4u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w16(esp.wrapping_add(0x18u32), (0x2u32) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((ebx & 0xffff)) as u16);
                pc = 21; continue;
            }
            20 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w16(esp.wrapping_add(0x18u32), (0x1u32) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xcu32)))) & 0xffff);
                eax = 0x5u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(edx, eax); fl = f;
                edx = r;
                eax = 0x66666667u32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w16(esp.wrapping_add(0x1au32), ((edx & 0xffff)) as u16);
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1au32));
                edx = m.r32(eax);
                ebx = m.r32(eax.wrapping_add(0x4b8u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(eax, edx);
                edx = m.r32(esp.wrapping_add(0x1au32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                m.w32(eax.wrapping_add(0x4b8u32), ebx);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6530, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 23; continue;
            }
            23 => {
                ebx = m.r32(edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                ebp = ebx.wrapping_add(edx.wrapping_mul(8));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xcu32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 24; continue;
            }
            24 => {
                edi = 0x10u32;
                pc = 25; continue;
            }
            25 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x6u32))), (edi & 0xffff), 16).1;
                ecx = ebx.wrapping_add(eax.wrapping_mul(8));
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0x8u32))), 16).1;
                if fl.zf { pc = 41; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 25; continue; }
                pc = 28; continue;
            }
            28 => {
                pc = 11; continue;
            }
            29 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = 0x40u32;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x138u32)))) & 0xffff);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = 0x40u32;
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(eax);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 43; continue;
            }
            31 => {
                ecx = m.r32(eax);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 43; continue;
            }
            32 => {
                ecx = m.r32(0x6de4b4u32);
                edx = m.r32(eax.wrapping_add(0x10u32));
                fl = lift::sub(edx, ecx, 32).1;
                if !fl.cf { pc = 42; continue; }
                pc = 33; continue;
            }
            33 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0xau32)))) & 0xffff);
                m.w8(esp.wrapping_add(0x14u32), (0x18u32) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x15u32), ((ecx & 0xff)) as u8);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esp.wrapping_add(0x18u32), (0x2u32) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((ebx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1cu32), ((edx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6530, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = m.r32(edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0xcu32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                ebx = ebp.wrapping_add(edx.wrapping_mul(8));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 34; continue;
            }
            34 => {
                edi = 0x10u32;
                pc = 35; continue;
            }
            35 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                fl = lift::sub(u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x6u32))), (edi & 0xffff), 16).1;
                edx = ebp.wrapping_add(eax.wrapping_mul(8));
                if !fl.zf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebx.wrapping_add(0x8u32))), 16).1;
                if fl.zf { pc = 41; continue; }
                pc = 37; continue;
            }
            37 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 35; continue; }
                pc = 38; continue;
            }
            38 => {
                pc = 11; continue;
            }
            39 => {
                m.w8(esi.wrapping_add(0x157u32), (0x0u32) as u8);
                pc = 40; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x517770, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 41; continue;
            }
            41 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x51a880, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 44; continue;
            }
            42 => {
                ecx = m.r32(eax);
                let (r, f) = lift::add(ecx, 0x100u32, 32); fl = f;
                ecx = r;
                pc = 43; continue;
            }
            43 => {
                m.w32(eax, ecx);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x51a880, ecx, &a); }
                eax = 0x1u32;
                pc = 44; continue;
            }
            44 => {
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

/// `0X00500330` office_get_asset_value
pub fn f_00500330(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x2c6u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edi = edx.wrapping_add(ecx.wrapping_mul(8));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 3; continue;
            }
            2 => {
                eax = m.r32(0x672a88u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x673a48u32); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66f4bcu32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(0x672a4cu32);
                esi = eax;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x673a44u32); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66a918u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(0x672ac4u32);
                ebx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(ebx, esi, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x673a64u32); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66f4b8u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(0x672b78u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = ebx.wrapping_add(eax.wrapping_mul(4));
                pc = 3; continue;
            }
            3 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x2ceu32)))) & 0xffff);
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f220, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if !fl.cf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi.wrapping_add(0x75cu32));
                esi = (esi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(esi.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                pc = 7; continue;
            }
            6 => {
                esi = 0xffffu32;
                pc = 7; continue;
            }
            7 => {
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::sub(esi, 0xffffu32, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x2d0u32)))) & 0xffff);
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f220, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(edi.wrapping_add(0x75cu32));
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(esi.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                pc = 12; continue;
            }
            11 => {
                esi = 0xffffu32;
                pc = 12; continue;
            }
            12 => {
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::sub(esi, 0xffffu32, 32).1;
                if fl.cf { pc = 9; continue; }
                pc = 13; continue;
            }
            13 => {
                edi = ebp.wrapping_add(0x4u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(edi);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esi.wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x60u32, 32).1;
                if fl.cf { pc = 14; continue; }
                pc = 17; continue;
            }
            17 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx;
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

/// `0X00500690` office_get_weekly_production_in_lots
pub fn f_00500690(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0xcu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = ecx.wrapping_add(0xc4u32);
                pc = 1; continue;
            }
            1 => {
                { let v = f64::from(m.r32(edx) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x672f0cu32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0xcu32), ecx);
                m.w32(esi.wrapping_add(eax.wrapping_mul(4)), ecx);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                { let v = lift::fld32(m, 0x66bc60u32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            3 => {
                { let v = lift::fld32(m, 0x66bc5cu32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            4 => {
                lift::fst32(m, esi.wrapping_add(eax.wrapping_mul(4)), fp.st(0)); fp.pop();
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x4u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, 0x14u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
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

/// `0X00500BB0` office_get_average_purchase_prices_scaled
pub fn f_00500bb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edi = ebp.wrapping_add(0x3b8u32);
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x672c14u32)))) & 0xff);
                { let v = lift::fld32(m, edi); fp.push(v); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            3 => {
                { let v = lift::fld32(m, 0x66a918u32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            4 => {
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(ebx.wrapping_add(esi.wrapping_mul(4)), eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x14u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = 0x14u32;
                edi = ebp.wrapping_add(0x408u32);
                pc = 6; continue;
            }
            6 => {
                { let v = lift::fld32(m, edi); fp.push(v); }
                { let v = lift::fld32(m, 0x66dcd8u32); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(ebx.wrapping_add(esi.wrapping_mul(4)), eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
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
            _ => unreachable!(),
        }
    }
}

/// `0X00503B50` convoy_ai_plan_expedition
pub fn f_00503b50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 16; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6de4a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), (edx & 0xffff), 16).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                edi = edx;
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 16; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = u32::from(m.r8(esi.wrapping_add(0x14u32))) & 0xfdu32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0x14u32), (r) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6de52au32))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 16; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6de52bu32))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 16; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6df31cu32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 16; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 16; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                m.w8(esp.wrapping_add(0xcu32), (0x1u32) as u8);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xau32)))) & 0xffff);
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx).wrapping_add(0x8u32)))) & 0xffff);
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(0x14u32));
                esi = ecx;
                let (r, f) = lift::sar(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::sub(ecx, esi, 32); fl = f;
                ecx = r;
                esi = m.r32(eax.wrapping_add(0x18u32));
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x118u32)), 0x1388u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 7; continue; }
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            13 => {
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w32(esp.wrapping_add(0x14u32), 0x1eu32);
                m.w16(esp.wrapping_add(0x1cu32), ((ebp & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                m.w16(esp.wrapping_add(0x1eu32), (0x0u32) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                let r = (eax & 0xff) & 0x1fu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                edx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add((eax & 0xff), 0x14u32, 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6df2f0u32;
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x14u32), 0x1fu32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edi = eax.wrapping_add(0x4d8u32);
                pc = 15; continue;
            }
            15 => {
                m.w16(esp.wrapping_add(0x18u32), ((ebp & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((esi & 0xffff)) as u16);
                m.w8(esp.wrapping_add(0x20u32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w8(esp.wrapping_add(0x10u32), ((eax & 0xff)) as u8);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = esp.wrapping_add(0x1eu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x534f10, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = esp.wrapping_add(0x14u32);
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((esi & 0xffff), 0x4u32, 16).1;
                if fl.cf { pc = 15; continue; }
                pc = 16; continue;
            }
            16 => {
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

/// `0X00505A10` summarize_merchant_fleet
pub fn f_00505a10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                edi = ecx;
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(edx, eax);
                m.w32(ebp, eax);
                m.w32(ecx, eax);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(ecx, eax);
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(edi.wrapping_add(0xf4u32))), 16).1;
                if fl.cf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ebp, 0x0u32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax, 0x0u32);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                pc = 3; continue;
            }
            3 => {
                eax = ebx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edi.wrapping_add(0x4u32));
                let (r, f) = lift::shl(esi, 0x7u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x134u32)))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x11u32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((eax & 0xffff), 0xeu32, 16).1;
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(edi.wrapping_add(0xf2u32))), 16).1;
                if !fl.cf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(m.r32(edx), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(edx, r);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ecx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x40u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(ecx, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                { let v = lift::fld32(m, esi.wrapping_add(0x12cu32)); fp.push(v); }
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x130u32)))) & 0xffff);
                { let v = lift::fld32(m, 0x66c1e8u32); let d = fp.st(0); fp.set(0, d * v); }
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, d / v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(ebp);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(ebp, edx);
                eax = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(ecx);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x14u32));
                m.w32(ecx, edx);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(edi.wrapping_add(0xf4u32))), 16).1;
                if fl.cf { pc = 2; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(ecx, 0x1u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(ebp);
                esi = m.r32(esp.wrapping_add(0x28u32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ebp, eax);
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                eax = m.r32(esi);
                ecx = edx;
                let (r, f) = lift::sar(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx;
                let (r, f) = lift::shr(edx, 0x1fu32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esi, eax);
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

/// `0X00515A60` find_next_unused_map_location
pub fn f_00515a60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(0x6de4b0u32);
                esp = esp.wrapping_add(0xc);
                fl = lift::logic(eax & eax, 32);
                ecx = (ecx & !0xff) | ((0x1u32) & 0xff);
                if (fl.cf || fl.zf) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x6de4b8u32)))) & 0xff);
                m.w8(esp.wrapping_add(edx).wrapping_add(0x4u32), ((ecx & 0xff)) as u8);
                if (!fl.cf && !fl.zf) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ecx.wrapping_add(0x6dd7acu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xeu32)))) & 0xff);
                eax = edx;
                fl = lift::sub(eax, 0x28u32, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w8(esp.wrapping_add(eax).wrapping_add(0x4u32), (0x1u32) as u8);
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, 0x14u32, 32).1;
                if fl.cf { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(eax, 0x28u32, 32).1;
                if fl.cf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 11; continue;
            }
            11 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(eax).wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 8; continue;
            }
            13 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 14; continue;
            }
            14 => {
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00515B80` pirate_band_draw_hideout_pieces
pub fn f_00515b80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_mul(4).wrapping_add(0x6ddbc0u32)))) & 0xff);
                m.w32(ecx, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x8u32))), (eax & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(edi.wrapping_add(0x4u32));
                esi = edx.wrapping_add(eax.wrapping_mul(8));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x6u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0xc8u32, 8).1;
                if !fl.cf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3u32)))) & 0xff);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                edx = lift::sx8(u32::from(m.r8(ecx.wrapping_add(0x672fc0u32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 4; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x4u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005172A0` plan_route
pub fn f_005172a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = m.r32(0x6de4b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), 0x2u32);
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x22u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1eu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x16u32), ((eax & 0xffff)) as u16);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x6de4b8u32)))) & 0xff);
                m.w16(esp.wrapping_add(0x14u32), ((edx & 0xffff)) as u16);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6ddbb0u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6ddbb4u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x18u32), ((edx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((eax & 0xffff)) as u16);
                m.w8(esi.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                pc = 3; continue;
            }
            2 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1eu32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x22u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x28u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x14u32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x2cu32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x16u32), ((edx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x18u32), ((eax & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((ecx & 0xffff)) as u16);
                m.w8(esi.wrapping_add(0x38u32), (0xffu32) as u8);
                pc = 3; continue;
            }
            3 => {
                eax = 0x7fffu32;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w16(esi.wrapping_add(0x1cu32), ((eax & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x20u32), ((eax & 0xffff)) as u16);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x36u32)))) & 0xff);
                m.w16(esi.wrapping_add(0x34u32), ((ebx & 0xffff)) as u16);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                m.w16(esi.wrapping_add(0x2u32), ((ebx & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0x28u32), ebx);
                m.w32(esi.wrapping_add(0x2cu32), ebx);
                if fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x24u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(eax);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 8; continue;
            }
            8 => {
                edi = esi.wrapping_add(0x24u32);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edi, ebx);
                ecx = m.r32(0x6cbdc8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x445010, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(edi);
                m.w8(esi.wrapping_add(0x36u32), ((eax & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5169c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6de4b4u32);
                ebx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                m.w32(esi.wrapping_add(0x48u32), eax);
                fl = lift::sub(edx, m.r32(0x6dfc14u32), 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6cbdd8u32);
                eax = ebx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x44f940, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(u32::from(m.r8(esi)), 0xffu32, 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = lift::sx16(u32::from(m.r16(esp.wrapping_add(0x1au32))));
                edx = lift::sx16(u32::from(m.r16(esp.wrapping_add(0x18u32))));
                eax = lift::sx16(u32::from(m.r16(esp.wrapping_add(0x16u32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = lift::sx16(u32::from(m.r16(esp.wrapping_add(0x1cu32))));
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0678u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, 0x621750, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 12; continue;
            }
            12 => {
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

/// `0X005187F0` ship_get_avg_prices_per_load
pub fn f_005187f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edi = ebp.wrapping_add(0xb4u32);
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x672c14u32)))) & 0xff);
                { let v = lift::fld32(m, edi); fp.push(v); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            3 => {
                { let v = lift::fld32(m, 0x66a918u32); let d = fp.st(0); fp.set(0, d * v); }
                pc = 4; continue;
            }
            4 => {
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(ebx.wrapping_add(esi.wrapping_mul(4)), eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x14u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = ebp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0xb4u32);
                pc = 6; continue;
            }
            6 => {
                { let v = lift::fld32(m, edi); fp.push(v); }
                { let v = lift::fld32(m, 0x66dcd8u32); let d = fp.st(0); fp.set(0, d * v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(ebx.wrapping_add(esi.wrapping_mul(4)), eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
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
            _ => unreachable!(),
        }
    }
}

/// `0X00521310` town_view_draw_building_chain
pub fn f_00521310(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebp = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(ebp.wrapping_add(0x75cu32));
                edi = edx.wrapping_add(eax.wrapping_mul(8));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub(eax, 0x38u32, 32).1;
                ecx = lift::sx8(u32::from(m.r8(eax.wrapping_add(0x672fc0u32))));
                if (!fl.cf && !fl.zf) { pc = 19; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x5214f4u32)))) & 0xff);
                match m.r32(edx.wrapping_mul(4).wrapping_add(0x5214c8u32)) { 0x521360 => { pc = 4; continue; } 0x521369 => { pc = 5; continue; } 0x5213a1 => { pc = 8; continue; } 0x5213b0 => { pc = 9; continue; } 0x5213bd => { pc = 10; continue; } 0x5213c8 => { pc = 12; continue; } 0x5213e0 => { pc = 14; continue; } 0x52140f => { pc = 16; continue; } 0x521418 => { pc = 17; continue; } 0x52142a => { pc = 18; continue; } 0x52143a => { pc = 19; continue; } _ => unreachable!() }
            }
            4 => {
                esi = lift::sx8(u32::from(m.r8(edi.wrapping_add(0x7u32))));
                pc = 19; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x6u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7u32)))) & 0xff);
                esi = eax;
                let (r, f) = lift::shl(esi, 0x10u32, 32, fl); fl = f;
                esi = r;
                pc = 19; continue;
            }
            7 => {
                let (r, f) = lift::add(eax, 0xffffffebu32, 32); fl = f;
                eax = r;
                esi = 0x3u32;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(esi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                esi = edx;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7u32)))) & 0xff);
                let r = esi & 0xffffu32; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let r = esi | edx; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            8 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x7a3u32)))) & 0xff);
                esi = edx;
                pc = 19; continue;
            }
            9 => {
                eax = m.r32(ebp.wrapping_add(0x2d4u32));
                fl = lift::sub(eax, 0x9c4u32, 32).1;
                if (fl.sf == fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            11 => {
                fl = lift::sub(eax, 0x1388u32, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esi = 0x1u32;
                pc = 19; continue;
            }
            13 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, 0x2710u32, 32).1;
                edx = (edx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                esi = edx;
                pc = 19; continue;
            }
            14 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if fl.cf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x2c0u32)))) & 0xff);
                ecx = 0xau32;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x6de4b8u32)))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672d28u32)))) & 0xff);
                esi = edx;
                pc = 19; continue;
            }
            16 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7u32)))) & 0xff);
                esi = eax;
                pc = 19; continue;
            }
            17 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7u32)))) & 0xff);
                esi = edx;
                let (r, f) = lift::shl(esi, 0x10u32, 32, fl); fl = f;
                esi = r;
                let r = esi | 0xffu32; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            18 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7u32)))) & 0xff);
                esi = eax;
                let (r, f) = lift::shl(esi, 0x10u32, 32, fl); fl = f;
                esi = r;
                let r = esi | 0x1ffu32; fl = lift::logic(r, 32);
                esi = r;
                pc = 19; continue;
            }
            19 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x6u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 25; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x28u32, 8).1;
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 24; continue;
            }
            23 => {
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672bb4u32)))) & 0xff);
                ecx = edx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1u32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x5891c0, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 25; continue;
            }
            25 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x4u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 26; continue;
            }
            26 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00521580` town_view_redraw_all_buildings
pub fn f_00521580(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = esi.wrapping_add(0x7b8u32);
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x7a8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x15u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x76cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                edi = esi.wrapping_add(0x7c0u32);
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x7a8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e51acu32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x16u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x2u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x76cu32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esi.wrapping_add(0x7a8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(edi.wrapping_mul(2)).wrapping_add(0x7d0u32)))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = 0x10u32;
                pc = 12; continue;
            }
            11 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(esi).wrapping_add(0x7f0u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(edi.wrapping_mul(2)).wrapping_add(0x7d1u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x10u32, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x762u32)))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x764u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(0x6de4a8u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x784u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 14; continue;
            }
            14 => {
                ebx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x530400, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x2c4u32)))) & 0xffff);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x2cau32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2c6u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f74f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x802u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2cu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x803u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x2f0u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if (fl.cf || fl.zf) { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x672a48u32))) & 0xff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x803u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x802u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x5891c0, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 19; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0x8u32))) & 0x4u32, 8);
                if !fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 19; continue;
            }
            19 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x2ceu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x2d0u32)))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(0x6de4a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x805u32)))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x804u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2du32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0x76cu32));
                let r = eax & 0x80u32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((eax & 0xff), 0x80u32, 8).1;
                if !fl.zf { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2c0u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                { let v = lift::fld32(m, edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x818u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66db78u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                fl = lift::sub(eax, 0x19u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = 0x19u32;
                pc = 25; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80bu32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x80au32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1bu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                edx = m.r32(esi.wrapping_add(0x76cu32));
                let r = edx & 0x20000u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, 0x20000u32, 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x809u32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x808u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 27; continue;
            }
            27 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x76au32)))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x521310, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = m.r32(esi.wrapping_add(0x7a8u32));
                edi = m.r32(esi.wrapping_add(0x7acu32));
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esp.wrapping_add(0x18u32), edi);
                if (fl.zf || fl.sf != fl.of) { pc = 35; continue; }
                pc = 28; continue;
            }
            28 => {
                ebp = ebx.wrapping_add(0x1u32);
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 34; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esi.wrapping_add(0x7acu32));
                edx = m.r32(esi.wrapping_add(0x80cu32));
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(edi))), 0x81u32, 8).1;
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                let (r, f) = lift::imul(ecx, edi); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x17u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x590680, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 32; continue;
            }
            32 => {
                fl = lift::logic(edi & edi, 32);
                if (!fl.zf && fl.sf == fl.of) { pc = 30; continue; }
                pc = 33; continue;
            }
            33 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                pc = 34; continue;
            }
            34 => {
                fl = lift::logic(ebx & ebx, 32);
                if (!fl.zf && fl.sf == fl.of) { pc = 29; continue; }
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
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00522A00` town_update_defense_queue_preview
pub fn f_00522a00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x766u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x776u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub(eax, ecx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if !fl.cf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                esi = m.r32(ebx.wrapping_add(0x75cu32));
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x3u32))), 0x36u32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 5; continue;
            }
            4 => {
                let (r, f) = lift::add(edx, 0x3u32, 32); fl = f;
                edx = r;
                pc = 5; continue;
            }
            5 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(eax.wrapping_mul(8)).wrapping_add(0x4u32)))) & 0xffff);
                eax = edi;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                eax = edx.wrapping_add(edx.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0xcu32), eax);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x766u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x776u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(edi, ecx, 32).1;
                if !fl.cf { pc = 17; continue; }
                pc = 11; continue;
            }
            11 => {
                esi = eax;
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ebx.wrapping_add(0x75cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(edi.wrapping_mul(8))))) & 0xff);
                eax = edx.wrapping_add(edi.wrapping_mul(8));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                m.w32(esi, ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                m.w32(esi.wrapping_add(0x4u32), edx);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x36u32, 8).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x7u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2eu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5928a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                m.w16(esi.wrapping_add(0x8u32), ((eax & 0xffff)) as u16);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(esi, 0xcu32, 32); fl = f;
                esi = r;
                pc = 15; continue;
            }
            14 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                edx = (edx & !0xff00) | (((u32::from(m.r8(eax.wrapping_add(0x7u32)))) & 0xff) << 8);
                m.w16(esi.wrapping_add(0x8u32), ((edx & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x582770, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(ebp, 0x3u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(esi, 0x24u32, 32); fl = f;
                esi = r;
                pc = 15; continue;
            }
            15 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if fl.cf { pc = 12; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x593800, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X0052A7D0` calc_ship_current_value
pub fn f_0052a7d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x52a8b8u32)) { 0x52a7e9 => { pc = 2; continue; } 0x52a7f0 => { pc = 3; continue; } 0x52a7f7 => { pc = 4; continue; } 0x52a7fe => { pc = 5; continue; } _ => unreachable!() }
            }
            2 => {
                esi = 0x66def0u32;
                pc = 7; continue;
            }
            3 => {
                esi = 0x66df50u32;
                pc = 7; continue;
            }
            4 => {
                esi = 0x66dfb0u32;
                pc = 7; continue;
            }
            5 => {
                esi = 0x66e010u32;
                pc = 7; continue;
            }
            6 => {
                esi = m.r32(esp.wrapping_add(0x14u32));
                pc = 7; continue;
            }
            7 => {
                { let v = lift::fld32(m, 0x673a54u32); fp.push(v); }
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, m.r32(esi.wrapping_add(0x10u32))); fl = f;
                eax = r;
                { let v = lift::fld32(m, 0x673a5cu32); fp.push(v); }
                { let v = lift::fld32(m, 0x66a918u32); let d = fp.st(0); fp.set(0, d * v); }
                edx = ((eax as i32) >> 31) as u32;
                ebx = eax;
                ebp = edx;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, m.r32(esi.wrapping_add(0xcu32))); fl = f;
                eax = r;
                { let v = lift::fld32(m, 0x673a48u32); fp.push(v); }
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::adc(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, m.r32(esi.wrapping_add(0x8u32))); fl = f;
                eax = r;
                { let v = lift::fld32(m, 0x673a38u32); fp.push(v); }
                { let v = lift::fld32(m, 0x66a91cu32); let d = fp.st(0); fp.set(0, d * v); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::adc(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, m.r32(esi.wrapping_add(0x4u32))); fl = f;
                eax = r;
                { let v = lift::fld32(m, 0x673a44u32); fp.push(v); }
                { let v = lift::fld32(m, 0x66a918u32); let d = fp.st(0); fp.set(0, d * v); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::adc(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, m.r32(esi)); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::adc(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::adc(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                eax = m.r32(edi.wrapping_add(0x14u32));
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

/// `0X0052D4F0` trade_center_get_sell_revenue
pub fn f_0052d4f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x28u32));
                ebp = ecx;
                ebx = 0x1u32;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x5u32)))) & 0xff);
                let r = edi & ebx; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x28u32), edi);
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x673984u32));
                esi = m.r32(ebp.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esi = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esi);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esi, 0x0u32);
                pc = 6; continue;
            }
            2 => {
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi, ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0x2u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
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
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                pc = 5; continue;
            }
            4 => {
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
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                pc = 5; continue;
            }
            5 => {
                m.w32(esi, ecx);
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x5u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((ebx & 0xff), 0x1u32, 8).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x20u32), eax);
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub((ebx & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w32(esp.wrapping_add(0x1cu32), 0x80000000u32);
                m.w32(esp.wrapping_add(0x20u32), 0x1u32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), 0x2u32);
                pc = 12; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = 0x1u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x20u32), eax);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w32(esp.wrapping_add(0x1cu32), 0x33333333u32);
                m.w32(esp.wrapping_add(0x14u32), 0x80000000u32);
                pc = 12; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x1cu32), 0x4cccccccu32);
                m.w32(esp.wrapping_add(0x14u32), 0x99999999u32);
                pc = 12; continue;
            }
            12 => {
                { let v = lift::fild64(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ebp).wrapping_add(0x2u32)))) & 0xff);
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, eax.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                m.w32(esp.wrapping_add(0x2cu32), eax);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = lift::fild64(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                edi = edx;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esi = eax;
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, edx.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = esi;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edi;
                let (r, f) = lift::sbb(eax, edx, fl.cf, 32); fl = f;
                eax = r;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(ebx.wrapping_mul(4).wrapping_add(0x673984u32));
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, m.r32(ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x14u32)), 32); fl = f;
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
                eax = m.r32(esp.wrapping_add(0x10u32));
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
                eax = edx;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sar(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X0052D950` expedition_route_length_from_start
pub fn f_0052d950(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebp = esp;
                esp &= 0xfffffff8;
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4081c800u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x6739e0u32); let d = fp.st(0); fp.set(0, d * v); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ebx = m.r32(ebp.wrapping_add(0x8u32));
                esi = eax;
                eax = 0x104u32;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x2u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), edi);
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::add(ecx, 0xfffffff3u32, 32); fl = f;
                ecx = r;
                eax = edi.wrapping_add(0xffffff1eu32);
                edx = ecx;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                ecx = eax;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x10u32))), 0xd8u32, 16).1;
                if (fl.cf || fl.zf) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 7; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x6739d8u32); let d = fp.st(0); fp.set(0, d * v); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::add(eax, eax, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = esi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x40aa2a00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x6739e0u32); let d = fp.st(0); fp.set(0, d * v); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x2u32)))) & 0xffff);
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x18u32), edi);
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x2e8ba2e9u32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                edx = edx.wrapping_add(eax).wrapping_add(0xfffffe84u32);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x10u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x10u32), ((eax & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x12u32), ((edx & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x12u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                eax = edi;
                edx = ecx;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x18u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 7; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x40c74680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x6739e0u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx))) & 0xffff);
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x20u32), ((edx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x22u32), ((eax & 0xffff)) as u16);
                ecx = m.r32(esp.wrapping_add(0x22u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(ecx, 0x40u32, 32); fl = f;
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ecx;
                let (r, f) = lift::sub(eax, 0x145u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                ecx = eax;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x20u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 7; continue;
            }
            7 => {
                { let v = lift::fld64(m, 0x6739d8u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X0052DB50` expedition_route_length_between
pub fn f_0052db50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub((ebp & 0xffff), 0xd8u32, 16).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if fl.cf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ecx = ebp;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x104u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ebx = 0x1u32;
                pc = 4; continue;
            }
            3 => {
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x2e8ba2e9u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                ecx = edx.wrapping_add(eax).wrapping_add(0xfffffe84u32);
                fl = lift::sub(esi, ecx, 32).1;
                ebx = (ebx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::add(ebx, 0x2u32, 32); fl = f;
                ebx = r;
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x38u32));
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((edi & 0xffff), 0xd8u32, 16).1;
                if !fl.cf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 10; continue;
            }
            6 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ecx = edi;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x104u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 9; continue;
            }
            8 => {
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                eax = 0x2e8ba2e9u32;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                ecx = edx.wrapping_add(eax).wrapping_add(0xfffffe84u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(esi, ecx, 32).1;
                edx = (edx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x38u32));
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(ebx, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esp.wrapping_add(0x34u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x20u32), ((ebp & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x38u32), ((edi & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x2u32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x3au32), ((eax & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x22u32), ((edx & 0xffff)) as u16);
                ebp = ebx;
                pc = 13; continue;
            }
            12 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x34u32));
                m.w16(esp.wrapping_add(0x22u32), ((edx & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x38u32), ((ebp & 0xffff)) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                ebp = ecx;
                m.w32(esp.wrapping_add(0x10u32), ebx);
                m.w16(esp.wrapping_add(0x20u32), ((edi & 0xffff)) as u16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w16(esp.wrapping_add(0x3au32), ((edx & 0xffff)) as u16);
                ebx = ebp;
                pc = 13; continue;
            }
            13 => {
                fl = lift::logic(ebx & ebx, 32);
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(ecx, 0x1u32, 32).1;
                ebp = 0x2u32;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ebx = 0x1u32;
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(ebx, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 29; continue; }
                pc = 17; continue;
            }
            17 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x1au32)))) & 0xffff);
                pc = 19; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                pc = 19; continue;
            }
            19 => {
                eax = ebx;
                let (r, f) = lift::sub(eax, 0x0u32, 32); fl = f;
                eax = r;
                if fl.zf { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 26; continue; }
                pc = 22; continue;
            }
            22 => {
                esi = (esi & !0xffff) | ((0x145u32) & 0xffff);
                edi = (edi & !0xffff) | ((0x40u32) & 0xffff);
                pc = 25; continue;
            }
            23 => {
                esi = (esi & !0xffff) | ((0xd8u32) & 0xffff);
                edi = (edi & !0xffff) | ((0x46u32) & 0xffff);
                pc = 25; continue;
            }
            24 => {
                esi = (esi & !0xffff) | ((0xe2u32) & 0xffff);
                edi = (edi & !0xffff) | ((0xdu32) & 0xffff);
                pc = 25; continue;
            }
            25 => {
                m.w16(esp.wrapping_add(0x18u32), ((esi & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((edi & 0xffff)) as u16);
                pc = 26; continue;
            }
            26 => {
                fl = lift::sub(ebx, ecx, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x38u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x3au32)))) & 0xffff);
                m.w16(esp.wrapping_add(0x18u32), ((esi & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x1au32), ((edi & 0xffff)) as u16);
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x1au32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x22u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = ecx;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                ecx = eax;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x34u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                edx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63aae4, ecx, &a); }
                { let v = lift::fld64(m, 0x6739e0u32); let d = fp.st(0); fp.set(0, d * v); }
                esp = esp.wrapping_add(0x8);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::imul(eax, ebp); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x14u32));
                ebp = 0x1u32;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                fl = lift::sub(ebx, eax, 32).1;
                m.w16(esp.wrapping_add(0x20u32), ((esi & 0xffff)) as u16);
                m.w16(esp.wrapping_add(0x22u32), ((edi & 0xffff)) as u16);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
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

/// `0X0053A3E0` op_deliver_wares_for_alderman_mission
pub fn f_0053a3e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(esp.wrapping_add(0xcu32), edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd748u32)), 16).1;
                if !fl.cf { pc = 33; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6de4b0u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.cf { pc = 33; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = 0x6dd73cu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                m.w32(esp.wrapping_add(0x14u32), esi);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x6u32))), 0x32u32, 16).1;
                if !fl.zf { pc = 33; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xcu32))), 0x1u32, 8).1;
                if !fl.zf { pc = 33; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x9u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xdu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 33; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (ecx & 0xff), 8).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x11u32))), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 33; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 33; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xau32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                fl = lift::sub((eax & 0xff), 0x4u32, 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                if !fl.cf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672bf4u32)))) & 0xff);
                ebp = ebx;
                ebx = eax;
                pc = 15; continue;
            }
            10 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(0x672bf7u32))) & 0xff);
                ebp = ebx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(u32::from(m.r8(ebx.wrapping_add(0x672bf4u32))), (eax & 0xff), 8).1;
                if fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x3u32, 32).1;
                if fl.cf { pc = 11; continue; }
                pc = 13; continue;
            }
            13 => {
                pc = 15; continue;
            }
            14 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = eax;
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x672c14u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let r = eax & 0xfffff8f8u32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(eax, 0x7d0u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                let r = (eax & 0xffff) ^ (eax & 0xffff); fl = lift::logic(r, 16);
                eax = (eax & !0xffff) | ((r) & 0xffff);
                eax = (eax & !0xff) | (((edx & 0xff)) & 0xff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6de4aau32)), 16).1;
                if !fl.cf { pc = 34; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xffff) | (((ecx & 0xff)) & 0xffff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edi = (edi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esi = eax;
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 21; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0x4u32));
                ebx = m.r32(esp.wrapping_add(0x18u32));
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = edi;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 20; continue;
            }
            19 => {
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                pc = 20; continue;
            }
            20 => {
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0x4u32), ecx);
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x6de4a0u32;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x9u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6dd894u32);
                ecx = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.cf { pc = 31; continue; }
                pc = 22; continue;
            }
            22 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 31; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x8u32)))) & 0xff);
                let (r, f) = lift::shl(esi, 0x7u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 30; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x136u32)))) & 0xff);
                let r = (edx & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 30; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0x54u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                ebx = m.r32(esp.wrapping_add(0x18u32));
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ebx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::sub(eax, edi, 32).1;
                if (fl.sf != fl.of) { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = edi;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 29; continue;
            }
            28 => {
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0x54u32), ecx);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x10u32));
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(0x6dd894u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x4u32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 22; continue; }
                pc = 31; continue;
            }
            31 => {
                esi = m.r32(esp.wrapping_add(0x1cu32));
                pc = 32; continue;
            }
            32 => {
                ecx = (ecx & !0xff) | (((ebx & 0xff)) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x13u32)))) & 0xff);
                let (r, f) = lift::add((ecx & 0xff), (ecx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shr(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                ecx = 0x4u32;
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shl(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(esi).wrapping_add(0x14u32)))) & 0xff);
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((0x3u32) & 0xff);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w8(ebx.wrapping_add(esi).wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                let r = eax & 0x300u32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sar(eax, (ecx & 0xff), 32, fl); fl = f;
                eax = r;
                ecx = ebx.wrapping_add(ebx);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x13u32)))) & 0xff);
                let (r, f) = lift::shl((edx & 0xff), (ecx & 0xff), 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                edx = (edx & !0xff) | ((!(edx & 0xff)) & 0xff);
                let r = (edx & 0xff) & (ebx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) | (edx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(0x13u32), ((eax & 0xff)) as u8);
                pc = 33; continue;
            }
            33 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            34 => {
                edi = m.r32(esp.wrapping_add(0x1cu32));
                pc = 32; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00540F50` op_sea_battle_units_virtual_command
pub fn f_00540f50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6u32)))) & 0xff);
                edi = (edi & !0xffff) | (((ecx & 0xff)) & 0xffff);
                let (r, f) = lift::shl(edi, 0x6u32, 32, fl); fl = f;
                edi = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = edi;
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ecx, 0x20u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = eax;
                edx = 0x1u32;
                let r = ecx & 0x1fu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                fl = lift::logic(m.r32(esi.wrapping_add(0xcu32)) & edx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                ecx = m.r32(edx.wrapping_add(0x4u32));
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5u32)))) & 0xff);
                edx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6u32)))) & 0xff);
                eax = edi;
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edx, 0x20u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6u32)))) & 0xff);
                edi = (edi & !0xffff) | (((ecx & 0xff)) & 0xffff);
                let (r, f) = lift::shl(edi, 0x6u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, 0x20u32, 32); fl = f;
                edi = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = eax;
                edx = 0x1u32;
                let r = ecx & 0x1fu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                fl = lift::logic(m.r32(esi.wrapping_add(0x8u32)) & edx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x4u32)))) & 0xff);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                ecx = m.r32(edx.wrapping_add(0x4u32));
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5u32)))) & 0xff);
                edx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x6u32)))) & 0xff);
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 13; continue;
            }
            13 => {
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

/// `0X00541AB0` op_sync_lobby_player_presence
pub fn f_00541ab0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6cbb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0xcu32), edi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 1; continue;
            }
            1 => {
                edx = 0x1u32;
                ecx = esi;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                fl = lift::logic(m.r32(edi.wrapping_add(0x4u32)) & edx, 32);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x204u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 18; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(eax.wrapping_add(esi).wrapping_add(0x204u32), (0x1u32) as u8);
                ecx = m.r32(0x6cbb94u32);
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                edi = esi.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x3u32, 32, fl); fl = f;
                edi = r;
                edx = m.r32(edi.wrapping_add(ecx).wrapping_add(0x1c14u32));
                ecx = edi.wrapping_add(ecx).wrapping_add(0x1c14u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(edi.wrapping_add(eax).wrapping_add(0x2354u32));
                ecx = edi.wrapping_add(eax).wrapping_add(0x2354u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x43e1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbb94u32);
                m.w8(eax.wrapping_add(esi).wrapping_add(0x1fcu32), ((ebx & 0xff)) as u8);
                ecx = m.r32(0x6cbb94u32);
                m.w8(ecx.wrapping_add(esi).wrapping_add(0x1f4u32), ((ebx & 0xff)) as u8);
                edx = m.r32(0x6cbb94u32);
                m.w32(edx.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x1d4u32), 0xffffffffu32);
                pc = 16; continue;
            }
            4 => {
                fl = lift::sub(esi, m.r32(0x6df31cu32), 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w8(eax.wrapping_add(0x20du32), (0x1u32) as u8);
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(0x6cbb94u32);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1eu32);
                pc = 8; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                pc = 8; continue;
            }
            8 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x43ff40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                { let a = []; eax = h.call_at(esp, m, 0x43e4f0, ecx, &a); }
                esi = 0x9u32;
                pc = 17; continue;
            }
            9 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x204u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (ebx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 18; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub((ecx & 0xff), 0x2u32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 18; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w8(eax.wrapping_add(esi).wrapping_add(0x204u32), ((ebx & 0xff)) as u8);
                eax = m.r32(0x6cbb94u32);
                edx = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w8(eax.wrapping_add(esi).wrapping_add(0x1f4u32), ((ebx & 0xff)) as u8);
                ecx = m.r32(0x6cbb94u32);
                let (r, f) = lift::sub(edx, esi, 32); fl = f;
                edx = r;
                m.w8(ecx.wrapping_add(esi).wrapping_add(0x1fcu32), ((ebx & 0xff)) as u8);
                eax = m.r32(0x6cbb94u32);
                edi = esi.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x3u32, 32, fl); fl = f;
                edi = r;
                edx = m.r32(edi.wrapping_add(eax).wrapping_add(0x2354u32));
                ecx = edi.wrapping_add(eax).wrapping_add(0x2354u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(edi.wrapping_add(eax).wrapping_add(0x1c14u32));
                ecx = edi.wrapping_add(eax).wrapping_add(0x1c14u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0x20cu32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(ecx.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x1d4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                let r = u32::from(m.r8(eax.wrapping_add(0x1u32))) & 0x7fu32; fl = lift::logic(r, 8);
                m.w8(eax.wrapping_add(0x1u32), (r) as u8);
                ecx = m.r32(0x6cbb94u32);
                pc = 14; continue;
            }
            14 => {
                m.w32(ecx.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x1d4u32), 0xffffffffu32);
                ecx = m.r32(0x6cbb94u32);
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x43e1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(0x6cbb94u32);
                { let a = []; eax = h.call_at(esp, m, 0x43e3c0, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0xcu32));
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(0x6cbb94u32);
                pc = 18; continue;
            }
            18 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(m.r32(edi.wrapping_add(0x8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(m.r32(0x6df31cu32), ebx, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 21; continue;
            }
            21 => {
                esi = 0x1u32;
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54bdd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x8u32, 32).1;
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w32(0x6df2f0u32, ebx);
                pc = 24; continue;
            }
            24 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X00608230` open_for_player
pub fn f_00608230(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x1cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x2cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w8(esp.wrapping_add(0xcu32), ((ebx & 0xff)) as u8);
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(0x6dfc14u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x35au32)))) & 0xffff);
                fl = lift::sub(edx, ecx, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x5du32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e51acu32);
                edx = esi.wrapping_add(0xa80u32);
                eax = esi.wrapping_add(0xa7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x589ed0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x6107b9, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x662u32)))) & 0xff);
                m.w8(0x6e59ccu32, ((ecx & 0xff)) as u8);
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                let (r, f) = lift::add((ebx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                m.w8(esp.wrapping_add(0xcu32), ((ebx & 0xff)) as u8);
                if fl.cf { pc = 4; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x60f479, ecx, &a); }
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0060FEC9` op_sea_battle_set_manual_control
pub fn f_0060fec9(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(0x6dfc14u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                eax = ebx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(esi.wrapping_add(0x12e0u32), (0x0u32) as u8);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(0x6e59ccu32))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x662u32))), 16).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(0x6e59ccu32, (0xffu32) as u8);
                pc = 5; continue;
            }
            4 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x662u32)))) & 0xff);
                m.w8(0x6e59ccu32, ((eax & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
                edi = (edi & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xffff);
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                m.w8(esp.wrapping_add(0xfu32), (0x0u32) as u8);
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = edi;
                ecx = esi.wrapping_add(0x15cu32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(u32::from(m.r16(ecx)), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(esp.wrapping_add(0xfu32), (0x1u32) as u8);
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 7; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                if (fl.cf || fl.zf) { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x35au32))), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(eax.wrapping_add(esi).wrapping_add(0x5du32), ((edx & 0xff)) as u8);
                m.w16(esi.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x15cu32), (0x0u32) as u16);
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub((ecx & 0xffff), (eax & 0xffff), 16).1;
                if fl.cf { pc = 11; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                if (fl.cf || fl.zf) { pc = 22; continue; }
                pc = 15; continue;
            }
            15 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = esi.wrapping_add(0x5du32);
                ecx = esi.wrapping_add(0x35au32);
                edi = eax;
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(u32::from(m.r8(edx)), 0x0u32, 8).1;
                if !fl.zf { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(ecx)), (eax & 0xffff), 16).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 19; continue;
            }
            19 => {
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 16; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                pc = 22; continue;
            }
            22 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                ecx = 0x6e55d0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w8(esi.wrapping_add(0xac8u32), ((edx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x612c90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            24 => {
                m.w8(esi.wrapping_add(0xac8u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x612d20, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                ebx = 0xffffu32;
                if (fl.cf || fl.zf) { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = esi.wrapping_add(0x15cu32);
                let r = eax & ebx; fl = lift::logic(r, 32);
                eax = r;
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(u32::from(m.r16(ecx)), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                m.w8(esp.wrapping_add(0xfu32), (0x0u32) as u8);
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 27; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 43; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 38; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x28u32));
                let r = eax & ebx; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x13du32)))) & 0xff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(edx.wrapping_mul(2)).wrapping_add(0x35au32))), (edi & 0xffff), 16).1;
                if !fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x11u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(0x24u32), ((edx & 0xffff)) as u16);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                m.w16(edx.wrapping_add(0x128u32), ((ebx & 0xffff)) as u16);
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(esi.wrapping_add(0xcu32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 34; continue; }
                pc = 37; continue;
            }
            37 => {
                pc = 39; continue;
            }
            38 => {
                edi = m.r32(esp.wrapping_add(0x28u32));
                pc = 39; continue;
            }
            39 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65cu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if fl.zf { pc = 43; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = eax & ebx; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x13du32)))) & 0xff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(edx.wrapping_mul(2)).wrapping_add(0x35au32))), (edi & 0xffff), 16).1;
                if !fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x11u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(0x24u32), ((edx & 0xffff)) as u16);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                m.w16(edx.wrapping_add(0x128u32), ((ebx & 0xffff)) as u16);
                pc = 42; continue;
            }
            42 => {
                ecx = m.r32(esi.wrapping_add(0xcu32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 40; continue; }
                pc = 43; continue;
            }
            43 => {
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

/// `0X006116A9` op_sea_battle_order_side_ships
pub fn f_006116a9(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let t = m.r32(eax.wrapping_add(0x28u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x35au32))), (ecx & 0xffff), 16).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65cu32)))) & 0xffff);
                pc = 5; continue;
            }
            4 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x14u32))), 16).1;
                if !fl.cf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                edx = ecx;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                edi = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x12au32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0xeu32, 8);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = (eax & 0xff) | 0x2u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(edi.wrapping_add(0x12au32), ((eax & 0xff)) as u8);
                m.w8(esi.wrapping_add(0xac4u32), (0x0u32) as u8);
                m.w16(esi.wrapping_add(0xac6u32), ((ecx & 0xffff)) as u16);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esi.wrapping_add(0xcu32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x14u32))), 16).1;
                if fl.cf { pc = 7; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00611AE0` load
pub fn f_00611ae0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x668346u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ebx = ecx;
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (fl.cf || fl.zf) { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x40u32)), 0x45u32, 32).1;
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x48u32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2f24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = eax;
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x20u32), ebp);
                fl = lift::logic(ebp & ebp, 32);
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                if fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x603a70, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(ebp, 0x67aaa0u32);
                m.w32(ebp.wrapping_add(0x2f1cu32), eax);
                m.w16(ebp.wrapping_add(0x2f20u32), ((eax & 0xffff)) as u16);
                ecx = ebp;
                pc = 11; continue;
            }
            5 => {
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1e98u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x20u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x34u32), 0x1u32);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x603a70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            8 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = ebp;
                pc = 11; continue;
            }
            9 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 10; continue;
            }
            10 => {
                ecx = eax;
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                m.w32(ebx.wrapping_add(edi.wrapping_mul(4)), ecx);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(ebx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = eax;
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                ecx = eax;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edi, ecx, 32).1;
                if fl.cf { pc = 1; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(ebx.wrapping_add(0x404u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x40u32));
                fl = lift::sub(eax, 0x32u32, 32).1;
                m.w8(ebx.wrapping_add(0x3fcu32), (0xffu32) as u8);
                m.w8(ebx.wrapping_add(0x42cu32), (0x0u32) as u8);
                m.w8(ebx.wrapping_add(0x3fdu32), (0x0u32) as u8);
                if (fl.cf || fl.zf) { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(esi, 0x8u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x612a20, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if fl.cf { pc = 15; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if fl.cf { pc = 14; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(0x6dccf8u32);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(ebx.wrapping_add(0xc44u32), eax);
                eax = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ebx.wrapping_add(0x408u32), 0x0u32);
                m.w8(ebx.wrapping_add(0x3feu32), (0x1u32) as u8);
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

/// `0X00612D20` op_sea_battles_resume_if_all_done
pub fn f_00612d20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x4u32)))) & 0xff);
                esp = esp.wrapping_sub(0x14);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6df31cu32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x404u32)))) & 0xff);
                let r = (edx & 0xff) ^ (edx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                m.w8(esp.wrapping_add(0x1cu32), ((edx & 0xff)) as u8);
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xac8u32))), 0x0u32, 8).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                m.w8(esp.wrapping_add(0x1cu32), ((edx & 0xff)) as u8);
                if fl.cf { pc = 3; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esp.wrapping_add(0x4u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = esp.wrapping_add(0x4u32);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x8u32), 0xc8u32);
                m.w32(esp.wrapping_add(0xcu32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                ebx = m.r32(esp);
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
        0x4db7c0 => f_004db7c0(m, h, ecx, esp),
        0x4ddf40 => f_004ddf40(m, h, ecx, esp),
        0x4e63e4 => f_004e63e4(m, h, ecx, esp),
        0x4fdf50 => f_004fdf50(m, h, ecx, esp),
        0x500330 => f_00500330(m, h, ecx, esp),
        0x500690 => f_00500690(m, h, ecx, esp),
        0x500bb0 => f_00500bb0(m, h, ecx, esp),
        0x503b50 => f_00503b50(m, h, ecx, esp),
        0x505a10 => f_00505a10(m, h, ecx, esp),
        0x50ac40 => f_0050ac40(m, h, ecx, esp),
        0x515a60 => f_00515a60(m, h, ecx, esp),
        0x515b80 => f_00515b80(m, h, ecx, esp),
        0x5172a0 => f_005172a0(m, h, ecx, esp),
        0x5187f0 => f_005187f0(m, h, ecx, esp),
        0x521310 => f_00521310(m, h, ecx, esp),
        0x521580 => f_00521580(m, h, ecx, esp),
        0x522a00 => f_00522a00(m, h, ecx, esp),
        0x52a7d0 => f_0052a7d0(m, h, ecx, esp),
        0x52d4f0 => f_0052d4f0(m, h, ecx, esp),
        0x52d6b0 => f_0052d6b0(m, h, ecx, esp),
        0x52d950 => f_0052d950(m, h, ecx, esp),
        0x52db50 => f_0052db50(m, h, ecx, esp),
        0x52e700 => f_0052e700(m, h, ecx, esp),
        0x52e960 => f_0052e960(m, h, ecx, esp),
        0x53a3e0 => f_0053a3e0(m, h, ecx, esp),
        0x540f50 => f_00540f50(m, h, ecx, esp),
        0x541ab0 => f_00541ab0(m, h, ecx, esp),
        0x608230 => f_00608230(m, h, ecx, esp),
        0x60fec9 => f_0060fec9(m, h, ecx, esp),
        0x6116a9 => f_006116a9(m, h, ecx, esp),
        0x611ae0 => f_00611ae0(m, h, ecx, esp),
        0x612d20 => f_00612d20(m, h, ecx, esp),
        _ => return None,
    }))
}
