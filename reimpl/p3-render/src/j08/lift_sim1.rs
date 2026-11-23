//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0047B8C0` is_valid_building_position
pub fn f_0047b8c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6de4b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                let r = (ecx & 0xff) ^ (ecx & 0xff); fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub(eax, edx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if !fl.cf { pc = 48; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                esi = eax.wrapping_add(edx.wrapping_mul(8));
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2c8u32));
                let r = edx & 0x10u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((edx & 0xff), 0x10u32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esi.wrapping_add(0x7acu32));
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::imul(edi, edx); fl = f;
                edi = r;
                edx = m.r32(esi.wrapping_add(0x80cu32));
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                edx = ebx;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(u32::from(m.r8(edi.wrapping_add(edx))), 0x80u32, 8).1;
                if !fl.cf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xff) | ((0x1u32) & 0xff);
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(ebp.wrapping_add(0x1070u32));
                fl = lift::sub(edi, 0x35u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 48; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x47c018u32)))) & 0xff);
                match m.r32(edx.wrapping_mul(4).wrapping_add(0x47bfb4u32)) { 0x47b947 => { pc = 6; continue; } 0x47b96e => { pc = 11; continue; } 0x47b98a => { pc = 14; continue; } 0x47b9bb => { pc = 15; continue; } 0x47b9e0 => { pc = 17; continue; } 0x47ba54 => { pc = 24; continue; } 0x47ba9e => { pc = 30; continue; } 0x47bacf => { pc = 31; continue; } 0x47bb00 => { pc = 32; continue; } 0x47bb31 => { pc = 33; continue; } 0x47bb62 => { pc = 34; continue; } 0x47bb93 => { pc = 35; continue; } 0x47bbc4 => { pc = 36; continue; } 0x47bbf5 => { pc = 37; continue; } 0x47bc26 => { pc = 38; continue; } 0x47bc57 => { pc = 39; continue; } 0x47bc88 => { pc = 40; continue; } 0x47bca5 => { pc = 41; continue; } 0x47bcbc => { pc = 43; continue; } 0x47bced => { pc = 44; continue; } 0x47bd39 => { pc = 48; continue; } 0x47bdde => { pc = 66; continue; } 0x47be37 => { pc = 72; continue; } 0x47be90 => { pc = 78; continue; } 0x47bf34 => { pc = 90; continue; } _ => unreachable!() }
            }
            6 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x802u32))), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x803u32)))) & 0xff);
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1u32;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            11 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x80au32))), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub((ebx & 0xff), u32::from(m.r8(esi.wrapping_add(0x80bu32))), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 13; continue;
            }
            13 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1u32;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x1du32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            15 => {
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 48; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4fd020, ecx, &a); }
                esp = esp.wrapping_add(16);
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4fd020, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                fl = lift::sub(edi, 0x3u32, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x774u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 23; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x677929u32))) & 0xff);
                pc = 20; continue;
            }
            20 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ebp.wrapping_add(0x1070u32));
                ecx = lift::sx8(u32::from(m.r8(eax.wrapping_add(0x672fc0u32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5928a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(edi & edi, 32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            22 => {
                fl = lift::sub(edi, 0x2u32, 32).1;
                if !fl.zf { pc = 19; continue; }
                pc = 23; continue;
            }
            23 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x67792fu32))) & 0xff);
                pc = 20; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1eu32);
                ecx = esi.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4fd020, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = eax;
                fl = lift::sub(edi, 0x3u32, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x774u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 29; continue; }
                pc = 26; continue;
            }
            26 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x677929u32))) & 0xff);
                pc = 27; continue;
            }
            27 => {
                edx = m.r32(ebp.wrapping_add(0x1070u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = lift::sx8(u32::from(m.r8(edx.wrapping_add(0x672fc0u32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 21; continue;
            }
            28 => {
                fl = lift::sub(edi, 0x2u32, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 29; continue;
            }
            29 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x67792fu32))) & 0xff);
                pc = 27; continue;
            }
            30 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0xeu32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            31 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x1cu32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            32 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x19u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            33 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x1bu32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            34 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x14u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            35 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x1au32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            36 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x16u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            37 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x17u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            38 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x15u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            39 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0x18u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x7a4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4fd020, ecx, &a); }
                esp = esp.wrapping_add(16);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            41 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x808u32))), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 42; continue;
            }
            42 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x809u32)))) & 0xff);
                pc = 8; continue;
            }
            43 => {
                ecx = m.r32(esi.wrapping_add(0x7acu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(ecx, eax); fl = f;
                ecx = r;
                ebp = m.r32(esi.wrapping_add(0x80cu32));
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebx)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (edx & 0xff) & 0x3fu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((edx & 0xff), 0xfu32, 8).1;
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            44 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                m.w8(esp.wrapping_add(0x14u32), (0x0u32) as u8);
                if (fl.sf == fl.of) { pc = 65; continue; }
                pc = 45; continue;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xff), 0x32u32, 8).1;
                if !fl.zf { pc = 49; continue; }
                pc = 46; continue;
            }
            46 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub((edx & 0xff), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 64; continue; }
                pc = 47; continue;
            }
            47 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 64; continue; }
                pc = 48; continue;
            }
            48 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            49 => {
                fl = lift::sub((ecx & 0xff), 0x37u32, 8).1;
                if fl.zf { pc = 56; continue; }
                pc = 50; continue;
            }
            50 => {
                fl = lift::sub((ecx & 0xff), 0x34u32, 8).1;
                if fl.zf { pc = 56; continue; }
                pc = 51; continue;
            }
            51 => {
                fl = lift::sub((ecx & 0xff), 0x38u32, 8).1;
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                fl = lift::sub((ecx & 0xff), 0x35u32, 8).1;
                if !fl.zf { pc = 64; continue; }
                pc = 53; continue;
            }
            53 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebp = edx.wrapping_add(0x1u32);
                fl = lift::sub(ebp, ecx, 32).1;
                if !fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                eax = ebx;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(edx, eax, 32).1;
                pc = 62; continue;
            }
            55 => {
                let (r, f) = lift::add(edx, 0xfffffffdu32, 32); fl = f;
                edx = r;
                pc = 59; continue;
            }
            56 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebp = edx.wrapping_add(0xffffffffu32);
                fl = lift::sub(ebp, ecx, 32).1;
                if !fl.zf { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                pc = 61; continue;
            }
            58 => {
                let (r, f) = lift::add(edx, 0x3u32, 32); fl = f;
                edx = r;
                pc = 59; continue;
            }
            59 => {
                fl = lift::sub(edx, ecx, 32).1;
                if !fl.zf { pc = 64; continue; }
                pc = 60; continue;
            }
            60 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::sub(ecx, 0x6u32, 32); fl = f;
                ecx = r;
                pc = 61; continue;
            }
            61 => {
                edx = ebx;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                pc = 62; continue;
            }
            62 => {
                if !fl.zf { pc = 64; continue; }
                pc = 63; continue;
            }
            63 => {
                m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
                pc = 64; continue;
            }
            64 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 45; continue; }
                pc = 65; continue;
            }
            65 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
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
            66 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 48; continue; }
                pc = 67; continue;
            }
            67 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xff), 0x30u32, 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 68; continue;
            }
            68 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 69; continue;
            }
            69 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 89; continue; }
                pc = 70; continue;
            }
            70 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 67; continue; }
                pc = 71; continue;
            }
            71 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            72 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 48; continue; }
                pc = 73; continue;
            }
            73 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xff), 0x32u32, 8).1;
                if !fl.zf { pc = 76; continue; }
                pc = 74; continue;
            }
            74 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 76; continue; }
                pc = 75; continue;
            }
            75 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 89; continue; }
                pc = 76; continue;
            }
            76 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 73; continue; }
                pc = 77; continue;
            }
            77 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            78 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 48; continue; }
                pc = 79; continue;
            }
            79 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x6u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 84; continue; }
                pc = 80; continue;
            }
            80 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x37u32, 8).1;
                if fl.zf { pc = 82; continue; }
                pc = 81; continue;
            }
            81 => {
                fl = lift::sub((ecx & 0xff), 0x38u32, 8).1;
                if !fl.zf { pc = 84; continue; }
                pc = 82; continue;
            }
            82 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 84; continue; }
                pc = 83; continue;
            }
            83 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 86; continue; }
                pc = 84; continue;
            }
            84 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 79; continue; }
                pc = 85; continue;
            }
            85 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            86 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x3u32))), 0x38u32, 8).1;
                if !fl.zf { pc = 89; continue; }
                pc = 87; continue;
            }
            87 => {
                m.w32(ebp.wrapping_add(0x1070u32), 0x35u32);
                edx = lift::sx8(u32::from(m.r8(0x672ff5u32)));
                pc = 88; continue;
            }
            88 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x7u32)))) & 0xff);
                let (r, f) = lift::shl(ecx, 0x10u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5928a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x58cc90, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 89; continue;
            }
            89 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            90 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x768u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 48; continue; }
                pc = 91; continue;
            }
            91 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51f1f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x6u32)))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 96; continue; }
                pc = 92; continue;
            }
            92 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x37u32, 8).1;
                if fl.zf { pc = 94; continue; }
                pc = 93; continue;
            }
            93 => {
                fl = lift::sub((ecx & 0xff), 0x38u32, 8).1;
                if !fl.zf { pc = 96; continue; }
                pc = 94; continue;
            }
            94 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                fl = lift::sub((edx & 0xff), (ecx & 0xff), 8).1;
                if !fl.zf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x1u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 98; continue; }
                pc = 96; continue;
            }
            96 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x776u32)))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 91; continue; }
                pc = 97; continue;
            }
            97 => {
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            98 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x3u32))), 0x37u32, 8).1;
                if !fl.zf { pc = 89; continue; }
                pc = 99; continue;
            }
            99 => {
                m.w32(ebp.wrapping_add(0x1070u32), 0x34u32);
                edx = lift::sx8(u32::from(m.r8(0x672ff4u32)));
                pc = 88; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004DEA20` st_2c_update_ai_build_interval
pub fn f_004dea20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x8u32)))) & 0xffff);
                ecx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebx = ecx.wrapping_add(eax.wrapping_mul(8));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(0x6de4aau32)), (eax & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0xcu32), eax);
                if (fl.cf || fl.zf) { pc = 11; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x8u32))), 0x0u32, 16).1;
                if !fl.zf { pc = 9; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(0x6de4a8u32);
                edi = m.r32(eax.wrapping_add(0x474u32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xcu32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if !fl.cf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x530400, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2c8u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2ccu32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4a6u32))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4a6u32))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.cf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6de4a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(esi, ecx, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(edi, ebp, 32).1;
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ebp = edi;
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.cf { pc = 2; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                m.w32(ebx.wrapping_add(0xcu32), 0x6978000u32);
                edx = m.r32(0x6de4b0u32);
                fl = lift::logic(edx & edx, 32);
                ecx = 0x1u32;
                if (fl.cf || fl.zf) { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(0x6de508u32);
                let (r, f) = lift::add(eax, 0x2d4u32, 32); fl = f;
                eax = r;
                pc = 13; continue;
            }
            13 => {
                esi = m.r32(eax);
                let (r, f) = lift::add(eax, 0x9f8u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                if !fl.zf { pc = 13; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(ebx.wrapping_add(0xcu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = m.r32(ebx.wrapping_add(0x8u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esi = eax;
                eax = ebp;
                let (r, f) = lift::shl(eax, 0xau32, 32, fl); fl = f;
                eax = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(ebx.wrapping_add(0xcu32), esi);
                edx = m.r32(0x6de52cu32);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(ebx.wrapping_add(0x8u32), ebp);
                ecx = edx;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(edx.wrapping_mul(2));
                fl = lift::sub(ecx, 0x400u32, 32).1;
                if (fl.cf || fl.zf) { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = esi;
                let (r, f) = lift::shl(edx, 0xau32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(ecx, 0x600u32, 32).1;
                m.w32(ebx.wrapping_add(0xcu32), edx);
                if (fl.cf || fl.zf) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = 0xaaaaaaabu32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let r = u64::from(eax) * u64::from(edx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0xau32, 32, fl); fl = f;
                edx = r;
                m.w32(ebx.wrapping_add(0xcu32), edx);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            17 => {
                eax = edx;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(ebx.wrapping_add(0xcu32), eax);
                pc = 18; continue;
            }
            18 => {
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

/// `0X004E7A14` st_trade_offer_letter_watchdog
pub fn f_004e7a14(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0x14u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                if !fl.cf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.cf { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(0x6dd730u32);
                ebx = (ebx & !0xff) | ((0x1eu32) & 0xff);
                pc = 3; continue;
            }
            3 => {
                eax = ecx;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x4u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xcu32)))) & 0xffff);
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x10u32)), 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x14u32)), 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ebp = m.r32(eax.wrapping_add(0x8u32));
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x8u32)), 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 3; continue; }
                pc = 8; continue;
            }
            8 => {
                pc = 10; continue;
            }
            9 => {
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let r = ebp | 0xffffffffu32; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if !fl.cf { pc = 21; continue; }
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dd736u32))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.cf { pc = 21; continue; }
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(0x6dd730u32);
                ebx = (ebx & !0xff) | ((0x1fu32) & 0xff);
                pc = 13; continue;
            }
            13 => {
                eax = ecx;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0x4u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 18; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xcu32)))) & 0xffff);
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x10u32)), 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x14u32)), 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ebp = m.r32(eax.wrapping_add(0x8u32));
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0x8u32)), 32).1;
                if fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 18; continue;
            }
            18 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 13; continue; }
                pc = 19; continue;
            }
            19 => {
                pc = 21; continue;
            }
            20 => {
                ebp = ecx;
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(0x6de584u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x534090, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                pc = 24; continue;
            }
            23 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, 0xffffu32, 32).1;
                if !fl.cf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                fl = lift::sub(ebp, 0xffffu32, 32).1;
                if !fl.cf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x40u32, 32); fl = f;
                eax = r;
                m.w32(esi, eax);
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
            28 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.cf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(esi.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6a50, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if !fl.cf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd730u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d6a50, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 32; continue;
            }
            32 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x10u32));
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6de584u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5340d0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 34; continue;
            }
            34 => {
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
            _ => unreachable!(),
        }
    }
}

/// `0X00504AC0` ships_list_merchant_ships_in_town_by_filter
pub fn f_00504ac0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(0x8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xeu32)))) & 0xffff);
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                fl = lift::sub(eax, 0x9u32, 32).1;
                m.w16(edi, (0x0u32) as u16);
                if (!fl.cf && !fl.zf) { pc = 117; continue; }
                pc = 5; continue;
            }
            5 => {
                match m.r32(eax.wrapping_mul(4).wrapping_add(0x5051b0u32)) { 0x504b32 => { pc = 6; continue; } 0x504ba0 => { pc = 14; continue; } 0x504c88 => { pc = 30; continue; } 0x504d09 => { pc = 40; continue; } 0x504e0b => { pc = 55; continue; } 0x504e77 => { pc = 63; continue; } 0x504f02 => { pc = 73; continue; } 0x504f2c => { pc = 75; continue; } 0x504fac => { pc = 85; continue; } 0x505126 => { pc = 109; continue; } _ => unreachable!() }
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ebx = 0x1u32;
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebx & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0x136u32))) & (ebx & 0xff), 8);
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(edi.wrapping_add(ecx.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 12; continue;
            }
            12 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 15; continue;
            }
            15 => {
                ebx = m.r32(ebp.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = esi.wrapping_add(esi.wrapping_mul(2));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic(u32::from(m.r8(edx.wrapping_add(0x136u32))) & 0x1u32, 8);
                if !fl.zf { pc = 28; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf6u32))), 16).1;
                if !fl.cf { pc = 25; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = ecx.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub(eax, esi, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x14u32)))) & 0xff);
                let r = (eax & 0xff) & 0x2u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 28; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x12u32))), 0x1u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 28; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                pc = 22; continue;
            }
            22 => {
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(0x18u32));
                fl = lift::sub(ecx, m.r32(eax.wrapping_add(0x14u32)), 32).1;
                if (fl.sf != fl.of) { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf4u32))), 16).1;
                if !fl.cf { pc = 28; continue; }
                pc = 24; continue;
            }
            24 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                pc = 22; continue;
            }
            25 => {
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(0x134u32))), 0x1u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(edx.wrapping_add(0x18u32));
                ecx = m.r32(edx.wrapping_add(0x14u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edi.wrapping_add(eax.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            30 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ebx = 0x1u32;
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 38; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf6u32))), 16).1;
                if fl.cf { pc = 38; continue; }
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebx & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xfu32))), 0x2u32, 8).1;
                if !fl.cf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0x136u32))) & (ebx & 0xff), 8);
                if !fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(edi.wrapping_add(ecx.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 38; continue;
            }
            38 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            40 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                ebx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 53; continue; }
                pc = 42; continue;
            }
            42 => {
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x1u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 53; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf6u32))), 16).1;
                if fl.cf { pc = 46; continue; }
                pc = 44; continue;
            }
            44 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf2u32))), 16).1;
                if fl.cf { pc = 53; continue; }
                pc = 45; continue;
            }
            45 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edi.wrapping_add(eax.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 53; continue;
            }
            46 => {
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = ecx.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if !fl.zf { pc = 53; continue; }
                pc = 47; continue;
            }
            47 => {
                fl = lift::logic(u32::from(m.r8(ebx.wrapping_add(0x136u32))) & 0x1u32, 8);
                if !fl.zf { pc = 53; continue; }
                pc = 48; continue;
            }
            48 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 53; continue; }
                pc = 49; continue;
            }
            49 => {
                edx = m.r32(ebp.wrapping_add(0x4u32));
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx))) & 0xff);
                fl = lift::sub(edx, esi, 32).1;
                if !fl.zf { pc = 52; continue; }
                pc = 50; continue;
            }
            50 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf2u32))), 16).1;
                if fl.cf { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w16(edi.wrapping_add(edx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                pc = 52; continue;
            }
            52 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 49; continue; }
                pc = 53; continue;
            }
            53 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x4u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 41; continue; }
                pc = 54; continue;
            }
            54 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            55 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 56; continue;
            }
            56 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ebx = 0x1u32;
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 61; continue; }
                pc = 58; continue;
            }
            58 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebx & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 61; continue; }
                pc = 59; continue;
            }
            59 => {
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0xeu32))) & 0x2u32, 8);
                if !fl.zf { pc = 61; continue; }
                pc = 60; continue;
            }
            60 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(edi.wrapping_add(ecx.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 61; continue;
            }
            61 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 57; continue; }
                pc = 62; continue;
            }
            62 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            63 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 64; continue;
            }
            64 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                ebx = 0x1u32;
                pc = 65; continue;
            }
            65 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 71; continue; }
                pc = 66; continue;
            }
            66 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf2u32))), 16).1;
                if fl.cf { pc = 71; continue; }
                pc = 67; continue;
            }
            67 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 71; continue; }
                pc = 68; continue;
            }
            68 => {
                ecx = m.r32(eax.wrapping_add(0x14u32));
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(eax.wrapping_add(0x18u32)), ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 71; continue; }
                pc = 69; continue;
            }
            69 => {
                fl = lift::sub(u32::from(m.r16(eax.wrapping_add(0x134u32))), (ebx & 0xffff), 16).1;
                if (!fl.cf && !fl.zf) { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(edi.wrapping_add(ecx.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 71; continue;
            }
            71 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 65; continue; }
                pc = 72; continue;
            }
            72 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            73 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x20u32));
                ebx = 0x1u32;
                ecx = edx;
                let (r, f) = lift::shl(ebx, (ecx & 0xff), 32, fl); fl = f;
                ebx = r;
                fl = lift::logic(m.r32(eax.wrapping_add(0x468u32)) & ebx, 32);
                if fl.zf { pc = 117; continue; }
                pc = 74; continue;
            }
            74 => {
                pc = 76; continue;
            }
            75 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                pc = 76; continue;
            }
            76 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 77; continue;
            }
            77 => {
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                eax = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 83; continue; }
                pc = 78; continue;
            }
            78 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 83; continue; }
                pc = 79; continue;
            }
            79 => {
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0x136u32))) & 0x1u32, 8);
                if !fl.zf { pc = 83; continue; }
                pc = 80; continue;
            }
            80 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::logic((ecx & 0xffff) & (ecx & 0xffff), 16);
                if fl.zf { pc = 82; continue; }
                pc = 81; continue;
            }
            81 => {
                fl = lift::sub((ecx & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 83; continue; }
                pc = 82; continue;
            }
            82 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(edi.wrapping_add(ecx.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 83; continue;
            }
            83 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x4u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 77; continue; }
                pc = 84; continue;
            }
            84 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            85 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 109; continue; }
                pc = 86; continue;
            }
            86 => {
                edx = m.r32(ebp.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                ebx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 108; continue; }
                pc = 87; continue;
            }
            87 => {
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x1u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 108; continue; }
                pc = 88; continue;
            }
            88 => {
                fl = lift::logic(u32::from(m.r8(ebx.wrapping_add(0x136u32))) & 0x1u32, 8);
                if !fl.zf { pc = 108; continue; }
                pc = 89; continue;
            }
            89 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf6u32))), 16).1;
                if fl.cf { pc = 97; continue; }
                pc = 90; continue;
            }
            90 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = ebx.wrapping_add(0xa4u32);
                ecx = 0x4u32;
                pc = 91; continue;
            }
            91 => {
                fl = lift::sub(m.r32(eax), 0xau32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 93; continue; }
                pc = 92; continue;
            }
            92 => {
                edx = 0x1u32;
                pc = 93; continue;
            }
            93 => {
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 91; continue; }
                pc = 94; continue;
            }
            94 => {
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, 0xau32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                eax = ecx;
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 108; continue; }
                pc = 96; continue;
            }
            96 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edi.wrapping_add(eax.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 108; continue;
            }
            97 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(ebp.wrapping_add(0x8u32));
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if !fl.zf { pc = 108; continue; }
                pc = 98; continue;
            }
            98 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 108; continue; }
                pc = 99; continue;
            }
            99 => {
                eax = m.r32(ebp.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx))) & 0xff);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 107; continue; }
                pc = 100; continue;
            }
            100 => {
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                eax = ebx.wrapping_add(0xa4u32);
                m.w32(esp.wrapping_add(0x24u32), 0x4u32);
                pc = 101; continue;
            }
            101 => {
                fl = lift::sub(m.r32(eax), 0xau32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 103; continue; }
                pc = 102; continue;
            }
            102 => {
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                pc = 103; continue;
            }
            103 => {
                edx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                if !fl.zf { pc = 101; continue; }
                pc = 104; continue;
            }
            104 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 106; continue; }
                pc = 105; continue;
            }
            105 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, 0xau32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 107; continue; }
                pc = 106; continue;
            }
            106 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edi.wrapping_add(eax.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 107; continue;
            }
            107 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x6u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 99; continue; }
                pc = 108; continue;
            }
            108 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x4u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 86; continue; }
                pc = 109; continue;
            }
            109 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 117; continue; }
                pc = 110; continue;
            }
            110 => {
                edx = m.r32(ebp.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = esi.wrapping_add(esi.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 116; continue; }
                pc = 111; continue;
            }
            111 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebp.wrapping_add(0xf6u32))), 16).1;
                if fl.cf { pc = 116; continue; }
                pc = 112; continue;
            }
            112 => {
                fl = lift::sub(u32::from(m.r16(ecx.wrapping_add(0x134u32))), 0x1u32, 16).1;
                if (!fl.cf && !fl.zf) { pc = 116; continue; }
                pc = 113; continue;
            }
            113 => {
                { let a = []; eax = h.call_at(esp, m, 0x519c40, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 116; continue; }
                pc = 114; continue;
            }
            114 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(ecx.wrapping_add(0x120u32));
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf != fl.of) { pc = 116; continue; }
                pc = 115; continue;
            }
            115 => {
                let (r, f) = lift::add(u32::from(m.r16(edi)), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(edi, (r) as u16);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w16(edi.wrapping_add(eax.wrapping_mul(2)), ((esi & 0xffff)) as u16);
                pc = 116; continue;
            }
            116 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0x4u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xf4u32)))) & 0xffff);
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 110; continue; }
                pc = 117; continue;
            }
            117 => {
                eax = edi;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005121C0` name_bank_load_name_files
pub fn f_005121c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(esi), 0x0u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c051cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5125e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebp = 0x6ddb90u32;
                ebx = 0x28u32;
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x51e440, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebp, 0x34u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x512730, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa8u32));
                edi = esi.wrapping_add(0xa8u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0504u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c053cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c04f4u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w16(esi.wrapping_add(0xd0u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax).wrapping_add(0x2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xbcu32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                edx = ecx.wrapping_add(ecx).wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xbcu32));
                esp = esp.wrapping_add(0x10);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c054cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5128d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = 0x1u32;
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xbcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x3u32, 32, fl); fl = f;
                ebx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add((ebp & 0xffff), (ebx & 0xffff), 16); fl = f;
                ebp = (ebp & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x15u32, 32).1;
                m.w16(edx.wrapping_add(ecx.wrapping_mul(2)), ((ebp & 0xffff)) as u16);
                if fl.cf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xbcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.cf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(edi);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(eax.wrapping_add(edx), (0x0u32) as u8);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd0u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0xbcu32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0504u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0568u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c04f4u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w16(esi.wrapping_add(0xd2u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax).wrapping_add(0x2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xc0u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                edx = ecx.wrapping_add(ecx).wrapping_add(0x2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xc0u32));
                esp = esp.wrapping_add(0x10);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5128d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = 0x1u32;
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x3u32, 32, fl); fl = f;
                ebx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add((ebp & 0xffff), (ebx & 0xffff), 16); fl = f;
                ebp = (ebp & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x15u32, 32).1;
                m.w16(edx.wrapping_add(ecx.wrapping_mul(2)), ((ebp & 0xffff)) as u16);
                if fl.cf { pc = 10; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.cf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(edi);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(eax.wrapping_add(edx), (0x0u32) as u8);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd2u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 12; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0xb0u32));
                edi = esi.wrapping_add(0xb0u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 18; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0504u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c058cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c04f4u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0504u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c059cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c04f4u32);
                ecx = 0x6dd524u32;
                m.w16(esi.wrapping_add(0xd4u32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let (r, f) = lift::add((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xd4u32))), 16); fl = f;
                eax = (eax & !0xffff) | ((r) & 0xffff);
                m.w16(esi.wrapping_add(0xd6u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax).wrapping_add(0x2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xc4u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd6u32)))) & 0xffff);
                edx = ecx.wrapping_add(ecx).wrapping_add(0x2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xc4u32));
                esp = esp.wrapping_add(0x10);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c05acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5128d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = 0x1u32;
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi.wrapping_add(0xc4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd6u32)))) & 0xffff);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x3u32, 32, fl); fl = f;
                ebx = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add((ebp & 0xffff), (ebx & 0xffff), 16); fl = f;
                ebp = (ebp & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x15u32, 32).1;
                m.w16(edx.wrapping_add(ecx.wrapping_mul(2)), ((ebp & 0xffff)) as u16);
                if fl.cf { pc = 15; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esi.wrapping_add(0xc4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd6u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.cf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(edi);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(eax.wrapping_add(edx), (0x0u32) as u8);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd6u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0xc4u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0x28u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.cf { pc = 17; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                edi = esi.wrapping_add(0xb4u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0504u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x12cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c05c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c04f4u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w16(esi.wrapping_add(0xd8u32), ((eax & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0xdau32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax).wrapping_add(0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esi.wrapping_add(0xc8u32), eax);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                edx = ecx.wrapping_add(ecx).wrapping_add(0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_add(0x10);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c05d8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5128d0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(eax, ebx, 32).1;
                m.w16(esi.wrapping_add(0xdau32), ((edx & 0xffff)) as u16);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xccu32), ebx);
                pc = 24; continue;
            }
            24 => {
                m.w16(esi.wrapping_add(0xdcu32), ((ebx & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0xe0u32), ebx);
                m.w32(esi.wrapping_add(0xe4u32), ebx);
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

/// `0X005131B0` name_bank_reconcile_ship_name_id
pub fn f_005131b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                if fl.zf { pc = 66; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(ebx.wrapping_add(0xd8u32))), 16).1;
                if fl.cf { pc = 59; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub((edi & 0xffff), 0x12cu32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c05f8u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c05f8u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 4; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((edi & 0xffff), 0x12cu32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0608u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0608u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 10; continue;
            }
            10 => {
                pc = 13; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x2u32) as u16);
                pc = 59; continue;
            }
            12 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x9u32) as u16);
                pc = 59; continue;
            }
            13 => {
                fl = lift::sub((edi & 0xffff), 0x12cu32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0614u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0614u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub((edi & 0xffff), 0x12cu32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0620u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0620u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 19; continue; }
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub((edi & 0xffff), 0x12cu32, 16).1;
                if !fl.zf { pc = 28; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0630u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0630u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 28; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 23; continue; }
                pc = 25; continue;
            }
            25 => {
                pc = 28; continue;
            }
            26 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x124u32) as u16);
                pc = 59; continue;
            }
            27 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0xa3u32) as u16);
                pc = 59; continue;
            }
            28 => {
                fl = lift::sub((edi & 0xffff), 0x12du32, 16).1;
                if !fl.zf { pc = 43; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c063cu32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c063cu32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 37; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 30; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub((edi & 0xffff), 0x12du32, 16).1;
                if !fl.zf { pc = 43; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0648u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0648u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 39; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 34; continue; }
                pc = 36; continue;
            }
            36 => {
                pc = 39; continue;
            }
            37 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x107u32) as u16);
                pc = 59; continue;
            }
            38 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x125u32) as u16);
                pc = 59; continue;
            }
            39 => {
                fl = lift::sub((edi & 0xffff), 0x12du32, 16).1;
                if !fl.zf { pc = 43; continue; }
                pc = 40; continue;
            }
            40 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0654u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0654u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 43; continue; }
                pc = 41; continue;
            }
            41 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 42; continue;
            }
            42 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 41; continue; }
                pc = 43; continue;
            }
            43 => {
                fl = lift::sub((edi & 0xffff), 0x12eu32, 16).1;
                if !fl.zf { pc = 54; continue; }
                pc = 44; continue;
            }
            44 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c065cu32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c065cu32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 47; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 52; continue; }
                pc = 46; continue;
            }
            46 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 45; continue; }
                pc = 47; continue;
            }
            47 => {
                fl = lift::sub((edi & 0xffff), 0x12eu32, 16).1;
                if !fl.zf { pc = 54; continue; }
                pc = 48; continue;
            }
            48 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0668u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0668u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 54; continue; }
                pc = 49; continue;
            }
            49 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 53; continue; }
                pc = 50; continue;
            }
            50 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 49; continue; }
                pc = 51; continue;
            }
            51 => {
                pc = 54; continue;
            }
            52 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x88u32) as u16);
                pc = 59; continue;
            }
            53 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0xe1u32) as u16);
                pc = 59; continue;
            }
            54 => {
                fl = lift::sub((edi & 0xffff), 0x12fu32, 16).1;
                if !fl.zf { pc = 58; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                esi = 0x6c0670u32;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(0x6c0670u32)), 8).1;
                eax = edx;
                if !fl.zf { pc = 58; continue; }
                pc = 56; continue;
            }
            56 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 63; continue; }
                pc = 57; continue;
            }
            57 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 56; continue; }
                pc = 58; continue;
            }
            58 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = 0x7fffu32;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, ((ecx & 0xffff)) as u16);
                pc = 59; continue;
            }
            59 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebx.wrapping_add(0xd8u32))), 16).1;
                if !fl.cf { pc = 64; continue; }
                pc = 60; continue;
            }
            60 => {
                ecx = m.r32(ebx.wrapping_add(0xc8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(ebx.wrapping_add(0xb4u32));
                eax = esi;
                esi = edx;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 61; continue;
            }
            61 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                m.w8(esi, ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 61; continue; }
                pc = 62; continue;
            }
            62 => {
                pc = 66; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(eax, (0x108u32) as u16);
                pc = 59; continue;
            }
            64 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                esi = edx;
                eax = m.r32(eax.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                pc = 65; continue;
            }
            65 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                m.w8(esi, ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 65; continue; }
                pc = 66; continue;
            }
            66 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xd8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), ebp);
                edi = edx;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 83; continue; }
                pc = 67; continue;
            }
            67 => {
                ecx = m.r32(ebx.wrapping_add(0xc8u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(ebx.wrapping_add(0xb4u32));
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 68; continue;
            }
            68 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 99; continue; }
                pc = 69; continue;
            }
            69 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 68; continue; }
                pc = 70; continue;
            }
            70 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((ebp & 0xffff) & (ebp & 0xffff), 16);
                if (fl.cf || fl.zf) { pc = 76; continue; }
                pc = 71; continue;
            }
            71 => {
                eax = m.r32(ebx.wrapping_add(0xb4u32));
                ebp = m.r32(ebx.wrapping_add(0xc8u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 72; continue;
            }
            72 => {
                eax = edi;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                eax = esi;
                esi = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esi = edx;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(eax)), 8).1;
                if !fl.zf { pc = 75; continue; }
                pc = 73; continue;
            }
            73 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 93; continue; }
                pc = 74; continue;
            }
            74 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ebx & 0xff), u32::from(m.r8(eax)), 8).1;
                if fl.zf { pc = 73; continue; }
                pc = 75; continue;
            }
            75 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(esp.wrapping_add(0x28u32))), 16).1;
                if fl.cf { pc = 72; continue; }
                pc = 76; continue;
            }
            76 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp.wrapping_add(0x28u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0xdau32)))) & 0xffff);
                fl = lift::sub((edi & 0xffff), (eax & 0xffff), 16).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.cf { pc = 94; continue; }
                pc = 77; continue;
            }
            77 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ebp = edi;
                let r = ebp & 0xffffu32; fl = lift::logic(r, 32);
                ebp = r;
                esi = m.r32(eax.wrapping_add(0xccu32));
                eax = m.r32(eax.wrapping_add(0xb8u32));
                m.w32(esp.wrapping_add(0x28u32), esi);
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 78; continue;
            }
            78 => {
                ebx = m.r32(esp.wrapping_add(0x28u32));
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                eax = esi;
                esi = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esi = edx;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(eax)), 8).1;
                if !fl.zf { pc = 81; continue; }
                pc = 79; continue;
            }
            79 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 93; continue; }
                pc = 80; continue;
            }
            80 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ebx & 0xff), u32::from(m.r8(eax)), 8).1;
                if fl.zf { pc = 79; continue; }
                pc = 81; continue;
            }
            81 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(esp.wrapping_add(0x14u32))), 16).1;
                if fl.cf { pc = 78; continue; }
                pc = 82; continue;
            }
            82 => {
                pc = 94; continue;
            }
            83 => {
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(ebx.wrapping_add(0xdau32))), 16).1;
                if !fl.cf { pc = 88; continue; }
                pc = 84; continue;
            }
            84 => {
                ecx = ebp;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(ebx.wrapping_add(0xccu32));
                esi = (esi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(ebx.wrapping_add(0xb8u32));
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 85; continue;
            }
            85 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 99; continue; }
                pc = 86; continue;
            }
            86 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 85; continue; }
                pc = 87; continue;
            }
            87 => {
                pc = 70; continue;
            }
            88 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esi = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(esi)), 8).1;
                if !fl.zf { pc = 70; continue; }
                pc = 89; continue;
            }
            89 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 92; continue; }
                pc = 90; continue;
            }
            90 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi)), 8).1;
                if fl.zf { pc = 89; continue; }
                pc = 91; continue;
            }
            91 => {
                pc = 70; continue;
            }
            92 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = 0x7fffu32;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w16(edx, ((ecx & 0xffff)) as u16);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            93 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w16(ecx, ((edi & 0xffff)) as u16);
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
            94 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 95; continue;
            }
            95 => {
                ecx = eax;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(edx))), 0x0u32, 8).1;
                if fl.zf { pc = 100; continue; }
                pc = 96; continue;
            }
            96 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((eax & 0xffff), 0x20u32, 16).1;
                if fl.cf { pc = 95; continue; }
                pc = 97; continue;
            }
            97 => {
                ebp = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = 0x7fffu32;
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w16(eax, ((ecx & 0xffff)) as u16);
                esi = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                pc = 98; continue;
            }
            98 => {
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(esi)))) & 0xff);
                m.w8(eax.wrapping_add(edx), ((ecx & 0xff)) as u8);
                esi = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(esi))), 0x0u32, 8).1;
                if !fl.zf { pc = 98; continue; }
                pc = 99; continue;
            }
            99 => {
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
            100 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x512d40, ecx, &a); }
                esp = esp.wrapping_add(8);
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
            _ => unreachable!(),
        }
    }
}

/// `0X00517C10` ship_fix_route_after_load
pub fn f_00517c10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0x24u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 33; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x34u32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esi.wrapping_add(0x28u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(ecx);
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 9; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(edx & edx, 32);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x1eu32))), 16).1;
                if !fl.zf { pc = 15; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0x22u32))), 16).1;
                pc = 14; continue;
            }
            6 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x1eu32))), 16).1;
                ebx = (ebx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                fl = lift::logic(edx & edx, 32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2u32)))) & 0xffff);
                ecx = ebx;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x22u32))), 16).1;
                eax = (eax & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                let r = ecx | eax; fl = lift::logic(r, 32);
                ecx = r;
                pc = 13; continue;
            }
            8 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x22u32))), 16).1;
                eax = (eax & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                let r = ecx | eax; fl = lift::logic(r, 32);
                ecx = r;
                pc = 13; continue;
            }
            9 => {
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 21; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x2u32)))) & 0xffff);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x22u32))), 16).1;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0x1eu32))), 16).1;
                edx = (edx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                pc = 12; continue;
            }
            12 => {
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 13; continue;
            }
            13 => {
                fl = lift::logic(ecx & ecx, 32);
                pc = 14; continue;
            }
            14 => {
                if fl.zf { pc = 33; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(0x66d7ccu32);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1eu32)))) & 0xffff);
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf != fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(0x66d7d0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x22u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x50d040, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = 0x64u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6dd7a0u32;
                m.w16(esi.wrapping_add(0x1eu32), ((eax & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x22u32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x50ce50, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6de4b0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 31; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5172a0, ecx, &a); }
                esp = esp.wrapping_add(16);
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
            20 => {
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0x1eu32))), 16).1;
                edx = (edx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                pc = 12; continue;
            }
            21 => {
                ebx = ecx.wrapping_add(eax.wrapping_mul(4));
                ebp = m.r32(esi.wrapping_add(0x1cu32));
                eax = lift::sx16(u32::from(m.r16(ebx)));
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esi.wrapping_add(0x2cu32));
                ecx = eax;
                eax = lift::sx16(u32::from(m.r16(ebx.wrapping_add(0x2u32))));
                ebx = m.r32(esi.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(edi as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = ecx;
                ecx = eax;
                eax = edx;
                pc = 23; continue;
            }
            23 => {
                fl = lift::logic(ecx & ecx, 32);
                if (fl.sf == fl.of) { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::add(eax, 0x2u32, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 15; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = m.r32(esi.wrapping_add(0x28u32));
                let (r, f) = lift::imul(edi, ecx); fl = f;
                edi = r;
                let (r, f) = lift::imul(edx, ecx); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                pc = 27; continue;
            }
            27 => {
                ebp = m.r32(esp.wrapping_add(0x10u32));
                ebx = edx;
                let (r, f) = lift::sar(ebx, 0x10u32, 32, fl); fl = f;
                ebx = r;
                fl = lift::sub(ebp, ebx, 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                ebx = edi;
                let (r, f) = lift::sar(ebx, 0x10u32, 32, fl); fl = f;
                ebx = r;
                fl = lift::sub(ebp, ebx, 32).1;
                if fl.zf { pc = 33; continue; }
                pc = 29; continue;
            }
            29 => {
                ebx = m.r32(esi.wrapping_add(0x28u32));
                ebp = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 27; continue; }
                pc = 30; continue;
            }
            30 => {
                pc = 15; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x24u32));
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = edx.wrapping_add(ecx.wrapping_mul(4));
                ecx = lift::sx16(u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffffeu32))));
                edx = lift::sx16(u32::from(m.r16(eax.wrapping_add(0xfffffffcu32))));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5172a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 33; continue;
            }
            33 => {
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
            _ => unreachable!(),
        }
    }
}

/// `0X00518860` ship_run_trade_route_stop
pub fn f_00518860(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de588u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x535400, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                if !fl.cf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(edx.wrapping_mul(2));
                edx = m.r32(0x6dd72cu32);
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x2u32)))) & 0xff);
                ecx = m.r32(0x6de4b0u32);
                fl = lift::sub(ebx, ecx, 32).1;
                if fl.cf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = eax;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                ecx = edi.wrapping_add(edi.wrapping_mul(4));
                ecx = edi.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                m.w16(esi.wrapping_add(0x132u32), ((ecx & 0xffff)) as u16);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w16(esi.wrapping_add(0x132u32), (0xffffu32) as u16);
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                ecx = 0x6dd7a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x509030, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6dd728u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d4e80, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                if fl.cf { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x136u32)))) & 0xff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                let r = (ebx & 0xff) & 0xfcu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = u32::from(m.r8(esi.wrapping_add(0x3du32))) | 0x8u32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0x3du32), (r) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esi.wrapping_add(0x136u32), ((ebx & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 23; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x2u32)))) & 0xff);
                edx = edx.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                m.w16(esi.wrapping_add(0x134u32), (0x2u32) as u16);
                m.w8(esi.wrapping_add(0x37u32), ((eax & 0xff)) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x3u32)))) & 0xff);
                ebx = 0x1u32;
                fl = lift::logic((ebx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edi, 0x6u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(0x6de508u32);
                edi = eax.wrapping_add(edi.wrapping_mul(8));
                eax = m.r32(edi.wrapping_add(0x76cu32));
                let r = eax & 0x40u32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(edx.wrapping_add(0x3u32), ((ecx & 0xff)) as u8);
                pc = 13; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(eax.wrapping_mul(8));
                fl = lift::sub(edx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x52acd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 29; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                m.w8(esi.wrapping_add(0x37u32), ((eax & 0xff)) as u8);
                eax = (eax & !0xffff) | (((eax & 0xff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6dd728u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d5200, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3fu32)))) & 0xff);
                m.w16(esi.wrapping_add(0x134u32), ((ebx & 0xffff)) as u16);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = (edx & !0xffff) | ((0x500u32) & 0xffff);
                let (r, f) = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x3eu32))), 16); fl = f;
                edx = (edx & !0xffff) | ((r) & 0xffff);
                let (r, f) = lift::add(u32::from(m.r16(esi.wrapping_add(0x138u32))), (edx & 0xffff), 16); fl = f;
                m.w16(esi.wrapping_add(0x138u32), (r) as u16);
                pc = 16; continue;
            }
            15 => {
                m.w16(esi.wrapping_add(0x138u32), (0x40u32) as u16);
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w8(esi.wrapping_add(0x3fu32), ((ebx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x519ba0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 25; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x136u32)))) & 0xff);
                eax = 0x66666667u32;
                let r = (ecx & 0xff) & 0xfcu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = u32::from(m.r8(esi.wrapping_add(0x3du32))) | 0x8u32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0x3du32), (r) as u8);
                m.w8(esi.wrapping_add(0x136u32), ((ecx & 0xff)) as u8);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = (edx & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x23u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x548ca0, ecx, &a); }
                esp = esp.wrapping_add(24);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            21 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xeu32)))) & 0xff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = ecx & 0x3u32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = (edx & !0xffff) | ((u32::from(m.r8(ecx.wrapping_add(0x673660u32)))) & 0xffff);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x40u32))), (edx & 0xffff), 16).1;
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                if !fl.cf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x28u32);
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x548ca0, ecx, &a); }
                esp = esp.wrapping_add(24);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 23; continue;
            }
            25 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                pc = 26; continue;
            }
            26 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd72cu32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                m.w16(esi.wrapping_add(0x132u32), ((eax & 0xffff)) as u16);
                if fl.zf { pc = 30; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6dd728u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d5ab0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 28; continue;
            }
            28 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x132u32)))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd72cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x2u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x38u32), ((eax & 0xff)) as u8);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x5172a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 29; continue;
            }
            29 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            30 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x136u32)))) & 0xff);
                edx = (edx & !0xffff) | ((u32::from(m.r8(esi.wrapping_add(0x39u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r8(esi))) & 0xffff);
                let r = u32::from(m.r8(esi.wrapping_add(0x3du32))) | 0x8u32; fl = lift::logic(r, 8);
                m.w8(esi.wrapping_add(0x3du32), (r) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = (ebx & 0xff) & 0xfcu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w8(esi.wrapping_add(0x136u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x548ca0, ecx, &a); }
                esp = esp.wrapping_add(24);
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

/// `0X00532290` sum_world_stock_and_consumption_per_ware
pub fn f_00532290(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x60u32);
                edi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x20u32), edi);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(edi.wrapping_add(0x10u32));
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::sub(ebx, esi, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(edi.wrapping_add(0x68u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esi;
                edi = 0x18u32;
                ecx = edx.wrapping_add(0x310u32);
                pc = 3; continue;
            }
            3 => {
                ebp = m.r32(ecx.wrapping_add(0xfffffcf4u32));
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(m.r32(ebx.wrapping_add(eax)), ebp, 32); fl = f;
                m.w32(ebx.wrapping_add(eax), r);
                ebp = m.r32(ecx.wrapping_add(0xfffffd50u32));
                let (r, f) = lift::add(ebp, m.r32(ecx.wrapping_add(0xfffffffcu32)), 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(m.r32(eax.wrapping_add(0xfffffffcu32)), ebp, 32); fl = f;
                m.w32(eax.wrapping_add(0xfffffffcu32), r);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x784u32)))) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = ecx.wrapping_add(ecx.wrapping_mul(4));
                eax = ecx.wrapping_add(edx.wrapping_mul(2));
                ecx = m.r32(edi.wrapping_add(0x74u32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = ecx.wrapping_add(eax.wrapping_mul(4));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x2cau32)))) & 0xffff);
                let (r, f) = lift::add(edx, 0x64u32, 32); fl = f;
                edx = r;
                pc = 6; continue;
            }
            6 => {
                ebp = m.r32(edx);
                let (r, f) = lift::add(edx, 0x4u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(m.r32(esi.wrapping_add(eax.wrapping_mul(4))), ebp, 32); fl = f;
                m.w32(esi.wrapping_add(eax.wrapping_mul(4)), r);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x18u32, 32).1;
                if fl.cf { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub(ecx, edx, 32).1;
                if fl.cf { pc = 5; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(edi.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x9f8u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                if fl.cf { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                pc = 10; continue;
            }
            10 => {
                edx = 0xfffffff8u32;
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(edx, 0xbu32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 22; continue; }
                pc = 12; continue;
            }
            12 => {
                match m.r32(edx.wrapping_mul(4).wrapping_add(0x5323e4u32)) { 0x53238b => { pc = 13; continue; } 0x532392 => { pc = 14; continue; } 0x532399 => { pc = 15; continue; } 0x5323a0 => { pc = 16; continue; } 0x5323a7 => { pc = 17; continue; } 0x5323ae => { pc = 18; continue; } 0x5323cb => { pc = 22; continue; } _ => unreachable!() }
            }
            13 => {
                ecx = 0xe10u32;
                pc = 19; continue;
            }
            14 => {
                ecx = 0x14820u32;
                pc = 19; continue;
            }
            15 => {
                ecx = 0x4e20u32;
                pc = 19; continue;
            }
            16 => {
                ecx = 0xfa0u32;
                pc = 19; continue;
            }
            17 => {
                ecx = 0x320u32;
                pc = 19; continue;
            }
            18 => {
                ecx = 0x27100u32;
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(edi.wrapping_add(0x10u32));
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                ecx = m.r32(ebx);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.cf || fl.zf) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                m.w32(ebx, 0x0u32);
                pc = 22; continue;
            }
            21 => {
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(ebx, ecx);
                pc = 22; continue;
            }
            22 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                eax = edx.wrapping_add(0x8u32);
                fl = lift::sub(eax, 0x18u32, 32).1;
                if fl.cf { pc = 11; continue; }
                pc = 23; continue;
            }
            23 => {
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
            _ => unreachable!(),
        }
    }
}

/// `0X0054A310` reset_for_new_game
pub fn f_0054a310(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6e55d0u32;
                { let a = []; eax = h.call_at(esp, m, 0x611840, ecx, &a); }
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(0x6dfc14u32, 0xffffu32);
                m.w32(0x6dfc04u32, ebp);
                m.w32(0x6dfc18u32, ebp);
                eax = 0x6dfc38u32;
                pc = 1; continue;
            }
            1 => {
                m.w16(eax, (0xffffu32) as u16);
                let (r, f) = lift::add(eax, 0xau32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x6dfc88u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x66de7cu32);
                ecx = 0x6dfc90u32;
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x8u32, 32, fl); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x8dcu32), eax);
                m.w32(esi.wrapping_add(0x8d4u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x54ddf0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x530230, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6df278u32;
                { let a = []; eax = h.call_at(esp, m, 0x534890, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x504750, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = 0x6dd730u32;
                { let a = []; eax = h.call_at(esp, m, 0x4d64b0, ecx, &a); }
                ecx = 0x6dd728u32;
                { let a = []; eax = h.call_at(esp, m, 0x4d4c60, ecx, &a); }
                ecx = 0x6dd73cu32;
                { let a = []; eax = h.call_at(esp, m, 0x4d8520, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x54c3b0, ecx, &a); }
                ecx = esi.wrapping_add(0x89cu32);
                m.w16(esi.wrapping_add(0x56u32), ((ebp & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x482u32), ((ebp & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                ecx = esi.wrapping_add(0x8b8u32);
                { let a = []; eax = h.call_at(esp, m, 0x543a70, ecx, &a); }
                ecx = 0x6ddaa0u32;
                { let a = []; eax = h.call_at(esp, m, 0x5121c0, ecx, &a); }
                edi = esi.wrapping_add(0x8e0u32);
                ebx = 0x8u32;
                pc = 3; continue;
            }
            3 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esi.wrapping_add(0x900u32), 0x177c6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                let (r, f) = lift::add(esi, 0x904u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x47b8c0 => f_0047b8c0(m, h, ecx, esp),
        0x4dea20 => f_004dea20(m, h, ecx, esp),
        0x4e7a14 => f_004e7a14(m, h, ecx, esp),
        0x504ac0 => f_00504ac0(m, h, ecx, esp),
        0x5121c0 => f_005121c0(m, h, ecx, esp),
        0x5131b0 => f_005131b0(m, h, ecx, esp),
        0x517c10 => f_00517c10(m, h, ecx, esp),
        0x518860 => f_00518860(m, h, ecx, esp),
        0x532290 => f_00532290(m, h, ecx, esp),
        0x54a310 => f_0054a310(m, h, ecx, esp),
        _ => return None,
    }))
}
