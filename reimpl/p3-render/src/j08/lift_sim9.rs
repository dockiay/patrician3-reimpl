//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x544e50 => f_00544e50(m, h, ecx, esp),
        0x54bf80 => f_0054bf80(m, h, ecx, esp),
        _ => return None,
    }))
}
