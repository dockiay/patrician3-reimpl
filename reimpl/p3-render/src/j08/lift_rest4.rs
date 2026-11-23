//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00472470` choice_list_dialog_on_delete_answer
pub fn f_00472470(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f4a9u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(ebp.wrapping_add(0x8u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x1084u32));
                m.w8(esi.wrapping_add(0xadcu32), ((ebx & 0xff)) as u8);
                ecx = m.r32(0x66bdc0u32);
                m.w32(ebp.wrapping_add(0xfffffff0u32), esp);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b97ccu32);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b97d4u32);
                ecx = ebp.wrapping_add(0xffffffe4u32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = eax;
                ecx = esi.wrapping_add(0x938u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebp.wrapping_add(0xffffffe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp.wrapping_add(0xffffffecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = ebp.wrapping_add(0xffffffe8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x6u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffe4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffe0u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edx = m.r32(ebp.wrapping_add(0xffffffecu32));
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x7u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x653f49, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            2 => {
                eax = 0x472533u32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            3 => {
                esi = m.r32(ebp.wrapping_add(0x8u32));
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0x4u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = ebp.wrapping_add(0xffffffecu32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x680u32));
                ecx = esi.wrapping_add(0x680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x4a4u32));
                ecx = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x850u32));
                ecx = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x938u32));
                ecx = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(ebp.wrapping_add(0xfffffff4u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004BB690` sgl_MemoryFont_CreateFixed
pub fn f_004bb690(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    { let t = m.r32(0x6da964u32); let a = []; return h.call_at(esp, m, t, ecx, &a); }
    eax
}

/// `0X004BB6A0` sgl_MemoryFont_CreateKerning
pub fn f_004bb6a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    { let t = m.r32(0x6da960u32); let a = []; return h.call_at(esp, m, t, ecx, &a); }
    eax
}

/// `0X004BB6B0` sgl_MemoryFont_CreateProportional
pub fn f_004bb6b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    { let t = m.r32(0x6da95cu32); let a = []; return h.call_at(esp, m, t, ecx, &a); }
    eax
}

/// `0X004C0A20` game_window_create_main_window
pub fn f_004c0a20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(ebx.wrapping_add(0x1cu32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                esi = m.r32(0x66a51cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(0x6dcce0u32, edi);
                m.w32(0x6dcce4u32, ebp);
                { let t = esi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let t = esi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = edi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                let r = ecx | ebp; fl = lift::logic(r, 32);
                ecx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp.wrapping_add(0x1cu32));
                esi = eax;
                pc = 4; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7u32);
                { let t = esi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                edi = edi.wrapping_add(eax.wrapping_mul(2));
                { let t = esi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let t = esi; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esi = eax.wrapping_add(ecx.wrapping_mul(2));
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let t = m.r32(0x66a0c4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x651167, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = ((eax as i32) >> 31) as u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xc00000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24)), m.r32(esp.wrapping_add(28)), m.r32(esp.wrapping_add(32)), m.r32(esp.wrapping_add(36)), m.r32(esp.wrapping_add(40))]; eax = h.call_at(esp, m, 0x650acb, ecx, &a); }
                esp = esp.wrapping_add(44);
                edx = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(ebx.wrapping_add(0x1cu32));
                fl = lift::logic(eax & eax, 32);
                m.w32(edx.wrapping_add(0x44u32), ecx);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(0x66a4c0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6dcb90u32);
                m.w32(0x6dcba4u32, 0x0u32);
                fl = lift::logic(eax & eax, 32);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(ebx.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffff0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a510u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(ebx.wrapping_add(0x1cu32));
                let r = eax & 0xff3fffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffff0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a514u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(ebx.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a518u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(28);
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x65348d, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X004C65D0` keyframe_timer_save_to_ini
pub fn f_004c65d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp &= 0xfffffff8;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66213fu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                eax = m.r32(0x6c7cccu32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x2cu32), edi);
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi.wrapping_add(0x8u32));
                m.w8(esp.wrapping_add(0x2cu32), (0x3u32) as u8);
                fl = lift::sub(eax, edi, 32).1;
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x4u32)), edi, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x10u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                ecx = m.r32(0x6bf5c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x8u32)), 32).1;
                if !fl.cf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(0x8);
                ecx = esp.wrapping_add(0x18u32);
                { let v = lift::fld32(m, eax.wrapping_add(edi.wrapping_mul(4))); fp.push(v); }
                lift::fst64(m, esp, fp.st(0)); fp.pop();
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf638u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf63cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f49c, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x30u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f61f, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x2cu32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 5; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x8u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf640u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(ebp.wrapping_add(0x8u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(0x6bf5d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be110, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(ebp.wrapping_add(0x8u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(0x6bf5e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be110, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                pc = 11; continue;
            }
            11 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C8D60` combo_box_find_item_by_text
pub fn f_004c8d60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp &= 0xfffffff8;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x662325u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x30);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = m.r32(edi.wrapping_add(0x630u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = ebp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x44u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x3cu32));
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
            2 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = eax.wrapping_add(0xffffffffu32);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(edi.wrapping_add(0x62cu32));
                esi = m.r32(eax.wrapping_add(ecx.wrapping_mul(4)));
                ecx = esi.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x24u32), edx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x10u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x28u32), ((eax & 0xff)) as u8);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                edx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x30u32), edx);
                eax = m.r32(esi.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x18u32), 0x67078cu32);
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x44u32), (0x1u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(eax);
                edx = m.r32(ebp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = esp.wrapping_add(0x14u32);
                fl = lift::logic(eax & eax, 32);
                ebx = (ebx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x44u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(edi.wrapping_add(0x630u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
                pc = 1; continue;
            }
            8 => {
                ecx = ebp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x44u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = ebp;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x472470 => f_00472470(m, h, ecx, esp),
        0x4bb690 => f_004bb690(m, h, ecx, esp),
        0x4bb6a0 => f_004bb6a0(m, h, ecx, esp),
        0x4bb6b0 => f_004bb6b0(m, h, ecx, esp),
        0x4c0a20 => f_004c0a20(m, h, ecx, esp),
        0x4c65d0 => f_004c65d0(m, h, ecx, esp),
        0x4c8d60 => f_004c8d60(m, h, ecx, esp),
        _ => return None,
    }))
}
