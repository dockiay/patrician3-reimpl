//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X005128D0` name_bank_parse_name_list_file
pub fn f_005128d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebp = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic(ebp & ebp, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if (fl.cf || fl.zf) { pc = 20; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                fl = lift::sub(u32::from(m.r8(eax)), 0x23u32, 8).1;
                if fl.zf { pc = 20; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x24u32);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6dccc8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x6389ee, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 20; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, 0x1e0u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                esi = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x4);
                m.w32(ebx, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                edi = esi.wrapping_add(edx);
                edx = 0x1u32;
                fl = lift::sub(esi, edi, 32).1;
                m.w32(esp.wrapping_add(0x28u32), edi);
                m.w16(eax.wrapping_add(0x2u32), (0x0u32) as u16);
                m.w16(eax, (0x0u32) as u16);
                if !fl.cf { pc = 13; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x41u32, 8).1;
                if !fl.cf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub((ecx & 0xff), 0x20u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub((ecx & 0xff), 0x2eu32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, edi, 32).1;
                if !fl.cf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(u32::from(m.r8(esi)), 0x41u32, 8).1;
                if fl.cf { pc = 7; continue; }
                pc = 9; continue;
            }
            9 => {
                ebp = m.r32(ebx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                m.w8(ecx.wrapping_add(ebp), (0x0u32) as u8);
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2)))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)), (r) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                fl = lift::sub(edx, ebp, 32).1;
                if !fl.cf { pc = 15; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)).wrapping_add(0x2u32), ((ecx & 0xffff)) as u16);
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 12; continue;
            }
            11 => {
                ebp = m.r32(ebx);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                m.w8(edi.wrapping_add(ebp), ((ecx & 0xff)) as u8);
                let (r, f) = lift::add(u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2)))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)), (r) as u16);
                edi = m.r32(esp.wrapping_add(0x28u32));
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(esi, edi, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 13; continue;
            }
            13 => {
                esi = m.r32(ebx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0xffffffffu32))), 0x0u32, 8).1;
                if fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w8(ecx, (0x0u32) as u8);
                let (r, f) = lift::add(u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2)))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(eax.wrapping_add(edx.wrapping_mul(2)), (r) as u16);
                pc = 17; continue;
            }
            15 => {
                edx = m.r32(ebx);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(ecx.wrapping_add(edx), (0x0u32) as u8);
                ecx = m.r32(0x6dccc8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x6389f4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
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
            16 => {
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 17; continue;
            }
            17 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edi = eax.wrapping_add(edx.wrapping_mul(2));
                m.w16(eax.wrapping_add(ebp.wrapping_mul(2)), ((ecx & 0xffff)) as u16);
                eax = m.r32(ebx);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edx, ebp, 32).1;
                m.w8(ecx.wrapping_add(eax), (0x0u32) as u8);
                if !fl.cf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = ebp;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shr(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                let (r, f) = lift::adc(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(0x6dccc8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x6389f4, ecx, &a); }
                esp = esp.wrapping_add(0x8);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ecx, eax);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w8(eax, (0x0u32) as u8);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

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

/// `0X00548160` operations_send_player_name_chunks
pub fn f_00548160(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(ebp);
                edi = m.r32(eax.wrapping_add(0xfffffff8u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x8u32, 8).1;
                m.w32(esp.wrapping_add(0x10u32), 0xa3u32);
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                pc = 3; continue;
            }
            2 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x2cu32)))) & 0xff);
                m.w8(esp.wrapping_add(0x14u32), ((ecx & 0xff)) as u8);
                pc = 3; continue;
            }
            3 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(edi & edi, 32);
                m.w8(esp.wrapping_add(0x15u32), (0x0u32) as u8);
                if (fl.cf || fl.zf) { pc = 11; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, 0xeu32, 32).1;
                if !fl.cf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ebp);
                ecx = esi;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx)))) & 0xff);
                m.w8(esp.wrapping_add(eax).wrapping_add(0x15u32), ((ecx & 0xff)) as u8);
                pc = 7; continue;
            }
            6 => {
                edx = esp.wrapping_add(0x10u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x15u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x15u32), ((ecx & 0xff)) as u8);
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(esi, edi, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(eax, 0xeu32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x15u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add((ecx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x15u32), ((ecx & 0xff)) as u8);
                pc = 11; continue;
            }
            10 => {
                if !fl.cf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = 0xeu32;
                edi = esp.wrapping_add(eax).wrapping_add(0x16u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = ecx;
                let (r, f) = lift::shr(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                ecx = edx;
                let r = ecx & 0x3u32; fl = lift::logic(r, 32);
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 1);
                pc = 12; continue;
            }
            12 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
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
            _ => unreachable!(),
        }
    }
}

/// `0X0054AC80` join_lan_game
pub fn f_0054ac80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = ebp | 0xffffffffu32; fl = lift::logic(r, 32);
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::sub(m.r32(esi.wrapping_add(0x2cu32)), ebp, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x3u32;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(0x66a2bcu32);
                edi = 0xfffffffeu32;
                m.w32(esi.wrapping_add(0x24u32), eax);
                m.w32(esi.wrapping_add(0x28u32), ecx);
                m.w32(esi.wrapping_add(0x2cu32), edi);
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esi.wrapping_add(0x24u32), ebp);
                m.w32(esi.wrapping_add(0x28u32), ebp);
                if !fl.cf { pc = 26; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x40u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), 0xceu32);
                m.w16(esp.wrapping_add(0x14u32), ((edi & 0xffff)) as u16);
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                ebp = 0x34u32;
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x40u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = eax.wrapping_add(edx);
                fl = lift::sub(ebp, m.r32(ecx.wrapping_add(0xfffffff8u32)), 32).1;
                if !fl.cf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx)))) & 0xff);
                m.w8(esp.wrapping_add(edx).wrapping_add(0x16u32), ((eax & 0xff)) as u8);
                pc = 11; continue;
            }
            9 => {
                fl = lift::sub(edx, 0xeu32, 32).1;
                if !fl.cf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = 0xeu32;
                edi = esp.wrapping_add(edx).wrapping_add(0x16u32);
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = ebx;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                ebp = ecx;
                ebx = m.r32(0x66a2bcu32);
                let (r, f) = lift::shr(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                ecx = ebp;
                let r = ecx & 0x3u32; fl = lift::logic(r, 32);
                ecx = r;
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 1);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edx, 0xeu32, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                ebp = 0x34u32;
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                if fl.zf { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                m.w32(esi.wrapping_add(0x470u32), 0x1u32);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esi.wrapping_add(0x470u32), edi);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            17 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x48u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(u32::from(m.r16(esp.wrapping_add(0x14u32))), 0xeu32, 16); fl = f;
                m.w16(esp.wrapping_add(0x14u32), (r) as u16);
                eax = m.r32(esi.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x470u32), edi);
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                m.w32(esp.wrapping_add(0x10u32), 0xc3u32);
                fl = lift::logic((eax & 0xff) & 0xcu32, 8);
                m.w32(esp.wrapping_add(0x14u32), 0x85u32);
                if fl.zf { pc = 23; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebp & 0xffff), 16).1;
                if fl.cf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x56u32))), (ebp & 0xffff), 16).1;
                if !fl.cf { pc = 20; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esi.wrapping_add(0x46cu32));
                m.w32(esi.wrapping_add(0x470u32), 0x1u32);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esi.wrapping_add(0x470u32), edi);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 21; continue;
            }
            23 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esi.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x543f10, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esi.wrapping_add(0x470u32), edi);
                edi = 0xfffffffdu32;
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x32u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0x2cu32)), edi, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 25; continue;
            }
            25 => {
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
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(0x34u32));
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

/// `0X00603BC0` _SeaBattle
pub fn f_00603bc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x668228u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                m.w32(esp.wrapping_add(0x8u32), esi);
                m.w32(esi, 0x67ad44u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x662u32)))) & 0xffff);
                fl = lift::sub(m.r32(eax.wrapping_mul(4).wrapping_add(0x6e55d0u32)), ebx, 32).1;
                if fl.zf { pc = 79; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(eax.wrapping_mul(4).wrapping_add(0x6e55d0u32), ebx);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6e59d4u32))) & 0xff);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6e59d4u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(m.r32(eax.wrapping_mul(4).wrapping_add(0x6e55ccu32)), ebx, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6e59d4u32))) & 0xff);
                let (r, f) = lift::sub((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(0x6e59d4u32, ((eax & 0xff)) as u8);
                if !fl.zf { pc = 2; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x12eau32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w8(0x6e59cdu32, ((ebx & 0xff)) as u8);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6df31cu32);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x558u32)))) & 0xff);
                edx = (edx & !0xff) | ((0x1u32) & 0xff);
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(eax).wrapping_add(0x5du32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = (edx & 0xff) ^ (edx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.cf { pc = 8; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = 0x6e55d0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x612d20, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(0x6cbdd8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x662u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x457df0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(0x6e59ccu32))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0x662u32))), 16).1;
                if !fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(0x6e59ccu32, (0xffu32) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if fl.zf { pc = 58; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x15cu32))), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 58; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x130cu32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = 0x6e55d0u32;
                { let a = []; eax = h.call_at(esp, m, 0x611cd0, ecx, &a); }
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6dd7a4u32);
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x130cu32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r8(edx.wrapping_add(ecx)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x35cu32))), 16).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 24; continue;
            }
            23 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(edi.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x605420, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(edi.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1e94u32)))) & 0xffff);
                m.w16(edx.wrapping_add(0x1e94u32), ((eax & 0xffff)) as u16);
                pc = 58; continue;
            }
            25 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0xffffu32, 16).1;
                if fl.zf { pc = 46; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), 0xffffu32, 16).1;
                if fl.zf { pc = 46; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6e55d0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x612bd0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xad2u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if !fl.zf { pc = 36; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esi.wrapping_add(0xadcu32));
                eax = 0xa205b063u32;
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 31; continue;
            }
            30 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 31; continue;
            }
            31 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ebp = 0x3e8u32;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                ecx = eax;
                { let d = u64::from(ebp); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1e94u32)))) & 0xffff);
                ebx = edi;
                let r = ebx & 0x3ffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edx, ebx, 32).1;
                if !fl.cf { pc = 46; continue; }
                pc = 32; continue;
            }
            32 => {
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
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35cu32)))) & 0xffff);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                let (r, f) = lift::shr((edi & 0xffff), 0xau32, 16, fl); fl = f;
                edi = (edi & !0xffff) | ((r) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                pc = 45; continue;
            }
            36 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 46; continue; }
                pc = 37; continue;
            }
            37 => {
                ecx = m.r32(esi.wrapping_add(0xadcu32));
                eax = 0xa205b063u32;
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 40; continue;
            }
            39 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 40; continue;
            }
            40 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ebp = 0x3e8u32;
                m.w32(esi.wrapping_add(0xadcu32), eax);
                ecx = eax;
                { let d = u64::from(ebp); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x1e94u32)))) & 0xffff);
                ebx = edi;
                let r = ebx & 0x3ffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edx, ebx, 32).1;
                if !fl.cf { pc = 46; continue; }
                pc = 41; continue;
            }
            41 => {
                let (r, f) = lift::imul(ecx, 0x44bf19b3u32); fl = f;
                ecx = r;
                eax = 0xa205b063u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                if fl.zf { pc = 43; continue; }
                pc = 42; continue;
            }
            42 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let r = eax | 0x80000000u32; fl = lift::logic(r, 32);
                eax = r;
                pc = 44; continue;
            }
            43 => {
                let (r, f) = lift::shr(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 44; continue;
            }
            44 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x35au32)))) & 0xffff);
                m.w32(esi.wrapping_add(0xadcu32), eax);
                let (r, f) = lift::shr((edi & 0xffff), 0xau32, 16, fl); fl = f;
                edi = (edi & !0xffff) | ((r) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                pc = 45; continue;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd7a0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x50cb80, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 46; continue;
            }
            46 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                edi = 0xffffu32;
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if fl.zf { pc = 49; continue; }
                pc = 47; continue;
            }
            47 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & edi; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x12u32))), 0x9u32, 16).1;
                if !fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x130eu32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((ecx & 0xff), (ebx & 0xff), 8).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x50c9e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w16(esi.wrapping_add(0x660u32), ((edi & 0xffff)) as u16);
                pc = 50; continue;
            }
            49 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 50; continue;
            }
            50 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x130eu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65eu32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if !fl.zf { pc = 53; continue; }
                pc = 51; continue;
            }
            51 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if fl.zf { pc = 58; continue; }
                pc = 52; continue;
            }
            52 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                let r = eax & edi; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 56; continue;
            }
            53 => {
                let r = eax & edi; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x660u32))), (edi & 0xffff), 16).1;
                ecx = 0x6dd7a0u32;
                if !fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 57; continue;
            }
            55 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x35cu32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x50bfd0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x35au32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x660u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 56; continue;
            }
            56 => {
                ecx = 0x6dd7a0u32;
                pc = 57; continue;
            }
            57 => {
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x50bfd0, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 58; continue;
            }
            58 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x658u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 63; continue; }
                pc = 59; continue;
            }
            59 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub(edi, 0xffffu32, 32).1;
                if fl.zf { pc = 63; continue; }
                pc = 60; continue;
            }
            60 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                ebp = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::sub(ebp, ebx, 32).1;
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x61a6d0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 62; continue;
            }
            62 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edi.wrapping_mul(2))))) & 0xffff);
                edi = ecx;
                fl = lift::sub(edi, 0xffffu32, 32).1;
                if !fl.zf { pc = 60; continue; }
                pc = 63; continue;
            }
            63 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0x65au32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 68; continue; }
                pc = 64; continue;
            }
            64 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x65cu32)))) & 0xffff);
                fl = lift::sub(edi, 0xffffu32, 32).1;
                if fl.zf { pc = 68; continue; }
                pc = 65; continue;
            }
            65 => {
                edx = m.r32(esi.wrapping_add(0x4u32));
                ebp = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::sub(ebp, ebx, 32).1;
                if fl.zf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x61a6d0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 67; continue;
            }
            67 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edi.wrapping_mul(2))))) & 0xffff);
                edi = ecx;
                fl = lift::sub(edi, 0xffffu32, 32).1;
                if !fl.zf { pc = 65; continue; }
                pc = 68; continue;
            }
            68 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xaccu32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 72; continue; }
                pc = 69; continue;
            }
            69 => {
                edx = m.r32(esi.wrapping_add(0x668u32));
                edi = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x602da0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 71; continue;
            }
            71 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaccu32)))) & 0xffff);
                fl = lift::sub(ebp, eax, 32).1;
                if fl.cf { pc = 69; continue; }
                pc = 72; continue;
            }
            72 => {
                eax = m.r32(esi.wrapping_add(0x668u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 74; continue; }
                pc = 73; continue;
            }
            73 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x664u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 74; continue;
            }
            74 => {
                eax = m.r32(esi.wrapping_add(0x66cu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 76; continue; }
                pc = 75; continue;
            }
            75 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x66cu32), ebx);
                pc = 76; continue;
            }
            76 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 78; continue; }
                pc = 77; continue;
            }
            77 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa78u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 78; continue;
            }
            78 => {
                edi = esi.wrapping_add(0x1b10u32);
                ecx = 0xe0u32;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w16(esi.wrapping_add(0x10u32), (0xffffu32) as u16);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                lift::rep_stos(m, eax, &mut edi, &mut ecx, 4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 79; continue;
            }
            79 => {
                ecx = esi;
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x618f80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esi = m.r32(esp);
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
        0x5128d0 => f_005128d0(m, h, ecx, esp),
        0x51ac60 => f_0051ac60(m, h, ecx, esp),
        0x534a40 => f_00534a40(m, h, ecx, esp),
        0x548160 => f_00548160(m, h, ecx, esp),
        0x54ac80 => f_0054ac80(m, h, ecx, esp),
        0x603bc0 => f_00603bc0(m, h, ecx, esp),
        _ => return None,
    }))
}
