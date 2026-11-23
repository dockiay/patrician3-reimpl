//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0041C420` dialog_bg_resize_tile_map
pub fn f_0041c420(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = ecx;
                ecx = m.r32(0x6cbb00u32);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esp.wrapping_add(0xcu32), esi);
                ebp = eax;
                eax = m.r32(ebx.wrapping_add(0x98u32));
                fl = lift::sub(esi, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(ebp, m.r32(ebx.wrapping_add(0x9cu32)), 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let (r, f) = lift::imul(eax, esi); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edi = eax;
                eax = m.r32(ebx.wrapping_add(0x9cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                pc = 5; continue;
            }
            4 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x20u32), edi);
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(ebx.wrapping_add(0x98u32));
                eax = m.r32(ebx.wrapping_add(0xa0u32));
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(ebx.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let (r, f) = lift::imul(ecx, m.r32(ebx.wrapping_add(0x98u32))); fl = f;
                ecx = r;
                ebp = eax.wrapping_add(0xffffffffu32);
                eax = m.r32(ebx.wrapping_add(0xa0u32));
                edx = ebp;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::imul(edx, esi); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esi = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::imul(esi, m.r32(esp.wrapping_add(0x1cu32))); fl = f;
                esi = r;
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                pc = 10; continue;
            }
            10 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add((edx & 0xff), 0xeu32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(edi), ((edx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                eax = edx.wrapping_add(0xffffffffu32);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ebp = eax.wrapping_add(0xffffffffu32);
                pc = 12; continue;
            }
            12 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x9u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::add((edx & 0xff), 0x1eu32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(edi), ((edx & 0xff)) as u8);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 12; continue; }
                pc = 13; continue;
            }
            13 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add((edx & 0xff), 0x11u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(esi.wrapping_add(edi), ((edx & 0xff)) as u8);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 10; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(ebx.wrapping_add(0xa0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(ebx.wrapping_add(0x98u32), edx);
                m.w32(ebx.wrapping_add(0x9cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebx.wrapping_add(0xa0u32), edi);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
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

/// `0X0041C5A0` dialog_bg_build_tile_map
pub fn f_0041c5a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x94u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x20u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x41c420, ecx, &a); }
                esp = esp.wrapping_add(8);
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
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xa0u32), 0x0u32);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(0x6cbb00u32);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x20u32)))) & 0xff);
                m.w32(esi.wrapping_add(0x98u32), ebp);
                m.w8(esi.wrapping_add(0x94u32), ((ecx & 0xff)) as u8);
                edi = eax;
                ebx = edi;
                m.w32(esi.wrapping_add(0x9cu32), edi);
                let (r, f) = lift::imul(ebx, ebp); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0xa0u32), eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                esp = esp.wrapping_add(0x10);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x20u32), 0x4u32);
                m.w32(esp.wrapping_add(0x18u32), 0x19u32);
                m.w8(edx, (0x4u32) as u8);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(eax.wrapping_add(ebp).wrapping_add(0xffffffffu32), (0x8u32) as u8);
                eax = edi.wrapping_add(0xffffffffu32);
                edi = eax;
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::imul(edi, ebp); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w8(edi.wrapping_add(ecx), (0x19u32) as u8);
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(ebx.wrapping_add(edx).wrapping_add(0xffffffffu32), (0x1du32) as u8);
                pc = 7; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x1cu32), edi);
                let (r, f) = lift::imul(edi, ebp); fl = f;
                edi = r;
                m.w8(eax, (0x9u32) as u8);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x20u32), 0x9u32);
                m.w32(esp.wrapping_add(0x18u32), 0x14u32);
                m.w8(ecx.wrapping_add(ebp).wrapping_add(0xffffffffu32), (0xdu32) as u8);
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(edi.wrapping_add(edx), (0x14u32) as u8);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(ebx.wrapping_add(eax).wrapping_add(0xffffffffu32), (0x18u32) as u8);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                ebx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                ebx = 0x1u32;
                ecx = ebp.wrapping_add(0xffffffffu32);
                fl = lift::sub(ecx, ebx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x20u32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add((edx & 0xff), (ecx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(ebx.wrapping_add(eax), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add((edx & 0xff), (eax & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                eax = edi.wrapping_add(ebx);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w8(eax.wrapping_add(ecx), ((edx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                edi = ebp;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 11; continue;
            }
            11 => {
                ebp = eax.wrapping_add(0xffffffffu32);
                pc = 12; continue;
            }
            12 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add((edx & 0xff), 0xeu32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(edi.wrapping_add(eax), ((edx & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                ebx = eax.wrapping_add(0xffffffffu32);
                pc = 14; continue;
            }
            14 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x9u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add((edx & 0xff), 0x1eu32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(edi.wrapping_add(eax), ((edx & 0xff)) as u8);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add((edx & 0xff), 0x11u32, 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(edi.wrapping_add(eax), ((edx & 0xff)) as u8);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 12; continue; }
                pc = 16; continue;
            }
            16 => {
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

/// `0X00440E50` window_base44_begin_open
pub fn f_00440e50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic(eax & eax, 32);
                esi = ecx;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb2a0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                m.w32(0x6cbd94u32, eax);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6cbd98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = eax.wrapping_add(eax);
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6cbbc8u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4af740, ecx, &a); }
                esp = esp.wrapping_add(20);
                m.w32(esi.wrapping_add(0x94u32), 0x4u32);
                ecx = m.r32(0x6dccf0u32);
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                pc = 4; continue;
            }
            4 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00440ED0` window_base44_begin_close
pub fn f_00440ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, 0x3u32, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbd98u32);
                eax = 0x51eb851fu32;
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = 0x6cbbc8u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4af740, ecx, &a); }
                esp = esp.wrapping_add(20);
                m.w32(esi.wrapping_add(0x94u32), 0x2u32);
                ecx = m.r32(0x6dccf0u32);
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                m.w32(esi.wrapping_add(0x94u32), 0x2u32);
                ecx = m.r32(0x6dccf0u32);
                edx = m.r32(0x6cbd98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                if !fl.cf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 5; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0x9cu32), ecx);
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

/// `0X00440F70` side_menu_request_switch
pub fn f_00440f70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                ecx = m.r32(0x6cbd90u32);
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                fl = lift::sub(esi, ecx, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ecx.wrapping_add(0x94u32));
                fl = lift::sub(eax, 0x3u32, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xa0u32)), 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                { let v = f64::from(m.r32(esi.wrapping_add(0x14u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esi.wrapping_add(0x18u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esi.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(0x6cbbbcu32) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if fl.pf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                { let v = f64::from(m.r32(0x6cbbc0u32) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if fl.pf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                { let v = f64::from(m.r32(0x6cbbc4u32) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if !fl.pf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbbbcu32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbd90u32);
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(esi, ecx, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(m.r32(ecx.wrapping_add(0xa0u32)), edi, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(ecx.wrapping_add(0x94u32));
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w32(ecx.wrapping_add(0x94u32), 0x4u32);
                eax = m.r32(0x6cbd90u32);
                edx = m.r32(0x6dccf0u32);
                esi = m.r32(0x6cbd98u32);
                ecx = m.r32(eax.wrapping_add(0x9cu32));
                edi = edx;
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, esi, 32).1;
                if fl.cf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax.wrapping_add(0x9cu32), edx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                let (r, f) = lift::add(edx, edx, 32); fl = f;
                edx = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, esi, 32); fl = f;
                edx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(eax.wrapping_add(0x9cu32), edx);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            15 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w32(0x6cbd8cu32, esi);
                eax = m.r32(ecx.wrapping_add(0x94u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 24; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w32(ecx.wrapping_add(0x94u32), 0x2u32);
                eax = m.r32(0x6dccf0u32);
                ecx = m.r32(0x6cbd98u32);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(0x6cbd90u32);
                m.w32(ecx.wrapping_add(0x9cu32), eax);
                pc = 22; continue;
            }
            20 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(0x6cbd8cu32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w32(eax.wrapping_add(0xa0u32), edi);
                pc = 24; continue;
            }
            24 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0045C300` number_input_field_handle_key_input
pub fn f_0045c300(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65ea6eu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 43; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esi.wrapping_add(0x18cu32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4c0f50, ecx, &a); }
                edx = m.r32(esi);
                ebx = (ebx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                ecx = esp.wrapping_add(0x34u32);
                eax = edi.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x168u32));
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x100u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0500, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 4; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbe5u32))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esp.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbe5u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 42; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 42; continue;
            }
            7 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbe7u32))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esp.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbe7u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                ecx = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 42; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 42; continue;
            }
            10 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbeeu32))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esp.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbeeu32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 15; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                ebp = m.r32(esi.wrapping_add(0x168u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                ecx = m.r32(esi.wrapping_add(0x168u32));
                edx = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x40u32);
                ebx = (ebx & !0xff) | ((0x2u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x34u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x28u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x28u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x28u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698100u32);
                ecx = esp.wrapping_add(0x38u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x184u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 42; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(edx.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 42; continue;
            }
            15 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbc8u32))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esp.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbc8u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                ecx = m.r32(esi.wrapping_add(0x168u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                ecx = m.r32(esi.wrapping_add(0x168u32));
                edx = esp.wrapping_add(0x30u32);
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w8(esp.wrapping_add(0x28u32), (0x4u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x3cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x40u32);
                ebx = (ebx & !0xff) | ((0x5u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x34u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x6u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x28u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x28u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x28u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698108u32);
                ecx = esp.wrapping_add(0x38u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x184u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 42; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69810cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(edx.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 42; continue;
            }
            22 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbdbu32))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                m.w8(esp.wrapping_add(0x38u32), ((ecx & 0xff)) as u8);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbdbu32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = esi.wrapping_add(0x120u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0500, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            24 => {
                fl = lift::sub(ebx, 0x30u32, 32).1;
                if fl.cf { pc = 36; continue; }
                pc = 25; continue;
            }
            25 => {
                fl = lift::sub(ebx, 0x39u32, 32).1;
                if (!fl.cf && !fl.zf) { pc = 36; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                ecx = m.r32(esi.wrapping_add(0x168u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x3cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                edx = m.r32(esi.wrapping_add(0x168u32));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x30u32), (0x7u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x34u32), (0x8u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64cb60, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = esp.wrapping_add(0x34u32);
                ebx = (ebx & !0xff) | ((0x9u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x34u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x2cu32), (0xau32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x28u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x28u32), (0x8u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x28u32), (0x7u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x28u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esi.wrapping_add(0x168u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esi.wrapping_add(0x168u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x38u32));
                ebx = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63b37b, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x180u32));
                esp = esp.wrapping_add(0x8);
                lift::fst32(m, esp.wrapping_add(0x38u32), fp.st(0)); fp.pop();
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x38u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698110u32);
                pc = 30; continue;
            }
            28 => {
                eax = m.r32(esi.wrapping_add(0x184u32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698114u32);
                pc = 30; continue;
            }
            30 => {
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(edx.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 31; continue;
            }
            31 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x38u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66dcd8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = 0x1u32;
                pc = 34; continue;
            }
            33 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::sub(eax, 0x1u32, 32).1;
                edx = (edx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::logic(ecx & edx, 32);
                if fl.zf { pc = 42; continue; }
                pc = 35; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esp.wrapping_add(0x3cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64cc80, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 42; continue;
            }
            36 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbcdu32))) & 0xff);
                ebx = (ebx & !0xff) | (((ecx & 0xff)) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::shr((ebx & 0xff), 0x1u32, 8, fl); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(0x6dcbcdu32, ((ebx & 0xff)) as u8);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 42; continue;
            }
            38 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbe3u32))) & 0xff);
                ebx = (ebx & !0xff) | (((ecx & 0xff)) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::shr((ebx & 0xff), 0x1u32, 8, fl); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (ebx & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(0x6dcbe3u32, ((ebx & 0xff)) as u8);
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 42; continue;
            }
            40 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcbe4u32))) & 0xff);
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                ecx = (ecx & !0xff) | (((eax & 0xff)) & 0xff);
                let (r, f) = lift::shr((edx & 0xff), 0x1u32, 8, fl); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) | (eax & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w8(0x6dcbe4u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                m.w32(esi.wrapping_add(0x168u32), 0x0u32);
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1b10, ecx, &a); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esi.wrapping_add(0x188u32), eax);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00471060` choice_list_dialog_on_mouse_event
pub fn f_00471060(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x28u32), edx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                edi = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::imul(ecx, edx); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = esp.wrapping_add(0x20u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe2u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a4f4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 3; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esi.wrapping_add(0x494u32));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x24u32)), 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                m.w16(esi.wrapping_add(0xd8u32), (0xffffu32) as u16);
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
            6 => {
                eax = m.r32(edi.wrapping_add(0x4u32));
                ebx = m.r32(esp.wrapping_add(0x24u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = m.r32(esi.wrapping_add(0x494u32));
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                ebp = edx.wrapping_add(edi);
                fl = lift::sub(ebp, eax, 32).1;
                if fl.zf { pc = 13; continue; }
                pc = 7; continue;
            }
            7 => {
                ebp = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), ebp);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                ebp = m.r32(esp.wrapping_add(0x28u32));
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::imul(eax, ecx); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x24u32));
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = esp.wrapping_add(0x10u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx;
                let (r, f) = lift::imul(eax, edi); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x494u32)))) & 0xffff);
                let (r, f) = lift::add((eax & 0xffff), (edi & 0xffff), 16); fl = f;
                eax = (eax & !0xffff) | ((r) & 0xffff);
                m.w16(esi.wrapping_add(0xd8u32), ((eax & 0xffff)) as u16);
                pc = 13; continue;
            }
            12 => {
                m.w16(esi.wrapping_add(0xd8u32), (0xffffu32) as u16);
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x38u32)), 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::logic(ecx & ecx, 32);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                if fl.zf { pc = 45; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x4u32, 16).1;
                if !fl.zf { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xa6u32))), 16).1;
                if !fl.cf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ecx.wrapping_add(eax.wrapping_mul(4));
                ecx = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4caad0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                m.w32(esi.wrapping_add(0xad8u32), eax);
                pc = 42; continue;
            }
            18 => {
                m.w32(esi.wrapping_add(0xad8u32), 0xffffffffu32);
                pc = 42; continue;
            }
            19 => {
                fl = lift::sub((eax & 0xffff), 0x3u32, 16).1;
                if fl.zf { pc = 23; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(esi.wrapping_add(0xa6u32))), 16).1;
                if !fl.cf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx)))) & 0xff);
                m.w8(ecx, ((eax & 0xff)) as u8);
                pc = 42; continue;
            }
            22 => {
                m.w8(ecx, (0xffu32) as u8);
                pc = 42; continue;
            }
            23 => {
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                fl = lift::logic((ebx & 0xffff) & (ebx & 0xffff), 16);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                m.w32(esp.wrapping_add(0x34u32), 0x1u32);
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x34u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x34u32), r);
                pc = 27; continue;
            }
            27 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                fl = lift::logic((edx & 0xffff) & (edx & 0xffff), 16);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x34u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x34u32), r);
                pc = 29; continue;
            }
            29 => {
                edi = ebx;
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = edi;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(edx).wrapping_add(0xffffffffu32);
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 41; continue; }
                pc = 30; continue;
            }
            30 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((ebx & 0xffff) & (ebx & 0xffff), 16);
                if fl.zf { pc = 34; continue; }
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.sf == fl.of) { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(esp.wrapping_add(0x38u32));
                m.w8(edx, ((ecx & 0xff)) as u8);
                pc = 33; continue;
            }
            33 => {
                edx = 0x1u32;
                pc = 34; continue;
            }
            34 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if fl.zf { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(ebp, eax, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ebp, edi, 32).1;
                eax = (eax & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                fl = lift::logic(eax & ecx, 32);
                if fl.zf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xd8u32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(0xccu32));
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax, ((ecx & 0xff)) as u8);
                pc = 37; continue;
            }
            37 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 38; continue;
            }
            38 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xaau32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if fl.zf { pc = 42; continue; }
                pc = 39; continue;
            }
            39 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa8u32)))) & 0xffff);
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(ebp, eax, 32).1;
                ebx = (ebx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebp, ecx, 32).1;
                eax = (eax & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                fl = lift::logic(eax & ebx, 32);
                if fl.zf { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xd8u32)))) & 0xff);
                let (r, f) = lift::sub((ecx & 0xff), (edx & 0xff), 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                edx = m.r32(esi.wrapping_add(0xccu32));
                m.w8(edx, ((ecx & 0xff)) as u8);
                pc = 42; continue;
            }
            41 => {
                eax = m.r32(esp.wrapping_add(0x38u32));
                m.w8(eax, (0xffu32) as u8);
                pc = 42; continue;
            }
            42 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa4u32))), 0x4u32, 16).1;
                if fl.zf { pc = 47; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                m.w32(eax, 0x1u32);
                pc = 45; continue;
            }
            45 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa4u32))), 0x4u32, 16).1;
                if fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 47; continue;
            }
            47 => {
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

/// `0X004AF740` anim_button_play_set
pub fn f_004af740(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esi = ecx;
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc8u32)), ebx, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                ebx = esi.wrapping_add(0x94u32);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4c5840, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = ecx;
                pc = 4; continue;
            }
            4 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esi.wrapping_add(0xb8u32), eax);
                fl = lift::logic(ebp & ebp, 32);
                m.w32(esi.wrapping_add(0xc0u32), edi);
                m.w32(esi.wrapping_add(0xbcu32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c5850, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                eax = ebp;
                let (r, f) = lift::imul(ecx, edi); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xc4u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xc4u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            7 => {
                edi = esi.wrapping_add(0x94u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c5840, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                m.w32(esi.wrapping_add(0xc0u32), ebx);
                m.w32(esi.wrapping_add(0xc4u32), ebx);
                m.w32(esi.wrapping_add(0xbcu32), ebx);
                if (!fl.zf && fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = ecx;
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esi.wrapping_add(0xacu32), eax);
                m.w32(esi.wrapping_add(0xa8u32), edx);
                m.w32(esi.wrapping_add(0xc8u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c5850, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0xccu32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::imul(ecx, ebp); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                pc = 13; continue;
            }
            12 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 13; continue;
            }
            13 => {
                m.w32(esi.wrapping_add(0xd0u32), eax);
                eax = m.r32(0x6dccf8u32);
                m.w32(esi.wrapping_add(0xd4u32), eax);
                m.w32(esi.wrapping_add(0xb4u32), 0x0u32);
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

/// `0X004B0570` control_value_compare_greater
pub fn f_004b0570(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ebx = m.r32(edi.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::sub(ebx, eax, 32).1;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::sub((ebx & 0xff), (edx & 0xff), 8).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(edx, esi, 32).1;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                let r = eax | ebx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ecx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ebp; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004B05E0` control_value_compare_greater_equal
pub fn f_004b05e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ebx = m.r32(edi.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = (ecx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                fl = lift::sub(eax, ebx, 32).1;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(edx, esi, 32).1;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                let r = eax | ebx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ecx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ebp; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004B0650` control_value_compare_less
pub fn f_004b0650(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ebx = m.r32(edi.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = (ecx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                fl = lift::sub(eax, ebx, 32).1;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(edx, esi, 32).1;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let r = eax | ebx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ecx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ebp; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004B06C0` control_value_compare_less_equal
pub fn f_004b06c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x41u32, 8);
                if fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ebx = m.r32(edi.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = (ecx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                fl = lift::sub(ebx, eax, 32).1;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ebx & 0xff), (edx & 0xff), 8).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(edx, esi, 32).1;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                let r = eax | ebx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ecx; fl = lift::logic(r, 32);
                eax = r;
                let r = eax | ebp; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004B0730` control_value_equals
pub fn f_004b0730(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(edi.wrapping_add(0x10u32))), 8).1;
                ecx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                ebx = (ebx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx & ebx; fl = lift::logic(r, 32);
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ecx, esi, 32).1;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = edx & ebx; fl = lift::logic(r, 32);
                edx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = edx & eax; fl = lift::logic(r, 32);
                edx = r;
                let r = edx & ebp; fl = lift::logic(r, 32);
                edx = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((edx & 0xff)) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B07A0` control_value_not_equals
pub fn f_004b07a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x14u32));
                { let v = lift::fld64(m, esi.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld64(m, edi.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if !fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = 0x1u32;
                pc = 3; continue;
            }
            2 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(ecx, ebx, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((ecx & 0xff), u32::from(m.r8(edi.wrapping_add(0x10u32))), 8).1;
                ecx = m.r32(esi.wrapping_add(0xcu32));
                esi = m.r32(edi.wrapping_add(0xcu32));
                ebx = (ebx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx | ebx; fl = lift::logic(r, 32);
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ecx, esi, 32).1;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let r = edx | ebx; fl = lift::logic(r, 32);
                edx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = edx | eax; fl = lift::logic(r, 32);
                edx = r;
                let r = edx | ebp; fl = lift::logic(r, 32);
                edx = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((edx & 0xff)) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B8A40` screen_list_run_frame
pub fn f_004b8a40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6da846u32))) & 0xff);
                esp = esp.wrapping_sub(0x38);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = ebx | 0xffffffffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esi = ecx;
                if !fl.zf { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6dcb94u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6dcd04u32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                pc = 4; continue;
            }
            3 => {
                edx = m.r32(0x6dcd20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 4; continue;
            }
            4 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ebp = m.r32(0x66a52cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(20);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x30u32)), 0x12u32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x229u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a530u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(20);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 5; continue; }
                pc = 9; continue;
            }
            9 => {
                pc = 11; continue;
            }
            10 => {
                m.w8(0x6da846u32, (0x1u32) as u8);
                pc = 11; continue;
            }
            11 => {
                { let a = []; eax = h.call_at(esp, m, 0x4bd180, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6da846u32))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ebx = 0xfffffffeu32;
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(ebx, 0xfffffffeu32, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xcu32));
                edi = m.r32(eax.wrapping_add(0x8u32));
                ecx = edi;
                edx = m.r32(edi);
                { let t = m.r32(edx.wrapping_add(0xf4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x12cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbb40u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::sub(edi, ecx, 32).1;
                if fl.zf { pc = 20; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x10u32)), ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esi.wrapping_add(0xcu32));
                fl = lift::sub(m.r32(edx.wrapping_add(0x8u32)), ecx, 32).1;
                if !fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xf4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbb40u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x12cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 20; continue;
            }
            19 => {
                m.w32(esp.wrapping_add(0x10u32), 0xfffffffeu32);
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(m.r32(0x6dcb94u32), ebp, 32).1;
                if !fl.zf { pc = 45; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6dab50u32);
                edx = m.r32(esi.wrapping_add(0x208u32));
                eax = esi.wrapping_add(0x208u32);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(0x6dab48u32);
                ecx = m.r32(esi.wrapping_add(0x20cu32));
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 33; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x214u32));
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x1e8u32));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x214u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x1f0u32));
                ecx = esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x1f0u32);
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if !fl.sf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                m.w32(esp.wrapping_add(0x14u32), ebp);
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(ecx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if !fl.sf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                m.w32(esp.wrapping_add(0x18u32), ebp);
                pc = 27; continue;
            }
            27 => {
                ebx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1e8u32));
                eax = m.r32(0x6dcb94u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 32; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 32; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(0x670f6cu32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                eax = m.r32(ebx);
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x670f6cu32);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 30; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(0x6dcd04u32);
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(0x6dab50u32);
                m.w32(esi.wrapping_add(0x208u32), ecx);
                edx = m.r32(0x6dab48u32);
                m.w32(esi.wrapping_add(0x20cu32), edx);
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(0x6dab50u32);
                edx = m.r32(0x6dab48u32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = eax.wrapping_add(0xffffffffu32);
                eax = m.r32(esi.wrapping_add(0x214u32));
                ebp = edx.wrapping_add(0xffffffffu32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x24u32), ebp);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                let (r, f) = lift::sub(ecx, m.r32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x1f0u32)), 32); fl = f;
                ecx = r;
                edx = esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x1f0u32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                if !fl.sf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                m.w32(esp.wrapping_add(0x20u32), ebx);
                pc = 35; continue;
            }
            35 => {
                let (r, f) = lift::sub(ebp, m.r32(edx), 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x24u32), ebp);
                if !fl.sf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                m.w32(esp.wrapping_add(0x24u32), ebx);
                pc = 37; continue;
            }
            37 => {
                ebx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1e8u32));
                eax = m.r32(0x6dcb94u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 42; continue; }
                pc = 38; continue;
            }
            38 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 42; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = m.r32(0x670f6cu32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                edx = m.r32(ebx);
                esp = esp.wrapping_add(0x4);
                eax = esp.wrapping_add(0x20u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x670f6cu32);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 40; continue; }
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(0x6dcd04u32);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 42; continue;
            }
            42 => {
                ebp = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 46; continue; }
                pc = 43; continue;
            }
            43 => {
                eax = ebp;
                ebp = m.r32(ebp);
                edi = m.r32(eax.wrapping_add(0x8u32));
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4b57c0, ecx, &a); }
                fl = lift::logic(ebp & ebp, 32);
                if !fl.zf { pc = 43; continue; }
                pc = 44; continue;
            }
            44 => {
                pc = 46; continue;
            }
            45 => {
                eax = m.r32(0x6dcce4u32);
                ecx = m.r32(0x6dcce0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb800, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 46; continue;
            }
            46 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3f800000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x43c80000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x43c80000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bba50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3f800000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb260, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                { let a = []; eax = h.call_at(esp, m, 0x4bb200, ecx, &a); }
                pc = 48; continue;
            }
            48 => {
                ebp = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 52; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = ebp;
                ebp = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                edi = m.r32(eax.wrapping_add(0x8u32));
                ecx = edi;
                edx = m.r32(edi);
                { let t = m.r32(edx.wrapping_add(0x98u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 51; continue;
            }
            51 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xf8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(ebp & ebp, 32);
                if !fl.zf { pc = 49; continue; }
                pc = 52; continue;
            }
            52 => {
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 54; continue;
            }
            54 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                { let a = []; eax = h.call_at(esp, m, 0x4bb440, ecx, &a); }
                pc = 56; continue;
            }
            56 => {
                eax = m.r32(esi.wrapping_add(0x214u32));
                edx = m.r32(0x6dab50u32);
                edi = m.r32(0x6dab48u32);
                esp = esp.wrapping_sub(0xc);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x1e8u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x1f0u32));
                ebx = m.r32(eax);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(ecx, edx);
                m.w32(ecx.wrapping_add(0x4u32), edi);
                m.w32(ecx.wrapping_add(0x8u32), ebp);
                ecx = eax;
                { let t = m.r32(ebx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x214u32));
                ecx = m.r32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x1e8u32));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xdcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b8ff0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004B90E0` screen_list_push_window
pub fn f_004b90e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                fl = lift::logic(esi & esi, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2210, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x15cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi.wrapping_add(0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e6fc, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(0x66a52cu32);
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x209u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x200u32);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = edi; let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(20);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C6000` keyframe_timer_evaluate_now
pub fn f_004c6000(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6dccf8u32);
                m.w32(esp.wrapping_add(0x4u32), 0x0u32);
                m.w32(esp, eax);
                { let v = lift::fild64(m, esp); fp.push(v); }
                lift::fst32(m, esp, fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(0x14u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(ecx, 0x1u32);
                m.w32(edx, 0x0u32);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x20u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4c5e50, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4c5e50, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C6080` keyframe_timer_evaluate_at
pub fn f_004c6080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(0x14u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                ecx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(eax, 0x1u32);
                m.w32(ecx, 0x0u32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x20u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x4u32), edx);
                { let a = [m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; return h.call_at(esp, m, 0x4c5e50, ecx, &a); }
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4u32), eax);
                { let a = [m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; return h.call_at(esp, m, 0x4c5e50, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C60D0` keyframe_timer_start
pub fn f_004c60d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(0x8);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(ecx.wrapping_add(0x14u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::sub((ebx & 0xff), (edx & 0xff), 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x67149cu32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x44u32, 8);
                if fl.pf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6dccf8u32);
                m.w32(esp.wrapping_add(0x8u32), edx);
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                { let v = lift::fild64(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                m.w8(ecx.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(ecx.wrapping_add(0x21u32), ((eax & 0xff)) as u8);
                m.w32(ecx.wrapping_add(0xcu32), edx);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                lift::fst32(m, ecx.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            3 => {
                eax = m.r32(0x6dccf8u32);
                m.w32(esp.wrapping_add(0x8u32), edx);
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                { let v = lift::fild64(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                m.w8(ecx.wrapping_add(0x20u32), ((edx & 0xff)) as u8);
                m.w8(ecx.wrapping_add(0x21u32), ((eax & 0xff)) as u8);
                lift::fst32(m, ecx.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                pc = 4; continue;
            }
            4 => {
                m.w32(ecx.wrapping_add(0xcu32), edx);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C6210` keyframe_timer_add_time
pub fn f_004c6210(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x20u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x67149cu32);
                m.w32(esi.wrapping_add(0x14u32), eax);
                ecx = m.r32(0x6dccf8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w8(esi.wrapping_add(0x20u32), ((eax & 0xff)) as u8);
                { let v = lift::fild64(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                m.w8(esi.wrapping_add(0x21u32), ((eax & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xcu32), eax);
                lift::fst32(m, esi.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = edx.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(edx & edx, 32);
                edi = eax;
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                { let v = lift::fld32(m, ecx.wrapping_add(edx).wrapping_add(0xfffffffcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)), edx);
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x8u32), edx);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x4u32), edi);
                ecx = m.r32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xfffffffcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x1cu32), ecx);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004C6390` keyframe_timer_load_from_ini
pub fn f_004c6390(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66210au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                eax = m.r32(0x6c7cccu32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x13u32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(eax, ebx, 32).1;
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
                edi = esi.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5bau32);
                ecx = edi;
                m.w32(esi.wrapping_add(0x10u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                m.w32(esi.wrapping_add(0xcu32), ebx);
                m.w32(esi.wrapping_add(0x14u32), ebx);
                m.w32(esi.wrapping_add(0x18u32), ebx);
                m.w32(esi.wrapping_add(0x1cu32), ebx);
                m.w8(esi.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x21u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x30u32));
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6bf5c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x1fu32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(0x6bf5d0u32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebp = eax;
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(0x6bf5ecu32);
                edx = m.r32(0x6bf5e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5bau32);
                ecx = edi;
                m.w32(esi.wrapping_add(0x10u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                m.w32(esi.wrapping_add(0xcu32), ebx);
                m.w32(esi.wrapping_add(0x14u32), ebx);
                m.w32(esi.wrapping_add(0x18u32), ebx);
                m.w32(esi.wrapping_add(0x1cu32), ebx);
                m.w8(esi.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x21u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 22; continue;
            }
            11 => {
                ecx = ebp.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x4u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf62cu32);
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), ebx);
                edx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esi.wrapping_add(0x8u32), edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                edi = eax;
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esp.wrapping_add(0x18u32), 0x447a0000u32);
                if (!fl.cf && !fl.zf) { pc = 19; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = ecx.wrapping_add(eax.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf630u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                { let v = lift::fld32(m, ecx.wrapping_add(eax.wrapping_mul(4))); fp.push(v); }
                { let v = lift::fld32(m, ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xfffffffcu32)); let d = fp.st(0); fp.set(0, d - v); }
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                pc = 16; continue;
            }
            15 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                eax = m.r32(esi.wrapping_add(0x4u32));
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xfffffffcu32)); let d = fp.st(0); fp.set(0, d + v); }
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                lift::fst32(m, eax, fp.st(0)); fp.pop();
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(edi, ebx, 32).1;
                m.w32(esi.wrapping_add(0x1cu32), edx);
                m.w32(esi.wrapping_add(0x8u32), eax);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf634u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edi = eax;
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.cf || fl.zf) { pc = 12; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = esp.wrapping_add(0x2cu32);
                ecx = esi.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x30u32));
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(eax, 0x7fffffffu32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x10u32), eax);
                pc = 22; continue;
            }
            22 => {
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x24u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x18);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C9DD0` edit_box_draw
pub fn f_004c9dd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x48u32));
                ecx = esp;
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(ecx, edi);
                m.w32(ecx.wrapping_add(0x4u32), edx);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4d27c0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x168u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0xdcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x10);
                eax = esp.wrapping_add(0x14u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x20u32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xe0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esi.wrapping_add(0x140u32));
                edi = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x144u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(0x6dccf4u32);
                edx = m.r32(esp.wrapping_add(0x20u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15bu32)))) & 0xff);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(esp.wrapping_add(0x40u32), edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_add(0x4);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                { let d = u64::from(m.r32(esi.wrapping_add(0x160u32))); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                edx = eax;
                eax = ecx;
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::imul(edx, eax); fl = f;
                edx = r;
                eax = 0x10624dd3u32;
                { let r = u64::from(eax) * u64::from(edx); eax = r as u32; edx = (r >> 32) as u32; fl.cf = edx != 0; fl.of = edx != 0; }
                let (r, f) = lift::shr(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let r = (eax & 0xff) | 0xffu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sub((eax & 0xff), (edx & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15cu32)))) & 0xff);
                let (r, f) = lift::add((edx & 0xff), (eax & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                eax = (eax & !0xff) | (((edx & 0xff)) & 0xff);
                m.w8(esi.wrapping_add(0x15cu32), ((edx & 0xff)) as u8);
                fl = lift::sub((eax & 0xff), (ecx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(esi.wrapping_add(0x15cu32), ((ecx & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x158u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15cu32)))) & 0xff);
                let r = edx & 0xffffffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(ecx, 0x18u32, 32, fl); fl = f;
                ecx = r;
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x150u32));
                ecx = m.r32(esi.wrapping_add(0x14cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CCCF0` viper_list_box_remove_column
pub fn f_004cccf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x30u32));
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xa4u32)), 32).1;
                if (fl.sf == fl.of) { pc = 25; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                eax = ebx;
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                ecx = eax.wrapping_add(0xffffffffu32);
                m.w32(esp.wrapping_add(0x34u32), edx);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                ebx = edi.wrapping_add(0x1u32);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xacu32));
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ebx.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = eax.wrapping_add(edi.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa8u32));
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                edx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                eax = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0x24);
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(edx.wrapping_add(ecx.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esi.wrapping_add(0xacu32));
                m.w32(ecx.wrapping_add(eax.wrapping_mul(4)), 0x0u32);
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa8u32));
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                ecx = edi.wrapping_add(0x1u32);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = m.r32(edx.wrapping_add(ebx.wrapping_mul(4)));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(edx.wrapping_add(ebx.wrapping_mul(4)));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(ecx.wrapping_add(eax.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x38u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 24; continue; }
                pc = 13; continue;
            }
            13 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x34u32)) as i32); fp.push(v); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                ebp = ecx.wrapping_add(edi.wrapping_mul(4));
                ecx = m.r32(esp.wrapping_add(0x38u32));
                m.w32(esp.wrapping_add(0x3cu32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x3cu32)) as i32); fp.push(v); }
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x3cu32)), 32); fl = f;
                eax = r;
                m.w32(ebp, eax);
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edx = m.r32(esi.wrapping_add(0xb0u32));
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                m.w32(esp.wrapping_add(0x20u32), edx);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x20u32), ebp);
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(esi.wrapping_add(0xb0u32));
                edx = m.r32(esi.wrapping_add(0xacu32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(edi, ecx, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                ecx = esi.wrapping_add(0xb4u32);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(m.r32(esp.wrapping_add(0x1cu32)), eax, 32); fl = f;
                m.w32(esp.wrapping_add(0x1cu32), r);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x215u32)))) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                edx = m.r32(esi.wrapping_add(0x98u32));
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sbb(ecx, ecx, fl.cf, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(0, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::imul(ecx, m.r32(esi.wrapping_add(0xb0u32))); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                eax = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 23; continue;
            }
            23 => {
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CCFF0` viper_list_box_set_column_width
pub fn f_004ccff0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x34);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0xa4u32)), 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x58u32));
                edi = m.r32(esp.wrapping_add(0x2cu32));
                fl = lift::sub(ebx, edi, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                ebx = edi;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(eax);
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                ecx = esi.wrapping_add(0xb4u32);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp.wrapping_add(0x2cu32));
                pc = 10; continue;
            }
            9 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                if !fl.sf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = edi;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(esi.wrapping_add(0xa4u32)) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebx = eax;
                pc = 12; continue;
            }
            12 => {
                eax = edi;
                edi = m.r32(esi.wrapping_add(0xa4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(ecx, ebp, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                let (r, f) = lift::sub(eax, m.r32(edx.wrapping_add(ecx.wrapping_mul(4))), 32); fl = f;
                eax = r;
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.sf != fl.of) { pc = 13; continue; }
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x58u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 21; continue;
            }
            18 => {
                if fl.zf { pc = 22; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = ((eax as i32) >> 31) as u32;
                ecx = edi.wrapping_add(0xffffffffu32);
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esp.wrapping_add(0x58u32), eax);
                pc = 22; continue;
            }
            22 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(edx, ebp, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edi = m.r32(ecx.wrapping_add(edx.wrapping_mul(4)));
                ecx = ecx.wrapping_add(edx.wrapping_mul(4));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                m.w32(ecx, edi);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 25; continue;
            }
            25 => {
                edi = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edx, edi, 32).1;
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(edx.wrapping_add(ebp.wrapping_mul(4)), ebx);
                if (!fl.zf && fl.sf == fl.of) { pc = 47; continue; }
                pc = 27; continue;
            }
            27 => {
                { let v = f64::from(m.r32(esp.wrapping_add(0x58u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x58u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x58u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66bc6cu32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x41u32, 8);
                if fl.pf { pc = 47; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                m.w32(esp.wrapping_add(0x58u32), ebx);
                if (fl.zf || fl.sf != fl.of) { pc = 47; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xacu32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), edx);
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                eax = esp.wrapping_add(0x14u32);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x20u32), ebx);
                eax = esp.wrapping_add(0x20u32);
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 45; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(ecx);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edi, edx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if !fl.zf { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                eax = m.r32(esi);
                ecx = esi;
                ebx = esi.wrapping_add(0xb4u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebx = m.r32(esp.wrapping_add(0x58u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = esi.wrapping_add(0xb4u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edx = m.r32(esi);
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                pc = 39; continue;
            }
            38 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                pc = 39; continue;
            }
            39 => {
                fl = lift::logic(edi & edi, 32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                m.w32(esp.wrapping_add(0x38u32), ebx);
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                edx = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 44; continue;
            }
            44 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 32; continue; }
                pc = 45; continue;
            }
            45 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                eax = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                ebx = ebx.wrapping_add(edx).wrapping_add(0xffffffffu32);
                m.w32(esp.wrapping_add(0x58u32), ebx);
                if (fl.sf != fl.of) { pc = 29; continue; }
                pc = 46; continue;
            }
            46 => {
                ebp = m.r32(esp.wrapping_add(0x48u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 47; continue;
            }
            47 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 49; continue;
            }
            49 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 59; continue; }
                pc = 50; continue;
            }
            50 => {
                fl = lift::sub((eax & 0xff), 0x7fu32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 53; continue; }
                pc = 51; continue;
            }
            51 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(m.r32(edx.wrapping_add(ebp.wrapping_mul(4))), ecx, 32).1;
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0xa4u32)), 32).1;
                if (fl.sf != fl.of) { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 54; continue;
            }
            54 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                if fl.zf { pc = 64; continue; }
                pc = 55; continue;
            }
            55 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 58; continue; }
                pc = 56; continue;
            }
            56 => {
                let (r, f) = lift::sub(eax, 0x7eu32, 32); fl = f;
                eax = r;
                if !fl.zf { pc = 67; continue; }
                pc = 57; continue;
            }
            57 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x30);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                pc = 65; continue;
            }
            58 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x24);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                pc = 65; continue;
            }
            59 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x220u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x7fu32, 8).1;
                m.w8(esp.wrapping_add(0x48u32), ((eax & 0xff)) as u8);
                if (!fl.cf && !fl.zf) { pc = 62; continue; }
                pc = 60; continue;
            }
            60 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(m.r32(edx.wrapping_add(ebp.wrapping_mul(4))), ecx, 32).1;
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                fl = lift::sub(ebp, m.r32(esi.wrapping_add(0xa4u32)), 32).1;
                if (fl.sf != fl.of) { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 63; continue;
            }
            63 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                pc = 54; continue;
            }
            64 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 65; continue;
            }
            65 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 67; continue; }
                pc = 66; continue;
            }
            66 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d40, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 67; continue;
            }
            67 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebp;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CE6D0` viper_list_box_set_size_from_triple
pub fn f_004ce6d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                edx = m.r32(ebp.wrapping_add(0x4u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0x250u32));
                ecx = esi.wrapping_add(0x250u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 16; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = ebp;
                edi = esi.wrapping_add(0xb4u32);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = edi;
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x10u32), edx);
                edx = m.r32(edi);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = m.r32(esi.wrapping_add(0x238u32));
                fl = lift::logic(eax & eax, 32);
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ebp.wrapping_add(0x4u32));
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(edi);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp.wrapping_add(0x4u32));
                ecx = edi;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(m.r32(esi.wrapping_add(0xb0u32)) as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ba020, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                ebx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(ebx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(ebp);
                ecx = edi;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(ebx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9e60, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                pc = 7; continue;
            }
            6 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                ebp = eax;
                let (r, f) = lift::sub(ebp, edi, 32); fl = f;
                ebp = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = ebp;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                ebp = eax;
                pc = 9; continue;
            }
            9 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 14; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 14; continue;
            }
            14 => {
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4ccff0, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x1cu32)))) & 0xff);
                pc = 16; continue;
            }
            16 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CF890` viper_slider_on_mouse_event
pub fn f_004cf890(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(0x34);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1720, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi.wrapping_add(0x244u32));
                edi = esi.wrapping_add(0x244u32);
                eax = esp.wrapping_add(0x2cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x374u32));
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x350u32));
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0x358u32));
                ecx = ebx.wrapping_add(ebx);
                let (r, f) = lift::sub(ebp, ecx, 32); fl = f;
                ebp = r;
                ecx = m.r32(esi.wrapping_add(0x354u32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::imul(ebp, eax); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ebx, 32); fl = f;
                ebp = r;
                pc = 3; continue;
            }
            2 => {
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = lift::fld32(m, esi.wrapping_add(0x378u32)); fp.push(v); }
                ebp = eax;
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                m.w32(esp.wrapping_add(0x24u32), eax);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(esp.wrapping_add(0x48u32));
                edx = ((eax as i32) >> 31) as u32;
                ecx = m.r32(ebx);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(ebx.wrapping_add(0x4u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x20u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ebx = m.r32(esp.wrapping_add(0x4cu32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xa4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            6 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                ebx = m.r32(esp.wrapping_add(0x4cu32));
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x4cu32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x4cu32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x358u32));
                pc = 12; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x4cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4cu32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(ebx, 0xfffffffeu32, 32).1;
                m.w32(esp.wrapping_add(0x4cu32), eax);
                if !fl.zf { pc = 21; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x20u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(edx);
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 18; continue;
            }
            16 => {
                fl = lift::sub(eax, m.r32(esp.wrapping_add(0x24u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                pc = 18; continue;
            }
            18 => {
                m.w32(esp.wrapping_add(0x4cu32), eax);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d0bd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x395u32), (0x0u32) as u8);
                pc = 20; continue;
            }
            20 => {
                m.w8(esi.wrapping_add(0x395u32), (0x0u32) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            21 => {
                fl = lift::sub(ebx, 0x1u32, 32).1;
                if !fl.zf { pc = 20; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x20u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x395u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 20; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                m.w8(esi.wrapping_add(0x395u32), (0x1u32) as u8);
                ecx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                { let v = f64::from(m.r32(eax) as i32); fp.push(v); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(eax.wrapping_add(0x4u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d0cb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CFB30` viper_slider_on_key
pub fn f_004cfb30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b1850, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x244u32));
                ecx = esi.wrapping_add(0x244u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x14u32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esi.wrapping_add(0x358u32));
                pc = 3; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edi = eax;
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbe5u32))) & 0xff);
                eax = (eax & !0xff) | (((ecx & 0xff)) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::shr((eax & 0xff), 0x1u32, 8, fl); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(0x6dcbe5u32, ((eax & 0xff)) as u8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d0bd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            5 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x6dcbe7u32))) & 0xff);
                eax = (eax & !0xff) | (((ecx & 0xff)) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::shr((eax & 0xff), 0x1u32, 8, fl); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) & 0x7eu32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfeu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (eax & 0xff) | (ecx & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                m.w8(0x6dcbe7u32, ((eax & 0xff)) as u8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d0bd0, ecx, &a); }
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

/// `0X004D02A0` viper_slider_draw
pub fn f_004d02a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662837u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x70);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 57; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 57; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = m.r32(esp.wrapping_add(0x94u32));
                edi = m.r32(esp.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0xa8u32));
                eax = esp;
                ebx = m.r32(esp.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), edi);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x8);
                ecx = esi;
                m.w8(esp.wrapping_add(0x30u32), ((eax & 0xff)) as u8);
                { let t = m.r32(edx.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x3a8u32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x398u32));
                edx = eax;
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                let r = edx & 0xffffffu32; fl = lift::logic(r, 32);
                edx = r;
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x3acu32));
                m.w32(esi.wrapping_add(0x3a8u32), edx);
                edx = ecx;
                let r = edx ^ eax; fl = lift::logic(r, 32);
                edx = r;
                pc = 5; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x39cu32));
                edx = eax;
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                let r = edx & 0xffffffu32; fl = lift::logic(r, 32);
                edx = r;
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x3acu32));
                m.w32(esi.wrapping_add(0x3a8u32), edx);
                edx = eax;
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                pc = 5; continue;
            }
            5 => {
                let r = edx & 0xffffffu32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = edx ^ ecx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi.wrapping_add(0x3acu32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x38u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x16cu32));
                ecx = esi.wrapping_add(0x16cu32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x244u32));
                ebx = esi.wrapping_add(0x244u32);
                eax = esp.wrapping_add(0x5cu32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x358u32));
                m.w32(esp.wrapping_add(0x58u32), 0x0u32);
                edx = ecx.wrapping_add(ecx);
                ecx = m.r32(esi.wrapping_add(0x354u32));
                let (r, f) = lift::sub(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::imul(ebx, eax); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, m.r32(esp.wrapping_add(0x18u32)), 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x50u32), ebx);
                pc = 8; continue;
            }
            7 => {
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = lift::fld32(m, esi.wrapping_add(0x37cu32)); fp.push(v); }
                m.w32(esp.wrapping_add(0x50u32), eax);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ebx = m.r32(esp.wrapping_add(0x50u32));
                m.w32(esp.wrapping_add(0x58u32), eax);
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::add(ebx, ebp, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb800, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edx = m.r32(esp.wrapping_add(0xa8u32));
                ecx = esp;
                eax = m.r32(esi.wrapping_add(0x94u32));
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), edi);
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(esp.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb800, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(esp.wrapping_add(0xacu32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = esp;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x16cu32));
                m.w32(esp.wrapping_add(0x50u32), eax);
                m.w32(ebx, eax);
                eax = edi;
                m.w32(esp.wrapping_add(0x58u32), ecx);
                m.w32(ebx.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x58u32));
                ecx = esi.wrapping_add(0x16cu32);
                m.w32(esp.wrapping_add(0x54u32), edi);
                m.w32(ebx.wrapping_add(0x8u32), eax);
                eax = m.r32(esp.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x88u32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x38du32)))) & 0xff);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 21; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 26; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = eax;
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x358u32));
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ebx).wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff34u32);
                pc = 25; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ebx).wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff34u32);
                pc = 25; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x354u32));
                ebx = m.r32(esi.wrapping_add(0x350u32));
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w32(esp.wrapping_add(0x18u32), ebx);
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub(eax, ebx, 32).1;
                pc = 16; continue;
            }
            16 => {
                if fl.zf { pc = 26; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x358u32));
                pc = 20; continue;
            }
            19 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d - v); }
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                pc = 20; continue;
            }
            20 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(0x8);
                m.w32(esp.wrapping_add(0x30u32), ebx);
                eax = esp.wrapping_add(0x18u32);
                lift::fst32(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x30u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = lift::fld32(m, 0x66c1e8u32); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst64(m, esp, fp.st(0)); fp.pop();
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 26; continue;
            }
            21 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0x358u32));
                pc = 24; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x354u32));
                ecx = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), eax);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); fp.push(v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff24u32);
                pc = 25; continue;
            }
            25 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esi.wrapping_add(0x390u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                let (r, f) = lift::add(eax, 0x2cu32, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x68u32), eax);
                eax = m.r32(edx.wrapping_add(0x4u32));
                ecx = m.r32(edx.wrapping_add(0x8u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x70u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x3a0u32));
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 36; continue; }
                pc = 27; continue;
            }
            27 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3abu32)))) & 0xff);
                fl = lift::sub(eax, 0xffu32, 32).1;
                if fl.zf { pc = 31; continue; }
                pc = 28; continue;
            }
            28 => {
                m.w8(esi.wrapping_add(0x3abu32), (0x0u32) as u8);
                edx = m.r32(0x6dccf4u32);
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = 0xffu32;
                pc = 30; continue;
            }
            30 => {
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(edx, 0x18u32, 32, fl); fl = f;
                edx = r;
                let r = m.r32(esi.wrapping_add(0x3a8u32)) | edx; fl = lift::logic(r, 32);
                m.w32(esi.wrapping_add(0x3a8u32), r);
                fl = lift::sub(eax, 0xffu32, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                m.w32(esi.wrapping_add(0x3a0u32), 0x0u32);
                pc = 32; continue;
            }
            32 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3afu32)))) & 0xff);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 47; continue; }
                pc = 33; continue;
            }
            33 => {
                m.w8(esi.wrapping_add(0x3afu32), (0x0u32) as u8);
                edx = m.r32(0x6dccf4u32);
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf == fl.of) { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 35; continue;
            }
            35 => {
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(edx, 0x18u32, 32, fl); fl = f;
                edx = r;
                let r = m.r32(esi.wrapping_add(0x3acu32)) | edx; fl = lift::logic(r, 32);
                m.w32(esi.wrapping_add(0x3acu32), r);
                fl = lift::logic(eax & eax, 32);
                pc = 46; continue;
            }
            36 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 48; continue; }
                pc = 37; continue;
            }
            37 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3abu32)))) & 0xff);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 41; continue; }
                pc = 38; continue;
            }
            38 => {
                m.w8(esi.wrapping_add(0x3abu32), (0x0u32) as u8);
                edx = m.r32(0x6dccf4u32);
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf == fl.of) { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 40; continue;
            }
            40 => {
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(edx, 0x18u32, 32, fl); fl = f;
                edx = r;
                let r = m.r32(esi.wrapping_add(0x3a8u32)) | edx; fl = lift::logic(r, 32);
                m.w32(esi.wrapping_add(0x3a8u32), r);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                m.w32(esi.wrapping_add(0x3a0u32), 0x0u32);
                pc = 42; continue;
            }
            42 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3afu32)))) & 0xff);
                fl = lift::sub(eax, 0xffu32, 32).1;
                if fl.zf { pc = 47; continue; }
                pc = 43; continue;
            }
            43 => {
                m.w8(esi.wrapping_add(0x3afu32), (0x0u32) as u8);
                edx = m.r32(0x6dccf4u32);
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                fl = lift::sub(eax, 0xffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                eax = 0xffu32;
                pc = 45; continue;
            }
            45 => {
                edx = eax;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::shl(edx, 0x18u32, 32, fl); fl = f;
                edx = r;
                let r = m.r32(esi.wrapping_add(0x3acu32)) | edx; fl = lift::logic(r, 32);
                m.w32(esi.wrapping_add(0x3acu32), r);
                fl = lift::sub(eax, 0xffu32, 32).1;
                pc = 46; continue;
            }
            46 => {
                if !fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                m.w32(esi.wrapping_add(0x3a0u32), 0x0u32);
                pc = 48; continue;
            }
            48 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x38cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (fl.cf || fl.zf) { pc = 54; continue; }
                pc = 49; continue;
            }
            49 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 54; continue; }
                pc = 50; continue;
            }
            50 => {
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb800, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3a8u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x384u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0x380u32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x390u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 52; continue;
            }
            52 => {
                eax = m.r32(esi.wrapping_add(0x3acu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x384u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x380u32));
                esi = m.r32(esi.wrapping_add(0x390u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 54; continue;
            }
            54 => {
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_sub(0x8);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = esp;
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(esp.wrapping_add(0x6cu32));
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x48u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(esp.wrapping_add(0xa8u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(ebx, esi);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x70u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x68u32), esi);
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(ebx.wrapping_add(0x4u32), ecx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x6cu32), ecx);
                edi = m.r32(edx);
                m.w32(esp.wrapping_add(0x70u32), eax);
                m.w32(ebx.wrapping_add(0x8u32), eax);
                eax = m.r32(esp.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edx;
                { let t = m.r32(edi.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 56; continue;
            }
            56 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x88u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 57; continue;
            }
            57 => {
                ecx = m.r32(esp.wrapping_add(0x80u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x7c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D0BD0` viper_slider_set_value
pub fn f_004d0bd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = ecx;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = eax;
                pc = 5; continue;
            }
            5 => {
                eax = esi.wrapping_add(0x374u32);
                ecx = m.r32(esi.wrapping_add(0x374u32));
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ecx = esi.wrapping_add(0x244u32);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(ecx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = ((eax as i32) >> 31) as u32;
                ebp = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = eax;
                eax = m.r32(esi);
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = ebx.wrapping_add(ebx);
                let (r, f) = lift::sub(edi, ebp, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x354u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                m.w32(esp.wrapping_add(0x28u32), edi);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                lift::fst32(m, esp.wrapping_add(0x14u32), fp.st(0)); fp.pop();
                { let v = f64::from(m.r32(esp.wrapping_add(0x2cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x2cu32)); let d = fp.st(0); fp.set(0, d * v); }
                m.w32(esp.wrapping_add(0x2cu32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x2cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x2cu32)); let d = fp.st(0); fp.set(0, d / v); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x18u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x2cu32)); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d0cb0, ecx, &a); }
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

/// `0X004D0CB0` viper_slider_set_thumb_position
pub fn f_004d0cb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = esi.wrapping_add(0x244u32);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x10u32), edx);
                edx = m.r32(ecx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edi = eax;
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x1cu32), edi);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if !fl.pf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                pc = 6; continue;
            }
            4 => {
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0xcu32), ecx);
                pc = 6; continue;
            }
            6 => {
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d - v); }
                edx = m.r32(esp.wrapping_add(0xcu32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esi.wrapping_add(0x350u32));
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                ecx = esi;
                m.w32(esi.wrapping_add(0x374u32), edx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = edi.wrapping_add(edi);
                edx = m.r32(esi.wrapping_add(0x354u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x20u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); let d = fp.st(0); fp.set(0, d / v); }
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x20u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = f64::from(m.r32(esi.wrapping_add(0x35cu32)) as i32); fp.push(v); }
                ecx = eax;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w32(esi.wrapping_add(0x3a0u32), 0x1u32);
                pc = 10; continue;
            }
            8 => {
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                fl = lift::logic(((eax >> 8) & 0xff) & 0x5u32, 8);
                if fl.pf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w32(esi.wrapping_add(0x3a0u32), 0x2u32);
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(ecx, m.r32(esi.wrapping_add(0x358u32)), 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi);
                m.w32(esi.wrapping_add(0x358u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
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

/// `0X004D0E40` viper_slider_get_value
pub fn f_004d0e40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                eax = m.r32(esi.wrapping_add(0x244u32));
                ecx = esi.wrapping_add(0x244u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = m.r32(esi.wrapping_add(0x374u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x4u32)) as i32); fp.push(v); }
                fl = lift::sub(eax, 0xbf800000u32, 32).1;
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a4u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x4u32), fp.st(0)); fp.pop();
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x358u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x354u32));
                eax = m.r32(esi.wrapping_add(0x350u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                ecx = esi;
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esi.wrapping_add(0x374u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); let d = fp.st(0); fp.set(0, d - v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); let d = fp.st(0); fp.set(0, d * v); }
                lift::fst32(m, esp.wrapping_add(0xcu32), fp.st(0)); fp.pop();
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x8u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x8u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x8u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x4u32)); fp.push(v); }
                { let v = fp.st(0); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x8u32)); let d = fp.st(0); fp.set(0, v - d); }
                { let v = lift::fld32(m, esp.wrapping_add(0xcu32)); let d = fp.st(0); fp.set(0, v / d); }
                { let v = f64::from(m.r32(esi.wrapping_add(0x350u32)) as i32); fp.push(v); }
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::fst32(m, esp.wrapping_add(0xfffffffcu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xfffffffcu32)); let d = fp.st(0); fp.set(0, d + v); }
                { let v = lift::fld32(m, 0x66bc58u32); let d = fp.st(0); fp.set(0, d + v); }
                lift::fst32(m, esp.wrapping_add(0xfffffffcu32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0xfffffffcu32)); fp.push(v); }
                { if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; } return eax; }
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x41c420 => f_0041c420(m, h, ecx, esp),
        0x41c5a0 => f_0041c5a0(m, h, ecx, esp),
        0x440e50 => f_00440e50(m, h, ecx, esp),
        0x440ed0 => f_00440ed0(m, h, ecx, esp),
        0x440f70 => f_00440f70(m, h, ecx, esp),
        0x45c300 => f_0045c300(m, h, ecx, esp),
        0x471060 => f_00471060(m, h, ecx, esp),
        0x4af740 => f_004af740(m, h, ecx, esp),
        0x4b0570 => f_004b0570(m, h, ecx, esp),
        0x4b05e0 => f_004b05e0(m, h, ecx, esp),
        0x4b0650 => f_004b0650(m, h, ecx, esp),
        0x4b06c0 => f_004b06c0(m, h, ecx, esp),
        0x4b0730 => f_004b0730(m, h, ecx, esp),
        0x4b07a0 => f_004b07a0(m, h, ecx, esp),
        0x4b8a40 => f_004b8a40(m, h, ecx, esp),
        0x4b90e0 => f_004b90e0(m, h, ecx, esp),
        0x4c6000 => f_004c6000(m, h, ecx, esp),
        0x4c6080 => f_004c6080(m, h, ecx, esp),
        0x4c60d0 => f_004c60d0(m, h, ecx, esp),
        0x4c6210 => f_004c6210(m, h, ecx, esp),
        0x4c6390 => f_004c6390(m, h, ecx, esp),
        0x4c9dd0 => f_004c9dd0(m, h, ecx, esp),
        0x4cccf0 => f_004cccf0(m, h, ecx, esp),
        0x4ccff0 => f_004ccff0(m, h, ecx, esp),
        0x4ce6d0 => f_004ce6d0(m, h, ecx, esp),
        0x4cf890 => f_004cf890(m, h, ecx, esp),
        0x4cfb30 => f_004cfb30(m, h, ecx, esp),
        0x4d02a0 => f_004d02a0(m, h, ecx, esp),
        0x4d0bd0 => f_004d0bd0(m, h, ecx, esp),
        0x4d0cb0 => f_004d0cb0(m, h, ecx, esp),
        0x4d0e40 => f_004d0e40(m, h, ecx, esp),
        _ => return None,
    }))
}
