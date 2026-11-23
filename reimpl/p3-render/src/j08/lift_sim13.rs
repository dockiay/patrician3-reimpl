//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0051AC60` letter_render_map_fragment_bitmap
pub fn f_0051ac60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x480);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x520cu32);
                ecx = 0x6da820u32;
                m.w32(esp.wrapping_add(0x40u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = eax;
                eax = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x48u32), edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbb00, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x90u32));
                ecx = m.r32(esp.wrapping_add(0x94u32));
                ebp = eax;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                esi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edi.wrapping_add(0x4u32));
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x44u32), esi);
                m.w32(esp.wrapping_add(0x28u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb10, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb2a0, ecx, &a); }
                ecx = esp.wrapping_add(0x60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbb00, ecx, &a); }
                ecx = edi;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xeu32)))) & 0xffff);
                eax = esi;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_add(0x28);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xcu32)))) & 0xffff);
                ebx = edx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, 0x44u32, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = eax.wrapping_add(edi).wrapping_add(0xffffffbcu32);
                edi = 0x44u32;
                pc = 4; continue;
            }
            2 => {
                edx = edi.wrapping_add(esi);
                fl = lift::sub(edx, 0x91cu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::add(edi, esi, 32); fl = f;
                edi = r;
                eax = eax.wrapping_add(edi).wrapping_add(0xfffff6e4u32);
                edi = 0x91cu32;
                let (r, f) = lift::sub(edi, esi, 32); fl = f;
                edi = r;
                pc = 4; continue;
            }
            4 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(ebx, 0x42u32, 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = ecx.wrapping_add(ebx).wrapping_add(0xffffffbeu32);
                ebx = 0x42u32;
                pc = 9; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = ebx.wrapping_add(eax);
                fl = lift::sub(edx, 0x6c7u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                ecx = ecx.wrapping_add(ebx).wrapping_add(0xfffff939u32);
                ebx = 0x6c7u32;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                pc = 9; continue;
            }
            9 => {
                m.w32(esp.wrapping_add(0x24u32), ecx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(0x6cbdd8u32);
                edx = esp.wrapping_add(0x68u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = m.r32(eax.wrapping_add(0xbcu32));
                edx = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbb00, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x78u32));
                ecx = esp.wrapping_add(0x9cu32);
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                pc = 12; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(eax).wrapping_add(0x90u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x11u32));
                edx = m.r32(esp.wrapping_add(0x12u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = 0x55555556u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esi = edx;
                ecx = esi;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                eax = ecx;
                fl = lift::sub(eax, 0x80u32, 32).1;
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = eax;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let r = edx & 0x3fu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 15; continue;
            }
            14 => {
                edx = 0x80u32;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w8(esp.wrapping_add(0x10u32), (0xffu32) as u8);
                pc = 18; continue;
            }
            17 => {
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = edx & eax; fl = lift::logic(r, 32);
                edx = r;
                m.w8(esp.wrapping_add(0x10u32), ((edx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = ecx.wrapping_add(esi);
                fl = lift::sub(eax, 0x80u32, 32).1;
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = eax;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let r = edx & 0x3fu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 21; continue;
            }
            20 => {
                edx = 0x80u32;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                m.w8(esp.wrapping_add(0x12u32), (0xffu32) as u8);
                pc = 24; continue;
            }
            23 => {
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = edx & eax; fl = lift::logic(r, 32);
                edx = r;
                m.w8(esp.wrapping_add(0x12u32), ((edx & 0xff)) as u8);
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(esi, 0x80u32, 32).1;
                if (fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                let r = ecx & 0x3fu32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                pc = 27; continue;
            }
            26 => {
                eax = 0x80u32;
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(esi, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                m.w8(esp.wrapping_add(0x11u32), (0xffu32) as u8);
                pc = 30; continue;
            }
            29 => {
                fl = lift::logic(esi & esi, 32);
                ecx = (ecx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::sub((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = ecx & esi; fl = lift::logic(r, 32);
                ecx = r;
                m.w8(esp.wrapping_add(0x11u32), ((ecx & 0xff)) as u8);
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x90u32), edx);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x100u32, 32).1;
                m.w32(esp.wrapping_add(0x28u32), eax);
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 36; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                eax = esi.wrapping_add(ebx);
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x78u32))); fl = f;
                eax = r;
                let (r, f) = lift::imul(ecx, esi); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 35; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                ebx = m.r32(esp.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x90u32));
                m.w32(ecx.wrapping_add(0xfffffffcu32), ebx);
                if !fl.zf { pc = 33; continue; }
                pc = 34; continue;
            }
            34 => {
                ebx = m.r32(esp.wrapping_add(0x20u32));
                pc = 35; continue;
            }
            35 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(0x6cbdd8u32);
                ecx = m.r32(eax.wrapping_add(0xbcu32));
                edx = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb10, ecx, &a); }
                esi = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x44u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                ebx = 0x1u32;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x18u32), 0x3u32);
                pc = 37; continue;
            }
            37 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(esp.wrapping_add(0x20u32), 0x3u32);
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                fl = lift::logic(ebx & edx, 32);
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ebx, ebx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, ebp, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                if !fl.zf { pc = 38; continue; }
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 37; continue; }
                pc = 42; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7551u32);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x7u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 45; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, 0x70u32, 32); fl = f;
                edx = r;
                ebx = m.r32(edx);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x5cu32), eax);
                edi = m.r32(edx.wrapping_add(0x8u32));
                esi = m.r32(edx.wrapping_add(0xcu32));
                pc = 46; continue;
            }
            45 => {
                ebx = m.r32(esp.wrapping_add(0x58u32));
                esi = m.r32(esp.wrapping_add(0x64u32));
                edi = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                pc = 46; continue;
            }
            46 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let (r, f) = lift::sar(esi, 0x1u32, 32, fl); fl = f;
                esi = r;
                ecx = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x4cu32));
                edx = esp.wrapping_add(0x60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbb00, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x74u32));
                esp = esp.wrapping_add(0x28);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w32(esp.wrapping_add(0x1cu32), 0x1u32);
                m.w32(esp.wrapping_add(0x28u32), esi);
                pc = 47; continue;
            }
            47 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), 0x3u32);
                pc = 48; continue;
            }
            48 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0xau32)))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                fl = lift::logic(edx & ecx, 32);
                if !fl.zf { pc = 55; continue; }
                pc = 49; continue;
            }
            49 => {
                edx = eax;
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::imul(edx, esi); fl = f;
                edx = r;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                edi = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                fl = lift::logic(ecx & ecx, 32);
                edx = edi.wrapping_add(edx.wrapping_mul(4));
                if (fl.zf || fl.sf != fl.of) { pc = 55; continue; }
                pc = 50; continue;
            }
            50 => {
                esi = ecx;
                pc = 51; continue;
            }
            51 => {
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                ecx = ebp;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = edx;
                edx = edx.wrapping_add(ebp.wrapping_mul(4));
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                eax = m.r32(esp.wrapping_add(0x4cu32));
                pc = 53; continue;
            }
            53 => {
                ecx = eax;
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                edx = edx.wrapping_add(ecx.wrapping_mul(4));
                if !fl.zf { pc = 51; continue; }
                pc = 54; continue;
            }
            54 => {
                esi = m.r32(esp.wrapping_add(0x28u32));
                pc = 55; continue;
            }
            55 => {
                edi = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(edi, edi, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, ebp, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), edi);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                if !fl.zf { pc = 48; continue; }
                pc = 56; continue;
            }
            56 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, 0x3u32, 32).1;
                m.w32(esp.wrapping_add(0x28u32), esi);
                if (fl.sf != fl.of) { pc = 47; continue; }
                pc = 57; continue;
            }
            57 => {
                esi = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                eax = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x480);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00534A40` map_discovery_render_fog_surface
pub fn f_00534a40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xacc);
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(0x8u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(u32::from(m.r8(ecx.wrapping_add(0xcu32))) & 0xfu32, 8);
                if fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                ebx = 0x5u32;
                m.w8(esp.wrapping_add(0x13u32), (0x0u32) as u8);
                edx = 0x8u32;
                m.w32(esp.wrapping_add(0x18u32), 0x7u32);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                pc = 5; continue;
            }
            5 => {
                ebp = ebx;
                m.w32(esp.wrapping_add(0x38u32), ebx);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                esi = esp.wrapping_add(0x45cu32);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edi = ebp;
                let (r, f) = lift::imul(edi, ebp); fl = f;
                edi = r;
                pc = 7; continue;
            }
            7 => {
                eax = ecx;
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(eax, edx); fl = f;
                eax = r;
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(esi, (0x0u32) as u8);
                pc = 11; continue;
            }
            9 => {
                ebx = 0x9u32;
                edx = 0x3u32;
                m.w32(esp.wrapping_add(0x18u32), 0xdu32);
                m.w32(esp.wrapping_add(0x14u32), 0x2u32);
                pc = 5; continue;
            }
            10 => {
                let r = (ebx & 0xff) | 0xffu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sub((ebx & 0xff), (eax & 0xff), 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esi, ((ebx & 0xff)) as u8);
                ebx = m.r32(esp.wrapping_add(0x38u32));
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                eax = ecx.wrapping_add(ebx);
                fl = lift::sub(eax, 0x20u32, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                ecx = ebx.wrapping_add(ebp);
                fl = lift::sub(ecx, 0x10u32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                esi = esp.wrapping_add(0x65cu32);
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                ebp = edx;
                pc = 14; continue;
            }
            14 => {
                edi = ebp;
                ecx = edx;
                let (r, f) = lift::imul(edi, ebp); fl = f;
                edi = r;
                pc = 15; continue;
            }
            15 => {
                eax = ecx;
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x14u32))); fl = f;
                eax = r;
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w8(esi, (0x0u32) as u8);
                pc = 18; continue;
            }
            17 => {
                let r = (ebx & 0xff) | 0xffu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sub((ebx & 0xff), (eax & 0xff), 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esi, ((ebx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                ebx = ecx.wrapping_add(eax);
                fl = lift::sub(ebx, 0x30u32, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 19; continue;
            }
            19 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x23u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x96u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x190u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb2a0, ecx, &a); }
                ecx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x5cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbb00, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x60u32));
                eax = m.r32(esp.wrapping_add(0x70u32));
                esp = esp.wrapping_add(0x18);
                m.w32(esp.wrapping_add(0x3cu32), edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                ebp = eax;
                esi = 0x96u32;
                pc = 21; continue;
            }
            21 => {
                edi = edx;
                ecx = 0x64u32;
                eax = 0xc8c8c8c8u32;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                if !fl.zf { pc = 21; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x40u32));
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                fl = lift::sub(m.r32(eax), 0x0u32, 32).1;
                if fl.zf { pc = 77; continue; }
                pc = 24; continue;
            }
            24 => {
                ebp = m.r32(0x6df2e0u32);
                m.w32(esp.wrapping_add(0x34u32), 0x1u32);
                m.w32(esp.wrapping_add(0x30u32), 0x0u32);
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                fl = lift::logic(m.r32(edx) & ecx, 32);
                if fl.zf { pc = 76; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = ecx;
                ecx = edx;
                edx = m.r32(ecx.wrapping_add(0x30u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(eax & edx, 32);
                edx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                esi = eax.wrapping_add(edx);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(esi.wrapping_mul(2))))) & 0xffff);
                if !fl.zf { pc = 52; continue; }
                pc = 27; continue;
            }
            27 => {
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x2u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = ecx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffc5eu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x4d4873edu32;
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = edx;
                eax = esi.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0xffffffb0u32);
                let (r, f) = lift::sar(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx;
                let (r, f) = lift::shr(edx, 0x1fu32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = 0x4d4873edu32;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esi = edx;
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(esp.wrapping_add(0x38u32));
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf == fl.of) { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 34; continue;
            }
            31 => {
                eax = ecx.wrapping_add(edx);
                fl = lift::sub(eax, 0x190u32, 32).1;
                if (fl.sf != fl.of) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = 0xc8u32;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                pc = 34; continue;
            }
            33 => {
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                esi = edx;
                pc = 38; continue;
            }
            36 => {
                eax = esi.wrapping_add(edx);
                fl = lift::sub(eax, 0x96u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                esi = 0x96u32;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                pc = 38; continue;
            }
            38 => {
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                eax = esi;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esi = esi.wrapping_add(edx).wrapping_add(0xffffffffu32);
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                let (r, f) = lift::imul(esi, ebx); fl = f;
                esi = r;
                edi = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, edi, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 76; continue; }
                pc = 39; continue;
            }
            39 => {
                ebx = m.r32(esp.wrapping_add(0x38u32));
                edx = esp.wrapping_add(0x45cu32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x14u32), edx);
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                pc = 41; continue;
            }
            40 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                pc = 41; continue;
            }
            41 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 50; continue; }
                pc = 42; continue;
            }
            42 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                ebp = ebx;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 43; continue;
            }
            43 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax, ((ecx & 0xff)) as u8);
                pc = 46; continue;
            }
            45 => {
                m.w8(eax, (0x0u32) as u8);
                pc = 46; continue;
            }
            46 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(esi, ((ecx & 0xff)) as u8);
                pc = 49; continue;
            }
            48 => {
                m.w8(esi, (0x0u32) as u8);
                pc = 49; continue;
            }
            49 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 43; continue; }
                pc = 50; continue;
            }
            50 => {
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = ebx.wrapping_add(ebx);
                edi = edx;
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(esi, ecx, 32); fl = f;
                esi = r;
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, 0x20u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                if !fl.zf { pc = 40; continue; }
                pc = 51; continue;
            }
            51 => {
                pc = 75; continue;
            }
            52 => {
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x2u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                eax = ecx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffc5eu32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x4d4873edu32;
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                ecx = edx;
                let (r, f) = lift::sar(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                eax = ecx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esi.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0xffffffb0u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = 0x4d4873edu32;
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
                esi = edx;
                pc = 54; continue;
            }
            54 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf == fl.of) { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 59; continue;
            }
            56 => {
                eax = ecx.wrapping_add(edx);
                fl = lift::sub(eax, 0x190u32, 32).1;
                if (fl.sf != fl.of) { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = 0xc8u32;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                pc = 59; continue;
            }
            58 => {
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 59; continue;
            }
            59 => {
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 61; continue; }
                pc = 60; continue;
            }
            60 => {
                esi = edx;
                pc = 63; continue;
            }
            61 => {
                eax = esi.wrapping_add(edx);
                fl = lift::sub(eax, 0x96u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                esi = 0x96u32;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                pc = 63; continue;
            }
            63 => {
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                eax = esi;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esi = esi.wrapping_add(edx).wrapping_add(0xffffffffu32);
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                let (r, f) = lift::imul(esi, ebx); fl = f;
                esi = r;
                edi = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, edi, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 76; continue; }
                pc = 64; continue;
            }
            64 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = edx;
                m.w32(esp.wrapping_add(0x20u32), edx);
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = esp.wrapping_add(0x65cu32);
                ebx = edx.wrapping_add(edx);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                pc = 65; continue;
            }
            65 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edi = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf == fl.of) { pc = 74; continue; }
                pc = 66; continue;
            }
            66 => {
                ebp = edx;
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                pc = 67; continue;
            }
            67 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 69; continue; }
                pc = 68; continue;
            }
            68 => {
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax, ((ecx & 0xff)) as u8);
                pc = 70; continue;
            }
            69 => {
                m.w8(eax, (0x0u32) as u8);
                pc = 70; continue;
            }
            70 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(esi, ((ecx & 0xff)) as u8);
                pc = 73; continue;
            }
            72 => {
                m.w8(esi, (0x0u32) as u8);
                pc = 73; continue;
            }
            73 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 67; continue; }
                pc = 74; continue;
            }
            74 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ecx;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(esi, ecx, 32); fl = f;
                esi = r;
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, 0x30u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                if !fl.zf { pc = 65; continue; }
                pc = 75; continue;
            }
            75 => {
                ebp = m.r32(0x6df2e0u32);
                pc = 76; continue;
            }
            76 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                esi = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, esi, 32); fl = f;
                esi = r;
                fl = lift::sub(eax, 0x40u32, 32).1;
                m.w32(esp.wrapping_add(0x34u32), esi);
                m.w32(esp.wrapping_add(0x30u32), eax);
                if (fl.sf != fl.of) { pc = 25; continue; }
                pc = 77; continue;
            }
            77 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                esi = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, 0x40u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, 0x4u32, 32); fl = f;
                esi = r;
                fl = lift::sub(eax, 0x300u32, 32).1;
                m.w32(esp.wrapping_add(0x24u32), esi);
                m.w32(esp.wrapping_add(0x28u32), eax);
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 78; continue;
            }
            78 => {
                esi = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb10, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xff) | ((0x50u32) & 0xff);
                pc = 79; continue;
            }
            79 => {
                m.w8(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5fu32), ((eax & 0xff)) as u8);
                m.w8(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5eu32), ((ecx & 0xff)) as u8);
                m.w8(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5du32), ((ecx & 0xff)) as u8);
                m.w8(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5cu32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x100u32, 32).1;
                if (fl.sf != fl.of) { pc = 79; continue; }
                pc = 80; continue;
            }
            80 => {
                edx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bb210, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x1bu32)))) & 0xff);
                edx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_add(0x8);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                ecx = m.r32(esp.wrapping_add(0xae0u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                m.w16(ecx, ((eax & 0xffff)) as u16);
                ecx = m.r32(esp.wrapping_add(0xae0u32));
                eax = m.r32(edx.wrapping_add(0x60u32));
                m.w32(ecx, eax);
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xacc);
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x51ac60 => f_0051ac60(m, h, ecx, esp),
        0x534a40 => f_00534a40(m, h, ecx, esp),
        0x54aae0 => f_0054aae0(m, h, ecx, esp),
        0x605420 => f_00605420(m, h, ecx, esp),
        _ => return None,
    }))
}
