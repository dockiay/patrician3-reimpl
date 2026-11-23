//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X0046F760` choice_list_dialog_ctor
pub fn f_0046f760(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f366u32);
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
    { let a = []; eax = h.call_at(esp, m, 0x41c320, ecx, &a); }
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = esi.wrapping_add(0xf0u32);
    m.w32(esp.wrapping_add(0x14u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x4a4u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x58cu32);
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    eax = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x678u32), eax);
    ecx = esi.wrapping_add(0x680u32);
    m.w8(esp.wrapping_add(0x14u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x768u32);
    m.w8(esp.wrapping_add(0x14u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x850u32);
    m.w8(esp.wrapping_add(0x14u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x938u32);
    m.w8(esp.wrapping_add(0x14u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c9790, ecx, &a); }
    ecx = esi.wrapping_add(0xab8u32);
    m.w8(esp.wrapping_add(0x14u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi.wrapping_add(0xae0u32);
    m.w8(esp.wrapping_add(0x14u32), (0x9u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x41cdc0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    m.w32(esi, 0x66ebc8u32);
    m.w32(esi.wrapping_add(0xb0u32), ebx);
    m.w32(esi.wrapping_add(0xbcu32), ebx);
    m.w32(esi.wrapping_add(0xb4u32), ebx);
    m.w32(esi.wrapping_add(0xb8u32), ebx);
    m.w8(esi.wrapping_add(0xadcu32), ((ebx & 0xff)) as u8);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X0046F860` choice_list_dialog_dtor
pub fn f_0046f860(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f3f6u32);
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
                m.w32(esi, 0x66ebc8u32);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), 0x9u32);
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
                eax = m.r32(esi.wrapping_add(0xbcu32));
                m.w32(esi.wrapping_add(0xb0u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                m.w32(esi.wrapping_add(0xbcu32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                fl = lift::sub(eax, ebx, 32).1;
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
                ecx = esi.wrapping_add(0xae0u32);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x41cf20, ecx, &a); }
                ecx = esi.wrapping_add(0xab8u32);
                m.w8(esp.wrapping_add(0x14u32), (0x8u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
                ecx = esi.wrapping_add(0x938u32);
                m.w8(esp.wrapping_add(0x14u32), (0x7u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c9970, ecx, &a); }
                ecx = esi.wrapping_add(0x850u32);
                m.w8(esp.wrapping_add(0x14u32), (0x6u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x768u32);
                m.w8(esp.wrapping_add(0x14u32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x680u32);
                m.w8(esp.wrapping_add(0x14u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x678u32);
                m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esi.wrapping_add(0x58cu32);
                m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0x4a4u32);
                m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                ecx = esi.wrapping_add(0xf0u32);
                m.w8(esp.wrapping_add(0x14u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x41c3a0, ecx, &a); }
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

/// `0X00470430` choice_list_dialog_release_embedded_controls
pub fn f_00470430(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    ecx = esi.wrapping_add(0xf0u32);
    { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
    eax = m.r32(esi.wrapping_add(0x4a4u32));
    ecx = esi.wrapping_add(0x4a4u32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(esi.wrapping_add(0x58cu32));
    ecx = esi.wrapping_add(0x58cu32);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    { let a = []; return h.call_at(esp, m, 0x41c3e0, ecx, &a); }
    eax
}

/// `0X00470470` choice_list_dialog_close
pub fn f_00470470(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ebp = esi.wrapping_add(0xf8u32);
                edi = eax;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = ebp;
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xf0u32);
                { let a = []; eax = h.call_at(esp, m, 0x431060, ecx, &a); }
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x4a4u32));
                ebp = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x58cu32));
                ebp = esi.wrapping_add(0x58cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x938u32));
                ebp = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x768u32));
                ebp = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x680u32));
                ebp = esi.wrapping_add(0x680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x850u32));
                ebp = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0xa0u32)), ebx, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0xae0u32));
                ecx = esi.wrapping_add(0xae0u32);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(m.r32(esi.wrapping_add(0xa0u32)), ebx, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x41ccf0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(eax), ebx, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(eax, 0xffffffffu32);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa4u32))), 0x4u32, 16).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                m.w32(esi.wrapping_add(0xacu32), ebx);
                pc = 10; continue;
            }
            8 => {
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
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
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_add(0x4);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xc0u32), ebx);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0xb0u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb0u32), ebx);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xbcu32), ebx);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                pc = 21; continue;
            }
            21 => {
                m.w16(esi.wrapping_add(0xaau32), ((ebx & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0xa8u32), ((ebx & 0xffff)) as u16);
                m.w16(esi.wrapping_add(0xa6u32), ((ebx & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0xecu32), ebx);
                m.w32(esi.wrapping_add(0xad8u32), 0xffffffffu32);
                m.w8(esi.wrapping_add(0xadcu32), ((ebx & 0xff)) as u8);
                pc = 22; continue;
            }
            22 => {
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

/// `0X004706A0` choice_list_dialog_set_clip_rect_inner
pub fn f_004706a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x94u32)))) & 0xff);
    edx = m.r32(0x6cbaecu32);
    let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
    eax = (eax & !0xff) | ((r) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    ebx = m.r32(ecx.wrapping_add(0x18u32));
    let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    ebp = m.r32(ecx.wrapping_add(0x14u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(0x6cbb08u32);
    let r = eax & edx; fl = lift::logic(r, 32);
    eax = r;
    edx = m.r32(ecx.wrapping_add(0x30u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = eax;
    eax = m.r32(ecx.wrapping_add(0x2cu32));
    let (r, f) = lift::sub(eax, esi, 32); fl = f;
    eax = r;
    let (r, f) = lift::sub(edx, esi, 32); fl = f;
    edx = r;
    let (r, f) = lift::sub(eax, edi, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(edx, ebx, 32); fl = f;
    edx = r;
    let (r, f) = lift::add(eax, ebp, 32); fl = f;
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    eax = esi;
    edx = ((eax as i32) >> 31) as u32;
    let (r, f) = lift::sub(eax, edx, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ebp, esi, 32); fl = f;
    ebp = r;
    let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
    eax = r;
    let (r, f) = lift::add(eax, ebx, 32); fl = f;
    eax = r;
    let (r, f) = lift::add(ebp, edi, 32); fl = f;
    ebp = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebp);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bbaa0, ecx, &a); }
    esp = esp.wrapping_add(0x10);
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

/// `0X00470700` choice_list_dialog_load_layout
pub fn f_00470700(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f410u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(0x10);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    eax = m.r32(0x6c7cccu32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esp.wrapping_add(0x24u32), ebx);
    m.w32(esp.wrapping_add(0xcu32), eax);
    ecx = m.r32(esp.wrapping_add(0x30u32));
    edx = esp.wrapping_add(0xcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6b9774u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
    eax = m.r32(esp.wrapping_add(0x38u32));
    ecx = m.r32(esp.wrapping_add(0x18u32));
    esp = esp.wrapping_add(0xc);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6b9780u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6dd524u32;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
    esp = esp.wrapping_add(16);
    edx = m.r32(esp.wrapping_add(0x2cu32));
    m.w16(esi.wrapping_add(0xe6u32), ((eax & 0xffff)) as u16);
    eax = m.r32(esp.wrapping_add(0xcu32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6b9788u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = 0x6dd524u32;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    edx = esp.wrapping_add(0x34u32);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0xf8u32));
    ecx = esi.wrapping_add(0xf8u32);
    { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    let r = ecx ^ ecx; fl = lift::logic(r, 32);
    ecx = r;
    ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de59u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x4a4u32));
    ecx = esi.wrapping_add(0x4a4u32);
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x58cu32));
    ecx = esi.wrapping_add(0x58cu32);
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(0x66e104u32);
    m.w32(esi.wrapping_add(0x650u32), eax);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe6u32)))) & 0xffff);
    ecx = eax.wrapping_mul(8);
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    eax = esp.wrapping_add(0x10u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
    edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
    ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
    { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x14u32));
    let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
    ecx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w16(esi.wrapping_add(0xe8u32), ((ecx & 0xffff)) as u16);
    edx = esp.wrapping_add(0x34u32);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w32(esi.wrapping_add(0xacu32), ebx);
    m.w32(esi.wrapping_add(0xc0u32), ebx);
    m.w32(esi.wrapping_add(0xc8u32), ebx);
    m.w32(esi.wrapping_add(0xccu32), ebx);
    m.w16(esi.wrapping_add(0xaau32), ((ebx & 0xffff)) as u16);
    m.w16(esi.wrapping_add(0xa8u32), ((ebx & 0xffff)) as u16);
    m.w16(esi.wrapping_add(0xa6u32), ((ebx & 0xffff)) as u16);
    m.w32(esi.wrapping_add(0xecu32), ebx);
    m.w32(esi.wrapping_add(0xd0u32), ebx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0x938u32));
    edi = esi.wrapping_add(0x938u32);
    ecx = edi;
    { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    edx = m.r32(edi);
    eax = esi.wrapping_add(0xab8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xff000000u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d80, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    ecx = edi;
    m.w32(esi.wrapping_add(0xaa4u32), 0x10u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2e20, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6cbe00u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4cab50, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi.wrapping_add(0xaacu32), 0x1u32);
    let r = ecx ^ ecx; fl = lift::logic(r, 32);
    ecx = r;
    m.w32(edi.wrapping_add(0x170u32), 0x64u32);
    m.w8(edi.wrapping_add(0x179u32), (0x1u32) as u8);
    ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x680u32));
    edi = esi.wrapping_add(0x680u32);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(0x6b04fcu32);
    ecx = edi;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = ecx ^ ecx; fl = lift::logic(r, 32);
    ecx = r;
    ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x768u32));
    edi = esi.wrapping_add(0x768u32);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(0x6b0500u32);
    ecx = edi;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = ecx ^ ecx; fl = lift::logic(r, 32);
    ecx = r;
    ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66de5cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0x850u32));
    edi = esi.wrapping_add(0x850u32);
    ecx = edi;
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(0x69af30u32);
    ecx = edi;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x38u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6b9794u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0xae0u32));
    ecx = esi.wrapping_add(0xae0u32);
    { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esp.wrapping_add(0xcu32);
    m.w8(esp.wrapping_add(0x24u32), ((ebx & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esp.wrapping_add(0x2cu32);
    m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x1cu32));
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w32(0x0u32, ecx);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x1c);
    return eax;
    eax
}

/// `0X004709F0` choice_list_dialog_slot3_release_arg_return_zero
pub fn f_004709f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    ecx = esp.wrapping_add(0x4u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
    eax = (eax & !0xff) | ((r) & 0xff);
    return eax;
    eax
}

/// `0X00470A00` choice_list_dialog_update
pub fn f_00470a00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x65f425u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x13c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0xf0u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0x14u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe0u32)))) & 0xffff);
                edi = m.r32(esi.wrapping_add(0x18u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = edx.wrapping_add(eax);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xdau32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x34u32), ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe2u32)))) & 0xffff);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xe8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                let (r, f) = lift::imul(edi, ebp); fl = f;
                edi = r;
                ecx = edi.wrapping_add(ecx).wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 16; continue;
            }
            2 => {
                ecx = esi.wrapping_add(0x4a4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 18; continue;
            }
            4 => {
                edi = esi.wrapping_add(0x58cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xd4u32)), ebx, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0xd4u32), ebx);
                ecx = m.r32(0x6c10ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c10ecu32);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x471ea0, ecx, &a); }
                pc = 12; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                m.w32(esi.wrapping_add(0xd4u32), 0x1u32);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c1190u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                eax = m.r32(edi);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c1190u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa4u32)))) & 0xffff);
                ecx = esi;
                fl = lift::sub((eax & 0xffff), (ebx & 0xffff), 16).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                { let a = []; eax = h.call_at(esp, m, 0x471f10, ecx, &a); }
                pc = 12; continue;
            }
            9 => {
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                { let a = []; eax = h.call_at(esp, m, 0x471ff0, ecx, &a); }
                pc = 12; continue;
            }
            11 => {
                { let a = []; eax = h.call_at(esp, m, 0x4720d0, ecx, &a); }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 17; continue;
            }
            13 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xd8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esi.wrapping_add(0xdau32))), 16).1;
                if !fl.cf { pc = 18; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x3u32, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x40u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 16; continue;
            }
            16 => {
                m.w16(esi.wrapping_add(0xd8u32), (0xffffu32) as u16);
                pc = 17; continue;
            }
            17 => {
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                ecx = esi.wrapping_add(0x938u32);
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x678u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edi = esi.wrapping_add(0x678u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x15cu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 26; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = esp.wrapping_add(0x10u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esi.wrapping_add(0xacu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(u32::from(m.r16(esi.wrapping_add(0xa6u32))), (ebx & 0xffff), 16).1;
                if (fl.cf || fl.zf) { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(ebp.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 22; continue;
            }
            22 => {
                pc = 24; continue;
            }
            23 => {
                m.w8(esi.wrapping_add(0x67cu32), (0x1u32) as u8);
                pc = 24; continue;
            }
            24 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0xa6u32)))) & 0xffff);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                m.w8(esi.wrapping_add(0x67cu32), ((ebx & 0xff)) as u8);
                pc = 26; continue;
            }
            26 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x67cu32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xadcu32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 30; continue;
            }
            29 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(esi.wrapping_add(0x680u32));
                ebp = esi.wrapping_add(0x680u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x938u32);
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                eax = m.r32(eax);
                fl = lift::sub(m.r32(eax.wrapping_add(0xfffffff8u32)), ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 34; continue; }
                pc = 31; continue;
            }
            31 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xadcu32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 34; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x674u32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 35; continue;
            }
            34 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x67cu32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xadcu32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 39; continue;
            }
            38 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(esi.wrapping_add(0x850u32));
                edi = esi.wrapping_add(0x850u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 45; continue; }
                pc = 40; continue;
            }
            40 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                m.w32(eax, 0x1u32);
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                ecx = esi.wrapping_add(0x938u32);
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xccu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 44; continue;
            }
            44 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 45; continue;
            }
            45 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                edx = m.r32(esi);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x4a4u32));
                ecx = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x938u32));
                ecx = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x699948u32);
                edx = m.r32(0x699950u32);
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = m.r32(0x6b0508u32);
                m.w32(esp.wrapping_add(0x48u32), edx);
                edx = esp.wrapping_add(0x4cu32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x28u32), (0xffu32) as u8);
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                m.w16(esp.wrapping_add(0x32u32), ((ebx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w8(esp.wrapping_add(0x2bu32), (0xffu32) as u8);
                m.w8(esp.wrapping_add(0x29u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x2au32), ((ebx & 0xff)) as u8);
                m.w16(esp.wrapping_add(0x22u32), (0x2u32) as u16);
                m.w32(esp.wrapping_add(0x24u32), eax);
                m.w32(esp.wrapping_add(0x34u32), 0x472470u32);
                m.w32(esp.wrapping_add(0x2cu32), 0x3ecccccdu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = esp.wrapping_add(0x14u32);
                eax = esp.wrapping_add(0x4cu32);
                m.w32(esp.wrapping_add(0x30u32), esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xae0u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41d9b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0xadcu32), (0x1u32) as u8);
                pc = 47; continue;
            }
            47 => {
                ecx = esi.wrapping_add(0x768u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 55; continue; }
                pc = 48; continue;
            }
            48 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x67cu32)))) & 0xff);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 50; continue; }
                pc = 49; continue;
            }
            49 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x768u32));
                ecx = esi.wrapping_add(0x768u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x4a4u32));
                ecx = esi.wrapping_add(0x4a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x938u32));
                ecx = esi.wrapping_add(0x938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x699948u32);
                edx = m.r32(0x699950u32);
                m.w32(esp.wrapping_add(0x44u32), ecx);
                ecx = m.r32(0x6b0504u32);
                eax = (eax & !0xff) | ((0xffu32) & 0xff);
                m.w32(esp.wrapping_add(0x48u32), edx);
                m.w8(esp.wrapping_add(0x20u32), ((eax & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x23u32), ((eax & 0xff)) as u8);
                edx = esp.wrapping_add(0x4cu32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                m.w16(esp.wrapping_add(0x32u32), ((ebx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w8(esp.wrapping_add(0x29u32), ((ebx & 0xff)) as u8);
                m.w8(esp.wrapping_add(0x2au32), ((ebx & 0xff)) as u8);
                m.w16(esp.wrapping_add(0x22u32), (0x2u32) as u16);
                m.w32(esp.wrapping_add(0x24u32), eax);
                m.w32(esp.wrapping_add(0x34u32), 0x470f80u32);
                m.w32(esp.wrapping_add(0x2cu32), 0x3ecccccdu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d410, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = esp.wrapping_add(0x14u32);
                eax = esp.wrapping_add(0x4cu32);
                m.w32(esp.wrapping_add(0x30u32), esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0xae0u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x41d9b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0xadcu32), (0x1u32) as u8);
                pc = 55; continue;
            }
            50 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 52; continue; }
                pc = 51; continue;
            }
            51 => {
                m.w32(eax, 0x2u32);
                pc = 52; continue;
            }
            52 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xccu32)), ebx, 32).1;
                if fl.zf { pc = 54; continue; }
                pc = 53; continue;
            }
            53 => {
                ecx = esi.wrapping_add(0x938u32);
                { let a = []; eax = h.call_at(esp, m, 0x4d2ec0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xccu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 54; continue;
            }
            54 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 55; continue;
            }
            55 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x154u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14cu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x148);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00471050` choice_list_dialog_noop_slot54
pub fn f_00471050(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    return eax;
    eax
}

/// `0X00471FF0` choice_list_dialog_sort_towns_by_name
pub fn f_00471ff0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0xc);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(0xa6u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(eax & eax, 32);
                if (fl.cf || fl.zf) { pc = 15; continue; }
                pc = 1; continue;
            }
            1 => {
                pc = 3; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = 0x0u32;
                m.w32(esp.wrapping_add(0x18u32), eax);
                if fl.zf { pc = 15; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(ecx.wrapping_add(0xacu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                ebp = ebx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x1u32)))) & 0xff);
                edi = m.r32(ebp.wrapping_mul(4).wrapping_add(0x6dda00u32));
                esi = m.r32(ebx.wrapping_mul(4).wrapping_add(0x6dda00u32));
                edx = edi;
                let (r, f) = lift::sub(esi, edx, 32); fl = f;
                esi = r;
                pc = 5; continue;
            }
            5 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(edi)))) & 0xff);
                fl = lift::sub((edx & 0xff), (ebx & 0xff), 8).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 6; continue;
            }
            6 => {
                if (!fl.zf && fl.sf == fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                pc = 5; continue;
            }
            9 => {
                edx = m.r32(ecx.wrapping_add(0xc0u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32));
                esi = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                m.w32(edx.wrapping_add(eax.wrapping_mul(4)), edi);
                edx = m.r32(ecx.wrapping_add(0xc0u32));
                m.w32(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x4u32), esi);
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx.wrapping_add(0xacu32));
                m.w32(esp.wrapping_add(0x18u32), 0x1u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                m.w8(esp.wrapping_add(0x13u32), ((ebx & 0xff)) as u8);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x1u32)))) & 0xff);
                m.w8(edx, ((ebx & 0xff)) as u8);
                edx = m.r32(ecx.wrapping_add(0xacu32));
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                m.w8(edx.wrapping_add(eax).wrapping_add(0x1u32), ((ebx & 0xff)) as u8);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 4; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = edx;
                fl = lift::logic(eax & eax, 32);
                if (!fl.cf && !fl.zf) { pc = 2; continue; }
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
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004721F0` choice_list_dialog_scan_auto_route_files
pub fn f_004721f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65f47fu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x40);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = ecx;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w16(ebx.wrapping_add(0xa6u32), ((edi & 0xffff)) as u16);
                { let a = []; eax = h.call_at(esp, m, 0x638f07, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b97b4u32);
                ecx = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x5cu32), edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x60u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x6391ea, ecx, &a); }
                let (r, f) = lift::add(u32::from(m.r16(ebx.wrapping_add(0xa6u32))), 1, 16); fl = lift::Fl { cf: fl.cf, ..f };
                m.w16(ebx.wrapping_add(0xa6u32), (r) as u16);
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ebx.wrapping_add(0xacu32));
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(eax.wrapping_add(0xfffffffcu32));
                esi = eax.wrapping_add(0xfffffffcu32);
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
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                esi = (esi & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0xa6u32)))) & 0xffff);
                edx = esi.wrapping_mul(4).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x30u32), eax);
                fl = lift::sub(eax, edi, 32).1;
                m.w8(esp.wrapping_add(0x58u32), (0x2u32) as u8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4103c0u32);
                ebp = eax.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w32(eax, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                m.w32(esp.wrapping_add(0x10u32), ebp);
                pc = 7; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x10u32), edi);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x60u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, edi, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                let r = (edx & 0xffff) ^ (edx & 0xffff); fl = lift::logic(r, 16);
                edx = (edx & !0xffff) | ((r) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(ebx.wrapping_add(0xa6u32))), 16).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::logic(eax & ecx, 32);
                if fl.zf { pc = 20; continue; }
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(esp.wrapping_add(0x10u32));
                pc = 9; continue;
            }
            9 => {
                ecx = esp.wrapping_add(0x34u32);
                { let a = []; eax = h.call_at(esp, m, 0x6391ea, ecx, &a); }
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x30u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x38u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639351, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                m.w8(esp.wrapping_add(0x5cu32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x28u32);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0x28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                m.w8(esp.wrapping_add(0x58u32), (0x6u32) as u8);
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(esp.wrapping_add(0x10u32));
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(esi, edi, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 13; continue;
            }
            13 => {
                pc = 18; continue;
            }
            14 => {
                fl = lift::sub(edi, esi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                ebx = edi;
                ecx = ebp;
                let (r, f) = lift::sub(ebx, esi, 32); fl = f;
                ebx = r;
                pc = 16; continue;
            }
            16 => {
                eax = ecx.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 16; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edx.wrapping_add(esi.wrapping_mul(4));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                ebx = m.r32(esp.wrapping_add(0x20u32));
                pc = 19; continue;
            }
            19 => {
                ecx = esp.wrapping_add(0x14u32);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                m.w8(esp.wrapping_add(0x58u32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x58u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                esi = m.r32(esp.wrapping_add(0x30u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(esi & esi, 32);
                eax = (eax & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                let r = (ecx & 0xffff) ^ (ecx & 0xffff); fl = lift::logic(r, 16);
                ecx = (ecx & !0xffff) | ((r) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(ebx.wrapping_add(0xa6u32))), 16).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                fl = lift::logic(edx & eax, 32);
                if !fl.zf { pc = 9; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = esp.wrapping_add(0x1cu32);
                m.w32(ebx.wrapping_add(0xacu32), eax);
                m.w8(esp.wrapping_add(0x58u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x58u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x638f44, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x4c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00472536` choice_list_dialog_on_delete_answer_tail
pub fn f_00472536(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
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
    return eax;
    eax
}

/// `0X00472630` choice_list_dialog_scalar_deleting_dtor
pub fn f_00472630(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x46f860, ecx, &a); }
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

/// `0X004C5D50` keyframe_timer_ctor
pub fn f_004c5d50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6620dbu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6c7cccu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esp.wrapping_add(0x4u32), esi);
    ecx = esi.wrapping_add(0x24u32);
    m.w32(ecx, eax);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6dd5bau32);
    m.w32(esp.wrapping_add(0x14u32), eax);
    m.w32(esi, 0x6714a4u32);
    m.w32(esi.wrapping_add(0x10u32), 0xffffffffu32);
    m.w32(esi.wrapping_add(0x4u32), eax);
    m.w32(esi.wrapping_add(0x8u32), eax);
    m.w32(esi.wrapping_add(0xcu32), eax);
    m.w32(esi.wrapping_add(0x14u32), eax);
    m.w32(esi.wrapping_add(0x18u32), eax);
    m.w32(esi.wrapping_add(0x1cu32), eax);
    m.w8(esi.wrapping_add(0x20u32), ((eax & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x21u32), ((eax & 0xff)) as u8);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004C5DC0` keyframe_timer_dtor
pub fn f_004c5dc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6620f0u32);
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
                m.w32(esi, 0x6714a4u32);
                eax = m.r32(esi.wrapping_add(0x4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
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
                m.w32(esi.wrapping_add(0x10u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                m.w32(esi.wrapping_add(0xcu32), ebx);
                m.w32(esi.wrapping_add(0x14u32), ebx);
                m.w32(esi.wrapping_add(0x18u32), ebx);
                m.w32(esi.wrapping_add(0x1cu32), ebx);
                m.w8(esi.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x21u32), ((ebx & 0xff)) as u8);
                let (r, f) = lift::add(esi, 0x24u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5bau32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
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

/// `0X004C6150` keyframe_timer_clear
pub fn f_004c6150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5bau32);
                ecx = esi.wrapping_add(0x24u32);
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

/// `0X004C61A0` keyframe_timer_set_default
pub fn f_004c61a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c6210, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x447a0000u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c6210, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6beee0u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x10u32), ebx);
                edi = m.r32(esp);
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

/// `0X004C6750` keyframe_timer_get_id
pub fn f_004c6750(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0x10u32));
    return eax;
    eax
}

/// `0X004C6780` keyframe_timer_get_ini_path
pub fn f_004c6780(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0xcu32));
    let (r, f) = lift::add(ecx, 0x24u32, 32); fl = f;
    ecx = r;
    m.w32(esp.wrapping_add(0x4u32), 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C67A0` keyframe_timer_is_running
pub fn f_004c67a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(ecx.wrapping_add(0x10u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x20u32)))) & 0xff);
                return eax;
            }
            3 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C67C0` keyframe_timer_get_time_count
pub fn f_004c67c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0x8u32));
    return eax;
    eax
}

/// `0X004C67D0` keyframe_timer_copy_to
pub fn f_004c67d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = m.r32(esp.wrapping_add(0xcu32));
    eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x21u32)))) & 0xff);
    m.w8(edi.wrapping_add(0x21u32), ((eax & 0xff)) as u8);
    ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x20u32)))) & 0xff);
    m.w8(edi.wrapping_add(0x20u32), ((ecx & 0xff)) as u8);
    edx = m.r32(esi.wrapping_add(0x1cu32));
    m.w32(edi.wrapping_add(0x1cu32), edx);
    eax = m.r32(esi.wrapping_add(0x14u32));
    m.w32(edi.wrapping_add(0x14u32), eax);
    ecx = m.r32(esi.wrapping_add(0x18u32));
    m.w32(edi.wrapping_add(0x18u32), ecx);
    edx = m.r32(esi.wrapping_add(0x10u32));
    m.w32(edi.wrapping_add(0x10u32), edx);
    eax = m.r32(esi.wrapping_add(0x24u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = edi.wrapping_add(0x24u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6bf644u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x8u32));
    m.w32(edi.wrapping_add(0x8u32), edx);
    eax = m.r32(esi.wrapping_add(0xcu32));
    m.w32(edi.wrapping_add(0xcu32), eax);
    ecx = m.r32(esi.wrapping_add(0x8u32));
    let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
    ecx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
    m.w32(edi.wrapping_add(0x4u32), eax);
    edx = m.r32(esi.wrapping_add(0x8u32));
    ecx = m.r32(esi.wrapping_add(0x4u32));
    let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
    edx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
    esp = esp.wrapping_add(0x1c);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C6870` keyframe_timer_scalar_deleting_dtor
pub fn f_004c6870(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4c5dc0, ecx, &a); }
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

/// `0X004CD9F0` viper_list_box_select_row
pub fn f_004cd9f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x10u32));
                esi = ecx;
                fl = lift::logic(edi & edi, 32);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 5; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edi = eax.wrapping_add(0xffffffffu32);
                pc = 5; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x10u32), edi);
                pc = 6; continue;
            }
            6 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x310u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x20cu32));
                ecx = m.r32(esi.wrapping_add(0x208u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ce100, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0x224u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic(ecx & ecx, 32);
                m.w32(esi.wrapping_add(0x208u32), edi);
                m.w32(esp.wrapping_add(0xcu32), edx);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4cd640, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 11; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x310u32)))) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x208u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x9cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 21; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(eax);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4d2df0, ecx, &a); }
                pc = 23; continue;
            }
            21 => {
                eax = m.r32(esi.wrapping_add(0x200u32));
                pc = 23; continue;
            }
            22 => {
                eax = m.r32(esi.wrapping_add(0x20cu32));
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esi.wrapping_add(0x208u32));
                m.w32(esi.wrapping_add(0x20cu32), eax);
                eax = m.r32(esi.wrapping_add(0x204u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ce100, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x10u32));
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

/// `0X004CDC70` viper_list_box_compare_rows_by_text
pub fn f_004cdc70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6625e6u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = m.r32(eax);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                ecx = m.r32(ecx);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(eax);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = eax;
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(edi, 0x8u32, 32); fl = f;
                edi = r;
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(esi, 0x8u32, 32); fl = f;
                esi = r;
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x639cf0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = esp.wrapping_add(0x1cu32);
                esi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xcu32));
                eax = esi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CDDC0` viper_list_box_remove_all
pub fn f_004cddc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if fl.sf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4cc8b0, ecx, &a); }
                esp = esp.wrapping_add(8);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.sf { pc = 1; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if fl.sf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4cccf0, ecx, &a); }
                esp = esp.wrapping_add(8);
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                if !fl.sf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                ebx = esi.wrapping_add(0xb4u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9e60, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9f90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x230u32));
                m.w32(esp.wrapping_add(0xcu32), edi);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 33; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                fl = lift::sub(m.r32(esi.wrapping_add(0x9cu32)), eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                if !fl.sf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, edi, 32).1;
                m.w32(esp.wrapping_add(0x14u32), edi);
                m.w32(esp.wrapping_add(0x18u32), edi);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                if (fl.zf || fl.sf != fl.of) { pc = 30; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x215u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0xb0u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 12; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x18u32), edi);
                pc = 12; continue;
            }
            12 => {
                ecx = esi.wrapping_add(0xb4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x10u32));
                ebx = eax;
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ebx = eax;
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esi.wrapping_add(0xb0u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ebx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 24; continue;
            }
            24 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x9cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 29; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 28; continue;
            }
            28 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 25; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edi = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(esp.wrapping_add(0x14u32), edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x230u32));
                fl = lift::sub(ecx, eax, 32).1;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                m.w8(esi.wrapping_add(0x313u32), (0x1u32) as u8);
                pc = 32; continue;
            }
            32 => {
                m.w32(esi.wrapping_add(0x230u32), eax);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 33; continue;
            }
            33 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4cd9f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                m.w32(esi.wrapping_add(0x230u32), edi);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CE490` viper_list_box_on_scroll_position_changed
pub fn f_004ce490(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b9ec0, ecx, &a); }
                edi = esi.wrapping_add(0xffffff6cu32);
                ebx = eax;
                m.w32(esp.wrapping_add(0xcu32), ebx);
                fl = lift::sub(ebx, m.r32(edi.wrapping_add(0x230u32)), 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 1; continue;
            }
            1 => {
                esi = edi.wrapping_add(0xb4u32);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                fl = lift::sub(m.r32(edi.wrapping_add(0x9cu32)), eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(edi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                ebx = ecx;
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(edi.wrapping_add(0xa4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, esi, 32).1;
                m.w32(esp.wrapping_add(0x14u32), esi);
                m.w32(esp.wrapping_add(0x18u32), esi);
                m.w32(esp.wrapping_add(0x1cu32), esi);
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 5; continue;
            }
            5 => {
                pc = 7; continue;
            }
            6 => {
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x215u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(edi.wrapping_add(0xb0u32));
                m.w32(esp.wrapping_add(0x18u32), edx);
                pc = 10; continue;
            }
            9 => {
                m.w32(esp.wrapping_add(0x18u32), esi);
                pc = 10; continue;
            }
            10 => {
                ecx = edi.wrapping_add(0xb4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x10u32));
                ebx = eax;
                eax = m.r32(edi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ebx = eax;
                pc = 12; continue;
            }
            12 => {
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(edi.wrapping_add(0x98u32));
                eax = m.r32(eax.wrapping_add(esi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x10u32));
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 13; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(esi, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(edi.wrapping_add(0x98u32));
                eax = m.r32(eax.wrapping_add(esi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = m.r32(ecx.wrapping_add(esi.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(edi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(esi.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(ebp.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(edi.wrapping_add(0xb0u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, ebx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(esi, m.r32(edi.wrapping_add(0x9cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(edi.wrapping_add(0x98u32));
                eax = m.r32(edx.wrapping_add(esi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(eax.wrapping_add(ebp.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(edi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(edi.wrapping_add(0xacu32));
                esi = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(edi.wrapping_add(0xa4u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                let (r, f) = lift::add(esi, edx, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), esi);
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 28; continue;
            }
            28 => {
                ebx = m.r32(esp.wrapping_add(0x10u32));
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(edi.wrapping_add(0x230u32));
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                m.w8(edi.wrapping_add(0x313u32), (0x1u32) as u8);
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(edi);
                ecx = edi;
                m.w32(edi.wrapping_add(0x230u32), ebx);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 32; continue;
            }
            32 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CE680` viper_list_box_get_class_name
pub fn f_004ce680(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfc10u32);
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

/// `0X004CE6B0` viper_list_box_scalar_deleting_dtor
pub fn f_004ce6b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4cafe0, ecx, &a); }
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

/// `0X004CE820` viper_list_box_set_size_from_rect
pub fn f_004ce820(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x20u32));
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edi.wrapping_add(0xcu32));
                eax = m.r32(edi.wrapping_add(0x8u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi.wrapping_add(0xb4u32));
                let (r, f) = lift::add(esi, 0xb4u32, 32); fl = f;
                esi = r;
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x50u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CF5D0` viper_slider_ctor
pub fn f_004cf5d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66277fu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfefcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4acdd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esp.wrapping_add(0x24u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x244u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x330u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    m.w32(esi, 0x671c48u32);
    { let a = []; eax = h.call_at(esp, m, 0x4cf7a0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004CF670` viper_slider_ctor_named
pub fn f_004cf670(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6627c8u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(esp.wrapping_add(0x14u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4acdd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esp.wrapping_add(0x24u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x244u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x330u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    m.w32(esi, 0x671c48u32);
    { let a = []; eax = h.call_at(esp, m, 0x4cf7a0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004CF710` viper_slider_dtor
pub fn f_004cf710(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662803u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esp.wrapping_add(0x4u32), esi);
    m.w32(esi, 0x671c48u32);
    ecx = esi.wrapping_add(0x330u32);
    m.w32(esp.wrapping_add(0x10u32), 0x2u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
    ecx = esi.wrapping_add(0x244u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    eax = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x20u32), (0x0u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1640, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004CF7A0` viper_slider_reset
pub fn f_004cf7a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = esi.wrapping_add(0x94u32);
    { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
    ecx = esi.wrapping_add(0x16cu32);
    { let a = []; eax = h.call_at(esp, m, 0x4acea0, ecx, &a); }
    eax = m.r32(esi.wrapping_add(0x244u32));
    ecx = esi.wrapping_add(0x244u32);
    { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esi.wrapping_add(0x330u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b0540, ecx, &a); }
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xcu32);
    ecx = esi.wrapping_add(0x35cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esi.wrapping_add(0x350u32), ebx);
    m.w32(esi.wrapping_add(0x354u32), 0x1u32);
    m.w32(esi.wrapping_add(0x358u32), ebx);
    { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xcu32);
    edx = esi.wrapping_add(0x368u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
    eax = 0xbf800000u32;
    esp = esp.wrapping_add(0x18);
    m.w32(esi.wrapping_add(0x37cu32), eax);
    m.w32(esi.wrapping_add(0x378u32), eax);
    m.w32(esi.wrapping_add(0x374u32), eax);
    eax = 0xff000000u32;
    m.w32(esi.wrapping_add(0x388u32), ebx);
    m.w32(esi.wrapping_add(0x384u32), ebx);
    m.w32(esi.wrapping_add(0x380u32), ebx);
    m.w8(esi.wrapping_add(0x38cu32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x38du32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x390u32), ebx);
    m.w8(esi.wrapping_add(0x394u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x395u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x398u32), eax);
    m.w32(esi.wrapping_add(0x39cu32), 0xffffffffu32);
    m.w32(esi.wrapping_add(0x3a8u32), eax);
    m.w32(esi.wrapping_add(0x3acu32), eax);
    m.w32(esi.wrapping_add(0x3a0u32), ebx);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004CFC50` viper_slider_load_config
pub fn f_004cfc50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662825u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x38);
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
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x50u32), ebp);
                m.w32(esp.wrapping_add(0x14u32), eax);
                edi = m.r32(esp.wrapping_add(0x5cu32));
                m.w8(esp.wrapping_add(0x50u32), (0x1u32) as u8);
                fl = lift::sub(edi, ebp, 32).1;
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(edi, 0x7fffffffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x5cu32), ebp);
                edi = ebp;
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4cf7a0, ecx, &a); }
                ecx = m.r32(0x6bfe04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x64u32));
                ecx = m.r32(0x6bfe10u32);
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(0x6bfe1cu32);
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x58u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd602u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x58u32));
                ecx = m.r32(0x6bfe28u32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(0x6bfe34u32);
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x58u32));
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
                edi = 0x6dbb58u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd603u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edi = eax;
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x16cu32));
                ecx = esi.wrapping_add(0x16cu32);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x58u32));
                ecx = m.r32(0x6bfe40u32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(0x6bfe50u32);
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x58u32));
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
                edi = 0x6dbb58u32;
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
                m.w32(esp, 0x6dd604u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edi = eax;
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x244u32));
                edi = esi.wrapping_add(0x244u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(edi);
                ecx = esi.wrapping_add(0x330u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x58u32));
                eax = m.r32(0x6dd5fcu32);
                ecx = m.r32(0x6bfe5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(0x6c7cccu32);
                m.w8(esp.wrapping_add(0x1cu32), (0x0u32) as u8);
                m.w32(esp.wrapping_add(0x18u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                m.w8(esp.wrapping_add(0x17u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e1e0, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6bfdbcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0x1cu32), ((ebx & 0xff)) as u8);
                pc = 19; continue;
            }
            19 => {
                let (r, f) = lift::add((ebx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ebx & 0xff), 0x3u32, 8).1;
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                if fl.cf { pc = 16; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x1cu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x38cu32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                if fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 27; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w32(esi.wrapping_add(0x3acu32), 0xff000000u32);
                pc = 26; continue;
            }
            24 => {
                m.w32(esi.wrapping_add(0x3a8u32), 0xff000000u32);
                m.w32(esi.wrapping_add(0x3acu32), ebp);
                pc = 27; continue;
            }
            25 => {
                m.w32(esi.wrapping_add(0x3acu32), ebp);
                pc = 26; continue;
            }
            26 => {
                m.w32(esi.wrapping_add(0x3a8u32), ebp);
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x58u32));
                edx = m.r32(0x6dd5f8u32);
                eax = m.r32(0x6bfe6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x20u32), (0x0u32) as u8);
                m.w8(esp.wrapping_add(0x17u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e1e0, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 32; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = m.r32(edx.wrapping_mul(4).wrapping_add(0x6bfde8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                m.w8(esp.wrapping_add(0x13u32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0x1cu32), ((ebx & 0xff)) as u8);
                pc = 31; continue;
            }
            31 => {
                let (r, f) = lift::add((ebx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ebx & 0xff), 0x4u32, 8).1;
                m.w8(esp.wrapping_add(0x20u32), ((ebx & 0xff)) as u8);
                if fl.cf { pc = 28; continue; }
                pc = 32; continue;
            }
            32 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x1cu32)))) & 0xff);
                edx = m.r32(esp.wrapping_add(0x58u32));
                m.w8(esi.wrapping_add(0x38du32), ((ecx & 0xff)) as u8);
                eax = m.r32(0x6bfe90u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x58u32));
                m.w32(esi.wrapping_add(0x390u32), eax);
                eax = m.r32(0x6bfea8u32);
                ecx = m.r32(0x6bfea0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = esi.wrapping_add(0x384u32);
                ecx = esi.wrapping_add(0x380u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x68u32));
                eax = m.r32(0x6dd5f4u32);
                ecx = m.r32(0x6bfe78u32);
                esp = esp.wrapping_add(0x10);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0x398u32), eax);
                eax = m.r32(esp.wrapping_add(0x58u32));
                ecx = m.r32(0x6dd5f0u32);
                edx = m.r32(0x6bfe84u32);
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
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x58u32));
                m.w32(esi.wrapping_add(0x39cu32), eax);
                edx = m.r32(0x6bfebcu32);
                eax = m.r32(0x6bfeb4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = esp.wrapping_add(0x34u32);
                eax = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff08u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esp.wrapping_add(0x40u32), ebp);
                m.w32(esp.wrapping_add(0x44u32), ebp);
                m.w32(esp.wrapping_add(0x48u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x10);
                eax = esp.wrapping_add(0x30u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x58u32));
                edx = m.r32(0x6bfeccu32);
                eax = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(0x6bfec4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                edx = esp.wrapping_add(0x2cu32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 34; continue;
            }
            34 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w32(esp.wrapping_add(0x40u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x48u32), ebp);
                { let t = m.r32(eax.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x58u32));
                eax = m.r32(0x6bfee8u32);
                ecx = m.r32(0x6bfedcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = esi.wrapping_add(0x36cu32);
                ecx = esi.wrapping_add(0x368u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esi.wrapping_add(0x360u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esi.wrapping_add(0x35cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bff18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x18);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x50u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x50u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x58u32);
                m.w32(esp.wrapping_add(0x50u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x48u32));
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
                esp = esp.wrapping_add(0x44);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D0280` viper_slider_slot3_stub
pub fn f_004d0280(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    ecx = esp.wrapping_add(0x4u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
    eax = (eax & !0xff) | ((r) & 0xff);
    return eax;
    eax
}

/// `0X004D0B50` viper_slider_set_max
pub fn f_004d0b50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0x354u32), eax);
    return eax;
    eax
}

/// `0X004D0B60` viper_slider_set_min
pub fn f_004d0b60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0x350u32), eax);
    return eax;
    eax
}

/// `0X004D0F70` viper_slider_get_class_name
pub fn f_004d0f70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfefcu32);
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

/// `0X004D0FA0` viper_slider_scalar_deleting_dtor
pub fn f_004d0fa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4cf710, ecx, &a); }
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

/// `0X004D1130` state_switch_reset
pub fn f_004d1130(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xa4u32)), ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                esp = esp.wrapping_add(0x30);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                m.w16(esi.wrapping_add(0xa8u32), ((edi & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0xacu32), ebx);
                m.w32(esi.wrapping_add(0xb0u32), 0xffffffffu32);
                edi = m.r32(esp);
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

/// `0X004D1220` state_switch_init_from_arrays
pub fn f_004d1220(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4d1130, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x30u32)))) & 0xffff);
                ebx = m.r32(esp.wrapping_add(0x40u32));
                ebp = m.r32(esp.wrapping_add(0x38u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w16(esi.wrapping_add(0xa8u32), ((ecx & 0xffff)) as u16);
                fl = lift::sub(eax, edi, 32).1;
                eax = m.r32(esp.wrapping_add(0x3cu32));
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(ebx, edi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(ebp, edi, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esp.wrapping_add(0x34u32), edi);
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w8(esp.wrapping_add(0x30u32), (0x1u32) as u8);
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 8; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                pc = 8; continue;
            }
            8 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(eax.wrapping_add(ebp));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d18e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w8(esp.wrapping_add(0x30u32), ((eax & 0xff)) as u8);
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d1b10, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(esp.wrapping_add(0x30u32), ((eax & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebx, 0x20u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X004D1300` state_switch_load_from_config
pub fn f_004d1300(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6628aeu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x24);
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
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x3cu32), edi);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4d1130, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                fl = lift::sub(m.r32(ecx.wrapping_add(0xfffffff8u32)), 0x5u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6beee0u32);
                ecx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x48u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x48u32), edi);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x48u32));
                ecx = m.r32(0x6bff40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(0x6bff54u32);
                edx = m.r32(0x6bff4cu32);
                esp = esp.wrapping_add(0xc);
                ebx = 0x1u32;
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
                m.w32(esp.wrapping_add(0x40u32), ebx);
                m.w32(esp.wrapping_add(0x44u32), ebx);
                m.w32(esp.wrapping_add(0x48u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = esp.wrapping_add(0x2cu32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bffa4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                eax = m.r32(esi);
                esp = esp.wrapping_add(0x10);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(0x6bff64u32);
                ecx = m.r32(0x6bff5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x34u32), ebx);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = esp.wrapping_add(0x20u32);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x28u32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x34u32), edi);
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(0x6bff70u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebp = eax;
                eax = m.r32(0x6c7cccu32);
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                fl = lift::logic(ebp & ebp, 32);
                m.w8(esp.wrapping_add(0x3cu32), (0x3u32) as u8);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6bff7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(0x6bff88u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x5cu32));
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = eax;
                eax = m.r32(0x6beee0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bffb4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w8(esp.wrapping_add(0x44u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d18e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                ecx = esp.wrapping_add(0x24u32);
                ebx = (ebx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                m.w8(esp.wrapping_add(0x3cu32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ebp, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = esp.wrapping_add(0x10u32);
                edx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x44u32);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
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
                esp = esp.wrapping_add(0x30);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D1600` state_switch_slot3_release_string
pub fn f_004d1600(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    ecx = esp.wrapping_add(0x4u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    return eax;
    eax
}

/// `0X004D1710` state_switch_draw
pub fn f_004d1710(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp.wrapping_add(0x38u32));
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                eax = esp;
                ebp = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebx);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                edx = m.r32(0x670f20u32);
                m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esi);
                esp = esp.wrapping_add(0x10);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x98u32)), 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edx.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(0xc);
                esi = m.r32(esp.wrapping_add(0x4cu32));
                edx = esp;
                eax = m.r32(ecx);
                m.w32(edx, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), esi);
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D1840` state_switch_clear_states
pub fn f_004d1840(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::logic(eax & eax, 32);
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
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 4; continue;
            }
            4 => {
                m.w32(esi.wrapping_add(0x9cu32), 0x0u32);
                m.w32(esi.wrapping_add(0xa0u32), 0x0u32);
                m.w32(esi.wrapping_add(0xa4u32), 0x0u32);
                m.w32(esi.wrapping_add(0xb0u32), 0xffffffffu32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D18E0` state_switch_add_animation
pub fn f_004d18e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6628c3u32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                eax = ebx.wrapping_mul(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                edi = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                m.w32(esp.wrapping_add(0xcu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
                pc = 3; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)), eax);
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = esp;
                m.w32(esp.wrapping_add(0x2cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(edi.wrapping_add(ecx.wrapping_mul(4)));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(edi.wrapping_add(eax.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                m.w32(edi.wrapping_add(eax.wrapping_mul(4)), 0x0u32);
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x94u32), edi);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx.wrapping_add(0xffffffffu32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D19E0` state_switch_remove_animation_by_id
pub fn f_004d19e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ebx = edi.wrapping_mul(4);
                ecx = m.r32(ebx.wrapping_add(eax));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = eax.wrapping_mul(4).wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x94u32));
                ebp = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x10);
                ecx = eax.wrapping_add(0xffffffffu32);
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = edi.wrapping_add(0x1u32);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, ebp, 32); fl = f;
                ebx = r;
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = edx.wrapping_add(ecx.wrapping_mul(4));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = eax.wrapping_add(0xffffffffu32);
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(ecx, 0xffffffffu32, 32).1;
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esi);
                m.w32(esi.wrapping_add(0xb0u32), ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 19; continue;
            }
            19 => {
                ebx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0x94u32), ebp);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
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

/// `0X004D1B10` state_switch_add_state
pub fn f_004d1b10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esp.wrapping_add(0x14u32), edi);
                pc = 11; continue;
            }
            10 => {
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            13 => {
                edi = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                ebp = edi.wrapping_mul(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::shl(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                m.w32(ebx.wrapping_add(ecx.wrapping_mul(4)), edx);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(ebp.wrapping_add(eax.wrapping_mul(4)), ecx);
                edx = m.r32(esi.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x28);
                m.w32(esi.wrapping_add(0xa4u32), edi);
                eax = edi.wrapping_add(0xffffffffu32);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebp);
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

/// `0X004D1DF0` state_switch_find_animation_index_by_id
pub fn f_004d1df0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, ebp, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = edi.wrapping_add(0xffffffffu32);
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
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D1ED0` state_switch_get_class_name
pub fn f_004d1ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bffa0u32);
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

/// `0X004D1F00` state_switch_scalar_deleting_dtor
pub fn f_004d1f00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4d10e0, ecx, &a); }
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
        0x46f760 => f_0046f760(m, h, ecx, esp),
        0x46f860 => f_0046f860(m, h, ecx, esp),
        0x470430 => f_00470430(m, h, ecx, esp),
        0x470470 => f_00470470(m, h, ecx, esp),
        0x4706a0 => f_004706a0(m, h, ecx, esp),
        0x470700 => f_00470700(m, h, ecx, esp),
        0x4709f0 => f_004709f0(m, h, ecx, esp),
        0x470a00 => f_00470a00(m, h, ecx, esp),
        0x471050 => f_00471050(m, h, ecx, esp),
        0x471ff0 => f_00471ff0(m, h, ecx, esp),
        0x4721f0 => f_004721f0(m, h, ecx, esp),
        0x472536 => f_00472536(m, h, ecx, esp),
        0x472630 => f_00472630(m, h, ecx, esp),
        0x4c5d50 => f_004c5d50(m, h, ecx, esp),
        0x4c5dc0 => f_004c5dc0(m, h, ecx, esp),
        0x4c6150 => f_004c6150(m, h, ecx, esp),
        0x4c61a0 => f_004c61a0(m, h, ecx, esp),
        0x4c6750 => f_004c6750(m, h, ecx, esp),
        0x4c6780 => f_004c6780(m, h, ecx, esp),
        0x4c67a0 => f_004c67a0(m, h, ecx, esp),
        0x4c67c0 => f_004c67c0(m, h, ecx, esp),
        0x4c67d0 => f_004c67d0(m, h, ecx, esp),
        0x4c6870 => f_004c6870(m, h, ecx, esp),
        0x4cd9f0 => f_004cd9f0(m, h, ecx, esp),
        0x4cdc70 => f_004cdc70(m, h, ecx, esp),
        0x4cddc0 => f_004cddc0(m, h, ecx, esp),
        0x4ce490 => f_004ce490(m, h, ecx, esp),
        0x4ce680 => f_004ce680(m, h, ecx, esp),
        0x4ce6b0 => f_004ce6b0(m, h, ecx, esp),
        0x4ce820 => f_004ce820(m, h, ecx, esp),
        0x4cf5d0 => f_004cf5d0(m, h, ecx, esp),
        0x4cf670 => f_004cf670(m, h, ecx, esp),
        0x4cf710 => f_004cf710(m, h, ecx, esp),
        0x4cf7a0 => f_004cf7a0(m, h, ecx, esp),
        0x4cfc50 => f_004cfc50(m, h, ecx, esp),
        0x4d0280 => f_004d0280(m, h, ecx, esp),
        0x4d0b50 => f_004d0b50(m, h, ecx, esp),
        0x4d0b60 => f_004d0b60(m, h, ecx, esp),
        0x4d0f70 => f_004d0f70(m, h, ecx, esp),
        0x4d0fa0 => f_004d0fa0(m, h, ecx, esp),
        0x4d1130 => f_004d1130(m, h, ecx, esp),
        0x4d1220 => f_004d1220(m, h, ecx, esp),
        0x4d1300 => f_004d1300(m, h, ecx, esp),
        0x4d1600 => f_004d1600(m, h, ecx, esp),
        0x4d1710 => f_004d1710(m, h, ecx, esp),
        0x4d1840 => f_004d1840(m, h, ecx, esp),
        0x4d18e0 => f_004d18e0(m, h, ecx, esp),
        0x4d19e0 => f_004d19e0(m, h, ecx, esp),
        0x4d1b10 => f_004d1b10(m, h, ecx, esp),
        0x4d1df0 => f_004d1df0(m, h, ecx, esp),
        0x4d1ed0 => f_004d1ed0(m, h, ecx, esp),
        0x4d1f00 => f_004d1f00(m, h, ecx, esp),
        _ => return None,
    }))
}
