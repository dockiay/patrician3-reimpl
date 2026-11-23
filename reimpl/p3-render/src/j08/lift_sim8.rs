//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00542560` create_initial_ai_merchant_for_town
pub fn f_00542560(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6ddb74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = ecx;
                m.w32(esp.wrapping_add(0x18u32), eax);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xcu32)))) & 0xffff);
                edx = m.r32(0x6de508u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xeu32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x6u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esi = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::shl(eax, 0x3u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx).wrapping_add(0x2c8u32)))) & 0xff);
                edx = m.r32(esi);
                edi = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = (eax & !0xff) | ((!(eax & 0xff)) & 0xff);
                edx = m.r32(edi);
                eax = lift::sx8((eax & 0xff));
                let r = eax & 0x40u32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = (eax & 0xff) | 0xa0u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::shr(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6de4a0u32;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x530470, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebx = (ebx & !0xffff) | (((eax & 0xffff)) & 0xffff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6de4aau32))) & 0xffff);
                let r = ebx & 0xffffu32; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.cf { pc = 15; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi);
                eax = m.r32(edi);
                let r = ecx & 0x3u32; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(ecx.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(edi, eax);
                edx = m.r32(esi);
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = 0x6dd73cu32;
                m.w32(esi, edx);
                eax = m.r32(0x6de4b4u32);
                edx = eax.wrapping_add(ebx).wrapping_add(0xa00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d8cf0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = (ecx & !0xff) | (((ebx & 0xff)) & 0xff);
                m.w16(eax.wrapping_add(0x6u32), (0x4u32) as u16);
                m.w16(eax.wrapping_add(0x8u32), ((ecx & 0xffff)) as u16);
                ecx = 0xffff8300u32;
                m.w16(eax.wrapping_add(0xau32), ((ecx & 0xffff)) as u16);
                m.w16(eax.wrapping_add(0xcu32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x8u32)))) & 0xff);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x7u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0xeu32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let r = (edx & 0xff) & 0x3u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0x10u32), ((edx & 0xff)) as u8);
                edx = (edx & !0xff) | (((ecx & 0xff)) & 0xff);
                let (r, f) = lift::add((ecx & 0xff), 0x7u32, 8); fl = f;
                ecx = (ecx & !0xff) | ((r) & 0xff);
                let r = (edx & 0xff) & 0xfu32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                let r = (ecx & 0xff) & 0xfu32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                m.w8(eax.wrapping_add(0x11u32), ((edx & 0xff)) as u8);
                m.w8(eax.wrapping_add(0xfu32), ((ecx & 0xff)) as u8);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5303c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x10u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x12u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4f68d0, ecx, &a); }
                esp = esp.wrapping_add(20);
                eax = ebx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32)))) & 0xffff);
                eax = ebp.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(0x6ddb70u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                { let d = u64::from(ecx); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                ecx = esi;
                m.w8(esi.wrapping_add(0x1cu32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4f6420, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(m.r32(esp.wrapping_add(0x20u32))); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(esi.wrapping_add(0x1du32), ((edx & 0xff)) as u8);
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x34u32));
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(ecx))), 0x0u32, 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1du32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6ddb74u32))) & 0xff);
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::sub((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | (((eax & 0xff)) & 0xff);
                m.w8(esi.wrapping_add(0x1du32), ((eax & 0xff)) as u8);
                fl = lift::sub(u32::from(m.r8(edx.wrapping_add(ecx))), 0x0u32, 8).1;
                if !fl.zf { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4f6450, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x1du32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w8(eax.wrapping_add(ecx), (0x1u32) as u8);
                ecx = 0x6de4a0u32;
                m.w8(esi.wrapping_add(0x546u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(ebp.wrapping_add(0x8u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esi, eax);
                eax = m.r32(edi);
                { let d = u64::from(m.r32(0x6de4b0u32)); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x18u32), 0x4u32);
                let r = edx & 0xffffu32; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                pc = 7; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x34u32));
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd7a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x507d60, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x30u32));
                edi = eax;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                { let d = u64::from(m.r32(0x6de4b0u32)); let n = (u64::from(edx) << 32) | u64::from(eax); eax = (n / d) as u32; edx = (n % d) as u32; }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                eax = m.r32(0x6de508u32);
                m.w32(esp.wrapping_add(0x34u32), edx);
                if !fl.cf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::logic(u32::from(m.r8(ecx.wrapping_add(0x2c8u32))) & 0x1u32, 8);
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52a8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                let (r, f) = lift::add(m.r32(esp.wrapping_add(0x2cu32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esp.wrapping_add(0x2cu32), r);
                pc = 11; continue;
            }
            10 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = edx.wrapping_add(eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x52a8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 11; continue;
            }
            11 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5181d0, ecx, &a); }
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5182b0, ecx, &a); }
                m.w8(edi, ((ebx & 0xff)) as u8);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xeu32)))) & 0xffff);
                m.w16(edi.wrapping_add(0x4u32), ((eax & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x14u32)))) & 0xffff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x28u32)))) & 0xff);
                m.w16(esi.wrapping_add(0xeu32), ((ecx & 0xffff)) as u16);
                edx = m.r32(edi.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6dd7a0u32;
                m.w32(edi.wrapping_add(0x18u32), edx);
                m.w8(edi.wrapping_add(0x13au32), ((eax & 0xff)) as u8);
                m.w8(edi.wrapping_add(0x13bu32), ((eax & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x509a50, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let r = u32::from(m.r8(edi.wrapping_add(0x136u32))) | 0x10u32; fl = lift::logic(r, 8);
                m.w8(edi.wrapping_add(0x136u32), (r) as u8);
                pc = 13; continue;
            }
            13 => {
                let r = eax & 0x3u32; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x673660u32)))) & 0xffff);
                m.w16(edi.wrapping_add(0x40u32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if !fl.zf { pc = 6; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4f7bb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x542560 => f_00542560(m, h, ecx, esp),
        _ => return None,
    }))
}
