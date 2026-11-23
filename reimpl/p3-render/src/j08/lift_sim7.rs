//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0052D6B0` trade_center_get_buy_cost
pub fn f_0052d6b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x5u32)))) & 0xff);
                edx = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub((ebx & 0xff), (edx & 0xff), 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                m.w32(esp.wrapping_add(0x14u32), edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                if (fl.cf || fl.zf) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub((ebx & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
                pc = 6; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x30u32))), (eax & 0xff), 8).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), edx);
                m.w32(esp.wrapping_add(0x14u32), 0x4cccccccu32);
                pc = 7; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x1cu32), 0xe6666666u32);
                m.w32(esp.wrapping_add(0x14u32), 0x33333333u32);
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                { let v = lift::fild64(m, esp.wrapping_add(0x14u32)); fp.push(v); }
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx)))) & 0xff);
                ebp = eax;
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, ebp.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                { let v = lift::fild64(m, esp.wrapping_add(0x1cu32)); fp.push(v); }
                esi = eax;
                edi = edx;
                lift::fst32(m, esp.wrapping_add(0x30u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x30u32)); fp.push(v); }
                { let v = lift::fld32(m, ebp.wrapping_mul(4).wrapping_add(0x673a18u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = esi;
                ebp = edi;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sbb(ebp, edx, fl.cf, 32); fl = f;
                ebp = r;
                let r = ebx & 0xffu32; fl = lift::logic(r, 32);
                ebx = r;
                eax = m.r32(ebx.wrapping_mul(4).wrapping_add(0x673990u32));
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6e0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ebp = m.r32(esp.wrapping_add(0x28u32));
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                eax = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)).wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let (r, f) = lift::sub(esi, eax, 32); fl = f;
                esi = r;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sbb(edi, edx, fl.cf, 32); fl = f;
                edi = r;
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x63a6a0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = edx;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sar(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                eax = edx;
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
        0x52d6b0 => f_0052d6b0(m, h, ecx, esp),
        _ => return None,
    }))
}
