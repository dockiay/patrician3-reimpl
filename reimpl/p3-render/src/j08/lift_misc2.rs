//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0040A560` table_layout_load_from_ini
pub fn f_0040a560(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65b3c3u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x404);
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
                m.w32(esp.wrapping_add(0x41cu32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x428u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692e78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x428u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x430u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692e80u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x424u32));
                m.w32(esi.wrapping_add(0x18u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692e88u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x424u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692e90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esi.wrapping_add(0x14u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x2cu32), eax);
                eax = m.r32(esp.wrapping_add(0x424u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692e98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x424u32));
                m.w32(esi.wrapping_add(0x10u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ea4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x424u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692eb0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esi.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x28u32), eax);
                eax = m.r32(esp.wrapping_add(0x424u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ebcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x424u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x30u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x400u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ec8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692eccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = esi.wrapping_add(0x8u32);
                eax = esi.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ed0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x1cu32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x20u32), eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x20u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                edx = m.r32(esp.wrapping_add(0x454u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0x30);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x400u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cba86u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ed8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ee4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639f83, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x1cu32));
                ebp = 0x1u32;
                esp = esp.wrapping_add(0x18);
                m.w32(ecx, eax);
                eax = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692ee8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639f83, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                m.w32(edx.wrapping_add(ebp.wrapping_mul(4)), eax);
                pc = 6; continue;
            }
            5 => {
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x424u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x400u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cba87u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692eecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692efcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639f83, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x18);
                ebp = 0x1u32;
                m.w32(ecx, eax);
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edx, ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x692f00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639f83, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0xc);
                m.w32(ecx.wrapping_add(ebp.wrapping_mul(4)), eax);
                pc = 13; continue;
            }
            12 => {
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(ebp, edx, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esi.wrapping_add(0x34u32), ebx);
                edx = eax.wrapping_add(0xffffffffu32);
                fl = lift::sub(edx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x20u32));
                edi = m.r32(esi.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                ebp = edx;
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(edi.wrapping_add(eax));
                ebx = m.r32(esi.wrapping_add(0x34u32));
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x34u32), ebx);
                ecx = ebx;
                ebx = m.r32(eax.wrapping_add(0xfffffffcu32));
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esi.wrapping_add(0x34u32), ebx);
                if !fl.zf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esi.wrapping_add(0x1cu32));
                m.w8(esp.wrapping_add(0x41cu32), ((ebx & 0xff)) as u8);
                eax = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                ebp = m.r32(esi.wrapping_add(0x34u32));
                edx = m.r32(esi.wrapping_add(0x10u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x34u32), ebp);
                let (r, f) = lift::imul(edx, eax); fl = f;
                edx = r;
                ecx = eax.wrapping_add(0xffffffffu32);
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::imul(ecx, m.r32(esi.wrapping_add(0x24u32))); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x38u32), ecx);
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x424u32);
                m.w32(esp.wrapping_add(0x41cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x414u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x410);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0040EB50` draw_text_with_shadow
pub fn f_0040eb50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x90000000u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
    ebx = m.r32(esp.wrapping_add(0x1cu32));
    ebp = m.r32(esp.wrapping_add(0x18u32));
    edi = m.r32(esp.wrapping_add(0x20u32));
    esi = eax;
    eax = ebx.wrapping_add(0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ecx = ebp.wrapping_add(0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
    esp = esp.wrapping_add(0x20);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebp = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004209E0` text_layout_ctor
pub fn f_004209e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4624a0, ecx, &a); }
    m.w32(esi, 0x66c44cu32);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00420A00` text_layout_dtor
pub fn f_00420a00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    m.w32(ecx, 0x66c44cu32);
    { let a = []; return h.call_at(esp, m, 0x4624d0, ecx, &a); }
    eax
}

/// `0X00420A10` text_layout_draw
pub fn f_00420a10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65c648u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
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
                m.w32(esp.wrapping_add(0x1cu32), esi);
                eax = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x38u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x462520, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x24u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x48u32), ecx);
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 3; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                edi = m.r32(esi.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x10u32), ebx);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x3u32, 8).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 7; continue;
            }
            5 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                pc = 7; continue;
            }
            7 => {
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(m.r32(edi), ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 12; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 10; continue;
            }
            10 => {
                esi = m.r32(edi.wrapping_add(0x4u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(esi, ebp, 32); fl = f;
                esi = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                eax = m.r32(esi.wrapping_add(0x4u32));
                esi = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(edi);
                esp = esp.wrapping_add(0x10);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xcu32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 10; continue; }
                pc = 11; continue;
            }
            11 => {
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(esp.wrapping_add(0x48u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(edx, 0x1cu32, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edx);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w32(esp.wrapping_add(0x18u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x40u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x48u32), ebx);
                if (fl.zf || fl.sf != fl.of) { pc = 27; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 15; continue;
            }
            15 => {
                edi = m.r32(esi.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x3u32, 8).1;
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                pc = 19; continue;
            }
            17 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                pc = 19; continue;
            }
            19 => {
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(m.r32(edi.wrapping_add(0x18u32)), ebx, 32).1;
                if (fl.cf || fl.zf) { pc = 26; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 22; continue;
            }
            22 => {
                esi = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::add(esi, ebp, 32); fl = f;
                esi = r;
                ecx = m.r32(esi.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0x4);
                eax = ecx;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x18u32));
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0x14u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, eax, 32).1;
                if fl.cf { pc = 22; continue; }
                pc = 25; continue;
            }
            25 => {
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                let (r, f) = lift::add(edx, 0x1cu32, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x48u32), eax);
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x18u32)), ebp, 32).1;
                if (fl.sf == fl.of) { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                m.w32(esp.wrapping_add(0x18u32), ebp);
                pc = 29; continue;
            }
            29 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x20);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00420DE0` text_layout_deleting_dtor
pub fn f_00420de0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x420a00, ecx, &a); }
                fl = lift::logic(u32::from(m.r8(esp.wrapping_add(0x8u32))) & 0x1u32, 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00460680` widget_invalidate_rect
pub fn f_00460680(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(0x28);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0x30u32));
    ecx = esp.wrapping_add(0x4u32);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x84u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x34u32));
    eax = esp.wrapping_add(0x10u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    edx = m.r32(ecx);
    { let t = m.r32(edx.wrapping_add(0x84u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x8u32));
    edx = m.r32(esi);
    m.w32(esp.wrapping_add(0x20u32), ecx);
    ecx = esi;
    { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    edx = esp.wrapping_add(0x1cu32);
    let (r, f) = lift::add(eax, ecx, 32); fl = f;
    eax = r;
    ecx = m.r32(esp.wrapping_add(0x10u32));
    m.w32(esp.wrapping_add(0x28u32), eax);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(esp.wrapping_add(0x24u32), ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = 0x6da5f0u32;
    m.w32(esp.wrapping_add(0x20u32), eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x28);
    return eax;
    eax
}

/// `0X00462420` play_click_denied_sound
pub fn f_00462420(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
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
    return eax;
    eax
}

/// `0X004624A0` text_layout_base_ctor
pub fn f_004624a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esi, 0x66e36cu32);
    m.w32(esi.wrapping_add(0x10u32), ebx);
    m.w32(esi.wrapping_add(0x8u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
    m.w8(esi.wrapping_add(0x4u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x5u32), (0x1u32) as u8);
    m.w32(esi.wrapping_add(0xcu32), ebx);
    m.w32(esi.wrapping_add(0x14u32), ebx);
    m.w32(esi.wrapping_add(0x18u32), ebx);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004624D0` text_layout_base_dtor
pub fn f_004624d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esi, 0x66e36cu32);
    { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w8(esi.wrapping_add(0x5u32), (0x1u32) as u8);
    m.w8(esi.wrapping_add(0x4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xcu32), eax);
    m.w32(esi.wrapping_add(0x14u32), eax);
    m.w32(esi.wrapping_add(0x18u32), eax);
    m.w32(esi.wrapping_add(0x10u32), eax);
    m.w32(esi.wrapping_add(0x8u32), eax);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00462500` text_layout_base_clear
pub fn f_00462500(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w8(esi.wrapping_add(0x5u32), (0x1u32) as u8);
    m.w8(esi.wrapping_add(0x4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xcu32), eax);
    m.w32(esi.wrapping_add(0x14u32), eax);
    m.w32(esi.wrapping_add(0x18u32), eax);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00462520` text_layout_base_layout_text
pub fn f_00462520(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x34);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x48u32));
                m.w32(edi.wrapping_add(0xcu32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x54u32));
                ebx = eax;
                eax = 0x1u32;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x48u32), esi);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esp.wrapping_add(0x64u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4u32)))) & 0xff);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x5u32)))) & 0xff);
                esp = esp.wrapping_add(0x14);
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), esi);
                m.w32(esp.wrapping_add(0x18u32), esi);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w8(esp.wrapping_add(0x20u32), ((edx & 0xff)) as u8);
                m.w32(edi.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x28u32), eax);
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x20u32, 8).1;
                m.w8(esi, ((eax & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x50u32), ((eax & 0xff)) as u8);
                if !fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 5; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x18u32), esi);
                pc = 73; continue;
            }
            6 => {
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::sub(u32::from(m.r8(ebp.wrapping_add(0x1u32))), 0x20u32, 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                edx = esi.wrapping_add(0x1u32);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                m.w32(edi.wrapping_add(0x14u32), eax);
                pc = 73; continue;
            }
            13 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w32(esp.wrapping_add(0x18u32), esi);
                pc = 73; continue;
            }
            17 => {
                fl = lift::sub((eax & 0xff), 0x5cu32, 8).1;
                if !fl.zf { pc = 73; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x6cu32, 8).1;
                m.w8(esp.wrapping_add(0x48u32), ((eax & 0xff)) as u8);
                if !fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                ecx = esi.wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 73; continue;
            }
            20 => {
                fl = lift::sub((eax & 0xff), 0x72u32, 8).1;
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w8(esp.wrapping_add(0x20u32), (0x2u32) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                edx = esi.wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 73; continue;
            }
            22 => {
                fl = lift::sub((eax & 0xff), 0x63u32, 8).1;
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w8(esp.wrapping_add(0x20u32), (0x3u32) as u8);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                eax = esi.wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 73; continue;
            }
            24 => {
                fl = lift::sub((eax & 0xff), 0x66u32, 8).1;
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::add(ebp, 0x2u32, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                ecx = esi.wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 73; continue;
            }
            26 => {
                fl = lift::sub((eax & 0xff), 0x64u32, 8).1;
                if !fl.zf { pc = 30; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x2u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x41u32, 8).1;
                if (fl.sf != fl.of) { pc = 73; continue; }
                pc = 28; continue;
            }
            28 => {
                fl = lift::sub((eax & 0xff), 0x5au32, 8).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 73; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(edi.wrapping_add(0x8u32));
                m.w8(esp.wrapping_add(0x30u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sub(eax, 0x41u32, 32); fl = f;
                eax = r;
                ecx = 0x6cc3d4u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c5870, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x462f00, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(edi.wrapping_add(0x10u32));
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edx.wrapping_add(ecx.wrapping_mul(4));
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffffcu32));
                edx = m.r32(eax.wrapping_add(0xfffffff8u32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                eax = edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffff0u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                ecx = m.r32(eax);
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub((edx & 0xff), 0x3u32, 8).1;
                edx = m.r32(eax.wrapping_add(0x4u32));
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                let r = ecx & 0x4u32; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = esi.wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                let (r, f) = lift::add(ebp, 0x2u32, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 73; continue;
            }
            30 => {
                fl = lift::sub((eax & 0xff), 0x74u32, 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub((eax & 0xff), 0x43u32, 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub((eax & 0xff), 0x4cu32, 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 33; continue;
            }
            33 => {
                fl = lift::sub((eax & 0xff), 0x42u32, 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub((eax & 0xff), 0x68u32, 8).1;
                if !fl.zf { pc = 73; continue; }
                pc = 35; continue;
            }
            35 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 38; continue;
            }
            38 => {
                m.w32(esp.wrapping_add(0x2cu32), esi);
                let (r, f) = lift::add(ebp, 0x2u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(esi, 0x2u32, 32); fl = f;
                esi = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((eax & 0xff), 0x74u32, 8).1;
                if !fl.zf { pc = 43; continue; }
                pc = 39; continue;
            }
            39 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x5fu32, 8).1;
                if fl.zf { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                edx = esi;
                eax = ebp;
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                pc = 41; continue;
            }
            41 => {
                m.w8(eax.wrapping_add(edx), ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ecx & 0xff), 0x5fu32, 8).1;
                if !fl.zf { pc = 41; continue; }
                pc = 42; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebp = eax.wrapping_add(ebp).wrapping_add(0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 54; continue;
            }
            43 => {
                fl = lift::sub((eax & 0xff), 0x43u32, 8).1;
                if !fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                ecx = m.r32(0x6cc37cu32);
                ebx = m.r32(ecx.wrapping_add(0x4u32));
                pc = 55; continue;
            }
            45 => {
                fl = lift::sub((eax & 0xff), 0x4cu32, 8).1;
                if !fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                edx = m.r32(0x6cc384u32);
                ebx = m.r32(edx.wrapping_add(0x4u32));
                pc = 55; continue;
            }
            47 => {
                fl = lift::sub((eax & 0xff), 0x42u32, 8).1;
                if !fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                eax = m.r32(0x6cc380u32);
                ebx = m.r32(eax.wrapping_add(0x4u32));
                pc = 55; continue;
            }
            49 => {
                fl = lift::sub((eax & 0xff), 0x68u32, 8).1;
                if !fl.zf { pc = 55; continue; }
                pc = 50; continue;
            }
            50 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x5fu32, 8).1;
                if fl.zf { pc = 53; continue; }
                pc = 51; continue;
            }
            51 => {
                edx = esi;
                eax = ebp;
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                pc = 52; continue;
            }
            52 => {
                m.w8(eax.wrapping_add(edx), ((ecx & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub((ecx & 0xff), 0x5fu32, 8).1;
                if !fl.zf { pc = 52; continue; }
                pc = 53; continue;
            }
            53 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebp = eax.wrapping_add(ebp).wrapping_add(0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 54; continue;
            }
            54 => {
                esi = esi.wrapping_add(eax).wrapping_add(0x1u32);
                pc = 55; continue;
            }
            55 => {
                ecx = esp.wrapping_add(0x38u32);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x1u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::sub((eax & 0xff), 0x20u32, 8).1;
                m.w32(esp.wrapping_add(0x48u32), 0x0u32);
                if fl.zf { pc = 64; continue; }
                pc = 56; continue;
            }
            56 => {
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
                if fl.zf { pc = 64; continue; }
                pc = 57; continue;
            }
            57 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 64; continue; }
                pc = 58; continue;
            }
            58 => {
                ebx = 0x1u32;
                pc = 59; continue;
            }
            59 => {
                fl = lift::sub((eax & 0xff), 0x20u32, 8).1;
                if fl.zf { pc = 64; continue; }
                pc = 60; continue;
            }
            60 => {
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
                if fl.zf { pc = 64; continue; }
                pc = 61; continue;
            }
            61 => {
                fl = lift::sub(u32::from(m.r8(ebx.wrapping_add(ebp).wrapping_add(0xffffffffu32))), 0x5cu32, 8).1;
                if !fl.zf { pc = 63; continue; }
                pc = 62; continue;
            }
            62 => {
                fl = lift::sub((eax & 0xff), 0x74u32, 8).1;
                if fl.zf { pc = 64; continue; }
                pc = 63; continue;
            }
            63 => {
                m.w8(ebx.wrapping_add(esi).wrapping_add(0xffffffffu32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x48u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(ebp)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 59; continue; }
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = esi.wrapping_add(0xffffffffu32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 66; continue; }
                pc = 65; continue;
            }
            65 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 72; continue; }
                pc = 66; continue;
            }
            66 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 72; continue; }
                pc = 67; continue;
            }
            67 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = eax.wrapping_mul(8);
                m.w8(edx, (0x0u32) as u8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi.wrapping_add(0x14u32));
                edx = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf == fl.of) { pc = 69; continue; }
                pc = 68; continue;
            }
            68 => {
                ebx = m.r32(edi.wrapping_add(0x10u32));
                edx = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                edx = m.r32(ebx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xcu32));
                pc = 70; continue;
            }
            69 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                pc = 70; continue;
            }
            70 => {
                ebx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edx;
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(edi.wrapping_add(0x14u32), ecx);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                pc = 72; continue;
            }
            72 => {
                edx = m.r32(edi.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esi.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x462f00, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebx = m.r32(esp.wrapping_add(0x24u32));
                pc = 73; continue;
            }
            73 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 78; continue; }
                pc = 74; continue;
            }
            74 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x5fu32, 8).1;
                if !fl.zf { pc = 78; continue; }
                pc = 75; continue;
            }
            75 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x20u32, 8).1;
                if fl.zf { pc = 77; continue; }
                pc = 76; continue;
            }
            76 => {
                fl = lift::sub((eax & 0xff), 0xau32, 8).1;
                if !fl.zf { pc = 78; continue; }
                pc = 77; continue;
            }
            77 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                ecx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x63aec0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_add(0x10);
                ecx = eax.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x48u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x63aec0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                m.w8(ebx, (0x5fu32) as u8);
                pc = 79; continue;
            }
            78 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esp.wrapping_add(0x48u32), eax);
                pc = 79; continue;
            }
            79 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 81; continue; }
                pc = 80; continue;
            }
            80 => {
                edx = m.r32(edi.wrapping_add(0x10u32));
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                let (r, f) = lift::add(eax, m.r32(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xcu32)), 32); fl = f;
                eax = r;
                pc = 82; continue;
            }
            81 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                pc = 82; continue;
            }
            82 => {
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 84; continue; }
                pc = 83; continue;
            }
            83 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 101; continue; }
                pc = 84; continue;
            }
            84 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 92; continue; }
                pc = 85; continue;
            }
            85 => {
                ecx = m.r32(edi.wrapping_add(0x10u32));
                edx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(m.r32(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xcu32)), 0x0u32, 32).1;
                if fl.zf { pc = 92; continue; }
                pc = 86; continue;
            }
            86 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(ecx, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 93; continue; }
                pc = 87; continue;
            }
            87 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic(ebx & ebx, 32);
                m.w32(edi.wrapping_add(0x14u32), eax);
                if fl.zf { pc = 90; continue; }
                pc = 88; continue;
            }
            88 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 90; continue; }
                pc = 89; continue;
            }
            89 => {
                edx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 90; continue;
            }
            90 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 101; continue; }
                pc = 91; continue;
            }
            91 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 101; continue;
            }
            92 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                pc = 93; continue;
            }
            93 => {
                fl = lift::sub(ebx, ecx, 32).1;
                if (fl.cf || fl.zf) { pc = 97; continue; }
                pc = 94; continue;
            }
            94 => {
                fl = lift::sub(u32::from(m.r8(ebx.wrapping_add(0xffffffffu32))), 0x2du32, 8).1;
                if fl.zf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                m.w8(ebx, (0x2du32) as u8);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                pc = 96; continue;
            }
            96 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                pc = 100; continue;
            }
            97 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                fl = lift::sub(ecx, edx, 32).1;
                if (!fl.cf && !fl.zf) { pc = 99; continue; }
                pc = 98; continue;
            }
            98 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                m.w32(esp.wrapping_add(0x10u32), esi);
                pc = 100; continue;
            }
            99 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x462c20, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 100; continue;
            }
            100 => {
                let (r, f) = lift::add(m.r32(edi.wrapping_add(0x14u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(edi.wrapping_add(0x14u32), r);
                pc = 101; continue;
            }
            101 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x5fu32, 8).1;
                if fl.zf { pc = 103; continue; }
                pc = 102; continue;
            }
            102 => {
                fl = lift::sub((ecx & 0xff), 0x2du32, 8).1;
                if !fl.zf { pc = 107; continue; }
                pc = 103; continue;
            }
            103 => {
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 106; continue; }
                pc = 104; continue;
            }
            104 => {
                fl = lift::sub(u32::from(m.r8(ebx)), 0x2du32, 8).1;
                if fl.zf { pc = 106; continue; }
                pc = 105; continue;
            }
            105 => {
                edx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                pc = 106; continue;
            }
            106 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                ebx = esi;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                pc = 107; continue;
            }
            107 => {
                eax = m.r32(esp.wrapping_add(0x4cu32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, edx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 109; continue; }
                pc = 108; continue;
            }
            108 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                pc = 109; continue;
            }
            109 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                fl = lift::sub(eax, edx, 32).1;
                if fl.zf { pc = 112; continue; }
                pc = 110; continue;
            }
            110 => {
                fl = lift::sub(eax, m.r32(edi.wrapping_add(0xcu32)), 32).1;
                if (fl.sf == fl.of) { pc = 112; continue; }
                pc = 111; continue;
            }
            111 => {
                m.w32(esp.wrapping_add(0x28u32), edx);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                pc = 112; continue;
            }
            112 => {
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 1; continue; }
                pc = 113; continue;
            }
            113 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0x14u32), 0x0u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00462C20` text_layout_base_add_segment
pub fn f_00462c20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                fl = lift::sub(esi, m.r32(edi.wrapping_add(0x8u32)), 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                { let a = []; eax = h.call_at(esp, m, 0x463000, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            2 => {
                ecx = m.r32(edi.wrapping_add(0x10u32));
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x34u32), edx);
                eax = edx;
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(edi.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x2cu32)))) & 0xff);
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esi = ecx.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x28u32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esi.wrapping_add(0x10u32), ((edx & 0xff)) as u8);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                eax = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ebp = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x8u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x10u32);
                m.w32(edi.wrapping_add(0xcu32), ecx);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x8u32), eax);
                ecx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi.wrapping_add(0xcu32), ecx);
                pc = 5; continue;
            }
            5 => {
                ebx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                if fl.zf { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = ebx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = eax;
                fl = lift::logic(ebx & ebx, 32);
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                pc = 8; continue;
            }
            8 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax)))) & 0xff);
                m.w8(eax, ((edx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esp.wrapping_add(0x2cu32), edx);
                if !fl.zf { pc = 8; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp;
                m.w8(edi.wrapping_add(ebx), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xcu32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xcu32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63a29f, ecx, &a); }
                ecx = m.r32(esi);
                esp = esp.wrapping_add(0x8);
                m.w32(esi.wrapping_add(0x4u32), eax);
                edx = ecx.wrapping_add(ecx.wrapping_mul(2));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esi, ecx);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x28u32)))) & 0xff);
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                edx = m.r32(esp.wrapping_add(0x34u32));
                m.w8(eax, ((ecx & 0xff)) as u8);
                m.w32(eax.wrapping_add(0x8u32), edi);
                m.w32(eax.wrapping_add(0x4u32), edx);
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00462D60` text_layout_base_free_lines
pub fn f_00462d60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                eax = m.r32(eax.wrapping_add(ebx).wrapping_add(0x14u32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x10u32));
                eax = ecx.wrapping_add(ebx);
                ecx = m.r32(ecx.wrapping_add(ebx));
                fl = lift::sub(ecx, edi, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x4u32)), edi, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.cf || fl.zf) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(eax.wrapping_add(0x4u32));
                eax = m.r32(edx.wrapping_add(edi).wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x10u32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0xcu32, 32); fl = f;
                edi = r;
                eax = ecx.wrapping_add(ebx);
                ecx = m.r32(ecx.wrapping_add(ebx));
                fl = lift::sub(ebp, ecx, 32).1;
                if fl.cf { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x10u32));
                eax = m.r32(edx.wrapping_add(ebx).wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x10u32));
                m.w32(ecx.wrapping_add(ebx).wrapping_add(0x4u32), edi);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(ebx, 0x1cu32, 32); fl = f;
                ebx = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 12; continue;
            }
            12 => {
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63a1b6, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x10u32), edi);
                pc = 15; continue;
            }
            15 => {
                m.w32(esi.wrapping_add(0x8u32), edi);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00462E10` text_layout_base_advance_scroll
pub fn f_00462e10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx.wrapping_add(0x18u32));
                esi = eax;
                let (r, f) = lift::sub(0, esi, 32); fl = f;
                esi = r;
                fl = lift::sub(edx, esi, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ecx.wrapping_add(0x18u32), edx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                ebp = m.r32(ecx.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                edi = m.r32(ecx.wrapping_add(0x10u32));
                edx = ebp;
                ebx = eax;
                m.w32(ecx.wrapping_add(0x14u32), ebp);
                esi = edx.wrapping_mul(8);
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ebx = m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                ebx = eax;
                m.w32(ecx.wrapping_add(0x14u32), edx);
                esi = edx.wrapping_mul(8);
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32)), 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = edx.wrapping_mul(8);
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                edx = m.r32(edi.wrapping_add(esi.wrapping_mul(4)).wrapping_add(0x8u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ecx.wrapping_add(0x18u32), edx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            6 => {
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                esi = m.r32(ecx.wrapping_add(0x14u32));
                edi = m.r32(ecx.wrapping_add(0x10u32));
                edx = m.r32(ecx.wrapping_add(0x18u32));
                ebx = esi.wrapping_mul(8);
                let (r, f) = lift::sub(ebx, esi, 32); fl = f;
                ebx = r;
                ebp = edx.wrapping_add(eax);
                ebx = m.r32(edi.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0x8u32));
                fl = lift::sub(ebp, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(ecx.wrapping_add(0x18u32), ebp);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            9 => {
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = esi.wrapping_add(0x1u32);
                m.w32(ecx.wrapping_add(0x14u32), edx);
                m.w32(ecx.wrapping_add(0x18u32), eax);
                esi = edx.wrapping_mul(8);
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                fl = lift::sub(eax, m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(ecx.wrapping_add(0x18u32), eax);
                m.w32(ecx.wrapping_add(0x14u32), edx);
                esi = edx.wrapping_mul(8);
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                fl = lift::sub(eax, m.r32(esi.wrapping_add(edi).wrapping_add(0x8u32)), 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 10; continue; }
                pc = 11; continue;
            }
            11 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00462F00` text_layout_base_add_inline_image
pub fn f_00462f00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = m.r32(edi.wrapping_add(0x8u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                { let a = []; eax = h.call_at(esp, m, 0x463000, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 3; continue;
            }
            2 => {
                edx = m.r32(edi.wrapping_add(0x10u32));
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ebx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffff0u32));
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi.wrapping_add(0x10u32));
                esi = ebp.wrapping_mul(8);
                let (r, f) = lift::sub(esi, ebp, 32); fl = f;
                esi = r;
                let (r, f) = lift::shl(esi, 0x2u32, 32, fl); fl = f;
                esi = r;
                ecx = esi.wrapping_add(eax);
                eax = m.r32(esi.wrapping_add(eax).wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(ecx.wrapping_add(0x14u32));
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63a29f, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(ecx).wrapping_add(0x14u32), eax);
                edx = m.r32(edi.wrapping_add(0x10u32));
                ebp = m.r32(esi.wrapping_add(edx).wrapping_add(0x14u32));
                eax = esi.wrapping_add(edx);
                eax = m.r32(esi.wrapping_add(edx).wrapping_add(0x18u32));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                m.w32(ebp.wrapping_add(eax.wrapping_mul(4)), ebx);
                ecx = m.r32(edi.wrapping_add(0x10u32));
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x24u32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(ecx).wrapping_add(0x18u32));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                m.w8(ebp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32), ((ebx & 0xff)) as u8);
                eax = m.r32(edi.wrapping_add(0x10u32));
                eax = m.r32(esi.wrapping_add(eax).wrapping_add(0x18u32));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = ebp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x10u32));
                esp = esp.wrapping_add(0x10);
                eax = esi.wrapping_add(ecx).wrapping_add(0x18u32);
                ecx = m.r32(esi.wrapping_add(ecx).wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                m.w32(eax, ecx);
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(edi.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(edx).wrapping_add(0x18u32));
                eax = esi.wrapping_add(edx);
                ebx = m.r32(eax.wrapping_add(0xcu32));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                edx = m.r32(ebp.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0xfffffff0u32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                m.w32(eax.wrapping_add(0xcu32), ebx);
                eax = m.r32(edi.wrapping_add(0x10u32));
                edx = m.r32(ebp.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(eax).wrapping_add(0x8u32));
                eax = esi.wrapping_add(eax).wrapping_add(0x8u32);
                fl = lift::sub(edx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(edi.wrapping_add(0xcu32), edx);
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                m.w32(eax, ecx);
                edx = m.r32(edi.wrapping_add(0x10u32));
                ecx = m.r32(edi.wrapping_add(0xcu32));
                eax = m.r32(esi.wrapping_add(edx).wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi.wrapping_add(0xcu32), ecx);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00463000` text_layout_base_append_empty_line
pub fn f_00463000(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = m.r32(esi.wrapping_add(0x10u32));
    let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
    eax = r;
    ecx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
    ecx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63a29f, ecx, &a); }
    ecx = m.r32(esi.wrapping_add(0x8u32));
    m.w32(esi.wrapping_add(0x10u32), eax);
    esp = esp.wrapping_add(0x8);
    edx = ecx.wrapping_mul(8);
    let (r, f) = lift::sub(edx, ecx, 32); fl = f;
    edx = r;
    let r = ecx ^ ecx; fl = lift::logic(r, 32);
    ecx = r;
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x4u32), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x8u32), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0xcu32), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w8(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32), (0x1u32) as u8);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x14u32), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    edx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x10u32));
    m.w32(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x18u32), ecx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
    eax = r;
    m.w32(esi.wrapping_add(0x8u32), eax);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004630B0` text_layout_base_scalar_deleting_dtor
pub fn f_004630b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                m.w32(esi, 0x66e36cu32);
                { let a = []; eax = h.call_at(esp, m, 0x462d60, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xff) | ((0x1u32) & 0xff);
                m.w8(esi.wrapping_add(0x4u32), ((eax & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xcu32), eax);
                m.w32(esi.wrapping_add(0x14u32), eax);
                m.w32(esi.wrapping_add(0x18u32), eax);
                m.w32(esi.wrapping_add(0x10u32), eax);
                m.w32(esi.wrapping_add(0x8u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x5u32), ((ecx & 0xff)) as u8);
                fl = lift::logic((ecx & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004673F0` draw_texture_excluding_rect
pub fn f_004673f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a4d0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x4u32));
                esi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = eax.wrapping_add(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x58u32));
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esi.wrapping_add(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x70u32));
                eax = m.r32(esp.wrapping_add(0x68u32));
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_add(0x48);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ecx.wrapping_add(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                ecx = m.r32(esp.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1060` sgl_font_ctor
pub fn f_004c1060(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x661f06u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = m.r32(0x6bf3e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0xcu32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6c7cccu32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                ecx = esi;
                m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
                m.w32(esi, 0x671220u32);
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb480, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w8(esi.wrapping_add(0xa5u32), ((ebx & 0xff)) as u8);
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1120` sgl_font_dtor
pub fn f_004c1120(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x661f26u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                m.w32(esp.wrapping_add(0x8u32), esi);
                m.w32(esi, 0x671220u32);
                m.w32(esp.wrapping_add(0x14u32), 0x1u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb480, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w8(esi.wrapping_add(0xa5u32), ((ebx & 0xff)) as u8);
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x9cu32);
                m.w8(esp.wrapping_add(0x14u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x4b1640, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xcu32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C11D0` sgl_font_unload
pub fn f_004c11d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb480, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w8(esi.wrapping_add(0xa5u32), ((ebx & 0xff)) as u8);
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1230` sgl_font_load_from_ini
pub fn f_004c1230(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x661f40u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(0x6c7cccu32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w32(esp.wrapping_add(0x8u32), eax);
                m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb480, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = esi.wrapping_add(0xa0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(ebp, ebx);
                m.w8(esi.wrapping_add(0xa5u32), (0x0u32) as u8);
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x34u32));
                m.w8(esi.wrapping_add(0xa5u32), (0x1u32) as u8);
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w8(esi.wrapping_add(0xa5u32), (0x0u32) as u8);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6bf34cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(0x6bf364u32);
                edx = m.r32(0x6bf358u32);
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
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
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(0x6bf380u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(esi.wrapping_add(0xa5u32), (0x0u32) as u8);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(0x6bf374u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::sub((eax & 0xff), 0x32u32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf3e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esp.wrapping_add(0x30u32), ebx);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x14);
                eax = esp.wrapping_add(0x14u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = 0x6dab58u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf3f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edi = eax;
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bca00, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0x94u32), eax);
                if !fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(0x6bf340u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x20u32))); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb2a0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                m.w32(esi.wrapping_add(0x94u32), eax);
                m.w8(esi.wrapping_add(0xa5u32), (0x0u32) as u8);
                pc = 16; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x9cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(0x6dd59cu32);
                edx = m.r32(0x6bf3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63c115, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(edi.wrapping_mul(4).wrapping_add(0x6bf3c8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ad50, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0xcu32, 32).1;
                if fl.cf { pc = 17; continue; }
                pc = 19; continue;
            }
            19 => {
                pc = 21; continue;
            }
            20 => {
                ebp = edi;
                pc = 21; continue;
            }
            21 => {
                eax = ebp;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 26; continue; }
                pc = 22; continue;
            }
            22 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 29; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(0x6bf340u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4bb690, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 28; continue;
            }
            25 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(esp.wrapping_add(0x2cu32), ((edx & 0xff)) as u8);
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(0x6bf340u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb6a0, ecx, &a); }
                pc = 27; continue;
            }
            26 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w8(esp.wrapping_add(0x2cu32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6bf340u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb6b0, ecx, &a); }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_add(0x18);
                pc = 28; continue;
            }
            28 => {
                m.w32(esi.wrapping_add(0x98u32), eax);
                pc = 29; continue;
            }
            29 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x98u32)), ebx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                pc = 33; continue;
            }
            32 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 33; continue;
            }
            33 => {
                ecx = esp.wrapping_add(0x8u32);
                m.w8(esp.wrapping_add(0x20u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x1c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1580` sgl_font_save_params_to_ini
pub fn f_004c1580(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x661f6au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa5u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x4u32), eax);
                m.w32(esp.wrapping_add(0xcu32), eax);
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = m.r32(esi);
                m.w8(esp.wrapping_add(0x24u32), (0x3u32) as u8);
                { let t = m.r32(eax.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6bf34cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esi);
                esp = esp.wrapping_add(0xc);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xa0u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf3f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x9cu32));
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf400u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x4cu32));
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(0x6bf358u32);
                esp = esp.wrapping_add(0x20);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be110, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                edx = m.r32(0x6bf380u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be110, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 3; continue;
            }
            3 => {
                ecx = esp.wrapping_add(0x8u32);
                m.w8(esp.wrapping_add(0x24u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0xcu32);
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x4u32);
                m.w8(esp.wrapping_add(0x24u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0x2cu32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x24);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1780` sgl_font_draw_text_at
pub fn f_004c1780(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = m.r32(edi.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esi = eax;
                eax = m.r32(edi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C17E0` sgl_font_draw_text_in_rect
pub fn f_004c17e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = m.r32(edi.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esi = eax;
                eax = m.r32(edi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb3f0, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1850` sgl_font_wrap_text
pub fn f_004c1850(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x661fb4u32);
                eax = m.r32(0x0u32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x3cu32));
                ebx = ecx;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                esi = m.r32(edi);
                eax = m.r32(esi.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 30; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ebx.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 5; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, m.r32(esp.wrapping_add(0x40u32)), 32).1;
                if (fl.sf != fl.of) { pc = 30; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                ecx = m.r32(edi);
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = eax;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w32(esp.wrapping_add(0x34u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf404u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63bd1a, ecx, &a); }
                esi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ebx.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 10; continue;
            }
            9 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(eax, m.r32(esp.wrapping_add(0x40u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x3cu32);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 31; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ee1d0u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 25; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebp = m.r32(eax);
                m.w8(esp.wrapping_add(0x34u32), (0x1u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(ebx.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 19; continue;
            }
            18 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esp.wrapping_add(0x40u32));
                m.w8(esp.wrapping_add(0x34u32), (0x0u32) as u8);
                fl = lift::sub(eax, ecx, 32).1;
                ecx = esp.wrapping_add(0x10u32);
                ebx = (ebx & !0xff) | ((u32::from((fl.zf || fl.sf != fl.of))) & 0xff);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x40u32);
                m.w8(esp.wrapping_add(0x38u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x34u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 22; continue;
            }
            21 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                ecx = esp.wrapping_add(0x20u32);
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x44u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x40u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf408u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x4u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w8(esp.wrapping_add(0x38u32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x34u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x34u32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x34u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = esp.wrapping_add(0x40u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf40cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63bd1a, ecx, &a); }
                esi = eax;
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf410u32);
                ecx = esp.wrapping_add(0x40u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f5e3, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x24u32));
                pc = 15; continue;
            }
            24 => {
                ebp = m.r32(esp.wrapping_add(0x28u32));
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = esp.wrapping_add(0x3cu32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf412u32);
                ecx = esp.wrapping_add(0x44u32);
                ebx = (ebx & !0xff) | ((0x6u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x40u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w8(esp.wrapping_add(0x38u32), (0x7u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x40u32);
                m.w8(esp.wrapping_add(0x34u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                m.w8(esp.wrapping_add(0x34u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 27; continue;
            }
            27 => {
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 29; continue;
            }
            29 => {
                ecx = esp.wrapping_add(0x3cu32);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 30; continue;
            }
            30 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 31; continue;
            }
            31 => {
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
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1B20` sgl_font_get_text_width
pub fn f_004c1b20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0xcu32));
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x63abe0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C1B90` sgl_font_get_font_handle
pub fn f_004c1b90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0x98u32));
    return eax;
    eax
}

/// `0X004C1BC0` sgl_font_get_class_name
pub fn f_004c1bc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bf3e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0xcu32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esi;
    m.w32(esp.wrapping_add(0x8u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C1BF0` sgl_font_scalar_deleting_dtor
pub fn f_004c1bf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4c1120, ecx, &a); }
                fl = lift::logic(u32::from(m.r8(esp.wrapping_add(0x8u32))) & 0x1u32, 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 2; continue;
            }
            2 => {
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x40a560 => f_0040a560(m, h, ecx, esp),
        0x40eb50 => f_0040eb50(m, h, ecx, esp),
        0x4209e0 => f_004209e0(m, h, ecx, esp),
        0x420a00 => f_00420a00(m, h, ecx, esp),
        0x420a10 => f_00420a10(m, h, ecx, esp),
        0x420de0 => f_00420de0(m, h, ecx, esp),
        0x460680 => f_00460680(m, h, ecx, esp),
        0x462420 => f_00462420(m, h, ecx, esp),
        0x4624a0 => f_004624a0(m, h, ecx, esp),
        0x4624d0 => f_004624d0(m, h, ecx, esp),
        0x462500 => f_00462500(m, h, ecx, esp),
        0x462520 => f_00462520(m, h, ecx, esp),
        0x462c20 => f_00462c20(m, h, ecx, esp),
        0x462d60 => f_00462d60(m, h, ecx, esp),
        0x462e10 => f_00462e10(m, h, ecx, esp),
        0x462f00 => f_00462f00(m, h, ecx, esp),
        0x463000 => f_00463000(m, h, ecx, esp),
        0x4630b0 => f_004630b0(m, h, ecx, esp),
        0x4673f0 => f_004673f0(m, h, ecx, esp),
        0x4c1060 => f_004c1060(m, h, ecx, esp),
        0x4c1120 => f_004c1120(m, h, ecx, esp),
        0x4c11d0 => f_004c11d0(m, h, ecx, esp),
        0x4c1230 => f_004c1230(m, h, ecx, esp),
        0x4c1580 => f_004c1580(m, h, ecx, esp),
        0x4c1780 => f_004c1780(m, h, ecx, esp),
        0x4c17e0 => f_004c17e0(m, h, ecx, esp),
        0x4c1850 => f_004c1850(m, h, ecx, esp),
        0x4c1b20 => f_004c1b20(m, h, ecx, esp),
        0x4c1b90 => f_004c1b90(m, h, ecx, esp),
        0x4c1bc0 => f_004c1bc0(m, h, ecx, esp),
        0x4c1bf0 => f_004c1bf0(m, h, ecx, esp),
        _ => return None,
    }))
}
