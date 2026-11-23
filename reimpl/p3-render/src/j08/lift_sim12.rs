//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0052BFE0` trade_center_trade_with_convoy
pub fn f_0052bfe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x663348u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x48);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(esp.wrapping_add(0x4u32), edi);
                fl = lift::sub(u32::from(m.r8(edi.wrapping_add(0x4u32))), 0x7fu32, 8).1;
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x48u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x54);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
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
                m.w32(esp, 0x0u32);
                pc = 5; continue;
            }
            5 => {
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
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 6; continue;
            }
            6 => {
                pc = 8; continue;
            }
            7 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 8; continue;
            }
            8 => {
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
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi).wrapping_add(0x2u32)))) & 0xff);
                eax = m.r32(ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x54u32));
                fl = lift::logic(eax & eax, 32);
                edi = ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x54u32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
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
                pc = 13; continue;
            }
            12 => {
                m.w32(edi, 0x0u32);
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(0x6dd894u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 16; continue;
            }
            16 => {
                esi = m.r32(esp.wrapping_add(0x68u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                pc = 17; continue;
            }
            17 => {
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
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 18; continue;
            }
            18 => {
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
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
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
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(0x6dd894u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 21; continue;
            }
            21 => {
                esi = m.r32(esp.wrapping_add(0x78u32));
                m.w32(esp.wrapping_add(0x7cu32), ebp);
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x74u32)), 0xc8u32, 32).1;
                if (fl.sf != fl.of) { pc = 57; continue; }
                pc = 23; continue;
            }
            23 => {
                edi = ebp;
                let r = edi & 0x80000001u32; fl = lift::logic(r, 32);
                edi = r;
                if !fl.sf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let r = edi | 0xfffffffeu32; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x6cu32));
                ecx = edi;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebx).wrapping_add(0x34u32)))) & 0xff);
                edx = 0x1u32;
                let (r, f) = lift::shl(edx, (ecx & 0xff), 32, fl); fl = f;
                edx = r;
                fl = lift::logic((eax & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 56; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(ecx)))) & 0xff);
                fl = lift::sub(edx, 0x18u32, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                if (fl.sf == fl.of) { pc = 56; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::logic(ebp & ebp, 32);
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::logic((eax & 0xff) & 0x2u32, 8);
                if fl.zf { pc = 32; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esp.wrapping_add(0x74u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esi = eax;
                let (r, f) = lift::sar(esi, 0x1u32, 32, fl); fl = f;
                esi = r;
                pc = 33; continue;
            }
            30 => {
                fl = lift::sub(ebp, 0x1u32, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub(ebp, 0x2u32, 32).1;
                if !fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                esi = m.r32(esp.wrapping_add(0x74u32));
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0xcu32));
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf == fl.of) { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                esi = eax;
                pc = 35; continue;
            }
            35 => {
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x78u32))), 0x1u32, 16).1;
                m.w8(esp.wrapping_add(0x28u32), (u32::from(fl.zf)) as u8);
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
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
                pc = 39; continue;
            }
            38 => {
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
                pc = 39; continue;
            }
            39 => {
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
                if fl.zf { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(ebx.wrapping_add(0x20u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
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
                pc = 36; continue;
            }
            42 => {
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
                if (fl.sf == fl.of) { pc = 55; continue; }
                pc = 43; continue;
            }
            43 => {
                pc = 45; continue;
            }
            44 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 45; continue;
            }
            45 => {
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 55; continue; }
                pc = 46; continue;
            }
            46 => {
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
                if (fl.zf || fl.sf != fl.of) { pc = 54; continue; }
                pc = 47; continue;
            }
            47 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
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
                pc = 50; continue;
            }
            49 => {
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
                pc = 50; continue;
            }
            50 => {
                fl = lift::sub(edi, esi, 32).1;
                if (fl.sf != fl.of) { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
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
                pc = 54; continue;
            }
            52 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
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
                pc = 54; continue;
            }
            54 => {
                eax = m.r32(0x6dd894u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0x24u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 44; continue; }
                pc = 55; continue;
            }
            55 => {
                ebp = m.r32(esp.wrapping_add(0x7cu32));
                pc = 56; continue;
            }
            56 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, 0x3u32, 32).1;
                m.w32(esp.wrapping_add(0x7cu32), ebp);
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(0x6dd894u32);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xau32)))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 61; continue; }
                pc = 58; continue;
            }
            58 => {
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
                if (fl.zf || fl.sf != fl.of) { pc = 60; continue; }
                pc = 59; continue;
            }
            59 => {
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                pc = 60; continue;
            }
            60 => {
                edx = m.r32(0x6dd894u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 58; continue; }
                pc = 61; continue;
            }
            61 => {
                eax = m.r32(esp.wrapping_add(0x38u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 65; continue; }
                pc = 62; continue;
            }
            62 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 65; continue; }
                pc = 63; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 83; continue; }
                pc = 65; continue;
            }
            65 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp.wrapping_add(0x70u32));
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x5u32))), 0x2u32, 8).1;
                if !fl.cf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
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
                pc = 68; continue;
            }
            67 => {
                eax = m.r32(0x6b9614u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6b95f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 68; continue;
            }
            68 => {
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 69; continue;
            }
            69 => {
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x38u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 74; continue; }
                pc = 70; continue;
            }
            70 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0x2u32)))) & 0xff);
                ebp = edx;
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
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
                pc = 73; continue;
            }
            72 => {
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
                pc = 73; continue;
            }
            73 => {
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
                pc = 74; continue;
            }
            74 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 69; continue; }
                pc = 75; continue;
            }
            75 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 76; continue;
            }
            76 => {
                ecx = m.r32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x48u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 82; continue; }
                pc = 77; continue;
            }
            77 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 79; continue; }
                pc = 78; continue;
            }
            78 => {
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
                pc = 80; continue;
            }
            79 => {
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
                pc = 80; continue;
            }
            80 => {
                ecx = edx;
                m.w32(esp.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x48u32), edx);
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 82; continue; }
                pc = 81; continue;
            }
            81 => {
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
                pc = 82; continue;
            }
            82 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 76; continue; }
                pc = 83; continue;
            }
            83 => {
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
            84 => {
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005472F0` operations_save_game
pub fn f_005472f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66389du32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x44);
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                m.w32(ebp.wrapping_add(0xfffffff0u32), esp);
                m.w32(ebp.wrapping_add(0xffffffd8u32), edi);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(ebp.wrapping_add(0xffffffe8u32), eax);
                m.w32(ebp.wrapping_add(0xfffffffcu32), esi);
                ecx = ebp.wrapping_add(0xffffffe4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x54c3f0, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653b0b, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffb8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653b0b, ecx, &a); }
                ebx = m.r32(ebp.wrapping_add(0x8u32));
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x4u32) as u8);
                m.w32(ebp.wrapping_add(0x8u32), esi);
                eax = m.r32(ebx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.sf { pc = 17; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(edi.wrapping_add(0x91cu32)), esi, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r16(0x6de4aau32)), 0x0u32, 16).1;
                if (fl.cf || fl.zf) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d6780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6cc3e8u32);
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0xdu32))), 0x5u32, 8).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c3860u32);
                ecx = ebp.wrapping_add(0xffffffecu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(0x6cc3e8u32);
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(0xdu32))), 0x3u32, 8).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c386cu32);
                ecx = ebp.wrapping_add(0xffffffecu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(0x6cc3e8u32);
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xdu32))), 0x3u32, 8).1;
                if !fl.cf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c3878u32);
                ecx = ebp.wrapping_add(0xffffffecu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(ebp.wrapping_add(0xffffffecu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a2b0u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5cu32);
                ecx = ebp.wrapping_add(0xffffffecu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(ebx).wrapping_add(0x8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp.wrapping_add(0xffffffecu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f60a, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0xcu32, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = ebp.wrapping_add(0xffffffecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c3884u32);
                eax = ebp.wrapping_add(0xffffffdcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp.wrapping_add(0xffffffe8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp.wrapping_add(0xffffffdcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9001u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x653c1d, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 45; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x549c60, ecx, &a); }
                ebx = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = ebx.wrapping_add(0x4u32);
                esi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ebp.wrapping_add(0xffffffe0u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edi = eax;
                fl = lift::logic(esi & esi, 32);
                m.w32(ebp.wrapping_add(0xffffffdcu32), edi);
                if fl.zf { pc = 41; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::logic(edi & edi, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                if fl.zf { pc = 43; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(ebp.wrapping_add(0xffffffd8u32));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x549cc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp.wrapping_add(0xffffffe4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x54c410, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x6u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x653d74, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 25; continue;
            }
            17 => {
                fl = lift::sub(m.r32(edi.wrapping_add(0x920u32)), esi, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(eax, m.r32(edi.wrapping_add(0x2cu32)), 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 19; continue;
            }
            19 => {
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
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ecx = m.r32(ebp.wrapping_add(0xfffffff4u32));
                m.w32(0x0u32, ecx);
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
            20 => {
                eax = m.r32(ebp.wrapping_add(0xffffffd8u32));
                eax = m.r32(eax.wrapping_add(0x8u32));
                fl = lift::sub(eax, 0xdu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(eax, 0x5u32, 32).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                m.w32(ebp.wrapping_add(0x8u32), 0x1u32);
                pc = 23; continue;
            }
            23 => {
                eax = 0x54752eu32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            24 => {
                esi = m.r32(ebp.wrapping_add(0xffffffe0u32));
                edi = m.r32(ebp.wrapping_add(0xffffffdcu32));
                pc = 25; continue;
            }
            25 => {
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x8u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
                pc = 30; continue;
            }
            26 => {
                ecx = m.r32(ebp.wrapping_add(0xffffffd8u32));
                eax = m.r32(ecx.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                m.w32(ebp.wrapping_add(0x8u32), 0x1u32);
                pc = 28; continue;
            }
            28 => {
                eax = 0x547560u32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            29 => {
                esi = m.r32(ebp.wrapping_add(0xffffffe0u32));
                edi = m.r32(ebp.wrapping_add(0xffffffdcu32));
                pc = 30; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(ebp.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 45; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = ebp.wrapping_add(0xffffffecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c388cu32);
                eax = ebp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp.wrapping_add(0xffffffe8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0xau32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp.wrapping_add(0x8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x9001u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x653c1d, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 45; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = 0x6dfc90u32;
                { let a = []; eax = h.call_at(esp, m, 0x54d7f0, ecx, &a); }
                ebx = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = ebx.wrapping_add(0x4u32);
                esi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ebp.wrapping_add(0x8u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edi = eax;
                fl = lift::logic(esi & esi, 32);
                m.w32(ebp.wrapping_add(0xffffffdcu32), edi);
                if fl.zf { pc = 41; continue; }
                pc = 33; continue;
            }
            33 => {
                fl = lift::logic(edi & edi, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                if fl.zf { pc = 43; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = 0x6dfc90u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54d870, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = ebp.wrapping_add(0xffffffe4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x54c410, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0xbu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x653d74, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 37; continue;
            }
            35 => {
                eax = 0x547629u32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            36 => {
                esi = m.r32(ebp.wrapping_add(0x8u32));
                edi = m.r32(ebp.wrapping_add(0xffffffdcu32));
                pc = 37; continue;
            }
            37 => {
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                ecx = ebp.wrapping_add(0xffffffc8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0xdu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
                pc = 40; continue;
            }
            38 => {
                eax = 0x54764au32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            39 => {
                esi = m.r32(ebp.wrapping_add(0x8u32));
                edi = m.r32(ebp.wrapping_add(0xffffffdcu32));
                pc = 40; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 45; continue;
            }
            41 => {
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 44; continue; }
                pc = 42; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 43; continue;
            }
            43 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 44; continue;
            }
            44 => {
                ecx = ebp.wrapping_add(0xffffffc8u32);
                { let a = []; eax = h.call_at(esp, m, 0x653e38, ecx, &a); }
                pc = 45; continue;
            }
            45 => {
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
                ecx = m.r32(ebp.wrapping_add(0xfffffff4u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            46 => {
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x52bfe0 => f_0052bfe0(m, h, ecx, esp),
        0x5472f0 => f_005472f0(m, h, ecx, esp),
        _ => return None,
    }))
}
