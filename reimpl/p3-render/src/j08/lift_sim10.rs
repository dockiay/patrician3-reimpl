//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x50ac40 => f_0050ac40(m, h, ecx, esp),
        _ => return None,
    }))
}
