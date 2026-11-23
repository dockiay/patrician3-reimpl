//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004023E0` window_set_mode
pub fn f_004023e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub(ecx, 0x3u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic(ecx & ecx, 32);
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            4 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                m.w32(esi.wrapping_add(0x8u32), eax);
                m.w32(esi.wrapping_add(0x4u32), ecx);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1ea0, ecx, &a); }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x3cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            7 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00410340` window_set_clip_rect_inner
pub fn f_00410340(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X0041C320` dialog_bg_ctor_default
pub fn f_0041c320(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(0x695afcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462160, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi, 0x66bc90u32);
    m.w32(esi.wrapping_add(0xa0u32), eax);
    m.w32(esi.wrapping_add(0x98u32), eax);
    m.w32(esi.wrapping_add(0x9cu32), eax);
    m.w8(esi.wrapping_add(0x94u32), ((eax & 0xff)) as u8);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X0041C360` dialog_bg_ctor_named
pub fn f_0041c360(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462160, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi, 0x66bc90u32);
    m.w32(esi.wrapping_add(0xa0u32), eax);
    m.w32(esi.wrapping_add(0x98u32), eax);
    m.w32(esi.wrapping_add(0x9cu32), eax);
    m.w8(esi.wrapping_add(0x94u32), ((eax & 0xff)) as u8);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X0041C3A0` dialog_bg_dtor
pub fn f_0041c3a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esi, 0x66bc90u32);
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
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                m.w8(esi.wrapping_add(0x94u32), ((ebx & 0xff)) as u8);
                ecx = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x462180, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0041C3E0` dialog_bg_free_tile_map
pub fn f_0041c3e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xa0u32));
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
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                m.w8(esi.wrapping_add(0x94u32), ((ebx & 0xff)) as u8);
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

/// `0X0041C7B0` dialog_bg_load_texture_config
pub fn f_0041c7b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65c2f0u32);
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
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x28u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b0cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x24u32));
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x24u32));
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x695b58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(0x6cbb08u32, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6da820u32;
                m.w32(0x6cbb00u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = 0x6da820u32;
                m.w32(0x6cbaf4u32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(0x6cbaf8u32, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = 0x6da820u32;
                m.w32(0x6cbafcu32, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbaf8u32);
                m.w32(0x6cbb04u32, eax);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                edx = m.r32(ecx.wrapping_add(0x14u32));
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::logic(edx & edx, 32);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                edx = m.r32(ecx);
                m.w32(0x6cbac4u32, edx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(0x6cbac8u32, edx);
                edx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(0x6cbaccu32, edx);
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(0x6cbad0u32, ecx);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6cbafcu32);
                edx = m.r32(ecx.wrapping_add(0x14u32));
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx.wrapping_add(0xcu32));
                ecx = m.r32(edx);
                m.w32(0x6cbad4u32, ecx);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(0x6cbad8u32, ecx);
                ecx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(0x6cbadcu32, ecx);
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(0x6cbae0u32, edx);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                ecx = m.r32(eax);
                m.w32(0x6cbab0u32, ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(0x6cbab4u32, edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(0x6cbab8u32, ecx);
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(0x6cbabcu32, edx);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(0x6cbaf4u32);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                ecx = m.r32(eax);
                m.w32(0x6cbae4u32, ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(0x6cbae8u32, edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(0x6cbaecu32, ecx);
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(0x6cbaf0u32, edx);
                pc = 12; continue;
            }
            12 => {
                ecx = esp;
                m.w8(esp.wrapping_add(0xcu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0xcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0041CA00` dialog_bg_draw_shadows
pub fn f_0041ca00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(0x6cbb04u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    ecx = m.r32(eax.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
    esi = m.r32(esp.wrapping_add(0xcu32));
    edx = m.r32(0x6cbabcu32);
    eax = m.r32(0x6cbab8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = m.r32(esi.wrapping_add(0x4u32));
    edx = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
    eax = m.r32(0x6cbaf8u32);
    ecx = m.r32(eax.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
    edx = m.r32(0x6cbad0u32);
    eax = m.r32(0x6cbaccu32);
    ecx = m.r32(esi.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    edx = m.r32(esi.wrapping_add(0x8u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
    eax = m.r32(0x6cbafcu32);
    ecx = m.r32(eax.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
    eax = m.r32(0x6cbae0u32);
    edx = m.r32(0x6cbadcu32);
    ecx = m.r32(esi.wrapping_add(0xcu32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    edx = m.r32(esi);
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
    esp = esp.wrapping_add(0x54);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X0041CAA0` dialog_bg_draw
pub fn f_0041caa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x24);
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(eax);
                eax = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = m.r32(0x6cbaf4u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x94u32)))) & 0xff);
                esp = esp.wrapping_add(0x4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 17; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6cbaf4u32);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                ebx = m.r32(eax);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ebp = m.r32(eax.wrapping_add(0x8u32));
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x30u32), edx);
                pc = 5; continue;
            }
            4 => {
                ebx = m.r32(esp.wrapping_add(0x24u32));
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(0x6cbaf4u32);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x2u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(eax, 0x20u32, 32); fl = f;
                eax = r;
                ebx = m.r32(eax);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ebp = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x30u32), eax);
                pc = 9; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(edi.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(0x6cbaf4u32);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, 0x10u32, 32); fl = f;
                ecx = r;
                ebx = m.r32(ecx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                ebp = m.r32(ecx.wrapping_add(0x8u32));
                eax = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x30u32), eax);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(edi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(0x6cbaf4u32);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x3u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, 0x30u32, 32); fl = f;
                edx = r;
                ebx = m.r32(edx);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                ebp = m.r32(edx.wrapping_add(0x8u32));
                eax = m.r32(edx.wrapping_add(0xcu32));
                pc = 16; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(edi.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(edi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(esi, ebp, 32); fl = f;
                esi = r;
                pc = 18; continue;
            }
            17 => {
                ebx = m.r32(esp.wrapping_add(0x24u32));
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(edi.wrapping_add(0x9cu32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, ebp, 32).1;
                m.w32(esp.wrapping_add(0x14u32), esi);
                m.w32(esp.wrapping_add(0x10u32), ebp);
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(edi.wrapping_add(0x98u32));
                ecx = m.r32(0x6cbb00u32);
                pc = 20; continue;
            }
            20 => {
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x38u32), 0x0u32);
                if (fl.zf || fl.sf != fl.of) { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(edi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ebp)))) & 0xff);
                edx = m.r32(0x6cbaf4u32);
                fl = lift::sub(eax, m.r32(edx.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(m.r32(edx.wrapping_add(0x4u32)), 0x0u32, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                ebx = m.r32(edx.wrapping_add(0xcu32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(eax);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), edx);
                edx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x2cu32), edx);
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x30u32), eax);
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(0x6cbb00u32);
                edx = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(edi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x18);
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(edx, eax, 32).1;
                m.w32(esp.wrapping_add(0x38u32), edx);
                if (fl.sf != fl.of) { pc = 21; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esi = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(edx, m.r32(edi.wrapping_add(0x9cu32)), 32).1;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf != fl.of) { pc = 20; continue; }
                pc = 26; continue;
            }
            26 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0041CCF0` dialog_bg_close_and_free_tiles
pub fn f_0041ccf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x462320, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
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
                m.w32(esi.wrapping_add(0xa0u32), 0x0u32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0041CD40` dialog_bg_scalar_deleting_dtor
pub fn f_0041cd40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x41c3a0, ecx, &a); }
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

/// `0X00440900` window_base44_ctor_default
pub fn f_00440900(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65e4d8u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x697a64u32);
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
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esi, 0x66d508u32);
    m.w32(esi.wrapping_add(0x9cu32), ebx);
    m.w32(esi.wrapping_add(0x98u32), ebx);
    m.w32(esi.wrapping_add(0x94u32), ebx);
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), ebx);
    m.w32(0x6cbd8cu32, ebx);
    m.w32(0x6cbd90u32, ebx);
    m.w32(0x6cbd94u32, ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w8(esi.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa0u32), ebx);
    m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa8u32), ebx);
    m.w8(0x697a54u32, ((eax & 0xff)) as u8);
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

/// `0X00440990` window_base44_ctor_named
pub fn f_00440990(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65e4eau32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(esp.wrapping_add(0x14u32));
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
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esi, 0x66d508u32);
    m.w32(esi.wrapping_add(0x9cu32), ebx);
    m.w32(esi.wrapping_add(0x98u32), ebx);
    m.w32(esi.wrapping_add(0x94u32), ebx);
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), ebx);
    m.w32(0x6cbd8cu32, ebx);
    m.w32(0x6cbd90u32, ebx);
    m.w32(0x6cbd94u32, ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w8(esi.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa0u32), ebx);
    m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa8u32), ebx);
    m.w8(0x697a54u32, ((eax & 0xff)) as u8);
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

/// `0X00440A20` window_base44_dtor
pub fn f_00440a20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65e4fcu32);
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
    m.w32(esi, 0x66d508u32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esi.wrapping_add(0x9cu32), ebx);
    m.w32(esi.wrapping_add(0x98u32), ebx);
    m.w32(esi.wrapping_add(0x94u32), ebx);
    m.w32(esp.wrapping_add(0x14u32), ebx);
    m.w32(0x6cbd8cu32, ebx);
    m.w32(0x6cbd90u32, ebx);
    m.w32(0x6cbd94u32, ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w8(esi.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa0u32), ebx);
    m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa8u32), ebx);
    ecx = esi;
    m.w8(0x697a54u32, ((eax & 0xff)) as u8);
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
    eax
}

/// `0X00440AB0` window_base44_reset_state
pub fn f_00440ab0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
    m.w32(esi.wrapping_add(0x9cu32), ebx);
    m.w32(esi.wrapping_add(0x98u32), ebx);
    m.w32(esi.wrapping_add(0x94u32), ebx);
    m.w32(0x6cbd8cu32, ebx);
    m.w32(0x6cbd90u32, ebx);
    m.w32(0x6cbd94u32, ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w8(esi.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa0u32), ebx);
    m.w8(esi.wrapping_add(0xa4u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xa8u32), ebx);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w8(0x697a54u32, ((eax & 0xff)) as u8);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00440B20` window_base44_draw
pub fn f_00440b20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x2c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                edi = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x6cbcf8u32);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(0x6cbce8u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(0x6cbcf0u32);
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                edi = m.r32(edx.wrapping_add(0xcu32));
                pc = 6; continue;
            }
            5 => {
                ebx = m.r32(esp.wrapping_add(0x34u32));
                edi = m.r32(esp.wrapping_add(0x38u32));
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6cbcc8u32);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(0x6cbcb8u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(0x6cbcc0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ebp = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x38u32), edx);
                pc = 10; continue;
            }
            9 => {
                ebp = m.r32(esp.wrapping_add(0x34u32));
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                m.w32(esp.wrapping_add(0x4cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_add(0xc);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                esp = esp.wrapping_add(0x1c);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(0x6cbd5cu32);
                eax = m.r32(0x6cbd58u32);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 25; continue;
            }
            13 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 25; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x98u32));
                eax = edi;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = ecx.wrapping_add(edx);
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 22; continue;
            }
            22 => {
                esi = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(esi, 0x4u32, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(esi, 0x2u32, 32).1;
                if !fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(0x6cbd5cu32);
                edx = m.r32(0x6cbd58u32);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 25; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 26; continue;
            }
            26 => {
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 27; continue;
            }
            27 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x2c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00440D90` window_base44_add_dirty_rect
pub fn f_00440d90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(0x10);
                fl = lift::sub(eax, 0x4u32, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(0xa8u32));
                fl = lift::sub(eax, edx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                esi = eax;
                if (!fl.zf && fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = edx;
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(ecx.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, m.r32(edx.wrapping_add(0x4u32)), 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), eax);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esi = m.r32(edx);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(ecx.wrapping_add(0x14u32));
                ecx = m.r32(ecx.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x8u32), eax);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 7; continue;
            }
            6 => {
                edx = ecx.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                eax = edx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(eax);
                esi = m.r32(eax.wrapping_add(0x4u32));
                edi = m.r32(eax.wrapping_add(0x8u32));
                ebx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(ecx.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, m.r32(ebp.wrapping_add(0x4u32)), 32); fl = f;
                eax = r;
                ecx = m.r32(ecx.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                ebx = m.r32(ebp);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), esi);
                m.w32(esp.wrapping_add(0xcu32), edx);
                m.w32(esp.wrapping_add(0x14u32), edi);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb140, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00441220` side_menu_set_current
pub fn f_00441220(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6cbd94u32);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(0x6cbd90u32, ecx);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00441240` side_menu_system_init
pub fn f_00441240(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65e51eu32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x24u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697a98u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b3ea0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697ab0u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697abcu32);
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x30u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697ad4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x44u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x2cu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(0x0u32, ecx);
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
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697adcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697aecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697afcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697b08u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697b18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697b24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6cbbb0u32;
                m.w32(0x6cbd98u32, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbcb4u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2de0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                let r = (eax & 0xff) & 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbce4u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2de0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = (ebx & 0xff) & (eax & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                ecx = esp;
                eax = esp.wrapping_add(0x20u32);
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbd60u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2de0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let r = (ebx & 0xff) & (eax & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbd24u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2de0, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697b34u32);
                let r = (ebx & 0xff) & (eax & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbbc8u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4af260, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbbc8u32);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6cbd74u32);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(0x6cbd64u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(0x6cbd6cu32);
                ecx = m.r32(eax);
                m.w32(0x6cbd50u32, ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(0x6cbd54u32, edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(0x6cbd58u32, ecx);
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(0x6cbd5cu32, edx);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6cbd38u32);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6cbd28u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(0x6cbd30u32);
                ecx = m.r32(eax);
                m.w32(0x6cbd14u32, ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(0x6cbd18u32, edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(0x6cbd1cu32, ecx);
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(0x6cbd20u32, edx);
                pc = 8; continue;
            }
            8 => {
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x2cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00441530` side_menu_system_shutdown
pub fn f_00441530(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6cbbc8u32);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6cbbc8u32;
                { let a = []; eax = h.call_at(esp, m, 0x4af9b0, ecx, &a); }
                ecx = 0x6cbcb4u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b35b0, ecx, &a); }
                ecx = 0x6cbd24u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b35b0, ecx, &a); }
                ecx = 0x6cbce4u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b35b0, ecx, &a); }
                ecx = 0x6cbd60u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b35b0, ecx, &a); }
                eax = m.r32(0x6cbd94u32);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb490, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(0x6cbd94u32, 0x0u32);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x8u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x697b4cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b41f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w32(0x6cbd98u32, 0x0u32);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004416F0` side_menu_get_host_height
pub fn f_004416f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    edx = m.r32(0x6cbbc8u32);
    esp = esp.wrapping_sub(0xc);
    eax = esp;
    ecx = 0x6cbbc8u32;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esp.wrapping_add(0x4u32));
    esp = esp.wrapping_add(0xc);
    return eax;
    eax
}

/// `0X00441750` side_menu_set_anchor_position
pub fn f_00441750(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                edx = m.r32(0x6cbbc8u32);
                esp = esp.wrapping_sub(0xc);
                m.w32(0x6cbbdcu32, eax);
                m.w32(0x6cbbbcu32, eax);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6cbbc8u32;
                m.w32(0x6cbbe0u32, esi);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esp.wrapping_add(0x8u32));
                m.w32(0x6cbd9cu32, ecx);
                ecx = m.r32(0x6cbd90u32);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                m.w32(0x6cbbc4u32, 0x0u32);
                fl = lift::logic(ecx & ecx, 32);
                m.w32(0x6cbbc0u32, eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cbbbcu32);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbd90u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004417D0` side_menu_get_anchor_position
pub fn f_004417d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(0x6cbbb0u32);
    ecx = m.r32(0x6cbbbcu32);
    edx = m.r32(0x6cbbc0u32);
    let (r, f) = lift::add(eax, ecx, 32); fl = f;
    eax = r;
    ecx = m.r32(0x6cbbb4u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(0x6cbbb8u32);
    let (r, f) = lift::add(ecx, edx, 32); fl = f;
    ecx = r;
    edx = m.r32(0x6cbbc4u32);
    let (r, f) = lift::add(edx, esi, 32); fl = f;
    edx = r;
    esi = m.r32(esp.wrapping_add(0x8u32));
    m.w32(esi, eax);
    m.w32(esi.wrapping_add(0x4u32), ecx);
    m.w32(esi.wrapping_add(0x8u32), edx);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00441810` side_menu_force_close_current
pub fn f_00441810(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(0x10);
    ecx = m.r32(0x6cbd90u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    m.w8(0x697a54u32, ((ebx & 0xff)) as u8);
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = m.r32(0x6cbd90u32);
    m.w32(ecx.wrapping_add(0x98u32), ebx);
    edx = m.r32(0x6cbd90u32);
    m.w32(edx.wrapping_add(0xa8u32), ebx);
    eax = m.r32(0x6cbd90u32);
    m.w32(eax.wrapping_add(0x94u32), ebx);
    eax = m.r32(0x6cbd90u32);
    ecx = m.r32(0x6cbd98u32);
    edx = m.r32(eax.wrapping_add(0x9cu32));
    let (r, f) = lift::sub(edx, ecx, 32); fl = f;
    edx = r;
    m.w32(eax.wrapping_add(0x9cu32), edx);
    ecx = m.r32(0x6cbd90u32);
    edx = m.r32(ecx);
    { let t = m.r32(edx.wrapping_add(0x128u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = m.r32(0x6cbd90u32);
    ecx = m.r32(eax.wrapping_add(0x14u32));
    esi = m.r32(eax.wrapping_add(0x2cu32));
    edx = m.r32(eax.wrapping_add(0x18u32));
    eax = m.r32(eax.wrapping_add(0x30u32));
    m.w32(esp.wrapping_add(0x8u32), ecx);
    let (r, f) = lift::add(esi, ecx, 32); fl = f;
    esi = r;
    ecx = esp.wrapping_add(0x8u32);
    let (r, f) = lift::add(eax, edx, 32); fl = f;
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6da5f0u32;
    m.w32(esp.wrapping_add(0x10u32), edx);
    m.w32(esp.wrapping_add(0x14u32), esi);
    m.w32(esp.wrapping_add(0x18u32), eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    ecx = 0x6cbbc8u32;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4af970, ecx, &a); }
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w8(0x697a54u32, (0x1u32) as u8);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X00462320` parchment_base_hide
pub fn f_00462320(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(0x10);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    eax = m.r32(esi.wrapping_add(0x14u32));
    edx = m.r32(esi.wrapping_add(0x2cu32));
    ecx = m.r32(esi.wrapping_add(0x18u32));
    m.w32(esp.wrapping_add(0x4u32), eax);
    let (r, f) = lift::add(edx, eax, 32); fl = f;
    edx = r;
    eax = m.r32(esi.wrapping_add(0x30u32));
    m.w32(esp.wrapping_add(0x8u32), ecx);
    let (r, f) = lift::add(eax, ecx, 32); fl = f;
    eax = r;
    ecx = esp.wrapping_add(0x4u32);
    m.w32(esp.wrapping_add(0xcu32), edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6da5f0u32;
    m.w32(esp.wrapping_add(0x14u32), eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    ecx = 0x6cc3b4u32;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64e7a6, ecx, &a); }
    esp = esp.wrapping_add(8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = 0x6cc3b4u32;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e749, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6e51acu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x593910, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6cbdd8u32);
    { let a = []; eax = h.call_at(esp, m, 0x453ed0, ecx, &a); }
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X00462390` parchment_base_show
pub fn f_00462390(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6cc3b4u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e6fc, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbb40u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b8510, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6cbb40u32);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b85c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x4u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x30u32));
                m.w32(esp.wrapping_add(0x8u32), ecx);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(0x66a4e8u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x6cbb40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x20);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004646C0` p3_win_app_get_runtime_class
pub fn f_004646c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = 0x66e4a4u32;
    return eax;
    eax
}

/// `0X004646D0` p3_win_app_ctor
pub fn f_004646d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x659ddb, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0xc0u32), eax);
    m.w32(esi, 0x66e4b0u32);
    m.w8(esi.wrapping_add(0xc4u32), (0x0u32) as u8);
    m.w8(esi.wrapping_add(0xc5u32), (0xffu32) as u8);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00464730` p3_win_app_find_file_with_cd_fallback
pub fn f_00464730(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65ef68u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                ecx = esp.wrapping_add(0x8u32);
                { let a = []; eax = h.call_at(esp, m, 0x638f07, ecx, &a); }
                esi = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            2 => {
                eax = lift::sx8(u32::from(m.r8(edi.wrapping_add(0xc5u32))));
                fl = lift::sub(eax, 0xffu32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698ad4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esi = m.r32(esi);
                esp = esp.wrapping_add(0x10);
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x638fca, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x638f44, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x28);
                return eax;
            }
            5 => {
                ecx = esp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x2cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x638f44, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x28);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00464980` p3_win_app_init_instance
pub fn f_00464980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x464800, ecx, &a); }
                m.w8(esi.wrapping_add(0xc5u32), ((eax & 0xff)) as u8);
                eax = m.r32(0x69a568u32);
                ecx = esi.wrapping_add(0xc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = 0x6cc3f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x463240, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            2 => {
                ecx = 0x6cc3f0u32;
                m.w32(esi.wrapping_add(0x1cu32), 0x6cc3f0u32);
                { let a = []; eax = h.call_at(esp, m, 0x463d20, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xc4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = 0x6cc3f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x463c60, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(0x66a4e0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = 0x1u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004649F0` p3_win_app_show_error_and_quit
pub fn f_004649f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(0x699854u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edi = esi.wrapping_add(0xc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698af8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ecx = 0x6cc3f0u32;
                m.w8(esi.wrapping_add(0xc4u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x463c60, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xc4u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x699854u32);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x50030u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(0x66a414u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(0x66a4e0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X00464A60` p3_win_app_scalar_deleting_dtor
pub fn f_00464a60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x464a80, ecx, &a); }
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

/// `0X00464A80` p3_win_app_dtor
pub fn f_00464a80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65ef48u32);
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
    ecx = esi.wrapping_add(0xc0u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x659f25, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B03E0` control_value_ctor
pub fn f_004b03e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66187bu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6c7cccu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esp.wrapping_add(0x8u32), esi);
    ecx = esi.wrapping_add(0x8u32);
    m.w32(ecx, eax);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6d95b8u32);
    m.w32(esp.wrapping_add(0x18u32), ebx);
    m.w32(esi, 0x67078cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esp.wrapping_add(0xcu32));
    m.w32(esi.wrapping_add(0xcu32), ebx);
    m.w32(esi.wrapping_add(0x18u32), ebx);
    m.w32(esi.wrapping_add(0x1cu32), ebx);
    m.w32(esi.wrapping_add(0x14u32), ebx);
    m.w8(esi.wrapping_add(0x10u32), ((ebx & 0xff)) as u8);
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

/// `0X004B0450` control_value_set_text
pub fn f_004b0450(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66188du32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    eax = m.r32(esp.wrapping_add(0x10u32));
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esp.wrapping_add(0x18u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esi.wrapping_add(0x8u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x14u32);
    m.w32(esp.wrapping_add(0xcu32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x4u32));
    eax = esi;
    m.w32(0x0u32, ecx);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0xc);
    return eax;
    eax
}

/// `0X004B15A0` window_base_ctor_default
pub fn f_004b15a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661978u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb628u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2bd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    m.w32(esi, 0x6708d0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B15F0` window_base_ctor_named
pub fn f_004b15f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66198au32);
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
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2bd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    m.w32(esi, 0x6708d0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B1650` window_reset_base_state
pub fn f_004b1650(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w8(esi.wrapping_add(0x41u32), ((eax & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x48u32), ((eax & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x49u32), ((eax & 0xff)) as u8);
                eax = esi.wrapping_add(0x4cu32);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = 0xau32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x18u32), ebx);
                m.w32(esi.wrapping_add(0x14u32), ebx);
                m.w32(esi.wrapping_add(0x1cu32), ecx);
                m.w32(esi.wrapping_add(0x28u32), ebx);
                m.w32(esi.wrapping_add(0x24u32), ebx);
                m.w32(esi.wrapping_add(0x20u32), ebx);
                m.w32(esi.wrapping_add(0x34u32), ebx);
                m.w32(esi.wrapping_add(0x30u32), ebx);
                m.w32(esi.wrapping_add(0x2cu32), ebx);
                m.w8(esi.wrapping_add(0x42u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x43u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x4au32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x4bu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x4u32), ebx);
                m.w32(esi.wrapping_add(0x8u32), ebx);
                m.w32(esi.wrapping_add(0x44u32), ebx);
                m.w8(esi.wrapping_add(0x40u32), ((ecx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x50u32), ebx);
                m.w32(eax, ebx);
                m.w32(esi.wrapping_add(0x58u32), ebx);
                m.w32(esi.wrapping_add(0x54u32), ebx);
                m.w32(esi.wrapping_add(0x38u32), ebx);
                m.w32(esi.wrapping_add(0x5cu32), ebx);
                m.w8(esi.wrapping_add(0xcu32), ((ebx & 0xff)) as u8);
                { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x10u32), ebx);
                m.w32(esi.wrapping_add(0x60u32), ebx);
                m.w32(esi.wrapping_add(0x64u32), ebx);
                m.w32(esi.wrapping_add(0x68u32), ebx);
                m.w32(esi.wrapping_add(0x6cu32), ebx);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebp = esi.wrapping_add(0x70u32);
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, 0x4u32, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6dccdcu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c4710, ecx, &a); }
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(ebp, 0xffffffffu32);
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(edi, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x104u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x114u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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

/// `0X004B17F0` window_set_enabled
pub fn f_004b17f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x4u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x41u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x41u32), ((eax & 0xff)) as u8);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x28u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                pc = 6; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x41u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x28u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x2cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x41u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b92c0, ecx, &a); }
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x41u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 10; continue;
            }
            10 => {
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
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

/// `0X004B18F0` window_draw_content
pub fn f_004b18f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x60u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x64u32));
                ebx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                edi = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x68u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esi.wrapping_add(0x6cu32));
                esp = esp.wrapping_sub(0xc);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                edi = m.r32(esp.wrapping_add(0x20u32));
                ebx = esp;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = m.r32(esi.wrapping_add(0x60u32));
                m.w32(ebx, eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x20u32), edi);
                esi = m.r32(ecx);
                m.w32(ebx.wrapping_add(0x4u32), edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ebx.wrapping_add(0x8u32), edi);
                { let t = m.r32(esi.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004B1980` window_add_dirty_rect
pub fn f_004b1980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x28);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x10u32));
                eax = m.r32(0x670f6cu32);
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(ecx.wrapping_add(0x4u32));
                ecx = m.r32(ecx);
                edi = eax.wrapping_add(edx);
                m.w32(esp.wrapping_add(0x2cu32), edi);
                edi = m.r32(esp.wrapping_add(0x10u32));
                ebp = ecx.wrapping_add(edi);
                m.w32(esp.wrapping_add(0x28u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                m.w32(esi.wrapping_add(0x10u32), ebx);
                pc = 4; continue;
            }
            2 => {
                let (r, f) = lift::add(esi, 0x4cu32, 32); fl = f;
                esi = r;
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), eax);
                ecx = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = esp.wrapping_add(0x28u32);
                edx = m.r32(esi.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x34u32), edx);
                eax = m.r32(esi.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                { let t = m.r32(0x66a4d0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(ecx.wrapping_add(0x4u32));
                ecx = m.r32(ecx);
                esi = eax.wrapping_add(edx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::add(ebp, esi, 32); fl = f;
                ebp = r;
                esi = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                eax = edx.wrapping_add(eax).wrapping_add(0x1u32);
                edx = m.r32(esp.wrapping_add(0x30u32));
                edi = ecx.wrapping_add(esi);
                m.w32(esp.wrapping_add(0x34u32), eax);
                let (r, f) = lift::add(ebx, edi, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ecx, esi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), ebx);
                eax = edx.wrapping_add(ecx).wrapping_add(0x1u32);
                m.w32(esp.wrapping_add(0x30u32), eax);
                pc = 4; continue;
            }
            4 => {
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb140, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B1B50` window_set_dirty_rect_from_ptr
pub fn f_004b1b50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(eax & eax, 32);
                m.w8(esi.wrapping_add(0x49u32), (0x0u32) as u8);
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = esi.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x6592c2, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x50u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x58u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(edi), 0x0u32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x54u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            6 => {
                let (r, f) = lift::add(esi, 0x4cu32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let t = m.r32(0x66a4d4u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
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

/// `0X004B1C40` window_set_content
pub fn f_004b1c40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x18);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esi.wrapping_add(0x60u32), ecx);
                m.w32(esi.wrapping_add(0x64u32), eax);
                m.w32(esi.wrapping_add(0x68u32), eax);
                m.w32(esi.wrapping_add(0x6cu32), eax);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ecx);
                edx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x64u32), eax);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x8u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x68u32), eax);
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B1D20` window_copy_base_state_to
pub fn f_004b1d20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esi = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ecx;
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4au32)))) & 0xff);
                m.w8(esi.wrapping_add(0x4au32), ((eax & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x4bu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x4bu32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x41u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x41u32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x42u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x42u32), ((eax & 0xff)) as u8);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x49u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x49u32), ((ecx & 0xff)) as u8);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x48u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x48u32), ((edx & 0xff)) as u8);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x40u32)))) & 0xff);
                ecx = edi.wrapping_add(0x4cu32);
                edx = esi.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esi.wrapping_add(0x40u32), ((eax & 0xff)) as u8);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(edi.wrapping_add(0x3cu32));
                m.w32(esi.wrapping_add(0x3cu32), eax);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esi.wrapping_add(0x4u32), ecx);
                edx = m.r32(edi.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x8u32), edx);
                eax = m.r32(edi.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x10u32), eax);
                ecx = m.r32(edi.wrapping_add(0x60u32));
                edx = edi.wrapping_add(0x64u32);
                m.w32(esi.wrapping_add(0x60u32), ecx);
                eax = esi.wrapping_add(0x64u32);
                ecx = m.r32(edx);
                m.w32(eax, ecx);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(eax.wrapping_add(0x4u32), ecx);
                edx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(eax.wrapping_add(0x8u32), edx);
                eax = m.r32(edi.wrapping_add(0x5cu32));
                m.w32(esi.wrapping_add(0x5cu32), eax);
                eax = m.r32(edi.wrapping_add(0x38u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(edi.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x14u32), eax);
                ecx = m.r32(edi.wrapping_add(0x18u32));
                m.w32(esi.wrapping_add(0x18u32), ecx);
                edx = m.r32(edi.wrapping_add(0x1cu32));
                m.w32(esi.wrapping_add(0x1cu32), edx);
                eax = m.r32(edi.wrapping_add(0x20u32));
                m.w32(esi.wrapping_add(0x20u32), eax);
                ecx = m.r32(edi.wrapping_add(0x24u32));
                m.w32(esi.wrapping_add(0x24u32), ecx);
                edx = m.r32(edi.wrapping_add(0x28u32));
                m.w32(esi.wrapping_add(0x28u32), edx);
                eax = m.r32(edi.wrapping_add(0x2cu32));
                m.w32(esi.wrapping_add(0x2cu32), eax);
                ecx = m.r32(edi.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0x30u32), ecx);
                edx = m.r32(edi.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0x34u32), edx);
                eax = esi.wrapping_add(0x70u32);
                let (r, f) = lift::sub(edi, esi, 32); fl = f;
                edi = r;
                ecx = 0x4u32;
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(edi.wrapping_add(eax));
                m.w32(eax, edx);
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = esi.wrapping_add(0x80u32);
                ecx = 0x4u32;
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(eax.wrapping_add(edi));
                m.w32(eax, edx);
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                if !fl.zf { pc = 5; continue; }
                pc = 6; continue;
            }
            6 => {
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

/// `0X004B1E10` window_set_sound_id
pub fn f_004b1e10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic(edi & edi, 32);
                ebx = ecx;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(edi, 0x4u32, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                esi = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6dccdcu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c4710, ecx, &a); }
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(ebx.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x70u32), esi);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
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
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B1EC0` window_load_sound_ids_from_config
pub fn f_004b1ec0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(0x6bec1cu32);
                ebp = ecx;
                eax = m.r32(edi);
                ecx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x670f74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esi = eax;
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6dccdcu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c4710, ecx, &a); }
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(ebp.wrapping_add(0x7cu32), esi);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(edi);
                edx = m.r32(0x6bec2cu32);
                ecx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x670f78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esi = eax;
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6dccdcu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c4710, ecx, &a); }
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(ebp.wrapping_add(0x78u32), esi);
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

/// `0X004B1F80` window_get_tooltip_text
pub fn f_004b1f80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6619b3u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x8);
                m.w32(esp.wrapping_add(0x4u32), 0x0u32);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x10u32), 0x1u32);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x4u32, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x80u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x1cu32));
                ecx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esp.wrapping_add(0x8u32), 0x1u32);
                ecx = esp.wrapping_add(0x4u32);
                m.w8(esp.wrapping_add(0x14u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xcu32));
                eax = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x14);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B2080` window_load_tooltip_ids_from_config
pub fn f_004b2080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6619c5u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                edi = m.r32(esp.wrapping_add(0x30u32));
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                let (r, f) = lift::add(ecx, 0x80u32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), esi);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ebp = ecx;
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6bb60cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(ebx);
                ecx = m.r32(edi);
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x670f5cu32);
                pc = 6; continue;
            }
            3 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x10u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(esi, 0x3u32, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 6; continue;
            }
            6 => {
                m.w32(ebp, eax);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(esi, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ebx);
                edx = m.r32(0x670f5cu32);
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6bb618u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(0x670f5cu32);
                ebx = 0x4u32;
                pc = 8; continue;
            }
            8 => {
                esi = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(ecx);
                fl = lift::sub(edx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(edx, edi, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w32(ecx, esi);
                pc = 16; continue;
            }
            14 => {
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w32(ecx, eax);
                pc = 16; continue;
            }
            16 => {
                edi = m.r32(0x670f5cu32);
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 8; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
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
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B21A0` window_set_visible
pub fn f_004b21a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x4u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x48u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x48u32), ((eax & 0xff)) as u8);
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = 0x6da5f0u32;
                m.w8(esi.wrapping_add(0xcu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b92c0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xcu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
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

/// `0X004B23B0` window_add_state_flags
pub fn f_004b23b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::logic(m.r32(esi.wrapping_add(0x44u32)) & edi, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let t = m.r32(eax.wrapping_add(0x2cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(u32::from(m.r8(esi.wrapping_add(0x44u32))) & 0x1u32, 8);
                if !fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x78u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6dccdcu32);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c45a0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x44u32));
                let r = eax | edi; fl = lift::logic(r, 32);
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x44u32), eax);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B4920` screen_window_ctor_default
pub fn f_004b4920(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661b30u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bbab8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x6c7cccu32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    m.w32(esi.wrapping_add(0xb0u32), ecx);
    edx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0xd8u32), edx);
    eax = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x140u32), eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xau32);
    ecx = esi.wrapping_add(0x144u32);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e603, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    m.w32(esi, 0x670b38u32);
    m.w32(esi.wrapping_add(0x94u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b4af0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B49C0` screen_window_ctor_named
pub fn f_004b49c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661b7au32);
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
    ecx = m.r32(0x6c7cccu32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    m.w32(esi.wrapping_add(0xb0u32), ecx);
    edx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0xd8u32), edx);
    eax = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x140u32), eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xau32);
    ecx = esi.wrapping_add(0x144u32);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e603, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    m.w32(esi, 0x670b38u32);
    m.w32(esi.wrapping_add(0x94u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b4af0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B4A60` screen_window_dtor
pub fn f_004b4a60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661bc4u32);
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
    m.w32(esi, 0x670b38u32);
    m.w32(esp.wrapping_add(0x10u32), 0x4u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b4af0, ecx, &a); }
    ecx = esi.wrapping_add(0x144u32);
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64e662, ecx, &a); }
    ecx = esi.wrapping_add(0x140u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0xd8u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0xb0u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
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

/// `0X004B4AF0` screen_window_reset_state
pub fn f_004b4af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x94u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
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
                m.w32(esp, 0xcu32);
                eax = esi.wrapping_add(0x130u32);
                let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x94u32), ebx);
                m.w32(esi.wrapping_add(0xc0u32), ebx);
                m.w32(esi.wrapping_add(0xc4u32), edi);
                m.w32(esi.wrapping_add(0xc8u32), edi);
                m.w32(esi.wrapping_add(0xccu32), edi);
                m.w8(esi.wrapping_add(0xb4u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0xb5u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0xb6u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x123u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x12cu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xd4u32), edi);
                m.w8(esi.wrapping_add(0xbcu32), ((ebx & 0xff)) as u8);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                m.w32(esi.wrapping_add(0xb8u32), 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esi.wrapping_add(0xfcu32), ebx);
                m.w32(esi.wrapping_add(0xf8u32), ebx);
                m.w32(esi.wrapping_add(0xf4u32), ebx);
                m.w32(esi.wrapping_add(0xf0u32), ebx);
                m.w32(esi.wrapping_add(0xecu32), ebx);
                m.w32(esi.wrapping_add(0xe8u32), ebx);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esi.wrapping_add(0xe8u32), edi);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0xecu32), ebx);
                m.w32(esi.wrapping_add(0xf0u32), eax);
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                m.w8(esi.wrapping_add(0x122u32), (0x1u32) as u8);
                eax = m.r32(0x670f58u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x118u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = esi.wrapping_add(0xdcu32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(ecx, edi);
                m.w32(ecx.wrapping_add(0x4u32), ebx);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(0x670f48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esi.wrapping_add(0x10cu32), edx);
                eax = m.r32(0x670f4cu32);
                m.w32(esi.wrapping_add(0x110u32), eax);
                ecx = m.r32(0x670f44u32);
                m.w32(esi.wrapping_add(0x104u32), ecx);
                edx = m.r32(0x670f54u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x108u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x144u32);
                m.w8(esi.wrapping_add(0x13cu32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64e642, ecx, &a); }
                m.w32(esi.wrapping_add(0x160u32), ebx);
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

/// `0X004B4C90` screen_window_insert_child_at_index_own_offset
pub fn f_004b4c90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(0xc);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0x18u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = ecx;
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    ecx = esp.wrapping_add(0x8u32);
    m.w32(esp.wrapping_add(0x8u32), eax);
    m.w32(esp.wrapping_add(0xcu32), eax);
    m.w32(esp.wrapping_add(0x10u32), eax);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x84u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esp.wrapping_add(0x18u32));
    edx = esp.wrapping_add(0x8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b4ce0, ecx, &a); }
    esp = esp.wrapping_add(12);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0xc);
    return eax;
    eax
}

/// `0X004B4CE0` screen_window_insert_child
pub fn f_004b4ce0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esi.wrapping_add(0x94u32));
                edi = ebp;
                ebx = eax;
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xc0u32));
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ecx = ebp.wrapping_add(0x1u32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                ecx = edi.wrapping_add(ebx).wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(edi.wrapping_add(ebx), edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x94u32), ebx);
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
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(edx));
                { let a = []; eax = h.call_at(esp, m, 0x4b2230, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                ebx = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x2cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xc4u32), eax);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(ebp, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xc8u32), eax);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(ebp, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0xd4u32));
                m.w8(esi.wrapping_add(0x13cu32), ((ebx & 0xff)) as u8);
                fl = lift::sub(ebp, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xd4u32), eax);
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xc0u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B4EB0` screen_window_remove_child
pub fn f_004b4eb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebp = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::logic(ebp & ebp, 32);
                esi = ecx;
                if fl.zf { pc = 47; continue; }
                pc = 1; continue;
            }
            1 => {
                ebx = m.r32(0x6bbabcu32);
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = ebx;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(m.r32(eax.wrapping_add(ecx)), ebp, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 47; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ecx);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, ebp, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(ecx, 0x10u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebx, edx, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 9; continue;
            }
            9 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            10 => {
                m.w32(0x6bbabcu32, ebx);
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 47; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 47; continue; }
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0xd0u32)), 32).1;
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = ebx;
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                ecx = m.r32(edx.wrapping_add(edi));
                { let a = []; eax = h.call_at(esp, m, 0x4b2230, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x2cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = m.r32(eax);
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b96c0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 19; continue;
            }
            19 => {
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esi.wrapping_add(0xc0u32));
                eax = m.r32(esi.wrapping_add(0xc4u32));
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esi.wrapping_add(0xc0u32), edx);
                if !fl.zf { pc = 24; continue; }
                pc = 21; continue;
            }
            21 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 23; continue;
            }
            23 => {
                m.w32(esi.wrapping_add(0xc4u32), 0xffffffffu32);
                pc = 26; continue;
            }
            24 => {
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xc4u32), eax);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(0xd4u32));
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b7370, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 30; continue;
            }
            28 => {
                if (fl.zf || fl.sf != fl.of) { pc = 30; continue; }
                pc = 29; continue;
            }
            29 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xd4u32), eax);
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            32 => {
                if (fl.zf || fl.sf != fl.of) { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xd0u32), eax);
                pc = 34; continue;
            }
            34 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b5c20, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 38; continue;
            }
            36 => {
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xc8u32), eax);
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                m.w8(esi.wrapping_add(0x13cu32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0xccu32), 0xffffffffu32);
                pc = 42; continue;
            }
            40 => {
                if (fl.zf || fl.sf != fl.of) { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 45; continue; }
                pc = 43; continue;
            }
            43 => {
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::shl(ebx, 0x4u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 45; continue;
            }
            45 => {
                let (r, f) = lift::add(esi, 0x144u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64e7a6, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic(eax & eax, 32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 47; continue; }
                pc = 46; continue;
            }
            46 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e749, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 47; continue;
            }
            47 => {
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

/// `0X004B5380` screen_window_load_layout_from_ini
pub fn f_004b5380(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x661bfeu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x30);
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
                m.w32(esp.wrapping_add(0x48u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = esp.wrapping_add(0x50u32);
                m.w8(esp.wrapping_add(0x48u32), (0x1u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                m.w8(esp.wrapping_add(0x48u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b1cb0, ecx, &a); }
                ecx = esi;
                edi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4b4af0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1c40, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x54u32));
                fl = lift::sub(ebp, ebx, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esp.wrapping_add(0x50u32));
                fl = lift::sub(m.r32(edx.wrapping_add(0xfffffff8u32)), ebx, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(0x6beee0u32);
                ecx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(0x6bb9f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bba00u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = esi;
                edi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4b1cb0, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x58u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x5cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                m.w8(esp.wrapping_add(0x50u32), (0x3u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b1cb0, ecx, &a); }
                edx = m.r32(eax);
                ecx = eax;
                m.w8(esp.wrapping_add(0x50u32), (0x2u32) as u8);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6da5dcu32);
                edx = m.r32(0x6bba18u32);
                eax = 0x1u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
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
                ecx = esp.wrapping_add(0x2cu32);
                edx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bbac4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(0x6dcce0u32);
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x34u32), ebx);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w32(esp.wrapping_add(0x3cu32), 0x320u32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x34u32), eax);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(0x6dcce4u32);
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x50u32);
                edi = esi.wrapping_add(0xb0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x54u32), edx);
                eax = m.r32(0x6bb9f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x54u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edi = m.r32(edi);
                edx = m.r32(0x6bba50u32);
                eax = m.r32(0x6bba48u32);
                ecx = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esp.wrapping_add(0x14u32);
                ecx = esi.wrapping_add(0x140u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x4cu32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x48u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x54u32);
                m.w8(esp.wrapping_add(0x48u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(0x6beeb0u32);
                edx = m.r32(0x6bba5cu32);
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
                m.w32(esp.wrapping_add(0x34u32), ebx);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w32(esp.wrapping_add(0x3cu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x20u32);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bbaccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(m.r32(esp.wrapping_add(0x1cu32)), eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = esi.wrapping_add(0xdcu32);
                m.w32(esi.wrapping_add(0xdcu32), edx);
                m.w32(eax.wrapping_add(0x4u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(eax.wrapping_add(0x8u32), ecx);
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(0x670f44u32);
                ecx = m.r32(0x6bba6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0x104u32), eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(0x670f48u32);
                edx = m.r32(0x6bba78u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x50u32));
                m.w32(esi.wrapping_add(0x10cu32), eax);
                edx = m.r32(0x670f4cu32);
                eax = m.r32(0x6bba94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x50u32));
                m.w32(esi.wrapping_add(0x110u32), eax);
                eax = m.r32(0x670f50u32);
                ecx = m.r32(0x6bba88u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0x114u32), eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(0x670f54u32);
                edx = m.r32(0x6bbaa0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x50u32));
                m.w32(esi.wrapping_add(0x108u32), eax);
                edx = m.r32(0x670f58u32);
                eax = m.r32(0x6bbaacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x118u32), eax);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                { let a = []; eax = h.call_at(esp, m, 0x4bd340, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x48u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x48u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x50u32);
                m.w32(esp.wrapping_add(0x48u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
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
                esp = esp.wrapping_add(0x3c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B57C0` screen_window_add_children_dirty_rects
pub fn f_004b57c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = esp.wrapping_add(0xcu32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x14u32), edi);
                m.w32(esp.wrapping_add(0x18u32), edi);
                m.w32(esp.wrapping_add(0x1cu32), edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b96c0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(ecx));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                { let a = []; eax = h.call_at(esp, m, 0x4b2220, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0xd4u32)), 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                m.w32(esp.wrapping_add(0x20u32), ecx);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                ebp = m.r32(eax.wrapping_add(0x4u32));
                let (r, f) = lift::add(ebp, m.r32(esp.wrapping_add(0x10u32)), 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                ebp = m.r32(eax.wrapping_add(0x8u32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ebp = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                ecx = m.r32(eax);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b96c0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x10u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x18);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B5A90` screen_window_draw_tooltip
pub fn f_004b5a90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x121u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x110u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x18u32));
                ebx = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esp = esp.wrapping_add(0x8);
                edx = m.r32(edi);
                eax = m.r32(edi.wrapping_add(0x4u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(edi.wrapping_add(0x8u32));
                m.w32(edi, edx);
                edx = m.r32(esi.wrapping_add(0xecu32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(edi.wrapping_add(0x4u32), eax);
                eax = m.r32(esi.wrapping_add(0xf0u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi.wrapping_add(0x8u32), ecx);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x104u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x10cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xf8u32));
                eax = m.r32(esi.wrapping_add(0xf4u32));
                ecx = m.r32(edi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 4; continue;
            }
            4 => {
                fl = lift::logic(u32::from(m.r8(esi.wrapping_add(0x104u32))) & 0x10u32, 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x114u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(edi);
                edx = m.r32(esi.wrapping_add(0xf4u32));
                eax = m.r32(edi.wrapping_add(0x4u32));
                edx = edx.wrapping_add(ecx).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xf4u32));
                edx = m.r32(edi);
                eax = m.r32(edi.wrapping_add(0x4u32));
                ecx = ecx.wrapping_add(edx).wrapping_add(0xffffffffu32);
                edx = m.r32(esi.wrapping_add(0xf8u32));
                edx = edx.wrapping_add(eax).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xf8u32));
                ecx = m.r32(edi.wrapping_add(0x4u32));
                edx = m.r32(esi.wrapping_add(0xf4u32));
                eax = eax.wrapping_add(ecx).wrapping_add(0xffffffffu32);
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = edx.wrapping_add(ecx).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                eax = m.r32(edi.wrapping_add(0x4u32));
                edx = m.r32(esi.wrapping_add(0xf8u32));
                ecx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = edx.wrapping_add(eax).wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x44);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                edx = m.r32(edi.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x100u32)))) & 0xff);
                esi = m.r32(esi.wrapping_add(0x108u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(edi);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B5C20` screen_window_set_hover_child
pub fn f_004b5c20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(ebx, 0x4u32, 32, fl); fl = f;
                ebx = r;
                ecx = m.r32(ebx.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 18; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(ebx.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            6 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(eax);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                edi = ebx;
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                ecx = m.r32(edi.wrapping_add(ecx));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 17; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w32(esi.wrapping_add(0xc8u32), ebx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            16 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w32(esi.wrapping_add(0xc8u32), 0xffffffffu32);
                pc = 18; continue;
            }
            18 => {
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

/// `0X004B5D60` screen_window_on_left_button_down
pub fn f_004b5d60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x14);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = esp.wrapping_add(0xcu32);
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x4u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x4u32), edx);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x11cu32), 0xffffffffu32);
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esp.wrapping_add(0x8u32), ecx);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0x121u32), (0x1u32) as u8);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esi.wrapping_add(0x124u32), eax);
                eax = m.r32(esi.wrapping_add(0xd4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                m.w8(esi.wrapping_add(0xb6u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0xb4u32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x128u32), ecx);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x158u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 11; continue;
            }
            11 => {
                edi = m.r32(0x6bbabcu32);
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = edi;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx.wrapping_add(edx)), eax, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 14; continue;
            }
            14 => {
                ebp = m.r32(esi.wrapping_add(0xc0u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ebp & ebp, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(edx);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                fl = lift::sub(edi, ebp, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
                edi = r;
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                m.w32(esi.wrapping_add(0xc4u32), edi);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xbcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = m.r32(eax.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xb0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 29; continue;
            }
            25 => {
                m.w32(0x6bbabcu32, edi);
                pc = 19; continue;
            }
            26 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc8u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esi.wrapping_add(0xc4u32), 0xffffffffu32);
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B5FA0` screen_window_on_left_button_up
pub fn f_004b5fa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x14);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = esp.wrapping_add(0xcu32);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x8u32));
                eax = m.r32(esi.wrapping_add(0x118u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x11cu32), eax);
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w8(esi.wrapping_add(0x121u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0xb4u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0xb6u32), (0x0u32) as u8);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b5c20, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc4u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                pc = 16; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0xd4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd4u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffeu32);
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(esi.wrapping_add(0xd4u32));
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                ecx = m.r32(ecx.wrapping_add(eax));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x158u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 23; continue;
            }
            23 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B61E0` screen_window_on_right_button_down
pub fn f_004b61e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x14);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esp.wrapping_add(0x8u32), eax);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::sub(ebx, ecx, 32); fl = f;
                ebx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0xcu32), ebx);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                m.w32(esp.wrapping_add(0x10u32), edi);
                m.w32(esi.wrapping_add(0x11cu32), 0xffffffffu32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0x121u32), ((eax & 0xff)) as u8);
                pc = 2; continue;
            }
            2 => {
                m.w8(esi.wrapping_add(0xb5u32), ((eax & 0xff)) as u8);
                m.w8(esi.wrapping_add(0xb4u32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0xd4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x158u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0xcu32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xcu32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 11; continue;
            }
            11 => {
                edi = m.r32(0x6bbabcu32);
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = edi;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx.wrapping_add(edx)), eax, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 14; continue;
            }
            14 => {
                ebx = m.r32(esi.wrapping_add(0xc0u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(edx);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 15; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
                edi = r;
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                m.w32(esi.wrapping_add(0xc4u32), edi);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xbcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = m.r32(eax.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xb0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 29; continue;
            }
            25 => {
                m.w32(0x6bbabcu32, edi);
                pc = 19; continue;
            }
            26 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc8u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esi.wrapping_add(0xc4u32), 0xffffffffu32);
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B6400` screen_window_on_right_button_up
pub fn f_004b6400(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x14);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x4u32), eax);
                eax = esp.wrapping_add(0xcu32);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x8u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                let (r, f) = lift::sub(edx, edi, 32); fl = f;
                edx = r;
                edi = m.r32(esp.wrapping_add(0xcu32));
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                ecx = m.r32(esi.wrapping_add(0x118u32));
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esp.wrapping_add(0x8u32), edx);
                m.w32(esp.wrapping_add(0xcu32), edi);
                m.w32(esi.wrapping_add(0x11cu32), ecx);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0x121u32), (0x1u32) as u8);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xd4u32));
                m.w8(esi.wrapping_add(0xb4u32), (0x0u32) as u8);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                m.w8(esi.wrapping_add(0xb5u32), (0x0u32) as u8);
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(eax & eax, 32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                ebp = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::sub(edx, m.r32(ecx.wrapping_add(0x4u32)), 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                edx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x10u32), edi);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = esp.wrapping_add(0xcu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi.wrapping_add(0xd4u32));
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                ecx = m.r32(edx.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xd4u32));
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                edx = m.r32(edx.wrapping_add(ecx));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x158u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 25; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b5c20, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xc4u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 17; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc8u32));
                pc = 24; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 23; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(m.r32(ecx), 0x0u32, 32).1;
                if fl.zf { pc = 23; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                edi = m.r32(esp.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edi, edx, 32); fl = f;
                edi = r;
                edx = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x8u32), edi);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0xcu32), edx);
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xb4u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xc4u32));
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                ecx = m.r32(eax.wrapping_add(ecx));
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfffffffdu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x18u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b7370, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B6CE0` screen_window_open
pub fn f_004b6ce0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6da5e2u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                esi = ecx;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = 0xfffffffeu32;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xb0u32));
                edi = esi.wrapping_add(0xb0u32);
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x14u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b3ea0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b90e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(ecx));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x2cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xa8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edi, 0x10u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 10; continue;
            }
            10 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esi.wrapping_add(0xb4u32), (0x0u32) as u8);
                m.w32(esi.wrapping_add(0xc8u32), eax);
                m.w32(esi.wrapping_add(0xccu32), eax);
                m.w32(esi.wrapping_add(0xc4u32), eax);
                m.w8(esi.wrapping_add(0xb5u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0xb6u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0x123u32), (0x0u32) as u8);
                m.w8(esi.wrapping_add(0x12cu32), (0x0u32) as u8);
                m.w32(esi.wrapping_add(0xd4u32), eax);
                m.w32(esi.wrapping_add(0x11cu32), eax);
                m.w8(esi.wrapping_add(0x13cu32), (0x1u32) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B6DF0` screen_window_close
pub fn f_004b6df0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9150, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb0u32));
                edi = esi.wrapping_add(0xb0u32);
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x90u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x10u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b41f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
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

/// `0X004B6E80` screen_window_update
pub fn f_004b6e80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xf4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x10u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x121u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x11cu32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x118u32));
                m.w32(esi.wrapping_add(0x11cu32), eax);
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x122u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x11cu32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::sub(eax, m.r32(0x6dccf4u32), 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0x11cu32), eax);
                if !fl.sf { pc = 18; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 19; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb5u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb6u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esi.wrapping_add(0x118u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esi.wrapping_add(0x11cu32), ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 18; continue;
            }
            18 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                eax = m.r32(eax.wrapping_add(edx));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(eax);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0x110u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd0u32));
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(eax.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x114u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x13cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 30; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(0x6dab48u32);
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb6u32)))) & 0xff);
                eax = m.r32(0x6dab50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                m.w8(esi.wrapping_add(0x13cu32), (0x0u32) as u8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            27 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb5u32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                m.w8(esi.wrapping_add(0x13cu32), (0x0u32) as u8);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            29 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                m.w8(esi.wrapping_add(0x13cu32), (0x0u32) as u8);
                pc = 30; continue;
            }
            30 => {
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

/// `0X004B7020` screen_window_update_children_slot62
pub fn f_004b7020(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(eax));
                { let a = []; eax = h.call_at(esp, m, 0x4b2220, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(edi.wrapping_add(ecx));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xf8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xc0u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(edi, 0x10u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 7; continue;
            }
            7 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 8; continue;
            }
            8 => {
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

/// `0X004B7130` screen_window_find_child_index
pub fn f_004b7130(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(0x6bbabcu32);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(ecx.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esi = m.r32(ecx.wrapping_add(0x94u32));
                edx = eax;
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(edx.wrapping_add(esi));
                fl = lift::sub(ebx, edi, 32).1;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                esi = m.r32(ecx.wrapping_add(0xc0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(esi & esi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx.wrapping_add(0x94u32));
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(edx);
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(ecx, edi, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, esi, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                pc = 10; continue;
            }
            10 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            11 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x6bbabcu32, eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B7860` screen_window_on_activate
pub fn f_004b7860(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b5c20, ecx, &a); }
    esp = esp.wrapping_add(8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi.wrapping_add(0xd4u32), edi);
    m.w32(esi.wrapping_add(0xc4u32), edi);
    m.w32(esi.wrapping_add(0xc8u32), edi);
    m.w32(esi.wrapping_add(0xccu32), edi);
    m.w8(esi.wrapping_add(0xb6u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0xb5u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0xb4u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x123u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x13cu32), (0x1u32) as u8);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004B78C0` screen_window_on_deactivate
pub fn f_004b78c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    let r = edi | 0xffffffffu32; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b5c20, ecx, &a); }
    esp = esp.wrapping_add(8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b8380, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi.wrapping_add(0xd4u32), edi);
    m.w32(esi.wrapping_add(0xc4u32), edi);
    m.w32(esi.wrapping_add(0xc8u32), edi);
    m.w32(esi.wrapping_add(0xccu32), edi);
    m.w8(esi.wrapping_add(0xb6u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0xb5u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0xb4u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x123u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x13cu32), (0x1u32) as u8);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004B7920` screen_window_set_hot_child
pub fn f_004b7920(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(ebx & ebx, 32);
                edi = ecx;
                if fl.zf { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                esi = m.r32(0x6bbabcu32);
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(esi, m.r32(edi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(edi.wrapping_add(0x94u32));
                eax = esi;
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                fl = lift::sub(m.r32(eax.wrapping_add(ecx)), ebx, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(edi.wrapping_add(0xc0u32));
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(edi.wrapping_add(0x94u32));
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(ecx);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 8; continue;
            }
            8 => {
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                let (r, f) = lift::add(ecx, 0x10u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(esi, edx, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = esi | 0xffffffffu32; fl = lift::logic(r, 32);
                esi = r;
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(edi.wrapping_add(0xc4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(edi.wrapping_add(0x94u32));
                edx = eax;
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                ebx = m.r32(edx.wrapping_add(ecx));
                if fl.zf { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(eax, esi, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                edx = ecx;
                eax = m.r32(eax.wrapping_add(edx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(esi, 0xffffffffu32, 32).1;
                m.w32(edi.wrapping_add(0xc4u32), esi);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(edi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(esi, 0x4u32, 32, fl); fl = f;
                esi = r;
                ecx = m.r32(esi.wrapping_add(eax));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xbcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 16; continue;
            }
            16 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            17 => {
                m.w32(0x6bbabcu32, esi);
                pc = 10; continue;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B79E0` screen_window_clear_hot_child
pub fn f_004b79e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x4u32, 32, fl); fl = f;
                ecx = r;
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                edi = m.r32(ecx.wrapping_add(edx));
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                ecx = edx;
                eax = m.r32(eax.wrapping_add(ecx));
                ecx = eax;
                edx = m.r32(eax);
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 3; continue;
            }
            3 => {
                eax = edi;
                m.w32(esi.wrapping_add(0xc4u32), 0xffffffffu32);
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

/// `0X004B7F00` screen_window_read_bg_themes_entry
pub fn f_004b7f00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661c18u32);
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
    m.w32(esp.wrapping_add(0x4u32), eax);
    ecx = m.r32(esp.wrapping_add(0x18u32));
    edx = m.r32(0x6bb9f0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = esp.wrapping_add(0x8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
    eax = m.r32(esi.wrapping_add(0xb0u32));
    ecx = m.r32(0x6bba50u32);
    edx = m.r32(0x6bba48u32);
    esp = esp.wrapping_add(0xc);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    eax = m.r32(esp.wrapping_add(0x8u32));
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
    ecx = esp.wrapping_add(0x1cu32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x18u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi.wrapping_add(0x140u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x18u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esp.wrapping_add(0x4u32);
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B7FE0` screen_window_set_tooltip_position
pub fn f_004b7fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::sub(m.r32(edi), eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(m.r32(edi.wrapping_add(0x4u32)), eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(edi);
                let (r, f) = lift::add(esi, 0xe8u32, 32); fl = f;
                esi = r;
                m.w32(esi, eax);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(esi.wrapping_add(0x4u32), ecx);
                edx = m.r32(edi.wrapping_add(0x8u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x8u32), edx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B8030` screen_window_set_default_tooltip_position
pub fn f_004b8030(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::sub(m.r32(edi), eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(m.r32(edi.wrapping_add(0x4u32)), eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(edi);
                let (r, f) = lift::add(esi, 0xdcu32, 32); fl = f;
                esi = r;
                m.w32(esi, eax);
                ecx = m.r32(edi.wrapping_add(0x4u32));
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                m.w32(esi.wrapping_add(0x4u32), ecx);
                edx = m.r32(edi.wrapping_add(0x8u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x8u32), edx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B80D0` screen_window_layout_tooltip
pub fn f_004b80d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x661c2au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x3c);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 19; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if (fl.sf == fl.of) { pc = 19; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xecu32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esi.wrapping_add(0xe8u32));
                edi = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xf4u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                edx = m.r32(esi.wrapping_add(0xf8u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x104u32));
                m.w8(esi.wrapping_add(0x100u32), ((ebx & 0xff)) as u8);
                fl = lift::logic(((eax >> 8) & 0xff) & 0x10u32, 8);
                edi = esi.wrapping_add(0xe8u32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                m.w32(edi, edx);
                edx = m.r32(esi.wrapping_add(0xe4u32));
                m.w32(esi.wrapping_add(0xecu32), ecx);
                m.w32(esi.wrapping_add(0xf0u32), edx);
                pc = 7; continue;
            }
            6 => {
                ecx = esi.wrapping_add(0xdcu32);
                edx = edi;
                ebp = m.r32(ecx);
                m.w32(edx, ebp);
                ebp = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), ebp);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(edx.wrapping_add(0x8u32), ecx);
                pc = 7; continue;
            }
            7 => {
                fl = lift::logic((eax & 0xff) & 0x10u32, 8);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w8(esi.wrapping_add(0x100u32), (0x2u32) as u8);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi.wrapping_add(0xd0u32));
                eax = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(eax);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(ecx);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x100u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0xd8u32);
                m.w32(esp.wrapping_add(0x58u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x54u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xd8u32));
                ecx = m.r32(esi.wrapping_add(0x108u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x108u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x100u32)))) & 0xff);
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                m.w32(esi.wrapping_add(0xf4u32), eax);
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
                edx = m.r32(esp.wrapping_add(0x18u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x100u32)))) & 0xff);
                m.w32(esi.wrapping_add(0xfcu32), ebx);
                eax = edx.wrapping_add(ecx.wrapping_mul(2));
                edx = m.r32(esi);
                ecx = esi;
                m.w32(esi.wrapping_add(0xf8u32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xf4u32));
                ebp = m.r32(edi);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, m.r32(esi.wrapping_add(0xf4u32)), 32); fl = f;
                eax = r;
                m.w32(edi, eax);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi);
                ebp = m.r32(esi.wrapping_add(0xf8u32));
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0xecu32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esi);
                ebp = m.r32(esi.wrapping_add(0xf8u32));
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0xecu32), eax);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x104u32));
                fl = lift::logic(((eax >> 8) & 0xff) & 0x10u32, 8);
                if fl.zf { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0xecu32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esi.wrapping_add(0xecu32), eax);
                if !fl.sf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                m.w32(esi.wrapping_add(0xecu32), ebx);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = m.r32(esi.wrapping_add(0xecu32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                edi = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xf4u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), ecx);
                m.w32(esp.wrapping_add(0x3cu32), eax);
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                edx = m.r32(esi.wrapping_add(0xf8u32));
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                ecx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x4cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esp.wrapping_add(0x40u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x48);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B8380` screen_window_show_tooltip
pub fn f_004b8380(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
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
                edx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b7240, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = eax;
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                edi = m.r32(eax.wrapping_add(ecx));
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x30u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 7; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x121u32))), 0x1u32, 8).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(edi, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x100u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(eax);
                edi = m.r32(ecx.wrapping_add(0xfffffff8u32));
                ecx = esp.wrapping_add(0x10u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                fl = lift::sub(edi, ebx, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esp.wrapping_add(0x30u32), ((ebx & 0xff)) as u8);
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x121u32), ((ebx & 0xff)) as u8);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xd0u32), ebp);
                { let a = []; eax = h.call_at(esp, m, 0x4b80d0, ecx, &a); }
                pc = 11; continue;
            }
            9 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x120u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xecu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0xe8u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xf4u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                edx = m.r32(esi.wrapping_add(0xf8u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xecu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi.wrapping_add(0xe8u32));
                edi = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xf4u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = eax.wrapping_add(edx).wrapping_add(0x1u32);
                edx = m.r32(esi.wrapping_add(0xf8u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ecx.wrapping_add(edx).wrapping_add(0x1u32);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x120u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x120u32), ((edx & 0xff)) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x1c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B85C0` screen_window_get_tooltip_rect
pub fn f_004b85c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(0xc);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    ecx = esp.wrapping_add(0x4u32);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi.wrapping_add(0xe8u32));
    eax = m.r32(esp.wrapping_add(0x4u32));
    ecx = edx.wrapping_add(eax);
    eax = m.r32(esp.wrapping_add(0x14u32));
    edx = m.r32(esp.wrapping_add(0x8u32));
    m.w32(eax.wrapping_add(0x8u32), ecx);
    m.w32(eax, ecx);
    ecx = m.r32(esi.wrapping_add(0xecu32));
    let (r, f) = lift::add(ecx, edx, 32); fl = f;
    ecx = r;
    edx = m.r32(eax.wrapping_add(0x8u32));
    m.w32(eax.wrapping_add(0xcu32), ecx);
    m.w32(eax.wrapping_add(0x4u32), ecx);
    ecx = m.r32(esi.wrapping_add(0xf4u32));
    let (r, f) = lift::add(edx, ecx, 32); fl = f;
    edx = r;
    ecx = m.r32(eax.wrapping_add(0xcu32));
    m.w32(eax.wrapping_add(0x8u32), edx);
    edx = m.r32(esi.wrapping_add(0xf8u32));
    let (r, f) = lift::add(ecx, edx, 32); fl = f;
    ecx = r;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(eax.wrapping_add(0xcu32), ecx);
    esp = esp.wrapping_add(0xc);
    return eax;
    eax
}

/// `0X004B8740` screen_list_ctor
pub fn f_004b8740(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661c6cu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bbbd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2bd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xau32);
    ecx = esi.wrapping_add(0x4u32);
    m.w32(esp.wrapping_add(0x18u32), edi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e603, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4acdd0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x28u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    m.w32(esi, 0x670ca4u32);
    m.w32(esi.wrapping_add(0x218u32), edi);
    m.w32(esi.wrapping_add(0x220u32), edi);
    m.w32(esi.wrapping_add(0x21cu32), edi);
    m.w32(esi.wrapping_add(0x224u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x4b8880, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B87E0` screen_list_dtor
pub fn f_004b87e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661ca2u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(0x8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    m.w32(esp.wrapping_add(0x4u32), esi);
    m.w32(esi, 0x670ca4u32);
    m.w32(esp.wrapping_add(0x14u32), 0x2u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b8880, ecx, &a); }
    eax = m.r32(0x670f40u32);
    edx = m.r32(0x6beee0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esp;
    m.w32(esp.wrapping_add(0x10u32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = 0x6da820u32;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b41f0, ecx, &a); }
    esp = esp.wrapping_add(8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4ace40u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    eax = esi.wrapping_add(0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xd8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0x4u32);
    m.w8(esp.wrapping_add(0x14u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64e662, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4b2be0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x14);
    return eax;
    eax
}

/// `0X004B8880` screen_list_reset
pub fn f_004b8880(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esi.wrapping_add(0x4u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e642, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x224u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x218u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x224u32)), ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x220u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(eax.wrapping_add(0xfffffffcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x224u32)), ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0x21cu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 9; continue;
            }
            9 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x224u32), ebx);
                m.w32(esi.wrapping_add(0x218u32), ebx);
                m.w32(esi.wrapping_add(0x220u32), ebx);
                m.w32(esi.wrapping_add(0x21cu32), ebx);
                m.w32(esi.wrapping_add(0x208u32), eax);
                m.w32(esi.wrapping_add(0x20cu32), eax);
                m.w32(esi.wrapping_add(0x210u32), eax);
                m.w8(esi.wrapping_add(0x228u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x229u32), ((ebx & 0xff)) as u8);
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

/// `0X004B8940` screen_list_load_cursors
pub fn f_004b8940(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x8);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0xcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esi = ecx;
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b3ea0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edi = esi.wrapping_add(0x20u32);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let (r, f) = lift::add(esi, 0x1f0u32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(eax, 0x1e8u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x670f34u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = esi.wrapping_add(0xffffffe0u32);
                m.w32(esi, eax);
                m.w32(esi.wrapping_add(0x4u32), eax);
                m.w32(esi.wrapping_add(0x8u32), eax);
                eax = esi;
                m.w32(ecx, edi);
                ecx = m.r32(eax);
                m.w32(edx, ecx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), ecx);
                ecx = edi;
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                edx = m.r32(edi);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(eax, 0x4u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(esi, 0xcu32, 32); fl = f;
                esi = r;
                let (r, f) = lift::add(edi, 0xd8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(ebp, 0x2u32, 32).1;
                m.w32(esp.wrapping_add(0x20u32), eax);
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                m.w8(eax.wrapping_add(0x228u32), (0x1u32) as u8);
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(eax.wrapping_add(0x214u32), 0x0u32);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                return eax;
            }
            5 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w8(ecx.wrapping_add(0x228u32), (0x1u32) as u8);
                pc = 6; continue;
            }
            6 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x8);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B9150` screen_list_remove_window
pub fn f_004b9150(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebp = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(ebp.wrapping_add(0xcu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                edx = eax;
                eax = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::sub(m.r32(ecx.wrapping_add(0x8u32)), esi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, edi, 32).1;
                if !fl.zf { pc = 1; continue; }
                pc = 3; continue;
            }
            3 => {
                let (r, f) = lift::sub(0, (ebx & 0xff), 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(ebx, ebx, fl.cf, 32); fl = f;
                ebx = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = ebx & esi; fl = lift::logic(r, 32);
                ebx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebp.wrapping_add(0x4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64e749, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(0x6dcb94u32), edi, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi);
                ecx = esi;
                m.w32(esp.wrapping_add(0x10u32), edi);
                m.w32(esp.wrapping_add(0x14u32), edi);
                m.w32(esp.wrapping_add(0x18u32), edi);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x160u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(m.r32(0x6dcb94u32), edi, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x670f6cu32);
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(edi.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x4);
                eax = esp.wrapping_add(0x10u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x670f6cu32);
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(0x6dcd04u32);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(ebp.wrapping_add(0x10u32));
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                fl = lift::sub(eax, edi, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(ebp.wrapping_add(0xcu32));
                ecx = m.r32(eax.wrapping_add(0x8u32));
                fl = lift::sub(ecx, edi, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                { let a = []; eax = h.call_at(esp, m, 0x4b7850, ecx, &a); }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(0, (ebx & 0xff), 8); fl = f;
                ebx = (ebx & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(ebx, ebx, fl.cf, 32); fl = f;
                ebx = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = ebx & esi; fl = lift::logic(r, 32);
                ebx = r;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = ebx;
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

/// `0X004B9240` screen_list_reset_with_temp_string
pub fn f_004b9240(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661cbcu32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6c7cccu32);
    m.w32(esp.wrapping_add(0xcu32), 0x0u32);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0xcu32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b8880, ecx, &a); }
    ecx = esp;
    m.w8(esp.wrapping_add(0xcu32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esp.wrapping_add(0x14u32);
    m.w32(esp.wrapping_add(0xcu32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x4u32));
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B96C0` window_add_dirty_rect_to_clippers
pub fn f_004b96c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6dcb94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                edi = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 7; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(0x670f6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                let r = esi ^ esi; fl = lift::logic(r, 32);
                esi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x10u32));
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                edx = m.r32(edi);
                esp = esp.wrapping_add(0x4);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x670f6cu32);
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                fl = lift::sub(esi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 5; continue;
            }
            5 => {
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6dcd04u32);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x6dcd20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb780, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 7; continue;
            }
            7 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C0980` game_window_ctor
pub fn f_004c0980(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x650184, ecx, &a); }
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi, 0x671150u32);
    m.w8(esi.wrapping_add(0x3cu32), ((eax & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x3du32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x40u32), eax);
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C09A0` game_window_dtor
pub fn f_004c09a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    m.w32(ecx, 0x671150u32);
    { let a = []; return h.call_at(esp, m, 0x650bfd, ecx, &a); }
    eax
}

/// `0X004C09B0` game_window_get_runtime_class
pub fn f_004c09b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = 0x671148u32;
    return eax;
    eax
}

/// `0X004C0F20` game_window_on_paint
pub fn f_004c0f20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = 0x6da5f0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4b9730, ecx, &a); }
                edx = m.r32(eax);
                ecx = eax;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                ecx = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x6504f2, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C0FD0` game_window_scalar_deleting_dtor
pub fn f_004c0fd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4c09a0, ecx, &a); }
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
        0x4023e0 => f_004023e0(m, h, ecx, esp),
        0x410340 => f_00410340(m, h, ecx, esp),
        0x41c320 => f_0041c320(m, h, ecx, esp),
        0x41c360 => f_0041c360(m, h, ecx, esp),
        0x41c3a0 => f_0041c3a0(m, h, ecx, esp),
        0x41c3e0 => f_0041c3e0(m, h, ecx, esp),
        0x41c7b0 => f_0041c7b0(m, h, ecx, esp),
        0x41ca00 => f_0041ca00(m, h, ecx, esp),
        0x41caa0 => f_0041caa0(m, h, ecx, esp),
        0x41ccf0 => f_0041ccf0(m, h, ecx, esp),
        0x41cd40 => f_0041cd40(m, h, ecx, esp),
        0x440900 => f_00440900(m, h, ecx, esp),
        0x440990 => f_00440990(m, h, ecx, esp),
        0x440a20 => f_00440a20(m, h, ecx, esp),
        0x440ab0 => f_00440ab0(m, h, ecx, esp),
        0x440b20 => f_00440b20(m, h, ecx, esp),
        0x440d90 => f_00440d90(m, h, ecx, esp),
        0x441220 => f_00441220(m, h, ecx, esp),
        0x441240 => f_00441240(m, h, ecx, esp),
        0x441530 => f_00441530(m, h, ecx, esp),
        0x4416f0 => f_004416f0(m, h, ecx, esp),
        0x441750 => f_00441750(m, h, ecx, esp),
        0x4417d0 => f_004417d0(m, h, ecx, esp),
        0x441810 => f_00441810(m, h, ecx, esp),
        0x462320 => f_00462320(m, h, ecx, esp),
        0x462390 => f_00462390(m, h, ecx, esp),
        0x4646c0 => f_004646c0(m, h, ecx, esp),
        0x4646d0 => f_004646d0(m, h, ecx, esp),
        0x464730 => f_00464730(m, h, ecx, esp),
        0x464980 => f_00464980(m, h, ecx, esp),
        0x4649f0 => f_004649f0(m, h, ecx, esp),
        0x464a60 => f_00464a60(m, h, ecx, esp),
        0x464a80 => f_00464a80(m, h, ecx, esp),
        0x4b03e0 => f_004b03e0(m, h, ecx, esp),
        0x4b0450 => f_004b0450(m, h, ecx, esp),
        0x4b15a0 => f_004b15a0(m, h, ecx, esp),
        0x4b15f0 => f_004b15f0(m, h, ecx, esp),
        0x4b1650 => f_004b1650(m, h, ecx, esp),
        0x4b17f0 => f_004b17f0(m, h, ecx, esp),
        0x4b18f0 => f_004b18f0(m, h, ecx, esp),
        0x4b1980 => f_004b1980(m, h, ecx, esp),
        0x4b1b50 => f_004b1b50(m, h, ecx, esp),
        0x4b1c40 => f_004b1c40(m, h, ecx, esp),
        0x4b1d20 => f_004b1d20(m, h, ecx, esp),
        0x4b1e10 => f_004b1e10(m, h, ecx, esp),
        0x4b1ec0 => f_004b1ec0(m, h, ecx, esp),
        0x4b1f80 => f_004b1f80(m, h, ecx, esp),
        0x4b2080 => f_004b2080(m, h, ecx, esp),
        0x4b21a0 => f_004b21a0(m, h, ecx, esp),
        0x4b23b0 => f_004b23b0(m, h, ecx, esp),
        0x4b4920 => f_004b4920(m, h, ecx, esp),
        0x4b49c0 => f_004b49c0(m, h, ecx, esp),
        0x4b4a60 => f_004b4a60(m, h, ecx, esp),
        0x4b4af0 => f_004b4af0(m, h, ecx, esp),
        0x4b4c90 => f_004b4c90(m, h, ecx, esp),
        0x4b4ce0 => f_004b4ce0(m, h, ecx, esp),
        0x4b4eb0 => f_004b4eb0(m, h, ecx, esp),
        0x4b5380 => f_004b5380(m, h, ecx, esp),
        0x4b57c0 => f_004b57c0(m, h, ecx, esp),
        0x4b5a90 => f_004b5a90(m, h, ecx, esp),
        0x4b5c20 => f_004b5c20(m, h, ecx, esp),
        0x4b5d60 => f_004b5d60(m, h, ecx, esp),
        0x4b5fa0 => f_004b5fa0(m, h, ecx, esp),
        0x4b61e0 => f_004b61e0(m, h, ecx, esp),
        0x4b6400 => f_004b6400(m, h, ecx, esp),
        0x4b6ce0 => f_004b6ce0(m, h, ecx, esp),
        0x4b6df0 => f_004b6df0(m, h, ecx, esp),
        0x4b6e80 => f_004b6e80(m, h, ecx, esp),
        0x4b7020 => f_004b7020(m, h, ecx, esp),
        0x4b7130 => f_004b7130(m, h, ecx, esp),
        0x4b7860 => f_004b7860(m, h, ecx, esp),
        0x4b78c0 => f_004b78c0(m, h, ecx, esp),
        0x4b7920 => f_004b7920(m, h, ecx, esp),
        0x4b79e0 => f_004b79e0(m, h, ecx, esp),
        0x4b7f00 => f_004b7f00(m, h, ecx, esp),
        0x4b7fe0 => f_004b7fe0(m, h, ecx, esp),
        0x4b8030 => f_004b8030(m, h, ecx, esp),
        0x4b80d0 => f_004b80d0(m, h, ecx, esp),
        0x4b8380 => f_004b8380(m, h, ecx, esp),
        0x4b85c0 => f_004b85c0(m, h, ecx, esp),
        0x4b8740 => f_004b8740(m, h, ecx, esp),
        0x4b87e0 => f_004b87e0(m, h, ecx, esp),
        0x4b8880 => f_004b8880(m, h, ecx, esp),
        0x4b8940 => f_004b8940(m, h, ecx, esp),
        0x4b9150 => f_004b9150(m, h, ecx, esp),
        0x4b9240 => f_004b9240(m, h, ecx, esp),
        0x4b96c0 => f_004b96c0(m, h, ecx, esp),
        0x4c0980 => f_004c0980(m, h, ecx, esp),
        0x4c09a0 => f_004c09a0(m, h, ecx, esp),
        0x4c09b0 => f_004c09b0(m, h, ecx, esp),
        0x4c0f20 => f_004c0f20(m, h, ecx, esp),
        0x4c0fd0 => f_004c0fd0(m, h, ecx, esp),
        _ => return None,
    }))
}
