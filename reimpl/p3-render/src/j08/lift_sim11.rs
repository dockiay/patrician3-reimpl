//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x6079d0 => f_006079d0(m, h, ecx, esp),
        _ => return None,
    }))
}
