//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X004201B0` list_box_deleting_dtor
pub fn f_004201b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x656fdb, ecx, &a); }
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

/// `0X004201D0` combo_box_deleting_dtor
pub fn f_004201d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x6570b4, ecx, &a); }
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

/// `0X0045C110` number_input_field_ctor
pub fn f_0045c110(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65e9e8u32);
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
    { let a = []; eax = h.call_at(esp, m, 0x4c9790, ecx, &a); }
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), ebx);
    m.w32(esi, 0x66dbb8u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c99f0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    m.w32(esi.wrapping_add(0x184u32), ebx);
    m.w32(esi.wrapping_add(0x180u32), ebx);
    m.w32(esi.wrapping_add(0x188u32), 0xffffffffu32);
    m.w8(esi.wrapping_add(0x18cu32), ((ebx & 0xff)) as u8);
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

/// `0X0045C180` number_input_field_dtor
pub fn f_0045c180(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65e9fau32);
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
    m.w32(esi, 0x66dbb8u32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    m.w32(esp.wrapping_add(0x14u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4c99f0, ecx, &a); }
    let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
    eax = r;
    ecx = esi;
    m.w32(esi.wrapping_add(0x184u32), ebx);
    m.w32(esi.wrapping_add(0x180u32), ebx);
    m.w32(esi.wrapping_add(0x188u32), eax);
    m.w8(esi.wrapping_add(0x18cu32), ((ebx & 0xff)) as u8);
    m.w32(esp.wrapping_add(0x14u32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x4c9970, ecx, &a); }
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

/// `0X0045C230` number_input_field_normalize_text
pub fn f_0045c230(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65ea0cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4ca6f0, ecx, &a); }
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                ecx = ebp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x10u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                edi = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(edi & edi, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((ecx & 0xff), 0x30u32, 8).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edi, 0x1u32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::logic(ecx & edx, 32);
                if fl.zf { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esp.wrapping_add(0x14u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64cc80, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0xcu32));
                edx = m.r32(esi.wrapping_add(0x168u32));
                ecx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::sub(edx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                edi = m.r32(eax.wrapping_add(0xfffffff8u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((ecx & 0xff), 0x30u32, 8).1;
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(edi, 0x1u32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                fl = lift::logic(ecx & edx, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x18u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0045C930` number_input_field_set_value
pub fn f_0045c930(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x65ea80u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esi.wrapping_add(0x180u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x184u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = ecx;
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x188u32)), 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(esi.wrapping_add(0x18cu32), (0x1u32) as u8);
                pc = 5; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0x188u32), eax);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x698118u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                eax = esp.wrapping_add(0x14u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4caad0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0xcu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0045CA10` number_input_field_get_class_name
pub fn f_0045ca10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfac8u32);
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

/// `0X0045CA40` number_input_field_scalar_deleting_dtor
pub fn f_0045ca40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x45c180, ecx, &a); }
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

/// `0X004AFC30` combo_list_box_ctor
pub fn f_004afc30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6617f6u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb590u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4caf40, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = esi.wrapping_add(0x314u32);
    m.w32(esp.wrapping_add(0x14u32), 0x0u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x4b2480, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    m.w32(esi, 0x670550u32);
    m.w32(esi.wrapping_add(0x94u32), 0x67054cu32);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
    ecx = m.r32(0x670f2cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b29f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dab38u32))) & 0xffff);
    ecx = edi;
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab40u32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab34u32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab3cu32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b29a0, ecx, &a); }
    esp = esp.wrapping_add(8);
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

/// `0X004AFCE0` combo_list_box_ctor_named
pub fn f_004afce0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661816u32);
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
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4caf40, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = esi.wrapping_add(0x314u32);
    m.w32(esp.wrapping_add(0x14u32), 0x0u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x4b2480, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    m.w32(esi, 0x670550u32);
    m.w32(esi.wrapping_add(0x94u32), 0x67054cu32);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
    ecx = m.r32(0x670f2cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b29f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dab38u32))) & 0xffff);
    ecx = edi;
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab40u32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab34u32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    let r = (edx & 0xffff) | u32::from(m.r16(0x6dab3cu32)); fl = lift::logic(r, 16);
    edx = (edx & !0xffff) | ((r) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b29a0, ecx, &a); }
    esp = esp.wrapping_add(8);
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

/// `0X004AFD90` combo_list_box_dtor
pub fn f_004afd90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661836u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    m.w32(esp.wrapping_add(0x8u32), esi);
    m.w32(esi, 0x670550u32);
    m.w32(esi.wrapping_add(0x94u32), 0x67054cu32);
    m.w32(esp.wrapping_add(0x14u32), 0x1u32);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
    eax = m.r32(0x670f2cu32);
    edi = esi.wrapping_add(0x314u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b29f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dab38u32))) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab40u32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab34u32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab3cu32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = edi;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b29a0, ecx, &a); }
    esp = esp.wrapping_add(8);
    ecx = edi;
    m.w8(esp.wrapping_add(0x14u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b2680, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4cafe0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004AFE40` combo_list_box_apply_default_style
pub fn f_004afe40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
    eax = m.r32(0x670f2cu32);
    let (r, f) = lift::add(esi, 0x314u32, 32); fl = f;
    esi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b29f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dab38u32))) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab40u32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab34u32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    let r = (ecx & 0xffff) | u32::from(m.r16(0x6dab3cu32)); fl = lift::logic(r, 16);
    ecx = (ecx & !0xffff) | ((r) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b29a0, ecx, &a); }
    esp = esp.wrapping_add(8);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004AFEB0` combo_list_box_load_layout
pub fn f_004afeb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x661848u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x24u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4cb770, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esi.wrapping_add(0x314u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1c40, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x8u32));
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004AFF30` combo_list_box_slot3_release_string
pub fn f_004aff30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66185au32);
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
    m.w32(esp, ecx);
    eax = esp.wrapping_add(0x20u32);
    ecx = esp;
    m.w32(esp.wrapping_add(0xcu32), esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4cbce0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x1cu32);
    ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
    m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004AFFC0` combo_list_box_get_class_name
pub fn f_004affc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb590u32);
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

/// `0X004AFFF0` combo_list_box_scalar_deleting_dtor
pub fn f_004afff0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4afd90, ecx, &a); }
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

/// `0X004B0010` combo_list_box_set_size_from_triple
pub fn f_004b0010(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
    eax = m.r32(esi.wrapping_add(0x314u32));
    ecx = esi.wrapping_add(0x314u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ce6d0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004B0040` combo_list_box_set_position_from_triple
pub fn f_004b0040(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = m.r32(esp.wrapping_add(0x8u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    eax = m.r32(edi.wrapping_add(0x314u32));
    ecx = edi.wrapping_add(0x314u32);
    { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(esi);
    let (r, f) = lift::add(edi, 0x14u32, 32); fl = f;
    edi = r;
    m.w32(edi, ecx);
    edx = m.r32(esi.wrapping_add(0x4u32));
    m.w32(edi.wrapping_add(0x4u32), edx);
    eax = m.r32(esi.wrapping_add(0x8u32));
    m.w32(edi.wrapping_add(0x8u32), eax);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C7E30` combo_box_ctor
pub fn f_004c7e30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662292u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bf9dcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi.wrapping_add(0x94u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x180u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4afc30, ecx, &a); }
    ecx = esi.wrapping_add(0x544u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    m.w32(esi, 0x671608u32);
    m.w32(esi.wrapping_add(0x630u32), 0x0u32);
    m.w32(esi.wrapping_add(0x62cu32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c7f50, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004C7ED0` combo_box_dtor
pub fn f_004c7ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6622ceu32);
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
    m.w32(esi, 0x671608u32);
    m.w32(esp.wrapping_add(0x10u32), 0x3u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c7f50, ecx, &a); }
    ecx = esi.wrapping_add(0x544u32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4ace40, ecx, &a); }
    ecx = esi.wrapping_add(0x180u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4afd90, ecx, &a); }
    ecx = esi.wrapping_add(0x94u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
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

/// `0X004C7F50` combo_box_reset
pub fn f_004c7f50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x180u32);
                { let a = []; eax = h.call_at(esp, m, 0x4afe40, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x62cu32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x630u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x62cu32));
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
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
                eax = m.r32(esi.wrapping_add(0x630u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x62cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0x62cu32), ebx);
                m.w32(esi.wrapping_add(0x630u32), ebx);
                m.w8(esi.wrapping_add(0x634u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x638u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x63cu32), ebx);
                m.w32(esi.wrapping_add(0x628u32), 0xff80f300u32);
                m.w8(esi.wrapping_add(0x17cu32), ((ebx & 0xff)) as u8);
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

/// `0X004C8190` combo_box_load_layout
pub fn f_004c8190(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6622f3u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x78);
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
                m.w32(esp.wrapping_add(0x90u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(esp.wrapping_add(0x98u32));
                m.w8(esp.wrapping_add(0x90u32), (0x2u32) as u8);
                m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::sub(eax, 0x5u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6beee0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x9cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                edi = m.r32(esp.wrapping_add(0x9cu32));
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4c7f50, ecx, &a); }
                edx = m.r32(0x6bf908u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xa4u32));
                edx = m.r32(0x6bf924u32);
                eax = m.r32(0x6bf91cu32);
                esp = esp.wrapping_add(0xc);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
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
                m.w32(esp.wrapping_add(0x48u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = esp.wrapping_add(0x2cu32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf9e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(eax, 0x2u32, 32).1;
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                m.w32(esp.wrapping_add(0x28u32), ebx);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x98u32));
                edx = m.r32(0x6bf92cu32);
                eax = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(0x6bf910u32);
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
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = esp.wrapping_add(0x1cu32);
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf9e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = esp.wrapping_add(0x40u32);
                m.w32(esp.wrapping_add(0x44u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x4cu32), ebx);
                { let t = m.r32(eax.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x98u32));
                eax = m.r32(0x6bf938u32);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x98u32));
                ecx = m.r32(0x6bf948u32);
                ebp = eax;
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
                edi = 0x6dbb58u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                m.w32(esp, 0x6dd5d2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                edi = eax;
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x180u32));
                edi = esi.wrapping_add(0x180u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w8(esp.wrapping_add(0x14u32), ((ebx & 0xff)) as u8);
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x98u32));
                edx = m.r32(0x6bf958u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x98u32));
                edx = m.r32(0x6beee0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebx = eax;
                eax = m.r32(0x6bf964u32);
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
                ebp = 0x6dbb58u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5d3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ebp = eax;
                pc = 14; continue;
            }
            14 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x94u32));
                ebp = esi.wrapping_add(0x94u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x98u32));
                ecx = m.r32(0x6bf974u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(0x6bf984u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(esp.wrapping_add(0x98u32));
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
                ebx = 0x6dbb58u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x22u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ba90, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4b2b00, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ebx = eax;
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x544u32));
                ecx = esi.wrapping_add(0x544u32);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                pc = 20; continue;
            }
            20 => {
                ebx = esi.wrapping_add(0x61cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0xa4u32));
                ecx = m.r32(0x6bf99cu32);
                edx = m.r32(0x6bf994u32);
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
                ecx = esi.wrapping_add(0x620u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf9f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0xa8u32));
                eax = m.r32(0x6bf9a8u32);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x10);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::sub(eax, 0x6u32, 32).1;
                m.w32(esi.wrapping_add(0x63cu32), eax);
                if (fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x63cu32), 0x5u32);
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x98u32));
                eax = m.r32(0x6dd5ccu32);
                ecx = m.r32(0x6bf9b8u32);
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
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esi.wrapping_add(0x628u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x98u32));
                eax = m.r32(0x6bf9c8u32);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::sub(eax, ebx, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x17cu32)))) & 0xff);
                if fl.zf { pc = 28; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 34; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                m.w8(esi.wrapping_add(0x17cu32), (0x1u32) as u8);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x544u32));
                ecx = esi.wrapping_add(0x544u32);
                eax = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ebx);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x17cu32)))) & 0xff);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x4cu32));
                let (r, f) = lift::sub(edx, m.r32(esp.wrapping_add(0x58u32)), 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                pc = 27; continue;
            }
            26 => {
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), eax);
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(ebp);
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x74u32));
                ecx = esp.wrapping_add(0x64u32);
                m.w32(esp.wrapping_add(0x68u32), edx);
                pc = 33; continue;
            }
            28 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 34; continue; }
                pc = 29; continue;
            }
            29 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                m.w8(esi.wrapping_add(0x17cu32), (0x0u32) as u8);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x544u32));
                ecx = esi.wrapping_add(0x544u32);
                eax = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x68u32));
                eax = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ebx);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x17cu32)))) & 0xff);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::sub(edx, m.r32(esp.wrapping_add(0x58u32)), 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                pc = 32; continue;
            }
            31 => {
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), eax);
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(ebp);
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x74u32));
                ecx = esp.wrapping_add(0x4cu32);
                m.w32(esp.wrapping_add(0x50u32), edx);
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 34; continue;
            }
            34 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x544u32));
                ecx = esi.wrapping_add(0x544u32);
                eax = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x68u32));
                eax = m.r32(esp.wrapping_add(0x5cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ebx);
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x17cu32)))) & 0xff);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(esp.wrapping_add(0x64u32));
                let (r, f) = lift::sub(edx, m.r32(esp.wrapping_add(0x58u32)), 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x34u32), edx);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                pc = 37; continue;
            }
            36 => {
                m.w32(esp.wrapping_add(0x34u32), eax);
                m.w32(esp.wrapping_add(0x38u32), eax);
                pc = 37; continue;
            }
            37 => {
                eax = m.r32(ebp);
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x74u32));
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x4cu32);
                m.w32(esp.wrapping_add(0x50u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0x10u32);
                edx = esp.wrapping_add(0x98u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 39; continue;
            }
            39 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x90u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x90u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x98u32);
                m.w32(esp.wrapping_add(0x90u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x88u32));
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
                esp = esp.wrapping_add(0x84);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C88C0` combo_box_slot3_return_true
pub fn f_004c88c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004C88E0` combo_box_draw
pub fn f_004c88e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x64);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x7cu32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                ebp = m.r32(esp.wrapping_add(0x80u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                edi = m.r32(esp.wrapping_add(0x38u32));
                eax = ebx.wrapping_add(ecx);
                ecx = edx.wrapping_add(ebp);
                edx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esp.wrapping_add(0x64u32), eax);
                m.w32(esp.wrapping_add(0x68u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x6cu32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x70u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x78u32));
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x90u32));
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(edx, ebx);
                m.w32(edx.wrapping_add(0x4u32), ebp);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(0xc);
                ecx = m.r32(esp.wrapping_add(0x90u32));
                eax = esp;
                edx = m.r32(esi.wrapping_add(0x544u32));
                edi = esi.wrapping_add(0x544u32);
                m.w32(eax, ebx);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ecx);
                eax = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x180u32));
                ebx = esi.wrapping_add(0x180u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esp.wrapping_add(0x7cu32));
                esp = esp.wrapping_sub(0xc);
                ecx = esp;
                eax = m.r32(ebx);
                m.w32(ecx, edx);
                edx = ebp;
                m.w32(ecx.wrapping_add(0x4u32), edx);
                edx = m.r32(esp.wrapping_add(0x90u32));
                m.w32(ecx.wrapping_add(0x8u32), edx);
                ecx = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ebp = m.r32(esp.wrapping_add(0x7cu32));
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                edx = m.r32(esi.wrapping_add(0x94u32));
                ecx = esi.wrapping_add(0x94u32);
                m.w32(eax, ebp);
                ebp = m.r32(esp.wrapping_add(0x8cu32));
                m.w32(eax.wrapping_add(0x4u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x90u32));
                m.w32(eax.wrapping_add(0x8u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x628u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x4cd9e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x630u32));
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4cd9b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x63cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x3u32, 32, fl); fl = f;
                ecx = r;
                edx = ecx.wrapping_add(0x6dcd54u32);
                ecx = ecx.wrapping_add(0x6dcd28u32);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = esi.wrapping_add(0x61cu32);
                edi = m.r32(edx.wrapping_add(0x4u32));
                ebx = m.r32(eax);
                edx = m.r32(edx.wrapping_add(0x8u32));
                ebp = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), edx);
                edx = m.r32(eax.wrapping_add(0x8u32));
                eax = m.r32(esp.wrapping_add(0x50u32));
                m.w32(esp.wrapping_add(0x4cu32), edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x80u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esp.wrapping_add(0x60u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x20u32), ebx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sar(edx, 0x1u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                ebp = m.r32(esp.wrapping_add(0x84u32));
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x62cu32));
                esi = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                edx = m.r32(edx.wrapping_add(esi.wrapping_mul(4)));
                edx = m.r32(edx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4c1780, ecx, &a); }
                esp = esp.wrapping_add(20);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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
                esp = esp.wrapping_add(0x64);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C8FE0` combo_box_clear_items
pub fn f_004c8fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esi.wrapping_add(0x180u32);
                { let a = []; eax = h.call_at(esp, m, 0x4cddc0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x62cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x630u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x62cu32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
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
                eax = m.r32(esi.wrapping_add(0x630u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x62cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0x62cu32), 0x0u32);
                m.w32(esi.wrapping_add(0x630u32), 0x0u32);
                m.w8(esi.wrapping_add(0x634u32), (0x0u32) as u8);
                m.w32(esi.wrapping_add(0x638u32), 0xffffffffu32);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C9080` combo_box_get_class_name
pub fn f_004c9080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bf9dcu32);
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

/// `0X004C90B0` combo_box_scalar_deleting_dtor
pub fn f_004c90b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4c7ed0, ecx, &a); }
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

/// `0X004C90D0` combo_box_add_dirty_rect
pub fn f_004c90d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1980, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x180u32));
                let (r, f) = lift::add(esi, 0x180u32, 32); fl = f;
                esi = r;
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b2220, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
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

/// `0X004C9160` combo_box_get_total_size
pub fn f_004c9160(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(ebx.wrapping_add(0x544u32));
                ecx = ebx.wrapping_add(0x544u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx.wrapping_add(0x180u32));
                edi = ebx.wrapping_add(0x180u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esp.wrapping_add(0xcu32));
                ecx = m.r32(esi);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esi, eax);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esi.wrapping_add(0x4u32));
                ecx = m.r32(esi.wrapping_add(0x8u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0x4u32), edx);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esi.wrapping_add(0x8u32), eax);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(ebx.wrapping_add(0x94u32));
                ecx = ebx.wrapping_add(0x94u32);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xcu32));
                ecx = m.r32(esi);
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w32(esi, eax);
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(edx, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                m.w32(esi.wrapping_add(0x4u32), edx);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esi.wrapping_add(0x8u32));
                fl = lift::sub(ecx, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w32(esi.wrapping_add(0x8u32), ecx);
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(eax, m.r32(esi), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                m.w32(esi, eax);
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0x4u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w32(esi.wrapping_add(0x4u32), edx);
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(ecx, m.r32(esi.wrapping_add(0x8u32)), 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                m.w32(esi.wrapping_add(0x8u32), ecx);
                pc = 19; continue;
            }
            19 => {
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

/// `0X004C9240` combo_box_set_size_from_triple
pub fn f_004c9240(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = m.r32(esp.wrapping_add(0xcu32));
    esi = ecx;
    ecx = m.r32(edi.wrapping_add(0x8u32));
    edx = m.r32(edi.wrapping_add(0x4u32));
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = m.r32(edi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0x4cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(12);
    edx = m.r32(esi.wrapping_add(0x180u32));
    ecx = esi.wrapping_add(0x180u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004C9790` edit_box_ctor_default
pub fn f_004c9790(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662362u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfac8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2040, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = esi.wrapping_add(0x100u32);
    m.w32(esp.wrapping_add(0x14u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi.wrapping_add(0x120u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x164u32), ecx);
    eax = 0x7fffffffu32;
    m.w32(esi, 0x671890u32);
    m.w32(esi.wrapping_add(0x168u32), 0xffffffffu32);
    m.w32(esi.wrapping_add(0x16cu32), eax);
    m.w32(esi.wrapping_add(0x170u32), eax);
    m.w8(esi.wrapping_add(0x179u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x140u32), ebx);
    m.w32(esi.wrapping_add(0x144u32), ebx);
    m.w32(esi.wrapping_add(0x148u32), ebx);
    m.w32(esi.wrapping_add(0x14cu32), ebx);
    m.w32(esi.wrapping_add(0x150u32), ebx);
    m.w32(esi.wrapping_add(0x154u32), ebx);
    edx = m.r32(0x670f24u32);
    ecx = esi;
    m.w32(esi.wrapping_add(0x158u32), edx);
    eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15bu32)))) & 0xff);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    m.w8(esi.wrapping_add(0x15cu32), ((eax & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x178u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x17bu32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x160u32), 0x3e8u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
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

/// `0X004C9880` edit_box_ctor_named
pub fn f_004c9880(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66239eu32);
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
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2040, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = esi.wrapping_add(0x100u32);
    m.w32(esp.wrapping_add(0x14u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi.wrapping_add(0x120u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x164u32), ecx);
    eax = 0x7fffffffu32;
    m.w32(esi, 0x671890u32);
    m.w32(esi.wrapping_add(0x168u32), 0xffffffffu32);
    m.w32(esi.wrapping_add(0x16cu32), eax);
    m.w32(esi.wrapping_add(0x170u32), eax);
    m.w8(esi.wrapping_add(0x179u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x140u32), ebx);
    m.w32(esi.wrapping_add(0x144u32), ebx);
    m.w32(esi.wrapping_add(0x148u32), ebx);
    m.w32(esi.wrapping_add(0x14cu32), ebx);
    m.w32(esi.wrapping_add(0x150u32), ebx);
    m.w32(esi.wrapping_add(0x154u32), ebx);
    edx = m.r32(0x670f24u32);
    ecx = esi;
    m.w32(esi.wrapping_add(0x158u32), edx);
    eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15bu32)))) & 0xff);
    m.w8(esp.wrapping_add(0x14u32), (0x3u32) as u8);
    m.w8(esi.wrapping_add(0x15cu32), ((eax & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x178u32), ((ebx & 0xff)) as u8);
    m.w8(esi.wrapping_add(0x17bu32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0x160u32), 0x3e8u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
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

/// `0X004C9970` edit_box_dtor
pub fn f_004c9970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6623ccu32);
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
    m.w32(esi, 0x671890u32);
    ecx = esi.wrapping_add(0x164u32);
    m.w32(esp.wrapping_add(0x10u32), 0x2u32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0x120u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
    ecx = esi.wrapping_add(0x100u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4d20a0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004C99F0` edit_box_reset_state
pub fn f_004c99f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = 0x7fffffffu32;
    m.w32(ecx.wrapping_add(0x168u32), 0xffffffffu32);
    m.w32(ecx.wrapping_add(0x16cu32), eax);
    m.w32(ecx.wrapping_add(0x170u32), eax);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w8(ecx.wrapping_add(0x179u32), ((eax & 0xff)) as u8);
    m.w32(ecx.wrapping_add(0x140u32), eax);
    m.w32(ecx.wrapping_add(0x144u32), eax);
    m.w32(ecx.wrapping_add(0x148u32), eax);
    m.w32(ecx.wrapping_add(0x14cu32), eax);
    m.w32(ecx.wrapping_add(0x150u32), eax);
    m.w32(ecx.wrapping_add(0x154u32), eax);
    edx = m.r32(0x670f24u32);
    m.w32(ecx.wrapping_add(0x158u32), edx);
    edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x15bu32)))) & 0xff);
    m.w8(ecx.wrapping_add(0x15cu32), ((edx & 0xff)) as u8);
    m.w8(ecx.wrapping_add(0x178u32), ((eax & 0xff)) as u8);
    m.w8(ecx.wrapping_add(0x17bu32), ((eax & 0xff)) as u8);
    m.w32(ecx.wrapping_add(0x160u32), 0x3e8u32);
    { let a = []; return h.call_at(esp, m, 0x4d2100, ecx, &a); }
    eax
}

/// `0X004C9A70` edit_box_load_from_ini
pub fn f_004c9a70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6623e6u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x20);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = 0x7fffffffu32;
                m.w32(esi.wrapping_add(0x168u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x16cu32), eax);
                m.w32(esi.wrapping_add(0x170u32), eax);
                m.w8(esi.wrapping_add(0x179u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x140u32), ebx);
                m.w32(esi.wrapping_add(0x144u32), ebx);
                m.w32(esi.wrapping_add(0x148u32), ebx);
                m.w32(esi.wrapping_add(0x14cu32), ebx);
                m.w32(esi.wrapping_add(0x150u32), ebx);
                m.w32(esi.wrapping_add(0x154u32), ebx);
                eax = m.r32(0x670f24u32);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                m.w32(esi.wrapping_add(0x158u32), eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15bu32)))) & 0xff);
                m.w8(esi.wrapping_add(0x15cu32), ((ecx & 0xff)) as u8);
                ecx = esi;
                m.w8(esi.wrapping_add(0x178u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x17bu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x160u32), 0x3e8u32);
                { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x44u32));
                edx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x4cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d21f0, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esp.wrapping_add(0x44u32), ((eax & 0xff)) as u8);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(0x6c0040u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x44u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x50u32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(0x670f24u32);
                eax = m.r32(0x6bfa98u32);
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
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                m.w32(esi.wrapping_add(0x158u32), eax);
                let (r, f) = lift::shr(eax, 0x18u32, 32, fl); fl = f;
                eax = r;
                m.w8(esi.wrapping_add(0x15cu32), ((eax & 0xff)) as u8);
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x28u32));
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(edi, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x28u32));
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                ecx = esi;
                m.w32(esi.wrapping_add(0x14cu32), 0x1u32);
                m.w32(esi.wrapping_add(0x150u32), edi);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x44u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 18; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(0x6bfab4u32);
                edx = m.r32(0x6bfaa8u32);
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
                if fl.zf { pc = 18; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = esp.wrapping_add(0x1cu32);
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bfaccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                edi = m.r32(esp.wrapping_add(0x1cu32));
                fl = lift::sub(ebp, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ebp = 0x1u32;
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x28u32));
                pc = 12; continue;
            }
            12 => {
                fl = lift::sub(ebp, ebx, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                m.w32(esp.wrapping_add(0x14u32), 0x1u32);
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(edi, ebx, 32).1;
                if (fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(edi, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x28u32));
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esi);
                m.w32(esi.wrapping_add(0x14cu32), ecx);
                ecx = esi;
                m.w32(esi.wrapping_add(0x150u32), edi);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 18; continue;
            }
            18 => {
                edi = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xd8u32));
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esi.wrapping_add(0x178u32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x174u32), eax);
                m.w8(esp.wrapping_add(0x38u32), ((ebx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x40u32);
                m.w32(esp.wrapping_add(0x38u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x44u32)))) & 0xff);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x2c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C9DB0` edit_box_slot3_return_false
pub fn f_004c9db0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004C9F80` edit_box_on_mouse_event
pub fn f_004c9f80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662400u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x24u32));
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1720, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::sub(edi, 0x1u32, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(ebx);
                edi = ecx;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esi.wrapping_add(0xf0u32));
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                if fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esi.wrapping_add(0x94u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                pc = 5; continue;
            }
            4 => {
                edi = ecx;
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0xfffffff8u32)), ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(eax);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w8(esp.wrapping_add(0x1cu32), (0x0u32) as u8);
                edx = m.r32(ecx);
                ecx = esp.wrapping_add(0x10u32);
                fl = lift::sub(eax, edx, 32).1;
                ebx = (ebx & !0xff) | ((u32::from((!fl.zf && fl.sf == fl.of))) & 0xff);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esi.wrapping_add(0x168u32), ebp);
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, m.r32(edx.wrapping_add(0xfffffff8u32)), 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                ebx = m.r32(eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 12; continue;
            }
            10 => {
                fl = lift::sub(ebx, edi, 32).1;
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esi.wrapping_add(0x168u32), 0x0u32);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x18u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X004CA110` edit_box_handle_key
pub fn f_004ca110(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66248au32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 36; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4c0f50, ecx, &a); }
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                eax = m.r32(esi);
                esp = esp.wrapping_add(0xc);
                ecx = esi;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                ecx = ebx.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x38u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x168u32));
                m.w32(esp.wrapping_add(0x24u32), 0x0u32);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
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
                fl = lift::sub(ebp, 0x25u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 35; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 35; continue;
            }
            7 => {
                fl = lift::sub(ebp, 0x27u32, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                ecx = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 35; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esi.wrapping_add(0x168u32), ecx);
                pc = 35; continue;
            }
            10 => {
                fl = lift::sub(ebp, 0x2eu32, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                edi = m.r32(esi.wrapping_add(0x168u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x10u32);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                eax = m.r32(esi.wrapping_add(0x168u32));
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x2cu32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x28u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x24u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w8(esp.wrapping_add(0x24u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x17bu32), (0x0u32) as u8);
                pc = 35; continue;
            }
            12 => {
                fl = lift::sub(ebp, 0x8u32, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 35; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x3cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = eax;
                edx = m.r32(esi.wrapping_add(0x168u32));
                eax = esp.wrapping_add(0x2cu32);
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x2cu32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w8(esp.wrapping_add(0x30u32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x28u32), (0x6u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x24u32), (0x5u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x2cu32);
                m.w8(esp.wrapping_add(0x24u32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x168u32));
                m.w8(esi.wrapping_add(0x17bu32), (0x0u32) as u8);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 35; continue;
            }
            15 => {
                fl = lift::sub(ebp, 0x1bu32, 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = esi.wrapping_add(0x120u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0500, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 35; continue;
            }
            17 => {
                fl = lift::sub(edi, 0x20u32, 32).1;
                if (fl.sf != fl.of) { pc = 29; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(edi, 0xffu32, 32).1;
                if (fl.sf == fl.of) { pc = 29; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(esi.wrapping_add(0x164u32));
                ecx = esi.wrapping_add(0x164u32);
                ebp = m.r32(edx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f6ce, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                eax = m.r32(esp.wrapping_add(0x34u32));
                if fl.zf { pc = 29; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                edx = m.r32(esi.wrapping_add(0x168u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x2cu32), (0x7u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0x168u32));
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x2cu32), (0x8u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0x9u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64cb60, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x34u32);
                m.w8(esp.wrapping_add(0x2cu32), (0xau32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x34u32);
                m.w8(esp.wrapping_add(0x28u32), (0xbu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x2cu32);
                m.w8(esp.wrapping_add(0x24u32), (0xau32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), (0x9u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x24u32), (0x8u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x24u32), (0x7u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x179u32)))) & 0xff);
                m.w8(esi.wrapping_add(0x17bu32), (0x0u32) as u8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0x174u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x170u32)), 32).1;
                if (fl.sf != fl.of) { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w8(esi.wrapping_add(0x17bu32), (0x1u32) as u8);
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x16cu32));
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                m.w8(esi.wrapping_add(0x17bu32), (0x1u32) as u8);
                pc = 26; continue;
            }
            26 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x17bu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esi.wrapping_add(0x168u32));
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                ecx = esp.wrapping_add(0x34u32);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d180, ecx, &a); }
                esp = esp.wrapping_add(8);
                ebp = eax;
                eax = m.r32(esi.wrapping_add(0x168u32));
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x3cu32);
                m.w8(esp.wrapping_add(0x2cu32), (0xcu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x30u32), (0xdu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64cb60, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x34u32);
                m.w8(esp.wrapping_add(0x2cu32), (0xeu32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64f436, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x38u32);
                m.w8(esp.wrapping_add(0x28u32), (0xfu32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x2cu32);
                m.w8(esp.wrapping_add(0x24u32), (0xeu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x24u32), (0xdu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x24u32), (0xcu32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x24u32), (0x7u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0x168u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x168u32), r);
                pc = 28; continue;
            }
            28 => {
                ecx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x24u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 35; continue;
            }
            29 => {
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
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc0u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 35; continue;
            }
            31 => {
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
                if fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                eax = m.r32(eax.wrapping_add(0xfffffff8u32));
                m.w32(esi.wrapping_add(0x168u32), eax);
                pc = 35; continue;
            }
            33 => {
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
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                m.w32(esi.wrapping_add(0x168u32), 0x0u32);
                pc = 35; continue;
            }
            35 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1b10, ecx, &a); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x34u32);
                m.w32(esp.wrapping_add(0x24u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x18);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CA640` edit_box_on_focus_gained
pub fn f_004ca640(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66249cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x168u32));
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x4u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x15bu32)))) & 0xff);
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                ecx = esi;
                m.w32(esi.wrapping_add(0x168u32), eax);
                eax = m.r32(esi);
                m.w8(esi.wrapping_add(0x15cu32), ((edx & 0xff)) as u8);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x120u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0500, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w8(0x6dd5deu32, (0x1u32) as u8);
                ecx = esp.wrapping_add(0x4u32);
                m.w8(esi.wrapping_add(0x42u32), (0x1u32) as u8);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x8u32));
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X004CA700` edit_box_layout_cursor
pub fn f_004ca700(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6624d6u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4d2fe0, ecx, &a); }
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 22; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x168u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esi.wrapping_add(0xb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x20u32), ebp);
                m.w32(esp.wrapping_add(0x14u32), ebp);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x40u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                ebx = esi.wrapping_add(0x94u32);
                edi = esi.wrapping_add(0x140u32);
                eax = ebx;
                ecx = edi;
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                edx = m.r32(eax);
                m.w32(ecx, edx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(ecx.wrapping_add(0x4u32), edx);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(ecx.wrapping_add(0x8u32), eax);
                eax = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, m.r32(ecx.wrapping_add(0xfffffff8u32)), 32).1;
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d0c0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x40u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = ebp;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x168u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(eax, m.r32(edx.wrapping_add(0xfffffff8u32)), 32).1;
                if (fl.sf == fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64d0c0, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x40u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x168u32));
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x64d230, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x40u32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x1cu32);
                m.w8(esp.wrapping_add(0x3cu32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38u32));
                ebp = eax;
                eax = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(edi);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(edi, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                edx = m.r32(esi.wrapping_add(0x150u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = esi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x144u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esi.wrapping_add(0x144u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x4d2e70, ecx, &a); }
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 18; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 16; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 21; continue; }
                pc = 11; continue;
            }
            11 => {
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(edi);
                eax = ebp;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi, ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x14cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                fl = lift::sub(eax, ecx, 32).1;
                ecx = esi;
                if (fl.zf || fl.sf != fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x14cu32));
                edx = ((eax as i32) >> 31) as u32;
                ebp = m.r32(ebx);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                ecx = esi;
                m.w32(ebx, ebp);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x14cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(edi, ecx);
                pc = 21; continue;
            }
            13 => {
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 21; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(edi);
                fl = lift::logic(ecx & ecx, 32);
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0x14cu32));
                m.w32(edi, 0x0u32);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(ebx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(ebx, ecx);
                pc = 21; continue;
            }
            16 => {
                fl = lift::logic(ebp & ebp, 32);
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(ebx);
                m.w32(edi, 0x0u32);
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(ebx, eax);
                pc = 21; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x14cu32));
                ecx = esi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.sf == fl.of) { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(ebx, eax);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(edi, eax);
                pc = 21; continue;
            }
            21 => {
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x30);
                return eax;
            }
            22 => {
                ecx = esi.wrapping_add(0x94u32);
                let (r, f) = lift::add(esi, 0x140u32, 32); fl = f;
                esi = r;
                edx = m.r32(ecx);
                m.w32(esi, edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esi.wrapping_add(0x4u32), eax);
                ecx = m.r32(ecx.wrapping_add(0x8u32));
                m.w32(esi.wrapping_add(0x8u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x30);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CAAD0` edit_box_set_text
pub fn f_004caad0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6624e8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    eax = m.r32(esp.wrapping_add(0x10u32));
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2ed0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi);
    ecx = esi;
    ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
    { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
    eax = r;
    ecx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esp.wrapping_add(0x18u32));
    edx = m.r32(esi);
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    eax = m.r32(eax.wrapping_add(0xfffffff8u32));
    m.w32(esi.wrapping_add(0x168u32), eax);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    ecx = esp.wrapping_add(0x18u32);
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0xc);
    return eax;
    eax
}

/// `0X004CABC0` edit_box_set_cursor_color
pub fn f_004cabc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0x158u32), eax);
    let (r, f) = lift::shr(eax, 0x18u32, 32, fl); fl = f;
    eax = r;
    m.w8(ecx.wrapping_add(0x15cu32), ((eax & 0xff)) as u8);
    return eax;
    eax
}

/// `0X004CACF0` edit_box_set_cursor_size
pub fn f_004cacf0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::sub(ebx, 0xffffffffu32, 32).1;
                esi = ecx;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ebx = 0x1u32;
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x10u32));
                pc = 4; continue;
            }
            4 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ebx = 0x1u32;
                pc = 6; continue;
            }
            6 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(edi, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x10u32));
                pc = 9; continue;
            }
            9 => {
                edx = m.r32(esi);
                ecx = esi;
                m.w32(esi.wrapping_add(0x14cu32), ebx);
                m.w32(esi.wrapping_add(0x150u32), edi);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CADC0` edit_box_scalar_deleting_dtor
pub fn f_004cadc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4c9970, ecx, &a); }
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

/// `0X004CADE0` edit_box_set_dirty_rect_unless_focused
pub fn f_004cade0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1b50, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CAE00` edit_box_clear_dirty_rect_unless_focused
pub fn f_004cae00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x4b1ba0, ecx, &a); }
            }
            2 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CAEA0` list_box_ctor_default
pub fn f_004caea0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662524u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bfc10u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi.wrapping_add(0xb4u32);
    m.w32(esp.wrapping_add(0x14u32), edi);
    m.w32(esi.wrapping_add(0x94u32), 0x671ad0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b9840, ecx, &a); }
    ecx = esi.wrapping_add(0x250u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b2480, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    m.w32(esi, 0x6719b8u32);
    m.w32(esi.wrapping_add(0x94u32), 0x6719b4u32);
    m.w32(esi.wrapping_add(0x98u32), edi);
    m.w32(esi.wrapping_add(0xa0u32), edi);
    m.w32(esi.wrapping_add(0xacu32), edi);
    m.w32(esi.wrapping_add(0xa8u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
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

/// `0X004CAF40` list_box_ctor_named
pub fn f_004caf40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662552u32);
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
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi.wrapping_add(0xb4u32);
    m.w32(esp.wrapping_add(0x14u32), edi);
    m.w32(esi.wrapping_add(0x94u32), 0x671ad0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b9840, ecx, &a); }
    ecx = esi.wrapping_add(0x250u32);
    m.w8(esp.wrapping_add(0x14u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b2480, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x14u32), (0x2u32) as u8);
    m.w32(esi, 0x6719b8u32);
    m.w32(esi.wrapping_add(0x94u32), 0x6719b4u32);
    m.w32(esi.wrapping_add(0x98u32), edi);
    m.w32(esi.wrapping_add(0xa0u32), edi);
    m.w32(esi.wrapping_add(0xacu32), edi);
    m.w32(esi.wrapping_add(0xa8u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
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

/// `0X004CAFE0` list_box_dtor
pub fn f_004cafe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662580u32);
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
    m.w32(esi, 0x6719b8u32);
    m.w32(esi.wrapping_add(0x94u32), 0x6719b4u32);
    m.w32(esp.wrapping_add(0x10u32), 0x2u32);
    { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
    ecx = esi.wrapping_add(0x250u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b2680, ecx, &a); }
    ecx = esi.wrapping_add(0xb4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b98e0, ecx, &a); }
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

/// `0X004CB060` list_box_reset_state
pub fn f_004cb060(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 9; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                fl = lift::sub(m.r32(eax.wrapping_add(ebp.wrapping_mul(4))), ebx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebp.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::sub(ebp, eax, 32).1;
                if (fl.sf != fl.of) { pc = 2; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xa0u32)), ebx, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::sub(eax, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0xa8u32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esi.wrapping_add(0xa8u32), ebx);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 20; continue;
            }
            20 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                ecx = esi.wrapping_add(0x23cu32);
                eax = 0xff808000u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esi.wrapping_add(0xacu32), ebx);
                m.w32(esi.wrapping_add(0xb0u32), ebx);
                m.w32(esi.wrapping_add(0x98u32), ebx);
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w32(esi.wrapping_add(0xa8u32), ebx);
                m.w32(esi.wrapping_add(0x9cu32), ebx);
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                m.w32(esi.wrapping_add(0x200u32), eax);
                m.w32(esi.wrapping_add(0x20cu32), eax);
                m.w32(esi.wrapping_add(0x204u32), 0xff008080u32);
                m.w32(esi.wrapping_add(0x208u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x210u32), ebx);
                m.w8(esi.wrapping_add(0x214u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x228u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x308u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x234u32), ebx);
                m.w32(esi.wrapping_add(0x238u32), ebx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                edi = esi.wrapping_add(0x250u32);
                esp = esp.wrapping_add(0xc);
                ecx = edi;
                m.w8(esi.wrapping_add(0x215u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x218u32), 0x1u32);
                m.w32(esi.wrapping_add(0x21cu32), ebx);
                m.w32(esi.wrapping_add(0x230u32), ebx);
                m.w8(esi.wrapping_add(0x310u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0x312u32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0x313u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x248u32), 0x80000000u32);
                m.w32(esi.wrapping_add(0x24cu32), 0x40000000u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b2580, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2970, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xb4u32);
                m.w8(esi.wrapping_add(0x310u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4b9930, ecx, &a); }
                edx = m.r32(esi);
                eax = esi.wrapping_add(0x94u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0x1fcu32), eax);
                m.w32(esi.wrapping_add(0x30cu32), ebx);
                m.w8(esi.wrapping_add(0x310u32), (0x1u32) as u8);
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                m.w8(esi.wrapping_add(0x220u32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0x224u32), ebx);
                m.w32(esi.wrapping_add(0x300u32), ebx);
                m.w32(esi.wrapping_add(0x304u32), ebx);
                m.w32(esi.wrapping_add(0x30cu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1c40, ecx, &a); }
                esp = esp.wrapping_add(4);
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

/// `0X004CB770` list_box_load_from_ini
pub fn f_004cb770(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6625aau32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x3c);
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
                m.w32(esp.wrapping_add(0x54u32), ebp);
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x4cb060, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                fl = lift::sub(m.r32(ecx.wrapping_add(0xfffffff8u32)), 0x5u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6beee0u32);
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x60u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x60u32), ebp);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x60u32));
                ecx = m.r32(0x6bfae0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(0x6bfaecu32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x200u32), eax);
                m.w32(esi.wrapping_add(0x20cu32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb04u32);
                ebx = 0x1u32;
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
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x204u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfaf8u32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x250u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b2a20, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfb14u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                m.w32(esi.wrapping_add(0x210u32), eax);
                edx = m.r32(0x6bfb28u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esi.wrapping_add(0xb0u32), eax);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                edx = m.r32(esp.wrapping_add(0x5cu32));
                eax = m.r32(0x6dd5e4u32);
                ecx = m.r32(0x6bfb98u32);
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
                m.w8(esp.wrapping_add(0x6cu32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e1e0, ecx, &a); }
                eax = m.r32(0x6bec38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(0x6bec40u32);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w8(esi.wrapping_add(0x228u32), (0x0u32) as u8);
                pc = 8; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0x228u32), (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esp.wrapping_add(0x5cu32));
                eax = m.r32(0x6bfb7cu32);
                ecx = m.r32(0x6bfb74u32);
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
                eax = esp.wrapping_add(0x2cu32);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bfc14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esp.wrapping_add(0x38u32), ebp);
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                m.w32(esp.wrapping_add(0x40u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x6cu32));
                eax = m.r32(0x6bfb8cu32);
                ecx = m.r32(0x6bfb84u32);
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w32(esp.wrapping_add(0x24u32), ebx);
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
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = esp.wrapping_add(0x24u32);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bfc1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                m.w32(esp.wrapping_add(0x38u32), eax);
                m.w32(esp.wrapping_add(0x34u32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x40u32), ebp);
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfba8u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0xb4u32));
                edi = esi.wrapping_add(0xb4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esi.wrapping_add(0x234u32), eax);
                { let t = m.r32(edx.wrapping_add(0x58u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb38u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
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
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w8(esi.wrapping_add(0x220u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb68u32);
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
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x224u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbe0u32);
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
                ebx = eax;
                fl = lift::sub(ebx, 0x2u32, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ebx = 0x2u32;
                pc = 12; continue;
            }
            12 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = esi.wrapping_add(0x248u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                edx = m.r32(0x6bfbecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x60u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x68u32));
                edx = m.r32(0x6dd5e0u32);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_add(0xc);
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
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                m.w32(edx, eax);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(ebp, ebx, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbfcu32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic(eax & eax, 32);
                m.w32(esi.wrapping_add(0x218u32), eax);
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                m.w32(esi.wrapping_add(0x218u32), 0x0u32);
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb58u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ba6e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfb48u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ba710, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(0x6bfbbcu32);
                edx = m.r32(esp.wrapping_add(0x10u32));
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
                ecx = esp.wrapping_add(0x28u32);
                ebx = eax;
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esi.wrapping_add(0x238u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(0x6bfbd0u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi);
                ecx = esi;
                ebx = eax;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(ebx, eax, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                fl = lift::logic(ebx & ebx, 32);
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                ecx = edi;
                ebp = eax;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                eax = esi.wrapping_add(0x23cu32);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                fl = lift::logic(ebx & ebx, 32);
                m.w32(eax, ebp);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                let (r, f) = lift::add(ebx, 0xfffffffeu32, 32); fl = f;
                ebx = r;
                m.w32(esi.wrapping_add(0x240u32), ebx);
                pc = 23; continue;
            }
            22 => {
                m.w32(esi.wrapping_add(0x240u32), 0x0u32);
                pc = 23; continue;
            }
            23 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x5cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x60u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x54u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x54u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x5cu32);
                m.w32(esp.wrapping_add(0x54u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x4cu32));
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
                esp = esp.wrapping_add(0x48);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CBCE0` list_box_slot3_return_true
pub fn f_004cbce0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004CBD00` list_box_draw
pub fn f_004cbd00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
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
                if fl.zf { pc = 34; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x88u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                edi = m.r32(esp.wrapping_add(0x8cu32));
                ebx = m.r32(esp.wrapping_add(0x50u32));
                eax = ecx.wrapping_add(ebp);
                ecx = edi.wrapping_add(edx);
                edx = m.r32(esp.wrapping_add(0x4cu32));
                m.w32(esp.wrapping_add(0x70u32), eax);
                m.w32(esp.wrapping_add(0x74u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x78u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x7cu32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 34; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = esp.wrapping_add(0x70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 34; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(0xc);
                ebx = m.r32(esp.wrapping_add(0x90u32));
                eax = m.r32(esp.wrapping_add(0x9cu32));
                edx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                m.w32(edx, ebp);
                m.w32(edx.wrapping_add(0x4u32), edi);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x8);
                m.w8(esp.wrapping_add(0x20u32), ((eax & 0xff)) as u8);
                eax = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x215u32)))) & 0xff);
                esp = esp.wrapping_add(0xc);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 12; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xa4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 12; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                pc = 7; continue;
            }
            7 => {
                ebx = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(m.r32(ebx.wrapping_add(eax.wrapping_mul(4))), 0x0u32, 32).1;
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ebx = m.r32(esi.wrapping_add(0xacu32));
                let (r, f) = lift::add(ecx, m.r32(ebx.wrapping_add(eax.wrapping_mul(4))), 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), ecx);
                pc = 9; continue;
            }
            9 => {
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                fl = lift::logic(ebx & ebx, 32);
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(esi.wrapping_add(0xb0u32));
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xc4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x200u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 15; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x204u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 15; continue;
            }
            15 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x34u32));
                ebx = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x90u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esp.wrapping_add(0x88u32), ebp);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x8cu32), edi);
                m.w32(esp.wrapping_add(0x90u32), edx);
                m.w32(esp.wrapping_add(0x5cu32), edi);
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x215u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(edx.wrapping_add(ebx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(ecx);
                edx = esp;
                m.w32(edx, ebp);
                m.w32(edx.wrapping_add(0x4u32), edi);
                edi = m.r32(esp.wrapping_add(0x9cu32));
                m.w32(edx.wrapping_add(0x8u32), edi);
                edx = m.r32(esp.wrapping_add(0x90u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xdcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp.wrapping_add(0x8cu32));
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 16; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ebx = m.r32(esi.wrapping_add(0x230u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0xb4u32);
                m.w32(esp.wrapping_add(0x5cu32), eax);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x230u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(esp.wrapping_add(0x10u32), eax);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = ecx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                pc = 22; continue;
            }
            22 => {
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 30; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 28; continue; }
                pc = 24; continue;
            }
            24 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(ecx);
                eax = esp;
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = ebp;
                ebp = esp;
                m.w32(ebp, eax);
                eax = m.r32(esp.wrapping_add(0x68u32));
                m.w32(ebp.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x9cu32));
                m.w32(ebp.wrapping_add(0x8u32), eax);
                eax = m.r32(esp.wrapping_add(0x90u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
                ecx = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xdcu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = m.r32(esp.wrapping_add(0x88u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 24; continue; }
                pc = 28; continue;
            }
            28 => {
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 29; continue;
            }
            29 => {
                edi = m.r32(esp.wrapping_add(0x8cu32));
                pc = 30; continue;
            }
            30 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x228u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                esp = esp.wrapping_sub(0xc);
                esi = m.r32(esp.wrapping_add(0x9cu32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                eax = esp;
                edx = m.r32(ecx);
                m.w32(eax, ebp);
                m.w32(eax.wrapping_add(0x4u32), edi);
                m.w32(eax.wrapping_add(0x8u32), esi);
                eax = m.r32(esp.wrapping_add(0x90u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 34; continue;
            }
            34 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x70);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004CC040` list_box_add_dirty_rect
pub fn f_004cc040(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0x230u32));
                edi = esi.wrapping_add(0xb4u32);
                ecx = edi;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), edi);
                { let a = []; eax = h.call_at(esp, m, 0x4ba040, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x230u32));
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0xcu32), ecx);
                if (fl.zf || fl.sf != fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(esp.wrapping_add(0xcu32), eax);
                pc = 2; continue;
            }
            2 => {
                ecx = esi.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let t = m.r32(0x66a4d0u32); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 17; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b2220, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1980, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x215u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 9; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 9; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = edi;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf == fl.of) { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 15; continue;
            }
            15 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                edx = m.r32(ebp.wrapping_add(0x8u32));
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                esi = m.r32(esp.wrapping_add(0x24u32));
                ebx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x30u32), edx);
                edx = m.r32(edi);
                eax = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xa0u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x24);
                return eax;
            }
            17 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1980, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
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

/// `0X004CC8B0` list_box_remove_row_or_cell
pub fn f_004cc8b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x10u32));
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0x9cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ebx = m.r32(esp.wrapping_add(0x14u32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::logic(ebx & ebx, 32);
                if !fl.zf { pc = 10; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                ecx = m.r32(ecx.wrapping_add(ebx.wrapping_mul(4)));
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
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                m.w32(ecx.wrapping_add(ebx.wrapping_mul(4)).wrapping_add(0xfffffffcu32), 0x0u32);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x4);
                m.w32(ecx.wrapping_add(edi.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                edx = eax.wrapping_add(0xffffffffu32);
                fl = lift::sub(edi, edx, 32).1;
                if (fl.sf == fl.of) { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x4u32));
                eax = eax.wrapping_mul(4).wrapping_add(0xfffffff8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                pc = 9; continue;
            }
            9 => {
                edi = m.r32(esi.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(esi.wrapping_add(0x9cu32), edi);
                edi = esi.wrapping_add(0xb4u32);
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
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            10 => {
                edx = m.r32(esi.wrapping_add(0x98u32));
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                ecx = m.r32(eax.wrapping_add(ebx.wrapping_mul(4)));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x98u32));
                ecx = m.r32(eax.wrapping_add(edi.wrapping_mul(4)));
                m.w32(ecx.wrapping_add(ebx.wrapping_mul(4)), 0x0u32);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 13; continue;
            }
            13 => {
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

/// `0X004CCAA0` list_box_add_column
pub fn f_004ccaa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6625d4u32);
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebp = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(esp.wrapping_add(0x18u32), ebp);
                ebx = ebp.wrapping_mul(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x100u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x28u32), 0x0u32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4d1fe0, ecx, &a); }
                pc = 3; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                m.w32(esp.wrapping_add(0x28u32), 0xffffffffu32);
                m.w32(edi.wrapping_add(ecx.wrapping_mul(4)), eax);
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), 0x0u32);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esi.wrapping_add(0xacu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa4u32));
                ecx = m.r32(esi.wrapping_add(0xa8u32));
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::shl(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esi.wrapping_add(0x204u32));
                m.w32(esi.wrapping_add(0xacu32), eax);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                esp = esp.wrapping_add(0x30);
                m.w32(esi.wrapping_add(0xa0u32), edi);
                m.w32(esi.wrapping_add(0xa8u32), ecx);
                ecx = m.r32(edi.wrapping_add(eax.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d70, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x200u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2d80, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x204u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2db0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x210u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(eax.wrapping_add(edx.wrapping_mul(4)));
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d2e20, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = m.r32(edx.wrapping_add(ecx.wrapping_mul(4)));
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                edx = m.r32(esi.wrapping_add(0x98u32));
                ebp = eax;
                eax = m.r32(edx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::shl(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x98u32));
                esp = esp.wrapping_add(0x14);
                m.w32(eax.wrapping_add(edi.wrapping_mul(4)), ebp);
                ecx = m.r32(esi.wrapping_add(0x98u32));
                edx = m.r32(esi.wrapping_add(0xa4u32));
                eax = m.r32(ecx.wrapping_add(edi.wrapping_mul(4)));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                m.w32(eax.wrapping_add(edx.wrapping_mul(4)), 0x0u32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                fl = lift::sub(edi, eax, 32).1;
                if (fl.sf != fl.of) { pc = 4; continue; }
                pc = 5; continue;
            }
            5 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0xa4u32), ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x4ccff0, ecx, &a); }
                esp = esp.wrapping_add(20);
                ecx = m.r32(esp.wrapping_add(0x20u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
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

/// `0X004D1FE0` text_box_ctor_default
pub fn f_004d1fe0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6628e6u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6c0148u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi.wrapping_add(0xb8u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    m.w32(esi, 0x671ea0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004D2040` text_box_ctor_named
pub fn f_004d2040(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662906u32);
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
    ecx = esi.wrapping_add(0xb8u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    m.w32(esi, 0x671ea0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004D20A0` text_box_dtor
pub fn f_004d20a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662926u32);
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
    m.w32(esi, 0x671ea0u32);
    m.w32(esp.wrapping_add(0x10u32), 0x1u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
    ecx = esi.wrapping_add(0xb8u32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
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

/// `0X004D2100` text_box_reset_defaults
pub fn f_004d2100(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                ecx = esi;
                m.w32(esi.wrapping_add(0xd8u32), ebx);
                m.w32(esi.wrapping_add(0xecu32), ebx);
                m.w32(esi.wrapping_add(0xf0u32), 0x3u32);
                m.w32(esi.wrapping_add(0xf4u32), 0x6u32);
                m.w32(esi.wrapping_add(0xe4u32), ebx);
                m.w32(esi.wrapping_add(0xe0u32), ebx);
                m.w32(esi.wrapping_add(0xdcu32), 0xffff0000u32);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi.wrapping_add(0xe4u32));
                m.w32(esi.wrapping_add(0xe0u32), 0xff00ff00u32);
                fl = lift::sub(eax, 0xff80ff80u32, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0xe4u32), 0xff80ff80u32);
                m.w8(esi.wrapping_add(0xfcu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xe8u32), ebx);
                m.w32(esi.wrapping_add(0xecu32), ebx);
                m.w32(esi.wrapping_add(0xf0u32), 0x3u32);
                m.w32(esi.wrapping_add(0xf4u32), 0x6u32);
                m.w32(esi.wrapping_add(0xa0u32), eax);
                m.w32(esi.wrapping_add(0xa4u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esi.wrapping_add(0xa8u32), eax);
                ecx = esi;
                m.w32(esi.wrapping_add(0xacu32), eax);
                edx = m.r32(esi);
                m.w32(esi.wrapping_add(0xb0u32), ebx);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                m.w32(esi.wrapping_add(0xf8u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                ecx = esi.wrapping_add(0xb8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
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

/// `0X004D21F0` text_box_load_from_config
pub fn f_004d21f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662948u32);
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
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                m.w32(esp.wrapping_add(0x3cu32), ebp);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
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
                m.w32(esp.wrapping_add(0x48u32), ebp);
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4d2100, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x48u32));
                ecx = m.r32(0x6c0040u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                ecx = m.r32(0x6c0080u32);
                edx = m.r32(0x6c0074u32);
                esp = esp.wrapping_add(0xc);
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                m.w32(esp.wrapping_add(0x18u32), ebp);
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
                m.w32(esp.wrapping_add(0x38u32), ebp);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esp.wrapping_add(0x1cu32);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c014cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                let r = edi | eax; fl = lift::logic(r, 32);
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
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
                eax = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0xecu32), edi);
                ecx = m.r32(0x6c0054u32);
                edx = m.r32(0x6c004cu32);
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
                m.w32(esp.wrapping_add(0x30u32), ebp);
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = esp.wrapping_add(0x18u32);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0158u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x34u32), ebp);
                { let t = m.r32(edx.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(0x6c0064u32);
                eax = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                m.w32(esp.wrapping_add(0x20u32), eax);
                eax = m.r32(0x6c005cu32);
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
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = esp.wrapping_add(0x20u32);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6c0160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x34u32), ebp);
                { let t = m.r32(eax.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(0x6dd620u32);
                ecx = m.r32(0x6c0090u32);
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
                m.w32(esi.wrapping_add(0xdcu32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(0x6c00b8u32);
                edx = m.r32(0x6c009cu32);
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
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = eax;
                fl = lift::sub(edi, ebp, 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(u32::from(m.r8(0x6dbb58u32)), 0x23u32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb59u32);
                m.w8(esp.wrapping_add(0x40u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x24u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 14; continue;
            }
            13 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 14; continue;
            }
            14 => {
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(0x6dd614u32);
                ecx = m.r32(0x6c00c8u32);
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
                edi = eax;
                eax = m.r32(esi.wrapping_add(0xe0u32));
                fl = lift::sub(edi, eax, 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0xe0u32), edi);
                edx = m.r32(0x6dd610u32);
                eax = m.r32(0x6c00d8u32);
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
                edi = eax;
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(edi, eax, 32).1;
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0xe4u32), edi);
                ecx = m.r32(0x6dd60cu32);
                edx = m.r32(0x6c00ecu32);
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
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0xe8u32), eax);
                edx = m.r32(0x6c00f8u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                m.w32(esi.wrapping_add(0xd8u32), eax);
                edx = m.r32(0x6dd61cu32);
                eax = m.r32(0x6c00a8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                m.w32(esi.wrapping_add(0xf0u32), 0x3u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(edi.wrapping_add(0x6bb5d4u32));
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(edi.wrapping_add(0x671e84u32));
                m.w32(esi.wrapping_add(0xf0u32), ecx);
                pc = 21; continue;
            }
            21 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0xcu32, 32).1;
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(0x6dd618u32);
                ecx = m.r32(0x6c00b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x14u32));
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
                m.w32(esi.wrapping_add(0xf4u32), 0x6u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(edi.wrapping_add(0x6bb5f4u32));
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(edi.wrapping_add(0x671e90u32));
                m.w32(esi.wrapping_add(0xf4u32), edx);
                pc = 25; continue;
            }
            25 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0xcu32, 32).1;
                if (fl.sf != fl.of) { pc = 23; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(0x6c0110u32);
                edx = m.r32(0x6c0104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 27; continue;
            }
            27 => {
                ecx = m.r32(edi.wrapping_mul(4).wrapping_add(0x6c012cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x671e9cu32)))) & 0xff);
                pc = 29; continue;
            }
            29 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                fl = lift::sub(edi, 0x3u32, 32).1;
                if (fl.sf != fl.of) { pc = 27; continue; }
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub((ebx & 0xff), 0x2u32, 8).1;
                if (!fl.cf && !fl.zf) { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                m.w8(esi.wrapping_add(0xfcu32), ((ebx & 0xff)) as u8);
                pc = 32; continue;
            }
            32 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x94u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
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

/// `0X004D27A0` text_box_slot3_release_string
pub fn f_004d27a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004D27C0` text_box_draw
pub fn f_004d27c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x4c);
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
                if fl.zf { pc = 25; continue; }
                pc = 1; continue;
            }
            1 => {
                ebp = m.r32(esp.wrapping_add(0x64u32));
                edx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x78u32));
                ecx = esp;
                edi = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(ecx, ebp);
                m.w32(ecx.wrapping_add(0x4u32), edx);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4b18f0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xdcu32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                edx = m.r32(esi);
                esp = esp.wrapping_add(0xc);
                m.w8(esp.wrapping_add(0x14u32), ((eax & 0xff)) as u8);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                ebx = eax.wrapping_add(edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x40u32);
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x2cu32), ebp);
                m.w32(esp.wrapping_add(0x30u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(esi.wrapping_add(0xdcu32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                fl = lift::logic(eax & 0xff000000u32, 32);
                m.w32(esp.wrapping_add(0x20u32), edx);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(esi.wrapping_add(0xa8u32));
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = eax;
                fl = lift::logic(ebx & ebx, 32);
                if fl.zf { pc = 23; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = m.r32(esi.wrapping_add(0x94u32));
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ecx = esi;
                m.w32(esp.wrapping_add(0x64u32), eax);
                eax = m.r32(esi.wrapping_add(0x98u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi);
                m.w32(esp.wrapping_add(0x68u32), eax);
                { let t = m.r32(edx.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 15; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x28u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0xe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xe4u32));
                m.w32(esp.wrapping_add(0x64u32), edx);
                pc = 16; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xe0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xe0u32));
                m.w32(esp.wrapping_add(0x64u32), ecx);
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esp.wrapping_add(0x20u32));
                ebp = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x54u32), edx);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xfcu32)))) & 0xff);
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                m.w32(esp.wrapping_add(0x54u32), eax);
                m.w32(esp.wrapping_add(0x58u32), ebp);
                if fl.zf { pc = 20; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xf0u32));
                ecx = m.r32(ebx.wrapping_add(0x8u32));
                esi = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x6cu32));
                edx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                pc = 22; continue;
            }
            19 => {
                edx = m.r32(esi.wrapping_add(0xe8u32));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x54u32), ecx);
                m.w32(esp.wrapping_add(0x58u32), eax);
                m.w32(esp.wrapping_add(0x5cu32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xf0u32));
                eax = m.r32(esp.wrapping_add(0x6cu32));
                ebp = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x54u32);
                ecx = ebp.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                eax = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ebp.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                eax = m.r32(esp.wrapping_add(0x68u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x50u32);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebp.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x70u32));
                eax = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp.wrapping_add(0x1u32);
                eax = edx.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                eax = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = ebp.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(esi.wrapping_add(0xf0u32));
                ecx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x6cu32));
                edx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebp.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                edx = m.r32(esi.wrapping_add(0xf0u32));
                eax = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x6cu32));
                ecx = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                eax = edx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                eax = esp.wrapping_add(0x50u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_add(0xffffffffu32);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                eax = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esi = m.r32(esi.wrapping_add(0xd8u32));
                eax = esp.wrapping_add(0x54u32);
                ecx = m.r32(esp.wrapping_add(0x70u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esi.wrapping_mul(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                pc = 21; continue;
            }
            20 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                m.w32(esp.wrapping_add(0x54u32), eax);
                eax = m.r32(esi.wrapping_add(0xe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x5cu32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xf0u32));
                edx = m.r32(ebx.wrapping_add(0x8u32));
                esp = esp.wrapping_add(0x4);
                eax = esp.wrapping_add(0x4cu32);
                ebp = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                edx = ebp.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                ecx = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xf0u32));
                eax = m.r32(ebx.wrapping_add(0x8u32));
                esi = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x4cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x6cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = esi.wrapping_mul(8);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4c17e0, ecx, &a); }
                esp = esp.wrapping_add(24);
                pc = 23; continue;
            }
            23 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 25; continue;
            }
            25 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x4c);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D2D10` text_box_set_decoration_pair
pub fn f_004d2d10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x8u32));
    edx = m.r32(esp.wrapping_add(0x4u32));
    let r = eax & 0xffffu32; fl = lift::logic(r, 32);
    eax = r;
    let r = edx & 0xffffu32; fl = lift::logic(r, 32);
    edx = r;
    let (r, f) = lift::shl(eax, 0x10u32, 32, fl); fl = f;
    eax = r;
    let r = eax | edx; fl = lift::logic(r, 32);
    eax = r;
    m.w32(ecx.wrapping_add(0xecu32), eax);
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    return eax;
    eax
}

/// `0X004D2D40` text_box_set_decoration
pub fn f_004d2d40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    edx = m.r32(ecx);
    m.w32(ecx.wrapping_add(0xecu32), eax);
    { let t = m.r32(edx.wrapping_add(0x11cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    return eax;
    eax
}

/// `0X004D2D70` text_box_set_background_color
pub fn f_004d2d70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0xdcu32), eax);
    return eax;
    eax
}

/// `0X004D2D80` text_box_set_text_color
pub fn f_004d2d80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xe0u32)), 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                m.w32(esi.wrapping_add(0xe0u32), edi);
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

/// `0X004D2DB0` text_box_set_highlight_color
pub fn f_004d2db0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::sub(edi, m.r32(esi.wrapping_add(0xe4u32)), 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                m.w32(esi.wrapping_add(0xe4u32), edi);
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

/// `0X004D2DF0` text_box_get_text_color
pub fn f_004d2df0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0xe0u32));
    return eax;
    eax
}

/// `0X004D2E20` text_box_set_font
pub fn f_004d2e20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    edx = m.r32(ecx);
    m.w32(ecx.wrapping_add(0xd8u32), eax);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    return eax;
    eax
}

/// `0X004D2E40` text_box_set_horizontal_align
pub fn f_004d2e40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                fl = lift::sub(eax, 0x1u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(ecx.wrapping_add(0xf0u32), eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X004D2E70` text_box_get_horizontal_align
pub fn f_004d2e70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0xf0u32));
    return eax;
    eax
}

/// `0X004D2E80` text_box_set_vertical_align
pub fn f_004d2e80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                fl = lift::sub(eax, 0x4u32, 32).1;
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x6u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(ecx.wrapping_add(0xf4u32), eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X004D2EB0` text_box_get_vertical_align
pub fn f_004d2eb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx.wrapping_add(0xf4u32));
    return eax;
    eax
}

/// `0X004D2EC0` text_box_get_text
pub fn f_004d2ec0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(ecx);
    { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
    eax = r;
    return eax;
    eax
}

/// `0X004D2ED0` text_box_set_text
pub fn f_004d2ed0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66295au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::add(eax, 0x8u32, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63ac5b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                ecx = m.r32(esi.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xf8u32), eax);
                edi = m.r32(edi);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b0450, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xcu32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
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

/// `0X004D2FB0` text_box_set_value_record
pub fn f_004d2fb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1890, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(esi);
    ecx = esi;
    ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004D31A0` text_box_set_size
pub fn f_004d31a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    edx = m.r32(esp.wrapping_add(0x8u32));
    m.w32(ecx.wrapping_add(0x2cu32), eax);
    eax = m.r32(esp.wrapping_add(0xcu32));
    m.w32(ecx.wrapping_add(0x30u32), edx);
    edx = m.r32(ecx);
    m.w32(ecx.wrapping_add(0x34u32), eax);
    { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    return eax;
    eax
}

/// `0X004D31F0` text_box_get_class_name
pub fn f_004d31f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6c0148u32);
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

/// `0X004D3220` text_box_scalar_deleting_dtor
pub fn f_004d3220(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4d20a0, ecx, &a); }
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

/// `0X005BE700` number_input_field_set_max_value
pub fn f_005be700(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(0x188u32));
                fl = lift::sub(eax, edx, 32).1;
                m.w32(ecx.wrapping_add(0x180u32), eax);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(esp.wrapping_add(0x4u32), eax);
                { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x45c930, ecx, &a); }
            }
            2 => {
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x4201b0 => f_004201b0(m, h, ecx, esp),
        0x4201d0 => f_004201d0(m, h, ecx, esp),
        0x45c110 => f_0045c110(m, h, ecx, esp),
        0x45c180 => f_0045c180(m, h, ecx, esp),
        0x45c230 => f_0045c230(m, h, ecx, esp),
        0x45c930 => f_0045c930(m, h, ecx, esp),
        0x45ca10 => f_0045ca10(m, h, ecx, esp),
        0x45ca40 => f_0045ca40(m, h, ecx, esp),
        0x4afc30 => f_004afc30(m, h, ecx, esp),
        0x4afce0 => f_004afce0(m, h, ecx, esp),
        0x4afd90 => f_004afd90(m, h, ecx, esp),
        0x4afe40 => f_004afe40(m, h, ecx, esp),
        0x4afeb0 => f_004afeb0(m, h, ecx, esp),
        0x4aff30 => f_004aff30(m, h, ecx, esp),
        0x4affc0 => f_004affc0(m, h, ecx, esp),
        0x4afff0 => f_004afff0(m, h, ecx, esp),
        0x4b0010 => f_004b0010(m, h, ecx, esp),
        0x4b0040 => f_004b0040(m, h, ecx, esp),
        0x4c7e30 => f_004c7e30(m, h, ecx, esp),
        0x4c7ed0 => f_004c7ed0(m, h, ecx, esp),
        0x4c7f50 => f_004c7f50(m, h, ecx, esp),
        0x4c8190 => f_004c8190(m, h, ecx, esp),
        0x4c88c0 => f_004c88c0(m, h, ecx, esp),
        0x4c88e0 => f_004c88e0(m, h, ecx, esp),
        0x4c8fe0 => f_004c8fe0(m, h, ecx, esp),
        0x4c9080 => f_004c9080(m, h, ecx, esp),
        0x4c90b0 => f_004c90b0(m, h, ecx, esp),
        0x4c90d0 => f_004c90d0(m, h, ecx, esp),
        0x4c9160 => f_004c9160(m, h, ecx, esp),
        0x4c9240 => f_004c9240(m, h, ecx, esp),
        0x4c9790 => f_004c9790(m, h, ecx, esp),
        0x4c9880 => f_004c9880(m, h, ecx, esp),
        0x4c9970 => f_004c9970(m, h, ecx, esp),
        0x4c99f0 => f_004c99f0(m, h, ecx, esp),
        0x4c9a70 => f_004c9a70(m, h, ecx, esp),
        0x4c9db0 => f_004c9db0(m, h, ecx, esp),
        0x4c9f80 => f_004c9f80(m, h, ecx, esp),
        0x4ca110 => f_004ca110(m, h, ecx, esp),
        0x4ca640 => f_004ca640(m, h, ecx, esp),
        0x4ca700 => f_004ca700(m, h, ecx, esp),
        0x4caad0 => f_004caad0(m, h, ecx, esp),
        0x4cabc0 => f_004cabc0(m, h, ecx, esp),
        0x4cacf0 => f_004cacf0(m, h, ecx, esp),
        0x4cadc0 => f_004cadc0(m, h, ecx, esp),
        0x4cade0 => f_004cade0(m, h, ecx, esp),
        0x4cae00 => f_004cae00(m, h, ecx, esp),
        0x4caea0 => f_004caea0(m, h, ecx, esp),
        0x4caf40 => f_004caf40(m, h, ecx, esp),
        0x4cafe0 => f_004cafe0(m, h, ecx, esp),
        0x4cb060 => f_004cb060(m, h, ecx, esp),
        0x4cb770 => f_004cb770(m, h, ecx, esp),
        0x4cbce0 => f_004cbce0(m, h, ecx, esp),
        0x4cbd00 => f_004cbd00(m, h, ecx, esp),
        0x4cc040 => f_004cc040(m, h, ecx, esp),
        0x4cc8b0 => f_004cc8b0(m, h, ecx, esp),
        0x4ccaa0 => f_004ccaa0(m, h, ecx, esp),
        0x4d1fe0 => f_004d1fe0(m, h, ecx, esp),
        0x4d2040 => f_004d2040(m, h, ecx, esp),
        0x4d20a0 => f_004d20a0(m, h, ecx, esp),
        0x4d2100 => f_004d2100(m, h, ecx, esp),
        0x4d21f0 => f_004d21f0(m, h, ecx, esp),
        0x4d27a0 => f_004d27a0(m, h, ecx, esp),
        0x4d27c0 => f_004d27c0(m, h, ecx, esp),
        0x4d2d10 => f_004d2d10(m, h, ecx, esp),
        0x4d2d40 => f_004d2d40(m, h, ecx, esp),
        0x4d2d70 => f_004d2d70(m, h, ecx, esp),
        0x4d2d80 => f_004d2d80(m, h, ecx, esp),
        0x4d2db0 => f_004d2db0(m, h, ecx, esp),
        0x4d2df0 => f_004d2df0(m, h, ecx, esp),
        0x4d2e20 => f_004d2e20(m, h, ecx, esp),
        0x4d2e40 => f_004d2e40(m, h, ecx, esp),
        0x4d2e70 => f_004d2e70(m, h, ecx, esp),
        0x4d2e80 => f_004d2e80(m, h, ecx, esp),
        0x4d2eb0 => f_004d2eb0(m, h, ecx, esp),
        0x4d2ec0 => f_004d2ec0(m, h, ecx, esp),
        0x4d2ed0 => f_004d2ed0(m, h, ecx, esp),
        0x4d2fb0 => f_004d2fb0(m, h, ecx, esp),
        0x4d31a0 => f_004d31a0(m, h, ecx, esp),
        0x4d31f0 => f_004d31f0(m, h, ecx, esp),
        0x4d3220 => f_004d3220(m, h, ecx, esp),
        0x5be700 => f_005be700(m, h, ecx, esp),
        _ => return None,
    }))
}
