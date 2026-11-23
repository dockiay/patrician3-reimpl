//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004237D0` game_screen_show_letter_announcer
pub fn f_004237d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = m.r32(edi.wrapping_add(0x2528u32));
                esi = edi.wrapping_add(0x2528u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ad8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0x18c0u32));
                ecx = edi.wrapping_add(0x18c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(edi.wrapping_add(0x2600u32));
                eax = m.r32(0x6dccf8u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi.wrapping_add(0x2604u32), ecx);
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

/// `0X00425EEE` game_screen_create_side_menus_and_dialogs
pub fn f_00425eee(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6cbdf0u32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0xeu32);
                eax = m.r32(eax.wrapping_add(0x16cu32));
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, 0x66de94u32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6e51acu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x599e10, ecx, &a); }
                esp = esp.wrapping_add(4);
                { let a = []; eax = h.call_at(esp, m, 0x441240, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x429a40, ecx, &a); }
                edx = m.r32(0x6c7cccu32);
                m.w32(ebp.wrapping_add(0xffffffa0u32), edx);
                ebx = (ebx & !0xff) | ((0x14u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6967ecu32);
                ecx = ebp.wrapping_add(0xffffffa0u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = ebp.wrapping_add(0xffffffa0u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba64u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cba64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696804u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb10u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xb68u32));
                edi = esi.wrapping_add(0xb68u32);
                ecx = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x84u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(m.r32(ebp.wrapping_add(0xffffffd4u32)), eax, 32); fl = f;
                m.w32(ebp.wrapping_add(0xffffffd4u32), r);
                { let a = []; eax = h.call_at(esp, m, 0x4416f0, ecx, &a); }
                edi = m.r32(ebp.wrapping_add(0xffffffd4u32));
                ecx = ebp.wrapping_add(0xffffff50u32);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esi.wrapping_add(0xc40u32));
                m.w32(ebp.wrapping_add(0xffffffd4u32), edi);
                edi = esi.wrapping_add(0xc40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x84u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(ebp.wrapping_add(0xffffff54u32));
                edx = m.r32(ebp.wrapping_add(0xffffffd4u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0xb68u32));
                m.w32(ebp.wrapping_add(0xffffff54u32), ecx);
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edi = ecx;
                ecx = esi.wrapping_add(0xb68u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a4d8u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x115cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x15u32) as u8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4786f0, ecx, &a); }
                pc = 3; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69681cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6cc95cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc95cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6cc95cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc95cu32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc95cu32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x700u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x16u32) as u8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5acfa0, ecx, &a); }
                pc = 6; continue;
            }
            5 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696834u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5524u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5524u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e5524u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5524u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5524u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x17u32) as u8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ae390, ecx, &a); }
                pc = 9; continue;
            }
            8 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69684cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5528u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5528u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e5528u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5528u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5528u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x810u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x18u32) as u8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a4110, ecx, &a); }
                pc = 12; continue;
            }
            11 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696864u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54e8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54e8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e8u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e8u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x650u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x19u32) as u8);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a0ab0, ecx, &a); }
                pc = 15; continue;
            }
            14 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69687cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54ccu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54ccu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54ccu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54ccu32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54ccu32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3d0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1au32) as u8);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ac410, ecx, &a); }
                pc = 18; continue;
            }
            17 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696894u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e551cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e551cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e551cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e551cu32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e551cu32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xb20u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1bu32) as u8);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a5d60, ecx, &a); }
                pc = 21; continue;
            }
            20 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6968acu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54f8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54f8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f8u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f8u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x7f8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1cu32) as u8);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a31f0, ecx, &a); }
                pc = 24; continue;
            }
            23 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6968c4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54e0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54e0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e0u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54e0u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6e8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1du32) as u8);
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a2600, ecx, &a); }
                pc = 27; continue;
            }
            26 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6968dcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54d8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54d8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54d8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54d8u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54d8u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8f8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1eu32) as u8);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a8af0, ecx, &a); }
                pc = 30; continue;
            }
            29 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 30; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6968f4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5500u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5500u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e5500u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5500u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5500u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6a8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1fu32) as u8);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x59fc40, ecx, &a); }
                pc = 33; continue;
            }
            32 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 33; continue;
            }
            33 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69690cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54c8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54c8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54c8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54c8u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54c8u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2c8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x20u32) as u8);
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5aa6b0, ecx, &a); }
                pc = 36; continue;
            }
            35 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 36; continue;
            }
            36 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696924u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e550cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e550cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e550cu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e550cu32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e550cu32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x21u32) as u8);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a4f50, ecx, &a); }
                pc = 39; continue;
            }
            38 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 39; continue;
            }
            39 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69693cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54f0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54f0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f0u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54f0u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x920u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x22u32) as u8);
                if fl.zf { pc = 41; continue; }
                pc = 40; continue;
            }
            40 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ab1e0, ecx, &a); }
                pc = 42; continue;
            }
            41 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 42; continue;
            }
            42 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696954u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5514u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5514u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e5514u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5514u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5514u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1e0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x23u32) as u8);
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a9a80, ecx, &a); }
                pc = 45; continue;
            }
            44 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 45; continue;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69696cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5508u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5508u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e5508u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5508u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5508u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x83c8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x24u32) as u8);
                if fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x486d20, ecx, &a); }
                pc = 48; continue;
            }
            47 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 48; continue;
            }
            48 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696984u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6ce6d0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6ce6d0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                { let a = []; eax = h.call_at(esp, m, 0x48ac30, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffd0u32));
                edx = m.r32(ebp.wrapping_add(0xffffffd4u32));
                eax = m.r32(ebp.wrapping_add(0xffffffd8u32));
                m.w32(0x6cc974u32, ecx);
                ecx = m.r32(ebp.wrapping_add(0xffffffb8u32));
                m.w32(0x6cc978u32, edx);
                edx = m.r32(ebp.wrapping_add(0xffffffb4u32));
                m.w32(0x6cc97cu32, eax);
                eax = m.r32(0x6cc960u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6cc960u32;
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x69699cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cc960u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x47c9b0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cc960u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(ebp.wrapping_add(0xffffffd0u32));
                edx = m.r32(ebp.wrapping_add(0xffffffd4u32));
                eax = m.r32(ebp.wrapping_add(0xffffffd8u32));
                m.w32(0x6ce6ecu32, ecx);
                ecx = m.r32(ebp.wrapping_add(0xffffffb8u32));
                m.w32(0x6ce6f0u32, edx);
                edx = m.r32(ebp.wrapping_add(0xffffffb4u32));
                m.w32(0x6ce6f4u32, eax);
                eax = m.r32(0x6ce6d8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6ce6d8u32;
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6969b4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6ce6d8u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x495c10, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ce6d8u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1ccu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x25u32) as u8);
                if fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5a7d40, ecx, &a); }
                pc = 51; continue;
            }
            50 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 51; continue;
            }
            51 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6969ccu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e54fcu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54fcu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6e54fcu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54fcu32);
                eax = ebp.wrapping_add(0xffffffd0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e54fcu32);
                eax = ebp.wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6ce6d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441220, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6969e4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba74u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6969fcu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6d9584u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba98u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a2cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbaacu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a44u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba90u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esi.wrapping_add(0xb0u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc940u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cc940u32);
                { let a = []; eax = h.call_at(esp, m, 0x46d010, ecx, &a); }
                ecx = m.r32(0x6cc940u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc950u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = 0x6df2f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x5465b0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x1238u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                let (r, f) = lift::add(ecx, 0x94u32, 32); fl = f;
                ecx = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c5850, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = eax;
                eax = 0x100u32;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w32(esi.wrapping_add(0x1258u32), 0x0u32);
                m.w8(esi.wrapping_add(0x125cu32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x1254u32), eax);
                ecx = m.r32(0x6e51c0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6dcce0u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(ebp.wrapping_add(0xffffffd0u32), ecx);
                ecx = m.r32(0x6e51c0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0xb68u32);
                edi = eax;
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e51c0u32);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(ebp.wrapping_add(0xffffffd4u32), eax);
                eax = ebp.wrapping_add(0xffffffd0u32);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esi.wrapping_add(0xb0u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51c0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e51c0u32);
                { let a = []; eax = h.call_at(esp, m, 0x59c9f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x100u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x26u32) as u8);
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x465010, ecx, &a); }
                pc = 54; continue;
            }
            53 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 54; continue;
            }
            54 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a74u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6cc7e0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x219u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1c3u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(0x6cc7e0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x480u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x27u32) as u8);
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x466d70, ecx, &a); }
                pc = 57; continue;
            }
            56 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 57; continue;
            }
            57 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696a8cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6cc7e8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e8u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cc7e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1adu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(0x6cc7e8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x28u32) as u8);
                if fl.zf { pc = 59; continue; }
                pc = 58; continue;
            }
            58 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x46afb0, ecx, &a); }
                pc = 60; continue;
            }
            59 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 60; continue;
            }
            60 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd50u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6cc7f4u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x29u32) as u8);
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5f37d0, ecx, &a); }
                pc = 63; continue;
            }
            62 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 63; continue;
            }
            63 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696aa4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55c0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e55c0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e55c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e55c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1d60u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2au32) as u8);
                if fl.zf { pc = 65; continue; }
                pc = 64; continue;
            }
            64 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5c88e0, ecx, &a); }
                pc = 66; continue;
            }
            65 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 66; continue;
            }
            66 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ac4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e556cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e556cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e556cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e556cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2270u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2bu32) as u8);
                if fl.zf { pc = 68; continue; }
                pc = 67; continue;
            }
            67 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5cb9b0, ecx, &a); }
                pc = 69; continue;
            }
            68 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 69; continue;
            }
            69 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ae4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5574u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5574u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e5574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1f18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2cu32) as u8);
                if fl.zf { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5bb750, ecx, &a); }
                pc = 72; continue;
            }
            71 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 72; continue;
            }
            72 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b04u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5550u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5550u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5550u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x294u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2du32) as u8);
                if fl.zf { pc = 74; continue; }
                pc = 73; continue;
            }
            73 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5dd9e0, ecx, &a); }
                pc = 75; continue;
            }
            74 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 75; continue;
            }
            75 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b24u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5584u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5584u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e5584u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xeda0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2eu32) as u8);
                if fl.zf { pc = 77; continue; }
                pc = 76; continue;
            }
            76 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5d8300, ecx, &a); }
                pc = 78; continue;
            }
            77 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 78; continue;
            }
            78 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b44u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e557cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e557cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e557cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e557cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x2fu32) as u8);
                if fl.zf { pc = 80; continue; }
                pc = 79; continue;
            }
            79 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b2800, ecx, &a); }
                pc = 81; continue;
            }
            80 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 81; continue;
            }
            81 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b64u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5534u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5534u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5534u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6e5534u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1440u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x30u32) as u8);
                if fl.zf { pc = 83; continue; }
                pc = 82; continue;
            }
            82 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b8180, ecx, &a); }
                pc = 84; continue;
            }
            83 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 84; continue;
            }
            84 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b84u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5544u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5544u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x15d0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x31u32) as u8);
                if fl.zf { pc = 86; continue; }
                pc = 85; continue;
            }
            85 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5be7b0, ecx, &a); }
                pc = 87; continue;
            }
            86 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 87; continue;
            }
            87 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696b9cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5558u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5558u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5558u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1988u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x32u32) as u8);
                if fl.zf { pc = 89; continue; }
                pc = 88; continue;
            }
            88 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5df530, ecx, &a); }
                pc = 90; continue;
            }
            89 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 90; continue;
            }
            90 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696bbcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e558cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e558cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e558cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x638u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x33u32) as u8);
                if fl.zf { pc = 92; continue; }
                pc = 91; continue;
            }
            91 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b1410, ecx, &a); }
                pc = 93; continue;
            }
            92 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 93; continue;
            }
            93 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696bdcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5530u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e5530u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e5530u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1398u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x34u32) as u8);
                if fl.zf { pc = 95; continue; }
                pc = 94; continue;
            }
            94 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5eb730, ecx, &a); }
                pc = 96; continue;
            }
            95 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 96; continue;
            }
            96 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696bfcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55a8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e55a8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e55a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x38cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x35u32) as u8);
                if fl.zf { pc = 98; continue; }
                pc = 97; continue;
            }
            97 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b03d0, ecx, &a); }
                pc = 99; continue;
            }
            98 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 99; continue;
            }
            99 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696c1cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e552cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e552cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1acu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x36u32) as u8);
                if fl.zf { pc = 101; continue; }
                pc = 100; continue;
            }
            100 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b7240, ecx, &a); }
                pc = 102; continue;
            }
            101 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 102; continue;
            }
            102 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696c3cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e553cu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e553cu32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1630u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x37u32) as u8);
                if fl.zf { pc = 104; continue; }
                pc = 103; continue;
            }
            103 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5eec00, ecx, &a); }
                pc = 105; continue;
            }
            104 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 105; continue;
            }
            105 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696c5cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55b8u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e55b8u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e55b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2e30u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x38u32) as u8);
                if fl.zf { pc = 107; continue; }
                pc = 106; continue;
            }
            106 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5e9520, ecx, &a); }
                pc = 108; continue;
            }
            107 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 108; continue;
            }
            108 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696c7cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55a0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e55a0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e55a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1feu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1a9u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2a4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x39u32) as u8);
                if fl.zf { pc = 110; continue; }
                pc = 109; continue;
            }
            109 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5bac20, ecx, &a); }
                pc = 111; continue;
            }
            110 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 111; continue;
            }
            111 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1b4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5548u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3au32) as u8);
                if fl.zf { pc = 113; continue; }
                pc = 112; continue;
            }
            112 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ee000, ecx, &a); }
                pc = 114; continue;
            }
            113 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 114; continue;
            }
            114 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1acu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55b4u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3bu32) as u8);
                if fl.zf { pc = 116; continue; }
                pc = 115; continue;
            }
            115 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5b6ac0, ecx, &a); }
                pc = 117; continue;
            }
            116 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 117; continue;
            }
            117 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x440u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5538u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edi = eax;
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), edi);
                fl = lift::logic(edi & edi, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3cu32) as u8);
                if fl.zf { pc = 119; continue; }
                pc = 118; continue;
            }
            118 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x41c320, ecx, &a); }
                ecx = m.r32(0x6c7cccu32);
                m.w32(edi.wrapping_add(0xacu32), ecx);
                ecx = edi.wrapping_add(0xb0u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3eu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
                ecx = edi.wrapping_add(0x198u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x3fu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
                ecx = edi.wrapping_add(0x280u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x40u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c9790, ecx, &a); }
                ecx = edi.wrapping_add(0x400u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x41u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
                ecx = edi.wrapping_add(0x420u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x42u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4209e0, ecx, &a); }
                m.w32(edi, 0x66ca08u32);
                pc = 120; continue;
            }
            119 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 120; continue;
            }
            120 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696c9cu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6cc93cu32, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc93cu32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1b4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x43u32) as u8);
                if fl.zf { pc = 122; continue; }
                pc = 121; continue;
            }
            121 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ed830, ecx, &a); }
                pc = 123; continue;
            }
            122 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 123; continue;
            }
            123 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1b4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e55b0u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x44u32) as u8);
                if fl.zf { pc = 125; continue; }
                pc = 124; continue;
            }
            124 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5c7c50, ecx, &a); }
                pc = 126; continue;
            }
            125 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 126; continue;
            }
            126 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1b4u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5564u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x45u32) as u8);
                if fl.zf { pc = 128; continue; }
                pc = 127; continue;
            }
            127 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x5ded90, ecx, &a); }
                pc = 129; continue;
            }
            128 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 129; continue;
            }
            129 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696cbcu32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), ((ebx & 0xff)) as u8);
                m.w32(0x6e5588u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                { let a = []; eax = h.call_at(esp, m, 0x41c7b0, ecx, &a); }
                eax = m.r32(0x6d1c7cu32);
                esp = esp.wrapping_add(0x8);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9150, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6d9594u32);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9150, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbb6cu32);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9150, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = ebp.wrapping_add(0x8u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2640u32));
                ecx = esi.wrapping_add(0x2640u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                ecx = ebp.wrapping_add(0xffffff98u32);
                m.w8(esi.wrapping_add(0x488u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp.wrapping_add(0xffffff98u32);
                { let a = []; eax = h.call_at(esp, m, 0x46c350, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                ecx = esi.wrapping_add(0x7d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esi.wrapping_add(0x7ccu32), eax);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(ebp.wrapping_add(0xffffffccu32), ebx);
                edi = esi.wrapping_add(0x168u32);
                pc = 130; continue;
            }
            130 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = ebp.wrapping_add(0x8u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                ecx = edi;
                m.w32(edi.wrapping_add(0x98u32), 0x0u32);
                m.w32(ebp.wrapping_add(0xffffffe4u32), eax);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = esi.wrapping_add(0x8e0u32);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(ebp.wrapping_add(0xffffffecu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebp.wrapping_add(0xffffffe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(ebp.wrapping_add(0xffffffccu32));
                let (r, f) = lift::add(edi, 0xa0u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x5u32, 32).1;
                m.w32(ebp.wrapping_add(0xffffffccu32), ebx);
                if (fl.sf != fl.of) { pc = 130; continue; }
                pc = 131; continue;
            }
            131 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                eax = esi.wrapping_add(0x7e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esi.wrapping_add(0x7b4u32), (0x0u32) as u8);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                ecx = ebp.wrapping_add(0xffffff98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46c340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp.wrapping_add(0xffffff98u32);
                { let a = []; eax = h.call_at(esp, m, 0x46c350, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esi.wrapping_add(0x7fcu32), eax);
                m.w32(ebp.wrapping_add(0xffffffccu32), ebx);
                edi = esi.wrapping_add(0x494u32);
                pc = 132; continue;
            }
            132 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = ebp.wrapping_add(0x8u32);
                ecx = esp;
                m.w32(ebp.wrapping_add(0xffffffecu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                ecx = edi;
                m.w32(edi.wrapping_add(0x98u32), 0x0u32);
                m.w32(ebp.wrapping_add(0xffffffe4u32), eax);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::imul(eax, ebx); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = esi.wrapping_add(0x8e0u32);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(ebp.wrapping_add(0xffffffecu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebp.wrapping_add(0xffffffe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(ebp.wrapping_add(0xffffffccu32));
                let (r, f) = lift::add(edi, 0xa0u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, 0x5u32, 32).1;
                m.w32(ebp.wrapping_add(0xffffffccu32), ebx);
                if (fl.sf != fl.of) { pc = 132; continue; }
                pc = 133; continue;
            }
            133 => {
                eax = m.r32(0x6cc3e8u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x25u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8520, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696cd4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ce8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696cf0u32);
                ecx = 0x6dd524u32;
                m.w8(esi.wrapping_add(0x2637u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0x2639u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(0x6c7cccu32);
                ebx = eax;
                m.w32(ebp.wrapping_add(0xffffffe0u32), edx);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x46u32) as u8);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 134; continue;
            }
            134 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696cf8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d18u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w8(esi.wrapping_add(edi).wrapping_add(0x2889u32), ((eax & 0xff)) as u8);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 134; continue; }
                pc = 135; continue;
            }
            135 => {
                edx = ebx.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xffffffecu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x47u32) as u8);
                if fl.zf { pc = 137; continue; }
                pc = 136; continue;
            }
            136 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4103c0u32);
                edi = eax.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(eax, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 138; continue;
            }
            137 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 138; continue;
            }
            138 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x46u32) as u8);
                m.w32(esi.wrapping_add(0x287cu32), edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ebx & ebx, 32);
                m.w32(esi.wrapping_add(0x2880u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 140; continue; }
                pc = 139; continue;
            }
            139 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbb44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d3cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0x287cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = edx.wrapping_add(edi.wrapping_mul(4));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d64u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0x2880u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ebx, 32).1;
                m.w8(edi.wrapping_add(edx).wrapping_add(0xffffffffu32), ((eax & 0xff)) as u8);
                if (fl.sf != fl.of) { pc = 139; continue; }
                pc = 140; continue;
            }
            140 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebx = esi.wrapping_add(0x2890u32);
                pc = 141; continue;
            }
            141 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbb45u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d88u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696d94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696da0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696db4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w8(esi.wrapping_add(edi).wrapping_add(0x289cu32), ((eax & 0xff)) as u8);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, 0x3u32, 32).1;
                if (fl.sf != fl.of) { pc = 141; continue; }
                pc = 142; continue;
            }
            142 => {
                ecx = m.r32(ebp.wrapping_add(0xcu32));
                edx = ebp.wrapping_add(0xffffffe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696dc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696dccu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696dd4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696de8u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w8(esi.wrapping_add(0x28c8u32), ((eax & 0xff)) as u8);
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax;
                ecx = edi.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(ebp.wrapping_add(0xcu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x48u32) as u8);
                if fl.zf { pc = 144; continue; }
                pc = 143; continue;
            }
            143 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4103c0u32);
                ebx = eax.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(eax, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 145; continue;
            }
            144 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 145; continue;
            }
            145 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x46u32) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x28c8u32)))) & 0xff);
                m.w32(esi.wrapping_add(0x28a0u32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x28a4u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x28c8u32)))) & 0xff);
                esp = esp.wrapping_add(0x4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (fl.cf || fl.zf) { pc = 147; continue; }
                pc = 146; continue;
            }
            146 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696df0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696df8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbb46u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e0cu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0x28a0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = edx.wrapping_add(edi.wrapping_mul(4));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e34u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0x28a4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w8(edi.wrapping_add(edx).wrapping_add(0xffffffffu32), ((eax & 0xff)) as u8);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x28c8u32)))) & 0xff);
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 146; continue; }
                pc = 147; continue;
            }
            147 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebx = esi.wrapping_add(0x28a8u32);
                pc = 148; continue;
            }
            148 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbb47u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ebp.wrapping_add(0xffffffe0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e84u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w8(esi.wrapping_add(edi).wrapping_add(0x28b0u32), ((eax & 0xff)) as u8);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 148; continue; }
                pc = 149; continue;
            }
            149 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e90u32);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696e98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbb48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696eacu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esi.wrapping_add(0x28bcu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ebcu32);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp.wrapping_add(0xffffffe0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ec8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696edcu32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = ebp.wrapping_add(0xffffffe0u32);
                m.w8(esi.wrapping_add(0x28c0u32), ((eax & 0xff)) as u8);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x14u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffa0u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0xeu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffffe8u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = ebp.wrapping_add(0xffffff98u32);
                m.w8(ebp.wrapping_add(0xfffffffcu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x46c330, ecx, &a); }
                ecx = ebp.wrapping_add(0x8u32);
                m.w32(ebp.wrapping_add(0xfffffffcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(ebp.wrapping_add(0xfffffff4u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X00427960` game_screen_shutdown
pub fn f_00427960(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(0x6cc378u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub(eax, ebx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
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
                ecx = 0x6e6220u32;
                m.w32(0x6cc378u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x613290, ecx, &a); }
                { let a = []; eax = h.call_at(esp, m, 0x441530, ecx, &a); }
                ecx = 0x6ce6d8u32;
                { let a = []; eax = h.call_at(esp, m, 0x495b30, ecx, &a); }
                ecx = 0x6cc960u32;
                { let a = []; eax = h.call_at(esp, m, 0x47c4a0, ecx, &a); }
                eax = m.r32(0x6e51acu32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc940u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6cbdd8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0x1608u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x16f0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x17d8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x18c0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x19a8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1b78u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1268u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1350u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1438u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1520u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1e30u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1d48u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1a90u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1c60u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xec8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xfa0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x1078u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1150u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1f18u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x2000u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x20e8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x21d0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x22b8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x23a0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                { let a = []; eax = h.call_at(esp, m, 0x587530, ecx, &a); }
                ecx = m.r32(0x6cc940u32);
                { let a = []; eax = h.call_at(esp, m, 0x46d1e0, ecx, &a); }
                ecx = m.r32(0x6cbdd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x449560, ecx, &a); }
                ecx = m.r32(0x6cba64u32);
                { let a = []; eax = h.call_at(esp, m, 0x401160, ecx, &a); }
                ecx = m.r32(0x6e51acu32);
                { let a = []; eax = h.call_at(esp, m, 0x587530, ecx, &a); }
                ecx = m.r32(0x6cba74u32);
                { let a = []; eax = h.call_at(esp, m, 0x403480, ecx, &a); }
                ecx = m.r32(0x6cbaacu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ee60, ecx, &a); }
                ecx = m.r32(0x6d9584u32);
                { let a = []; eax = h.call_at(esp, m, 0x4a6a50, ecx, &a); }
                ecx = m.r32(0x6cc950u32);
                { let a = []; eax = h.call_at(esp, m, 0x472a10, ecx, &a); }
                ecx = m.r32(0x6e51c0u32);
                { let a = []; eax = h.call_at(esp, m, 0x59c410, ecx, &a); }
                ecx = m.r32(0x6cbb10u32);
                { let a = []; eax = h.call_at(esp, m, 0x41cfd0, ecx, &a); }
                ecx = m.r32(0x6cba98u32);
                { let a = []; eax = h.call_at(esp, m, 0x40c060, ecx, &a); }
                ecx = m.r32(0x6cba90u32);
                { let a = []; eax = h.call_at(esp, m, 0x40b3a0, ecx, &a); }
                ecx = m.r32(0x6e51acu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6cc940u32);
                m.w32(0x6e51acu32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e51c0u32);
                m.w32(0x6cc940u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6cbb10u32);
                m.w32(0x6e51c0u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6cbaacu32);
                m.w32(0x6cbb10u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(0x6cba74u32);
                m.w32(0x6cbaacu32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(0x6d9584u32);
                m.w32(0x6cba74u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(0x6cc950u32);
                m.w32(0x6d9584u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6cbdd8u32);
                m.w32(0x6cc950u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(0x6cbdc8u32);
                m.w32(0x6cbdd8u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(0x6cba64u32);
                m.w32(0x6cbdc8u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(0x6cba98u32);
                m.w32(0x6cba64u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(0x6cba90u32);
                m.w32(0x6cba98u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(0x6ee120u32);
                m.w32(0x6cba90u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            30 => {
                ecx = m.r32(0x6cc95cu32);
                m.w32(0x6ee120u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x4788b0, ecx, &a); }
                ecx = m.r32(0x6cc95cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(0x6ce6d0u32);
                m.w32(0x6cc95cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x4877d0, ecx, &a); }
                ecx = m.r32(0x6ce6d0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            34 => {
                ecx = m.r32(0x6e5524u32);
                m.w32(0x6ce6d0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ad0e0, ecx, &a); }
                ecx = m.r32(0x6e5524u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(0x6e5528u32);
                m.w32(0x6e5524u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ae4d0, ecx, &a); }
                ecx = m.r32(0x6e5528u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(0x6e54e8u32);
                m.w32(0x6e5528u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a4230, ecx, &a); }
                ecx = m.r32(0x6e54e8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 40; continue;
            }
            40 => {
                ecx = m.r32(0x6e54ccu32);
                m.w32(0x6e54e8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a0c30, ecx, &a); }
                ecx = m.r32(0x6e54ccu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                ecx = m.r32(0x6e551cu32);
                m.w32(0x6e54ccu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ac510, ecx, &a); }
                ecx = m.r32(0x6e551cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 44; continue;
            }
            44 => {
                ecx = m.r32(0x6e54e0u32);
                m.w32(0x6e551cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a32f0, ecx, &a); }
                ecx = m.r32(0x6e54e0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 46; continue;
            }
            46 => {
                ecx = m.r32(0x6e54d8u32);
                m.w32(0x6e54e0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a2700, ecx, &a); }
                ecx = m.r32(0x6e54d8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 48; continue;
            }
            48 => {
                ecx = m.r32(0x6e5500u32);
                m.w32(0x6e54d8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a8bf0, ecx, &a); }
                ecx = m.r32(0x6e5500u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            50 => {
                ecx = m.r32(0x6e54f0u32);
                m.w32(0x6e5500u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a5050, ecx, &a); }
                ecx = m.r32(0x6e54f0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 52; continue;
            }
            52 => {
                ecx = m.r32(0x6e54f8u32);
                m.w32(0x6e54f0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a5e60, ecx, &a); }
                ecx = m.r32(0x6e54f8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 54; continue;
            }
            54 => {
                ecx = m.r32(0x6e54c8u32);
                m.w32(0x6e54f8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x59fd70, ecx, &a); }
                ecx = m.r32(0x6e54c8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 56; continue;
            }
            56 => {
                ecx = m.r32(0x6e550cu32);
                m.w32(0x6e54c8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5aa7b0, ecx, &a); }
                ecx = m.r32(0x6e550cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 58; continue; }
                pc = 57; continue;
            }
            57 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 58; continue;
            }
            58 => {
                ecx = m.r32(0x6e5514u32);
                m.w32(0x6e550cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ab300, ecx, &a); }
                ecx = m.r32(0x6e5514u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 60; continue; }
                pc = 59; continue;
            }
            59 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 60; continue;
            }
            60 => {
                ecx = m.r32(0x6e5508u32);
                m.w32(0x6e5514u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a9bb0, ecx, &a); }
                ecx = m.r32(0x6e5508u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 62; continue;
            }
            62 => {
                ecx = m.r32(0x6e54fcu32);
                m.w32(0x6e5508u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5a7e20, ecx, &a); }
                ecx = m.r32(0x6e54fcu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 64; continue; }
                pc = 63; continue;
            }
            63 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 64; continue;
            }
            64 => {
                ecx = m.r32(0x6e55c0u32);
                m.w32(0x6e54fcu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5f39e0, ecx, &a); }
                ecx = m.r32(0x6e55c0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 66; continue; }
                pc = 65; continue;
            }
            65 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 66; continue;
            }
            66 => {
                ecx = m.r32(0x6e556cu32);
                m.w32(0x6e55c0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5c8bc0, ecx, &a); }
                ecx = m.r32(0x6e556cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 68; continue; }
                pc = 67; continue;
            }
            67 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 68; continue;
            }
            68 => {
                ecx = m.r32(0x6e5574u32);
                m.w32(0x6e556cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5cbf30, ecx, &a); }
                ecx = m.r32(0x6e5574u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 70; continue; }
                pc = 69; continue;
            }
            69 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 70; continue;
            }
            70 => {
                ecx = m.r32(0x6e5550u32);
                m.w32(0x6e5574u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5bb9d0, ecx, &a); }
                ecx = m.r32(0x6e5550u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 72; continue; }
                pc = 71; continue;
            }
            71 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 72; continue;
            }
            72 => {
                ecx = m.r32(0x6e5584u32);
                m.w32(0x6e5550u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ddae0, ecx, &a); }
                ecx = m.r32(0x6e5584u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 74; continue; }
                pc = 73; continue;
            }
            73 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 74; continue;
            }
            74 => {
                ecx = m.r32(0x6e557cu32);
                m.w32(0x6e5584u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5d87e0, ecx, &a); }
                ecx = m.r32(0x6e557cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 76; continue; }
                pc = 75; continue;
            }
            75 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 76; continue;
            }
            76 => {
                ecx = m.r32(0x6e5534u32);
                m.w32(0x6e557cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b29b0, ecx, &a); }
                ecx = m.r32(0x6e5534u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 78; continue; }
                pc = 77; continue;
            }
            77 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 78; continue;
            }
            78 => {
                ecx = m.r32(0x6e5544u32);
                m.w32(0x6e5534u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b9210, ecx, &a); }
                ecx = m.r32(0x6e5544u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 80; continue; }
                pc = 79; continue;
            }
            79 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 80; continue;
            }
            80 => {
                ecx = m.r32(0x6e5558u32);
                m.w32(0x6e5544u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5bfb20, ecx, &a); }
                ecx = m.r32(0x6e5558u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 82; continue; }
                pc = 81; continue;
            }
            81 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 82; continue;
            }
            82 => {
                ecx = m.r32(0x6e558cu32);
                m.w32(0x6e5558u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5df950, ecx, &a); }
                ecx = m.r32(0x6e558cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 84; continue; }
                pc = 83; continue;
            }
            83 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 84; continue;
            }
            84 => {
                ecx = m.r32(0x6e5530u32);
                m.w32(0x6e558cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b15f0, ecx, &a); }
                ecx = m.r32(0x6e5530u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 86; continue; }
                pc = 85; continue;
            }
            85 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 86; continue;
            }
            86 => {
                ecx = m.r32(0x6e55a8u32);
                m.w32(0x6e5530u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5eb920, ecx, &a); }
                ecx = m.r32(0x6e55a8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 88; continue; }
                pc = 87; continue;
            }
            87 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 88; continue;
            }
            88 => {
                ecx = m.r32(0x6e552cu32);
                m.w32(0x6e55a8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b0510, ecx, &a); }
                ecx = m.r32(0x6e552cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 90; continue; }
                pc = 89; continue;
            }
            89 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 90; continue;
            }
            90 => {
                ecx = m.r32(0x6e553cu32);
                m.w32(0x6e552cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b7310, ecx, &a); }
                ecx = m.r32(0x6e553cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 92; continue; }
                pc = 91; continue;
            }
            91 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 92; continue;
            }
            92 => {
                ecx = m.r32(0x6e5548u32);
                m.w32(0x6e553cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5badb0, ecx, &a); }
                ecx = m.r32(0x6e5548u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 94; continue; }
                pc = 93; continue;
            }
            93 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 94; continue;
            }
            94 => {
                ecx = m.r32(0x6e55b4u32);
                m.w32(0x6e5548u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ee0f0, ecx, &a); }
                ecx = m.r32(0x6e55b4u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 96; continue;
            }
            96 => {
                ecx = m.r32(0x6e5538u32);
                m.w32(0x6e55b4u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5b6bb0, ecx, &a); }
                ecx = m.r32(0x6e5538u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 98; continue; }
                pc = 97; continue;
            }
            97 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 98; continue;
            }
            98 => {
                ecx = m.r32(0x6e55b8u32);
                m.w32(0x6e5538u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5eef10, ecx, &a); }
                ecx = m.r32(0x6e55b8u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 100; continue; }
                pc = 99; continue;
            }
            99 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 100; continue;
            }
            100 => {
                ecx = m.r32(0x6e55a0u32);
                m.w32(0x6e55b8u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5e9770, ecx, &a); }
                ecx = m.r32(0x6e55a0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 102; continue; }
                pc = 101; continue;
            }
            101 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 102; continue;
            }
            102 => {
                ecx = m.r32(0x6cc93cu32);
                m.w32(0x6e55a0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x46ca60, ecx, &a); }
                ecx = m.r32(0x6cc93cu32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 104; continue; }
                pc = 103; continue;
            }
            103 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 104; continue;
            }
            104 => {
                ecx = m.r32(0x6e55b0u32);
                m.w32(0x6cc93cu32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5ed920, ecx, &a); }
                ecx = m.r32(0x6e55b0u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 106; continue; }
                pc = 105; continue;
            }
            105 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 106; continue;
            }
            106 => {
                ecx = m.r32(0x6e5564u32);
                m.w32(0x6e55b0u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5c7d40, ecx, &a); }
                ecx = m.r32(0x6e5564u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 108; continue; }
                pc = 107; continue;
            }
            107 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 108; continue;
            }
            108 => {
                ecx = m.r32(0x6e5588u32);
                m.w32(0x6e5564u32, ebx);
                { let a = []; eax = h.call_at(esp, m, 0x5dee80, ecx, &a); }
                ecx = m.r32(0x6e5588u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 110; continue; }
                pc = 109; continue;
            }
            109 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 110; continue;
            }
            110 => {
                ecx = m.r32(0x6cc7e0u32);
                m.w32(0x6e5588u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 112; continue; }
                pc = 111; continue;
            }
            111 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 112; continue;
            }
            112 => {
                ecx = m.r32(0x6cc7e8u32);
                m.w32(0x6cc7e0u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 114; continue; }
                pc = 113; continue;
            }
            113 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 114; continue;
            }
            114 => {
                ecx = m.r32(0x6cc7f4u32);
                m.w32(0x6cc7e8u32, ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 116; continue; }
                pc = 115; continue;
            }
            115 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 116; continue;
            }
            116 => {
                m.w32(0x6cc7f4u32, ebx);
                ecx = m.r32(esi.wrapping_add(0x1230u32));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 118; continue; }
                pc = 117; continue;
            }
            117 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 118; continue;
            }
            118 => {
                ecx = m.r32(esi.wrapping_add(0x1238u32));
                m.w32(esi.wrapping_add(0x1230u32), ebx);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 120; continue; }
                pc = 119; continue;
            }
            119 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 120; continue;
            }
            120 => {
                eax = m.r32(esi.wrapping_add(0x1268u32));
                ecx = esi.wrapping_add(0x1268u32);
                m.w32(esi.wrapping_add(0x1238u32), ebx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x1b78u32));
                ecx = esi.wrapping_add(0x1b78u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x1350u32));
                ecx = esi.wrapping_add(0x1350u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x1438u32));
                ecx = esi.wrapping_add(0x1438u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x1520u32));
                ecx = esi.wrapping_add(0x1520u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x1f18u32));
                ecx = esi.wrapping_add(0x1f18u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x2000u32));
                ecx = esi.wrapping_add(0x2000u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x20e8u32));
                ecx = esi.wrapping_add(0x20e8u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x21d0u32));
                ecx = esi.wrapping_add(0x21d0u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x22b8u32));
                ecx = esi.wrapping_add(0x22b8u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x23a0u32));
                ecx = esi.wrapping_add(0x23a0u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x18c0u32));
                ecx = esi.wrapping_add(0x18c0u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x17d8u32));
                ecx = esi.wrapping_add(0x17d8u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x1d48u32));
                ecx = esi.wrapping_add(0x1d48u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x1c60u32));
                ecx = esi.wrapping_add(0x1c60u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x1a90u32));
                ecx = esi.wrapping_add(0x1a90u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x1e30u32));
                ecx = esi.wrapping_add(0x1e30u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                ecx = esi.wrapping_add(0x19a8u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x808u32);
                { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
                ecx = esi.wrapping_add(0x8e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
                ecx = esi.wrapping_add(0xb68u32);
                { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
                ecx = esi.wrapping_add(0xc40u32);
                { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
                ecx = esi.wrapping_add(0x2630u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x262cu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x2620u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x261cu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x2628u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x2624u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                ecx = esi.wrapping_add(0x2528u32);
                { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
                m.w32(esi.wrapping_add(0x2604u32), ebx);
                ecx = m.r32(0x6dd8b4u32);
                fl = lift::sub(ecx, ebx, 32).1;
                m.w32(0x6cc388u32, ebx);
                m.w32(0x6cc38cu32, ebx);
                m.w32(0x6cc390u32, ebx);
                if fl.zf { pc = 124; continue; }
                pc = 121; continue;
            }
            121 => {
                { let a = []; eax = h.call_at(esp, m, 0x511080, ecx, &a); }
                ecx = m.r32(0x6dd8b4u32);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 123; continue; }
                pc = 122; continue;
            }
            122 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 123; continue;
            }
            123 => {
                m.w32(0x6dd8b4u32, ebx);
                pc = 124; continue;
            }
            124 => {
                eax = m.r32(esi.wrapping_add(0x28a0u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 126; continue; }
                pc = 125; continue;
            }
            125 => {
                ecx = m.r32(eax.wrapping_add(0xfffffffcu32));
                edi = eax.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 126; continue;
            }
            126 => {
                eax = m.r32(esi.wrapping_add(0x28a4u32));
                m.w32(esi.wrapping_add(0x28a0u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 128; continue; }
                pc = 127; continue;
            }
            127 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 128; continue;
            }
            128 => {
                eax = m.r32(esi.wrapping_add(0x287cu32));
                m.w32(esi.wrapping_add(0x28a4u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 130; continue; }
                pc = 129; continue;
            }
            129 => {
                edx = m.r32(eax.wrapping_add(0xfffffffcu32));
                edi = eax.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 130; continue;
            }
            130 => {
                eax = m.r32(esi.wrapping_add(0x2880u32));
                m.w32(esi.wrapping_add(0x287cu32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 132; continue; }
                pc = 131; continue;
            }
            131 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 132; continue;
            }
            132 => {
                m.w32(esi.wrapping_add(0x2880u32), ebx);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de5au32))) & 0xff);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if (fl.cf || fl.zf) { pc = 135; continue; }
                pc = 133; continue;
            }
            133 => {
                ebx = 0x6cbe08u32;
                pc = 134; continue;
            }
            134 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de5au32))) & 0xff);
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 134; continue; }
                pc = 135; continue;
            }
            135 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(eax, 0x3e8u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esi.wrapping_add(0xb0u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b41f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                { let a = []; eax = h.call_at(esp, m, 0x4621c0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b4af0, ecx, &a); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x128u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
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

/// `0X00428590` game_screen_draw_at_origin
pub fn f_00428590(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = m.r32(ecx);
    esp = esp.wrapping_sub(0xc);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    ebx = esp;
    let r = edx ^ edx; fl = lift::logic(r, 32);
    edx = r;
    let r = esi ^ esi; fl = lift::logic(r, 32);
    esi = r;
    m.w32(ebx, eax);
    eax = m.r32(esp.wrapping_add(0x1cu32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(ebx.wrapping_add(0x4u32), edx);
    m.w32(ebx.wrapping_add(0x8u32), esi);
    { let t = m.r32(edi.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00429A40` game_screen_apply_resolution_layout
pub fn f_00429a40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x9b8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xa90u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xd18u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xdf0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0xb68u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0xc40u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x808u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba64u32);
                { let a = []; eax = h.call_at(esp, m, 0x401160, ecx, &a); }
                eax = m.r32(0x6dcce0u32);
                fl = lift::sub(eax, 0x320u32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x400u32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, 0x500u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xa90u32));
                ebp = m.r32(ebx);
                ecx = esi.wrapping_add(0xa90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ebp = m.r32(edi);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0xa90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb68u32));
                ecx = esi.wrapping_add(0xb68u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(eax.wrapping_add(0x8cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xa90u32);
                ebp = eax;
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = esi.wrapping_add(0xb68u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0xa90u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = esi.wrapping_add(0xdf0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696f28u32);
                pc = 6; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x9b8u32));
                ebp = m.r32(ebx);
                ecx = esi.wrapping_add(0x9b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ebp = m.r32(edi);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x9b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb68u32));
                ecx = esi.wrapping_add(0xb68u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(eax.wrapping_add(0x8cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x9b8u32));
                ecx = esi.wrapping_add(0x9b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = esi.wrapping_add(0xb68u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x9b8u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = esi.wrapping_add(0xd18u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696f10u32);
                pc = 6; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x8e0u32));
                ebp = m.r32(ebx);
                ecx = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ebp = m.r32(edi);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb68u32));
                ecx = esi.wrapping_add(0xb68u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let t = m.r32(eax.wrapping_add(0x8cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = esi.wrapping_add(0xb68u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x78u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x8e0u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696ef8u32);
                pc = 6; continue;
            }
            6 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba64u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cba64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b4c90, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(0x6e51acu32);
                edx = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ebp = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6dcce4u32);
                edx = m.r32(edi);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                eax = m.r32(ecx.wrapping_add(0xc640u32));
                edx = m.r32(ecx.wrapping_add(0xc644u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x590ae0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x593910, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbdd8u32);
                edx = m.r32(esi.wrapping_add(0x8e0u32));
                ecx = esi.wrapping_add(0x8e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ebp = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6dcce4u32);
                edx = m.r32(edi);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x44f640, ecx, &a); }
                eax = m.r32(0x6e51c0u32);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                ebp = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6e51c0u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1078u32));
                ebx = m.r32(esi.wrapping_add(0xec8u32));
                ecx = esi.wrapping_add(0x1078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x808u32));
                ecx = esi.wrapping_add(0x808u32);
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                ecx = esi.wrapping_add(0xec8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(ebx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x808u32));
                ebp = m.r32(esi.wrapping_add(0x1078u32));
                ecx = esi.wrapping_add(0x808u32);
                ebx = esi.wrapping_add(0x1078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xec8u32));
                ecx = esi.wrapping_add(0xec8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                ebp = m.r32(esi.wrapping_add(0xfa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0xfa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xfa0u32);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1078u32));
                ecx = esi.wrapping_add(0x1078u32);
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x1078u32));
                ecx = esi.wrapping_add(0x1078u32);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0xfa0u32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xec8u32));
                ebp = m.r32(esi.wrapping_add(0x1150u32));
                ebx = esi.wrapping_add(0xec8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = esi.wrapping_add(0x1078u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x1150u32);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc940u32);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ebp = m.r32(ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0xec8u32));
                ecx = esi.wrapping_add(0xec8u32);
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc940u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xb68u32));
                ebx = esi.wrapping_add(0xb68u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x8cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441750, ecx, &a); }
                edx = m.r32(edi);
                ebp = m.r32(esi.wrapping_add(0x2640u32));
                esp = esp.wrapping_add(0xc);
                ebx = esi.wrapping_add(0x2640u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x12u32);
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6dcce4u32);
                eax = m.r32(ebx);
                let (r, f) = lift::add(ecx, 0xffffffeeu32, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6ee120u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x429fe0, ecx, &a); }
                ecx = m.r32(0x6cbb10u32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x6cbb10u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x4);
                { let a = []; return h.call_at(esp, m, 0x41e860, ecx, &a); }
            }
            12 => {
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

/// `0X0042A6C0` game_screen_enter_sea_map_mode
pub fn f_0042a6c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x2634u32))), 0x1u32, 8).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6cbdd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b2200, ecx, &a); }
                ecx = esi.wrapping_add(0x1a90u32);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w8(esi.wrapping_add(0x2634u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b2200, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                edx = m.r32(esi.wrapping_add(0x1b78u32));
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esp.wrapping_add(0x10u32), ((ecx & 0xff)) as u8);
                ecx = esi.wrapping_add(0x1b78u32);
                edi = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1a90u32));
                ecx = esi.wrapping_add(0x1a90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6de4b0u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.cf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6cbdd8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b2200, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6de508u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0x6f1u32)))) & 0xff);
                eax = eax.wrapping_add(edx.wrapping_mul(8));
                fl = lift::sub(ecx, m.r32(0x6dfc14u32), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(eax.wrapping_add(0x2c8u32));
                let r = edx & 0xe0u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                ecx = esi.wrapping_add(0x19a8u32);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                ecx = esi.wrapping_add(0x19a8u32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x19a8u32));
                ecx = esi.wrapping_add(0x19a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x1c60u32));
                edi = esi.wrapping_add(0x1c60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x18c0u32));
                ebp = esi.wrapping_add(0x18c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc3e8u32);
                edx = m.r32(esi.wrapping_add(0x1608u32));
                ebx = esi.wrapping_add(0x1608u32);
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xdu32))), 0x3u32, 8).1;
                ecx = (ecx & !0xff) | ((u32::from(!fl.cf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                ecx = esi.wrapping_add(0x19a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1a90u32));
                ecx = esi.wrapping_add(0x1a90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc3e8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xdu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x17d8u32));
                ecx = esi.wrapping_add(0x17d8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x1f18u32));
                ecx = esi.wrapping_add(0x1f18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2000u32));
                ecx = esi.wrapping_add(0x2000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x20e8u32));
                ecx = esi.wrapping_add(0x20e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x21d0u32));
                ecx = esi.wrapping_add(0x21d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x22b8u32));
                ecx = esi.wrapping_add(0x22b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x23a0u32));
                ecx = esi.wrapping_add(0x23a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                esi = m.r32(esp);
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

/// `0X0042A910` game_screen_enter_sea_battle_mode
pub fn f_0042a910(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((0x2u32) & 0xff);
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x2634u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w8(esi.wrapping_add(0x2634u32), ((ebx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1b78u32));
                ecx = esi.wrapping_add(0x1b78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                edi = esi.wrapping_add(0x19a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1608u32));
                ebx = esi.wrapping_add(0x1608u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1a90u32));
                ebp = esi.wrapping_add(0x1a90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c60u32));
                ecx = esi.wrapping_add(0x1c60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x18c0u32));
                ecx = esi.wrapping_add(0x18c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1c60u32));
                ecx = esi.wrapping_add(0x1c60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x18c0u32));
                ecx = esi.wrapping_add(0x18c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1f18u32));
                ecx = esi.wrapping_add(0x1f18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2000u32));
                ecx = esi.wrapping_add(0x2000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x20e8u32));
                ecx = esi.wrapping_add(0x20e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x21d0u32));
                ecx = esi.wrapping_add(0x21d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x22b8u32));
                ecx = esi.wrapping_add(0x22b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x23a0u32));
                edi = esi.wrapping_add(0x23a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x14u32)))) & 0xffff);
                ecx = esi;
                m.w16(esi.wrapping_add(0x804u32), ((eax & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                edi = esi.wrapping_add(0x2528u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ad8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc3e8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xdu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x17d8u32));
                let (r, f) = lift::add(esi, 0x17d8u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
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

/// `0X0042AAE0` game_screen_enter_town_mode
pub fn f_0042aae0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xffff) | ((u32::from(m.r8(0x677937u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ebx = m.r32(0x6dfc14u32);
                esi = ecx;
                ecx = 0xffu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2634u32)))) & 0xff);
                let r = edi & ecx; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                m.w16(esi.wrapping_add(0x804u32), ((edi & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0x806u32), ((ebx & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x42afc0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w8(esi.wrapping_add(0x2634u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42ad80, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            2 => {
                let r = ebp | 0xffffffffu32; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub((edi & 0xffff), u32::from(m.r16(esi.wrapping_add(0x804u32))), 16).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                ecx = esi;
                m.w32(esi.wrapping_add(0x7c8u32), ebp);
                m.w32(esi.wrapping_add(0x7c4u32), ebp);
                m.w16(esi.wrapping_add(0x804u32), ((edi & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x42afc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(esi.wrapping_add(0x806u32))), 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                ecx = esi;
                m.w32(esi.wrapping_add(0x7c8u32), ebp);
                m.w32(esi.wrapping_add(0x7c4u32), ebp);
                m.w16(esi.wrapping_add(0x806u32), ((ebx & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x42afc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0x18c0u32));
                ebp = esi.wrapping_add(0x18c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c60u32));
                ecx = esi.wrapping_add(0x1c60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc3e8u32);
                edx = m.r32(esi.wrapping_add(0x1608u32));
                ecx = esi.wrapping_add(0x1608u32);
                fl = lift::sub(u32::from(m.r8(eax.wrapping_add(0xdu32))), 0x3u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(!fl.cf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1b78u32));
                ecx = esi.wrapping_add(0x1b78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6de4b0u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.cf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(0x6de508u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = edx.wrapping_add(ecx.wrapping_mul(8));
                edx = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x6f1u32)))) & 0xff);
                fl = lift::sub(ecx, edx, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(eax.wrapping_add(0x2c8u32));
                let r = edx & 0xe0u32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 11; continue;
            }
            10 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(esi.wrapping_add(0x19a8u32));
                edi = esi.wrapping_add(0x19a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1a90u32));
                ebx = esi.wrapping_add(0x1a90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1e30u32));
                ecx = esi.wrapping_add(0x1e30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1d48u32));
                ecx = esi.wrapping_add(0x1d48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1608u32));
                ecx = esi.wrapping_add(0x1608u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c60u32));
                ecx = esi.wrapping_add(0x1c60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1f18u32));
                ecx = esi.wrapping_add(0x1f18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x2000u32));
                ecx = esi.wrapping_add(0x2000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x20e8u32));
                ecx = esi.wrapping_add(0x20e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x21d0u32));
                ecx = esi.wrapping_add(0x21d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x22b8u32));
                ecx = esi.wrapping_add(0x22b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x23a0u32));
                ecx = esi.wrapping_add(0x23a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cc3e8u32);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xdu32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi.wrapping_add(0x17d8u32));
                let (r, f) = lift::add(esi, 0x17d8u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
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

/// `0X0042AD80` game_screen_set_mode_widgets_visible
pub fn f_0042ad80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2634u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x1e30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = esi.wrapping_add(0x1e30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x30u32));
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d48u32));
                edi = esi.wrapping_add(0x1d48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x1264u32));
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0x2630u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                eax = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x1264u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                m.w32(esp.wrapping_add(0x24u32), edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x262cu32));
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x1cu32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            2 => {
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esi.wrapping_add(0x1268u32));
                ecx = esi.wrapping_add(0x1268u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1350u32));
                ecx = esi.wrapping_add(0x1350u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1438u32));
                ecx = esi.wrapping_add(0x1438u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1520u32));
                ecx = esi.wrapping_add(0x1520u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc940u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x428e00, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x428f50, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            4 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(0x6e51c0u32);
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x42b310, ecx, &a); }
                pc = 6; continue;
            }
            6 => {
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

/// `0X0042AFC0` game_screen_update_top_bar_counters
pub fn f_0042afc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(0x6de508u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x6u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x7c4u32));
                ebp = m.r32(edx.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2d4u32));
                fl = lift::sub(ebp, eax, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x1d48u32));
                edi = esi.wrapping_add(0x1d48u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x1264u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                m.w32(esp.wrapping_add(0x20u32), ebx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x2630u32));
                ebx = esi.wrapping_add(0x2630u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696f40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esi.wrapping_add(0x7c4u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(edi);
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x1264u32));
                m.w32(esp.wrapping_add(0x20u32), ebp);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(ebx);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x2cu32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6de4a0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x5308a0, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2d4u32)))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x2d2u32)))) & 0xffff);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esi.wrapping_add(0x7c8u32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x1e30u32));
                edi = esi.wrapping_add(0x1e30u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x1264u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                m.w32(esp.wrapping_add(0x28u32), ebx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x262cu32));
                ebx = esi.wrapping_add(0x262cu32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = esp.wrapping_add(0x20u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696f44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esi.wrapping_add(0x7c8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(edi);
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x88u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2608u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x1264u32));
                m.w32(esp.wrapping_add(0x28u32), ebp);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esi.wrapping_add(0x2608u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(ebx);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x30u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_add(0x20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0042B6A0` game_screen_push_event_ticker_line
pub fn f_0042b6a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65d1afu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                fl = lift::sub((eax & 0xff), 0x5u32, 8).1;
                if !fl.cf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(0x200u32));
                esi = m.r32(eax.wrapping_add(0x168u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(edi).wrapping_add(0x168u32);
                { let t = m.r32(esi.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::shl(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = edx.wrapping_add(edi).wrapping_add(0x168u32);
                ecx = esi.wrapping_add(0x9cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6dccf0u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                let (r, f) = lift::add(eax, 0x2ee0u32, 32); fl = f;
                eax = r;
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x7d0u32), eax);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(edi.wrapping_add(0x168u32));
                esi = edi.wrapping_add(0x168u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(edi.wrapping_add(0x48cu32), eax);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(edi.wrapping_add(0x490u32), ebx);
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x488u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = eax;
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = ecx.wrapping_add(edi).wrapping_add(0x168u32);
                m.w8(edi.wrapping_add(0x488u32), ((eax & 0xff)) as u8);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            4 => {
                eax = m.r32(edi.wrapping_add(0x168u32));
                esi = edi.wrapping_add(0x168u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                m.w32(edi.wrapping_add(0x48cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(edi.wrapping_add(0x490u32), ebx);
                ebx = 0x4u32;
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x13cu32));
                ecx = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ebp = m.r32(esi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x13cu32));
                ecx = esi.wrapping_add(0x9cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(esi, 0xa0u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(edi.wrapping_add(0x3e8u32));
                esi = edi.wrapping_add(0x3e8u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = edi.wrapping_add(0x7d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                edx = edi.wrapping_add(0x7d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(0x6dccf0u32);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, 0x2ee0u32, 32); fl = f;
                eax = r;
                ebx = m.r32(esi);
                m.w32(edi.wrapping_add(0x7e0u32), eax);
                edi = m.r32(edi.wrapping_add(0x480u32));
                esp = esp.wrapping_add(0xc);
                eax = edi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebx.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = esi.wrapping_add(0x9cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 7; continue;
            }
            7 => {
                ecx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x18u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0042BB20` game_screen_push_letter_ticker_line
pub fn f_0042bb20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65d1c1u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                ecx = m.r32(0x6e51acu32);
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x5u32, 8).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                edi = m.r32(ecx.wrapping_add(esi).wrapping_add(0x494u32));
                eax = ecx.wrapping_add(esi);
                ecx = m.r32(ecx.wrapping_add(esi).wrapping_add(0x52cu32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                ecx = ecx.wrapping_add(esi).wrapping_add(0x494u32);
                { let t = m.r32(edi.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                ebp = m.r32(edx.wrapping_add(esi).wrapping_add(0x494u32));
                eax = edx.wrapping_add(esi).wrapping_add(0x494u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                edi = eax.wrapping_add(esi).wrapping_add(0x494u32);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(eax, 0x5u32, 32, fl); fl = f;
                eax = r;
                ecx = eax.wrapping_add(esi).wrapping_add(0x494u32);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                edi = ecx.wrapping_add(esi).wrapping_add(0x494u32);
                ecx = edi.wrapping_add(0x9cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6dccf0u32);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                let (r, f) = lift::add(ecx, 0x2ee0u32, 32); fl = f;
                ecx = r;
                edi = esi.wrapping_add(0x494u32);
                m.w32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x7e8u32), ecx);
                ecx = edi;
                eax = m.r32(edi);
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                m.w32(esi.wrapping_add(0x7b8u32), eax);
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7b4u32)))) & 0xff);
                ecx = eax;
                m.w32(esi.wrapping_add(0x7bcu32), ebx);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = ecx.wrapping_add(esi).wrapping_add(0x494u32);
                m.w8(esi.wrapping_add(0x7b4u32), ((eax & 0xff)) as u8);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x494u32));
                edi = esi.wrapping_add(0x494u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                m.w32(esi.wrapping_add(0x7b8u32), eax);
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x10u32), 0x4u32);
                m.w32(esi.wrapping_add(0x7bcu32), ebp);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(edi.wrapping_add(0x13cu32));
                ecx = m.r32(edi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ebp = m.r32(edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebx;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0x13cu32));
                ecx = edi.wrapping_add(0x9cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 0xa0u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x714u32));
                edi = esi.wrapping_add(0x714u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x7ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                edx = esi.wrapping_add(0x7e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(0x6dccf0u32);
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, 0x2ee0u32, 32); fl = f;
                eax = r;
                ebp = m.r32(edi);
                m.w32(esi.wrapping_add(0x7f8u32), eax);
                esi = m.r32(esi.wrapping_add(0x7acu32));
                esp = esp.wrapping_add(0xc);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                ecx = edi;
                { let t = m.r32(esi.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(esi.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi.wrapping_add(0x9cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 5; continue;
            }
            5 => {
                ecx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0042D310` game_screen_update
pub fn f_0042d310(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65d20fu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6df2f0u32);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2878u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w8(esi.wrapping_add(0x2878u32), (0x0u32) as u8);
                eax = m.r32(0x6af214u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6af210u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x44u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6d95a0u32);
                m.w32(esp.wrapping_add(0x40u32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4ac700, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(0x6d95a0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(0x6cbb94u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x696f5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb94u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x440310, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6e59ccu32);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x2000u32);
                ecx = edi;
                ebp = m.r32(eax.wrapping_mul(4).wrapping_add(0x6e55d0u32));
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x6103e9, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 15; continue;
            }
            7 => {
                edi = esi.wrapping_add(0x20e8u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x6103e9, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 15; continue;
            }
            9 => {
                ecx = esi.wrapping_add(0x22b8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x610c09, ecx, &a); }
                pc = 15; continue;
            }
            11 => {
                ecx = esi.wrapping_add(0x23a0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(0x6e59ccu32);
                edx = m.r32(0x6dfc14u32);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6df2f0u32;
                m.w32(esp.wrapping_add(0x1cu32), 0xa0u32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            13 => {
                edi = esi.wrapping_add(0x21d0u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x610fd9, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 15; continue;
            }
            15 => {
                ecx = 0x6dd7a0u32;
                { let a = []; eax = h.call_at(esp, m, 0x504900, ecx, &a); }
                ecx = 0x6df2f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x546870, ecx, &a); }
                ecx = esi.wrapping_add(0x1268u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ebp = m.r32(0x66a544u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                let r = edi & 0x1u32; fl = lift::logic(r, 32);
                edi = r;
                { let t = ebp; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffff8000u32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffu32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                ecx = esi;
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4290a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 34; continue;
            }
            17 => {
                ecx = esi.wrapping_add(0x1350u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ebx = m.r32(0x66a544u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                let r = edi & 0x1u32; fl = lift::logic(r, 32);
                edi = r;
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffff8000u32; fl = lift::logic(r, 32);
                eax = r;
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                ecx = (ecx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4292a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 34; continue;
            }
            19 => {
                ecx = esi.wrapping_add(0x1438u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ebx = m.r32(0x66a544u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                let r = edi & 0x1u32; fl = lift::logic(r, 32);
                edi = r;
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffff8000u32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi;
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4296c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            21 => {
                ecx = esi.wrapping_add(0x1520u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                ebx = m.r32(0x66a544u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x14u32);
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x10u32);
                let r = edi & 0x1u32; fl = lift::logic(r, 32);
                edi = r;
                { let t = ebx; let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax & 0xffff8000u32; fl = lift::logic(r, 32);
                eax = r;
                ecx = esi;
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic((edi & 0xffff) & (edi & 0xffff), 16);
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x429490, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            23 => {
                edi = esi.wrapping_add(0x1b78u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 30; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcc34u32))) & 0xff);
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
                m.w8(0x6dcc34u32, ((edx & 0xff)) as u8);
                if !fl.zf { pc = 30; continue; }
                pc = 26; continue;
            }
            26 => {
                edi = esi.wrapping_add(0x1a90u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 34; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcc37u32))) & 0xff);
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
                m.w8(0x6dcc37u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 34; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbdd8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(0x6dcc37u32, (0x0u32) as u8);
                pc = 34; continue;
            }
            30 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x2634u32)))) & 0xff);
                ecx = esi;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                { let a = []; eax = h.call_at(esp, m, 0x42a6c0, ecx, &a); }
                pc = 33; continue;
            }
            32 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x804u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42aae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 33; continue;
            }
            33 => {
                m.w8(0x6dcc34u32, (0x0u32) as u8);
                pc = 34; continue;
            }
            34 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b6e80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A34C0` hanse_side_menu_load_layout
pub fn f_005a34c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665152u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x14);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ebx);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx.wrapping_add(0xacu32));
                esi = ebx.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebx.wrapping_add(0x194u32);
                ebp = 0x6u32;
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::add(ebx, 0x708u32, 32); fl = f;
                ebx = r;
                ebp = 0x69c18cu32;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(esi.wrapping_add(0xc0u32), 0xff000000u32);
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(edi, 0xffffffe8u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 6; continue;
            }
            5 => {
                edx = m.r32(edi.wrapping_add(0x6b201cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 6; continue;
            }
            6 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0xffffffe4u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c52c4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c52d0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax;
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x1cu32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                m.w16(edx.wrapping_add(0x7e8u32), ((ecx & 0xffff)) as u16);
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A3610` hanse_side_menu_prerender
pub fn f_005a3610(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esi = m.r32(edi.wrapping_add(0x1cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                ebp = m.r32(edi.wrapping_add(0x14u32));
                ebx = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(edi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::sub(0, esi, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w8(esp.wrapping_add(0x1bu32), ((ecx & 0xff)) as u8);
                m.w32(edi.wrapping_add(0x94u32), edx);
                m.w8(edi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = 0x6dce78u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(0x8);
                edx = m.r32(0x6cbd94u32);
                ecx = esp;
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), esi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x14u32), 0x7u32);
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = esp;
                edx = m.r32(esi);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(edi.wrapping_add(0x94u32), eax);
                m.w8(edi.wrapping_add(0xa4u32), ((ecx & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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

/// `0X005A3720` hanse_side_menu_layout_buttons
pub fn f_005a3720(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4417d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), 0x64u32);
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x7e8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ecx = edi;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi);
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax;
                ebx = 0x7u32;
                ecx = esi;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A38E0` hanse_side_menu_apply_button_states
pub fn f_005a38e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                eax = m.r32(ebx.wrapping_add(0xacu32));
                esi = ebx.wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                ecx = eax;
                let (r, f) = lift::shr(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0x10u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(edi).wrapping_add(0x7eeu32)))) & 0xff);
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x7eau32)))) & 0xff);
                fl = lift::sub(ecx, edi, 32).1;
                ecx = esi;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x7u32, 32).1;
                if fl.cf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
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

/// `0X005A3980` hanse_side_menu_show
pub fn f_005a3980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                esi = edi.wrapping_add(0xacu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = esi;
                m.w8(edi.wrapping_add(0x7eau32), (0xffu32) as u8);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x1cu32), 0x64u32);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = esi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x7e8u32)))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(edi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x18u32), edx);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x7ecu32)))) & 0xff);
                ecx = ebx;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = edi.wrapping_add(0x708u32);
                m.w32(esp.wrapping_add(0x10u32), 0x7u32);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, 0x20u32, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.zf { pc = 2; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w8(edi.wrapping_add(0x7ecu32), (0x1u32) as u8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_add(0x4);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(edi.wrapping_add(0x7edu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6e5558u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X005A3AF0` hanse_side_menu_show_buttons
pub fn f_005a3af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx.wrapping_add(0xacu32);
                edi = 0x7u32;
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
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

/// `0X005A3C00` hanse_side_menu_update
pub fn f_005a3c00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                m.w8(esp.wrapping_add(0xfu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                edi = m.r32(0x66d4fcu32);
                edx = m.r32(0x66d504u32);
                fl = lift::sub(eax, edi, 32).1;
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::sub(eax, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx | edx; fl = lift::logic(r, 32);
                ecx = r;
                edx = m.r32(0x6e5558u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x12feu32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic((eax & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x466320, ecx, &a); }
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466430, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x5u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esi.wrapping_add(0x7edu32), (0x1u32) as u8);
                ecx = m.r32(0x6cbdb8u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x1c5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4436e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6e5558u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 46; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                edi = esi.wrapping_add(0xacu32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x5a38e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebp = esi.wrapping_add(0xacu32);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7edu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(0x6cc7e0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4665d0, ecx, &a); }
                m.w8(esi.wrapping_add(0x7edu32), (0x0u32) as u8);
                pc = 11; continue;
            }
            11 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(esi.wrapping_add(0x7eau32), ((ebx & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebx, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 14; continue;
            }
            14 => {
                pc = 18; continue;
            }
            15 => {
                edi = esi.wrapping_add(0xacu32);
                ebp = 0x7u32;
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w8(esi.wrapping_add(0x7eau32), ((ebx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub((eax & 0xff), 0x7u32, 8).1;
                if !fl.cf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32));
                ecx = esi.wrapping_add(edx.wrapping_mul(8)).wrapping_add(0xacu32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x7eau32)))) & 0xff);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xfu32)))) & 0xff);
                fl = lift::sub((ecx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6e5558u32);
                fl = lift::sub((eax & 0xff), 0x6u32, 8).1;
                if !fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x8u32);
                pc = 24; continue;
            }
            23 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 24; continue;
            }
            24 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5c1080, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6e5558u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x12feu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 27; continue;
            }
            27 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 32; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(0x6cc7e0u32);
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x2u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 31; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 37; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x1u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 36; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 37; continue;
            }
            37 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 42; continue; }
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(0x6cc7e0u32);
                edx = m.r32(ecx.wrapping_add(0xe8u32));
                eax = m.r32(ecx.wrapping_add(0xd0u32));
                edx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            40 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 41; continue;
            }
            41 => {
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                m.w32(esp.wrapping_add(0x10u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66cb34u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld64(m, 0x677fa8u32); fp.com(v); }
                fp.pop();
                eax = (eax & !0xffff) | fp.sw();
                let r = eax & 0x4100u32; fl = lift::logic(r, 32);
                eax = r;
                if !fl.zf { pc = 46; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = m.r32(0x6cc7e0u32);
                eax = m.r32(ecx.wrapping_add(0xe8u32));
                edx = m.r32(ecx.wrapping_add(0xd0u32));
                eax = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x3u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4667a0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x466490, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6cc7e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x466610, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 46; continue;
            }
            46 => {
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

/// `0X005A3FE0` hanse_side_menu_open
pub fn f_005a3fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ebx = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x441180, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6e51acu32);
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xc324u32)))) & 0xff);
                ecx = ebx;
                m.w8(ebx.wrapping_add(0x7ebu32), ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x5a37c0, ecx, &a); }
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                edi = ebx.wrapping_add(0xacu32);
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(esi).wrapping_add(0x7eeu32)))) & 0xff);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(esi, 0x7u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
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

/// `0X005AC6E0` market_hall_side_menu_load_layout
pub fn f_005ac6e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x665612u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x18u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp.wrapping_add(0xacu32));
                esi = ebp.wrapping_add(0xacu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = ebp.wrapping_add(0x194u32);
                ebx = 0x2u32;
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = 0x69db7cu32;
                ebx = ebp.wrapping_add(0x368u32);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(esi.wrapping_add(0xc0u32), 0xff000000u32);
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                fl = lift::sub(edi, 0x69db88u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5824u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c5830u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax;
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x18u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                m.w16(ebp.wrapping_add(0x3c8u32), ((ecx & 0xffff)) as u16);
                ecx = esp.wrapping_add(0x30u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4237d0 => f_004237d0(m, h, ecx, esp),
        0x425eee => f_00425eee(m, h, ecx, esp),
        0x427960 => f_00427960(m, h, ecx, esp),
        0x428590 => f_00428590(m, h, ecx, esp),
        0x429a40 => f_00429a40(m, h, ecx, esp),
        0x42a6c0 => f_0042a6c0(m, h, ecx, esp),
        0x42a910 => f_0042a910(m, h, ecx, esp),
        0x42aae0 => f_0042aae0(m, h, ecx, esp),
        0x42ad80 => f_0042ad80(m, h, ecx, esp),
        0x42afc0 => f_0042afc0(m, h, ecx, esp),
        0x42b6a0 => f_0042b6a0(m, h, ecx, esp),
        0x42bb20 => f_0042bb20(m, h, ecx, esp),
        0x42d310 => f_0042d310(m, h, ecx, esp),
        0x5a34c0 => f_005a34c0(m, h, ecx, esp),
        0x5a3610 => f_005a3610(m, h, ecx, esp),
        0x5a3720 => f_005a3720(m, h, ecx, esp),
        0x5a38e0 => f_005a38e0(m, h, ecx, esp),
        0x5a3980 => f_005a3980(m, h, ecx, esp),
        0x5a3af0 => f_005a3af0(m, h, ecx, esp),
        0x5a3c00 => f_005a3c00(m, h, ecx, esp),
        0x5a3fe0 => f_005a3fe0(m, h, ecx, esp),
        0x5ac6e0 => f_005ac6e0(m, h, ecx, esp),
        _ => return None,
    }))
}
