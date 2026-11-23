//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00408B10` slider_button_vector_deleting_dtor
pub fn f_00408b10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::logic((ebx & 0xff) & 0x2u32, 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xfffffffcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = esi.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x46cd00u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                eax = edi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x46cd00, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 6; continue;
            }
            6 => {
                eax = esi;
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

/// `0X00408B70` button_base_vector_deleting_dtor
pub fn f_00408b70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x8u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::logic((ebx & 0xff) & 0x2u32, 8);
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xfffffffcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = esi.wrapping_add(0xfffffffcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4c6a30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 3; continue;
            }
            3 => {
                eax = edi;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 6; continue;
            }
            6 => {
                eax = esi;
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

/// `0X00464AD0` viper_button_ctor
pub fn f_00464ad0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65ef98u32);
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
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x10u32);
    eax = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
    m.w32(esi, 0x66e550u32);
    { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
    esp = esp.wrapping_add(0xc);
    ecx = esi;
    m.w32(esi.wrapping_add(0xa4u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x464b90, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X00464B40` viper_button_dtor
pub fn f_00464b40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65efaau32);
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
    m.w32(esi, 0x66e550u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x464b90, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X00464B90` viper_button_release_resources
pub fn f_00464b90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                edi = esi.wrapping_add(0x94u32);
                ebp = 0x4u32;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(edi);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                m.w32(edi, ebx);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::sub(ecx, ebx, 32).1;
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
                edi = esi.wrapping_add(0xa8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cc7d8u32);
                ecx = edi;
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                m.w32(esi.wrapping_add(0x38u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6cc7d9u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = 0xff000000u32;
                m.w8(esi.wrapping_add(0x41u32), (0x1u32) as u8);
                m.w8(esi.wrapping_add(0xbcu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xc0u32), eax);
                m.w32(esi.wrapping_add(0xc4u32), eax);
                eax = m.r32(0x6bf7acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6bf7c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                edx = esi.wrapping_add(0xccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0xb0u32), 0x3u32);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                eax = 0x1u32;
                m.w32(esi.wrapping_add(0xe8u32), eax);
                m.w32(esi.wrapping_add(0xecu32), eax);
                m.w32(esi.wrapping_add(0xf0u32), eax);
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

/// `0X00464F30` viper_button_copy_settings_to
pub fn f_00464f30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0xe8u32));
    m.w32(edi.wrapping_add(0xe8u32), eax);
    ecx = m.r32(esi.wrapping_add(0xecu32));
    m.w32(edi.wrapping_add(0xecu32), ecx);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X00464F60` viper_button_get_class_name
pub fn f_00464f60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bf85cu32);
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

/// `0X00464F90` viper_button_scalar_deleting_dtor
pub fn f_00464f90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x464b40, ecx, &a); }
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

/// `0X0046CC20` slider_button_ctor_default
pub fn f_0046cc20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f248u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6b96ccu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0xcu32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c69a0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), edi);
    m.w32(esi, 0x66e950u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    m.w32(esi.wrapping_add(0xe8u32), edi);
    m.w32(esi.wrapping_add(0xecu32), edi);
    m.w32(esi.wrapping_add(0xf0u32), edi);
    m.w32(esi.wrapping_add(0xf4u32), edi);
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

/// `0X0046CC90` slider_button_ctor_named
pub fn f_0046cc90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65f25au32);
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
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c69a0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x14u32), edi);
    m.w32(esi, 0x66e950u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0xcu32));
    m.w32(esi.wrapping_add(0xe8u32), edi);
    m.w32(esi.wrapping_add(0xecu32), edi);
    m.w32(esi.wrapping_add(0xf0u32), edi);
    m.w32(esi.wrapping_add(0xf4u32), edi);
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

/// `0X0046CD00` slider_button_dtor
pub fn f_0046cd00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    m.w32(ecx, 0x66e950u32);
    { let a = []; return h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    eax
}

/// `0X0046CD10` slider_button_on_mouse_event
pub fn f_0046cd10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c6b90, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1bf0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xf0u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0xecu32));
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esi.wrapping_add(0x14u32), edx);
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esi.wrapping_add(0x18u32), eax);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x18u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w32(esi.wrapping_add(0x18u32), eax);
                pc = 5; continue;
            }
            5 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0046CD60` slider_button_set_track_length
pub fn f_0046cd60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    edx = ecx.wrapping_add(0x14u32);
    m.w32(ecx.wrapping_add(0xe8u32), eax);
    let (r, f) = lift::add(ecx, 0xecu32, 32); fl = f;
    ecx = r;
    eax = m.r32(edx);
    m.w32(ecx, eax);
    eax = m.r32(edx.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0x4u32), eax);
    edx = m.r32(edx.wrapping_add(0x8u32));
    m.w32(ecx.wrapping_add(0x8u32), edx);
    return eax;
    eax
}

/// `0X0046CD90` slider_button_reset
pub fn f_0046cd90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi.wrapping_add(0xe8u32), eax);
    m.w32(esi.wrapping_add(0xecu32), eax);
    m.w32(esi.wrapping_add(0xf0u32), eax);
    m.w32(esi.wrapping_add(0xf4u32), eax);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X0046CDC0` slider_button_copy_track_length_to
pub fn f_0046cdc0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi.wrapping_add(0xe8u32));
    m.w32(edi.wrapping_add(0xe8u32), eax);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X0046CDE0` slider_button_get_class_name
pub fn f_0046cde0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6b96ccu32);
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

/// `0X004AF090` anim_button_ctor
pub fn f_004af090(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x661786u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb574u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x10u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = esi.wrapping_add(0x94u32);
    let r = ebx ^ ebx; fl = lift::logic(r, 32);
    ebx = r;
    ecx = edi;
    m.w32(esp.wrapping_add(0x18u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4c5750, ecx, &a); }
    ecx = esi;
    m.w8(esp.wrapping_add(0x18u32), (0x1u32) as u8);
    m.w32(esi, 0x670430u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x4c5780, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x4u32));
    m.w32(esi.wrapping_add(0xa8u32), ebx);
    m.w32(esi.wrapping_add(0xacu32), ebx);
    m.w32(esi.wrapping_add(0xb8u32), ebx);
    m.w32(esi.wrapping_add(0xc8u32), ebx);
    m.w32(esi.wrapping_add(0xc0u32), ebx);
    m.w32(esi.wrapping_add(0xc4u32), ebx);
    m.w32(esi.wrapping_add(0xd0u32), ebx);
    ecx = m.r32(0x6dccf8u32);
    let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi.wrapping_add(0xd4u32), ecx);
    ecx = m.r32(esp.wrapping_add(0x10u32));
    m.w32(esi.wrapping_add(0xdcu32), eax);
    m.w32(esi.wrapping_add(0xe4u32), eax);
    m.w32(esi.wrapping_add(0xe8u32), eax);
    m.w32(esi.wrapping_add(0xb0u32), ebx);
    m.w8(esi.wrapping_add(0xd8u32), ((ebx & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xccu32), ebx);
    m.w32(esi.wrapping_add(0xb4u32), ebx);
    m.w32(esi.wrapping_add(0xe0u32), edx);
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004AF170` anim_button_dtor
pub fn f_004af170(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6617a6u32);
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
    m.w32(esp.wrapping_add(0xcu32), esi);
    m.w32(esi, 0x670430u32);
    m.w32(esp.wrapping_add(0x18u32), 0x1u32);
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    edi = esi.wrapping_add(0x94u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x4c5780, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x4u32));
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi.wrapping_add(0xa8u32), eax);
    m.w32(esi.wrapping_add(0xacu32), eax);
    m.w32(esi.wrapping_add(0xb8u32), eax);
    m.w32(esi.wrapping_add(0xc8u32), eax);
    m.w32(esi.wrapping_add(0xc0u32), eax);
    m.w32(esi.wrapping_add(0xc4u32), eax);
    m.w32(esi.wrapping_add(0xd0u32), eax);
    ecx = m.r32(0x6dccf8u32);
    m.w32(esi.wrapping_add(0xd4u32), ecx);
    let r = ebx | 0xffffffffu32; fl = lift::logic(r, 32);
    ebx = r;
    ecx = edi;
    m.w32(esi.wrapping_add(0xb0u32), eax);
    m.w8(esi.wrapping_add(0xd8u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xccu32), eax);
    m.w32(esi.wrapping_add(0xb4u32), eax);
    m.w32(esi.wrapping_add(0xdcu32), ebx);
    m.w32(esi.wrapping_add(0xe4u32), ebx);
    m.w32(esi.wrapping_add(0xe0u32), edx);
    m.w32(esi.wrapping_add(0xe8u32), ebx);
    m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c5770, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x18u32), ebx);
    { let a = []; eax = h.call_at(esp, m, 0x4b1640, ecx, &a); }
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
    eax
}

/// `0X004AF250` anim_button_slot3_release_string
pub fn f_004af250(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004AF260` anim_button_load_layout
pub fn f_004af260(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6617c8u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x28);
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
                m.w32(esp.wrapping_add(0x3cu32), edi);
                m.w32(esp.wrapping_add(0xcu32), eax);
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(0x6bb51cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x48u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edx = m.r32(0x6beee0u32);
                eax = m.r32(0x6bb530u32);
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x10u32));
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
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0xcu32);
                m.w8(esp.wrapping_add(0x3cu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x44u32);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(0x0u32, ecx);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                return eax;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(0x6bb524u32);
                eax = 0x1u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x4cu32), eax);
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(0x6bb544u32);
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
                m.w8(esp.wrapping_add(0x54u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bb578u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x48u32));
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x28u32), edi);
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(0x6bb53cu32);
                eax = m.r32(esp.wrapping_add(0xcu32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x1cu32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x20u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0x94u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c58d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = m.r32(0x6bb554u32);
                edx = m.r32(esp.wrapping_add(0xcu32));
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = m.r32(esp.wrapping_add(0x44u32));
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
                ecx = edi;
                m.w32(esi.wrapping_add(0xdcu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x4c5840, ecx, &a); }
                fl = lift::sub(m.r32(esi.wrapping_add(0xdcu32)), eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                m.w32(esi.wrapping_add(0xdcu32), 0xffffffffu32);
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esp.wrapping_add(0x44u32));
                ecx = m.r32(0x6bb564u32);
                edx = m.r32(esp.wrapping_add(0xcu32));
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
                ecx = edi;
                m.w32(esi.wrapping_add(0xe4u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x4c5840, ecx, &a); }
                fl = lift::sub(m.r32(esi.wrapping_add(0xe4u32)), eax, 32).1;
                if (fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w32(esi.wrapping_add(0xe4u32), 0xffffffffu32);
                pc = 8; continue;
            }
            8 => {
                eax = esp.wrapping_add(0x28u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c5870, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(esi);
                esp = esp.wrapping_add(0x8);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                edx = esp.wrapping_add(0xcu32);
                eax = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0xcu32);
                edx = esp.wrapping_add(0x44u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 10; continue;
            }
            10 => {
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x3cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0xcu32);
                m.w8(esp.wrapping_add(0x3cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x44u32);
                m.w32(esp.wrapping_add(0x3cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x34);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004AF4F0` anim_button_draw
pub fn f_004af4f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x10);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x20u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x2cu32));
                m.w32(esp.wrapping_add(0x8u32), eax);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(ecx, 0x3u32, 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 9; continue;
            }
            5 => {
                fl = lift::sub(ecx, 0x2u32, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                pc = 9; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xa8u32));
                eax = m.r32(esi.wrapping_add(0xacu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 9; continue;
            }
            9 => {
                ecx = esi.wrapping_add(0x94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4c5870, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = eax;
                m.w32(esi.wrapping_add(0xb0u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                edi = eax;
                eax = m.r32(esi.wrapping_add(0xe8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xb0u32));
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x20u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x30);
                pc = 10; continue;
            }
            10 => {
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

/// `0X004AF860` anim_button_add_dirty_rect_if_changed
pub fn f_004af860(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0x10u32));
                edx = m.r32(0x670f6cu32);
                esp = esp.wrapping_sub(0x10);
                fl = lift::sub(eax, edx, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(ecx.wrapping_add(0x10u32), eax);
                pc = 10; continue;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(0x4u32));
                edx = m.r32(ecx.wrapping_add(0xe0u32));
                fl = lift::sub(eax, edx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                let r = edx | 0xffffffffu32; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub(eax, 0x3u32, 32).1;
                m.w32(ecx.wrapping_add(0xe0u32), eax);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(m.r32(ecx.wrapping_add(0xdcu32)), edx, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(m.r32(ecx.wrapping_add(0xe4u32)), edx, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(ecx.wrapping_add(0xb0u32));
                edx = m.r32(ecx.wrapping_add(0xb4u32));
                fl = lift::sub(eax, edx, 32).1;
                pc = 9; continue;
            }
            8 => {
                edx = m.r32(ecx.wrapping_add(0xb0u32));
                eax = m.r32(ecx.wrapping_add(0xb4u32));
                fl = lift::sub(edx, eax, 32).1;
                pc = 9; continue;
            }
            9 => {
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                esi = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(ecx.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = m.r32(esi.wrapping_add(0x4u32));
                edi = m.r32(esi);
                esi = m.r32(ecx.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ecx.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                let (r, f) = lift::add(esi, eax, 32); fl = f;
                esi = r;
                eax = m.r32(ecx.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                ecx = esp.wrapping_add(0x8u32);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0xcu32), edx);
                m.w32(esp.wrapping_add(0x18u32), esi);
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb140, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 11; continue;
            }
            11 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004AF910` anim_button_consume_click
pub fn f_004af910(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xd8u32)))) & 0xff);
    m.w8(ecx.wrapping_add(0xd8u32), (0x0u32) as u8);
    return eax;
    eax
}

/// `0X004AF920` anim_button_on_mouse_event
pub fn f_004af920(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1720, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(esi.wrapping_add(0xd8u32), (0x1u32) as u8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xd8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                m.w8(esi.wrapping_add(0xd8u32), ((eax & 0xff)) as u8);
                pc = 5; continue;
            }
            5 => {
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

/// `0X004AF970` anim_button_set_static_set
pub fn f_004af970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = m.r32(esp.wrapping_add(0x4u32));
    m.w32(ecx.wrapping_add(0xacu32), eax);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(ecx.wrapping_add(0xc8u32), eax);
    m.w32(ecx.wrapping_add(0xccu32), eax);
    m.w32(ecx.wrapping_add(0xd0u32), eax);
    m.w32(ecx.wrapping_add(0xc0u32), eax);
    m.w32(ecx.wrapping_add(0xb8u32), eax);
    m.w32(ecx.wrapping_add(0xc4u32), eax);
    return eax;
    eax
}

/// `0X004AF9B0` anim_button_reset
pub fn f_004af9b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    ecx = esi.wrapping_add(0x94u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c5780, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x4u32));
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi.wrapping_add(0xa8u32), eax);
    m.w32(esi.wrapping_add(0xacu32), eax);
    m.w32(esi.wrapping_add(0xb8u32), eax);
    m.w32(esi.wrapping_add(0xc8u32), eax);
    m.w32(esi.wrapping_add(0xc0u32), eax);
    m.w32(esi.wrapping_add(0xc4u32), eax);
    m.w32(esi.wrapping_add(0xd0u32), eax);
    ecx = m.r32(0x6dccf8u32);
    m.w32(esi.wrapping_add(0xb0u32), eax);
    m.w8(esi.wrapping_add(0xd8u32), ((eax & 0xff)) as u8);
    m.w32(esi.wrapping_add(0xccu32), eax);
    m.w32(esi.wrapping_add(0xb4u32), eax);
    let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
    eax = r;
    m.w32(esi.wrapping_add(0xd4u32), ecx);
    m.w32(esi.wrapping_add(0xdcu32), eax);
    m.w32(esi.wrapping_add(0xe4u32), eax);
    m.w32(esi.wrapping_add(0xe0u32), edx);
    m.w32(esi.wrapping_add(0xe8u32), eax);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004AFA40` anim_button_get_playing_set
pub fn f_004afa40(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(ecx.wrapping_add(0xc8u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                return eax;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(0xacu32));
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004AFB80` anim_button_get_class_name
pub fn f_004afb80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb574u32);
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

/// `0X004AFBB0` anim_button_scalar_deleting_dtor
pub fn f_004afbb0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4af170, ecx, &a); }
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

/// `0X004B2480` filled_rect_ctor_default
pub fn f_004b2480(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6619d8u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb638u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi, 0x6709f0u32);
    ecx = m.r32(0x670f60u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6da820u32;
    m.w32(esp.wrapping_add(0x14u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi.wrapping_add(0xa8u32), eax);
    edx = m.r32(0x670f64u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = 0x6da820u32;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w32(esi.wrapping_add(0xacu32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x4b2580, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B2500` filled_rect_ctor_named
pub fn f_004b2500(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6619eau32);
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
    m.w32(esi, 0x6709f0u32);
    ecx = m.r32(0x670f60u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = 0x6da820u32;
    m.w32(esp.wrapping_add(0x14u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w32(esi.wrapping_add(0xa8u32), eax);
    edx = m.r32(0x670f64u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = 0x6da820u32;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    m.w32(esi.wrapping_add(0xacu32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x4b2580, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004B2580` filled_rect_init_defaults
pub fn f_004b2580(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf000u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf00u32);
    edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf0u32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x1u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xfu32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf000u32);
    m.w16(esi.wrapping_add(0x94u32), ((edi & 0xffff)) as u16);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf00u32);
    edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_add(0x48);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x3u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xfu32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf000u32);
    m.w16(esi.wrapping_add(0x96u32), ((edi & 0xffff)) as u16);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf00u32);
    edi = (edi & !0xffff) | (((eax & 0xffff)) & 0xffff);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf0u32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xfu32);
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bbdb0, ecx, &a); }
    let r = edi | eax; fl = lift::logic(r, 32);
    edi = r;
    esp = esp.wrapping_add(0x48);
    m.w16(esi.wrapping_add(0x98u32), ((edi & 0xffff)) as u16);
    m.w32(esi.wrapping_add(0x9cu32), 0x1u32);
    eax = m.r32(0x670f20u32);
    m.w32(esi.wrapping_add(0xa4u32), 0x0u32);
    m.w32(esi.wrapping_add(0xa0u32), eax);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    return eax;
    eax
}

/// `0X004B2680` filled_rect_dtor
pub fn f_004b2680(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    m.w32(ecx, 0x6709f0u32);
    { let a = []; return h.call_at(esp, m, 0x4b1640, ecx, &a); }
    eax
}

/// `0X004B2690` filled_rect_slot2_return_false
pub fn f_004b2690(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004B26A0` filled_rect_slot3_return_false
pub fn f_004b26a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004B26B0` filled_rect_draw
pub fn f_004b26b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = ecx;
                m.w32(esp.wrapping_add(0x8u32), esi);
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 29; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x60u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x24u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 29; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                m.w32(esp.wrapping_add(0x14u32), eax);
                eax = m.r32(esi.wrapping_add(0xa4u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if (fl.sf == fl.of) { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 6; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 6; continue;
            }
            6 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0xcu32), eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0xa4u32));
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x20u32), ebp);
                m.w32(esp.wrapping_add(0x24u32), ebx);
                m.w32(esp.wrapping_add(0x28u32), edi);
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xa8u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                eax = ecx;
                ecx = esp.wrapping_add(0x40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esi = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(esp.wrapping_add(0x50u32));
                esi = eax;
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20)), m.r32(esp.wrapping_add(24)), m.r32(esp.wrapping_add(28))]; eax = h.call_at(esp, m, 0x4bba80, ecx, &a); }
                esp = esp.wrapping_add(0x2c);
                fl = lift::logic(esi & esi, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 10; continue;
            }
            10 => {
                esi = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edi = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x28u32));
                ebx = esi.wrapping_add(ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                ebp = edi.wrapping_add(edx);
                m.w32(esp.wrapping_add(0x48u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(ecx.wrapping_add(0x9cu32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                fl = lift::logic(edx & edx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 27; continue; }
                pc = 11; continue;
            }
            11 => {
                let (r, f) = lift::add(ecx, 0x94u32, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                pc = 14; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x34u32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                m.w8(esp.wrapping_add(0x40u32), ((eax & 0xff)) as u8);
                eax = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bb430, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7a0, ecx, &a); }
                esp = esp.wrapping_add(0x18);
                pc = 10; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x40u32));
                pc = 14; continue;
            }
            14 => {
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (fl.sf == fl.of) { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                m.w16(esp.wrapping_add(0x3cu32), ((ecx & 0xffff)) as u16);
                pc = 18; continue;
            }
            17 => {
                m.w32(esp.wrapping_add(0x3cu32), 0x0u32);
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xfu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf00u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = ebx.wrapping_add(0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 24; continue;
            }
            24 => {
                eax = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xf000u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbd30, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & 0xff000000u32, 32);
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4bd680, ecx, &a); }
                esp = esp.wrapping_add(0x14);
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(esp.wrapping_add(0x40u32));
                let (r, f) = lift::add(ecx, 0x2u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                edx = m.r32(ecx.wrapping_add(0x9cu32));
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, edx, 32).1;
                m.w32(esp.wrapping_add(0x40u32), eax);
                if (fl.sf != fl.of) { pc = 13; continue; }
                pc = 27; continue;
            }
            27 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 29; continue;
            }
            29 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B2970` filled_rect_set_border_color0
pub fn f_004b2970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = (eax & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x4u32)))) & 0xffff);
    m.w16(ecx.wrapping_add(0x94u32), ((eax & 0xffff)) as u16);
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    return eax;
    eax
}

/// `0X004B29A0` filled_rect_set_border_color
pub fn f_004b29a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esp.wrapping_add(0x8u32)))) & 0xffff);
                m.w16(ecx.wrapping_add(eax.wrapping_mul(2)).wrapping_add(0x94u32), ((edx & 0xffff)) as u16);
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

/// `0X004B29F0` filled_rect_set_border_width
pub fn f_004b29f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (fl.sf == fl.of) { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(ecx.wrapping_add(0x9cu32), eax);
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

/// `0X004B2A20` filled_rect_set_fill_color
pub fn f_004b2a20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                edx = m.r32(0x670f20u32);
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(ecx.wrapping_add(0xa4u32), 0x0u32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                return eax;
            }
            2 => {
                fl = lift::sub(eax, m.r32(0x670f2cu32), 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = 0x1u32;
                m.w32(ecx.wrapping_add(0xa4u32), eax);
                return eax;
            }
            4 => {
                m.w32(ecx.wrapping_add(0xa0u32), eax);
                m.w32(ecx.wrapping_add(0xa4u32), 0xffffffffu32);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004B2AA0` filled_rect_get_class_name
pub fn f_004b2aa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bb638u32);
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

/// `0X004B2AD0` filled_rect_scalar_deleting_dtor
pub fn f_004b2ad0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4b2680, ecx, &a); }
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

/// `0X004B2AF0` filled_rect_set_enabled_noop
pub fn f_004b2af0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = (eax & !0xff) | ((0x1u32) & 0xff);
    return eax;
    eax
}

/// `0X004C6910` button_base_ctor_default
pub fn f_004c6910(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662170u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bf85cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x64f253u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4103c0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0xa8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esp.wrapping_add(0x24u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x10u32);
    edx = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x1cu32), (0x1u32) as u8);
    m.w32(esi, 0x6714c8u32);
    { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
    esp = esp.wrapping_add(0xc);
    ecx = esi;
    m.w32(esi.wrapping_add(0xa4u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004C69A0` button_base_ctor_named
pub fn f_004c69a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66219au32);
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
    m.w32(esp, 0x64f253u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4103c0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    ecx = esi.wrapping_add(0xa8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w32(esp.wrapping_add(0x24u32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x10u32);
    edx = esi.wrapping_add(0x94u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x1cu32), (0x1u32) as u8);
    m.w32(esi, 0x6714c8u32);
    { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
    esp = esp.wrapping_add(0xc);
    ecx = esi;
    m.w32(esi.wrapping_add(0xa4u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004C6A30` button_base_dtor
pub fn f_004c6a30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6621c4u32);
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
    m.w32(esi, 0x6714c8u32);
    m.w32(esp.wrapping_add(0x10u32), 0x1u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6aa0, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x64f253u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x2u32);
    eax = esi.wrapping_add(0xa8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
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

/// `0X004C6AA0` button_base_reset
pub fn f_004c6aa0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                { let a = []; eax = h.call_at(esp, m, 0x4b1650, ecx, &a); }
                edi = esi.wrapping_add(0x94u32);
                ebp = 0x4u32;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(edi);
                fl = lift::sub(ecx, ebx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                m.w32(edi, ebx);
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 1; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                fl = lift::sub(ecx, ebx, 32).1;
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
                edi = esi.wrapping_add(0xa8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5c8u32);
                ecx = edi;
                m.w32(esi.wrapping_add(0xa4u32), ebx);
                m.w32(esi.wrapping_add(0x38u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0xacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dd5c9u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = 0xff000000u32;
                m.w8(esi.wrapping_add(0xbcu32), ((ebx & 0xff)) as u8);
                m.w8(esi.wrapping_add(0xbdu32), ((ebx & 0xff)) as u8);
                m.w32(esi.wrapping_add(0xc0u32), eax);
                m.w32(esi.wrapping_add(0xc4u32), eax);
                m.w32(esi.wrapping_add(0xc8u32), eax);
                eax = m.r32(0x6bf7acu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6bf7c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xcu32);
                edx = esi.wrapping_add(0xccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0xb0u32), 0x3u32);
                m.w32(esi.wrapping_add(0xb4u32), ebx);
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                esp = esp.wrapping_add(0xc);
                m.w32(esi.wrapping_add(0xd8u32), ebx);
                m.w32(esi.wrapping_add(0xdcu32), ebx);
                m.w32(esi.wrapping_add(0xe4u32), ebx);
                m.w32(esi.wrapping_add(0xe0u32), ebx);
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

/// `0X004C6B90` button_base_on_mouse_event
pub fn f_004c6b90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esp.wrapping_add(0x8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1720, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebp = eax;
                eax = m.r32(esi);
                ecx = esi;
                m.w32(esp.wrapping_add(0x1cu32), ebp);
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w8(esi.wrapping_add(0xbdu32), (0x1u32) as u8);
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                ebx = 0x0u32;
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                if fl.zf { pc = 19; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xbdu32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(esi.wrapping_add(0xbcu32), (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xbdu32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 13; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::sub(edi, ebx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w8(esi.wrapping_add(0xbdu32), ((ebx & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(ebp, ebx, 32).1;
                if fl.zf { pc = 32; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0xb8u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                if fl.zf { pc = 29; continue; }
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if fl.zf { pc = 26; continue; }
                pc = 16; continue;
            }
            16 => {
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                if !fl.zf { pc = 32; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = esp.wrapping_add(0x1cu32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x20u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b04c0, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                return eax;
            }
            19 => {
                fl = lift::sub(edi, 0x1u32, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xbcu32))), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 9; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0xd8u32), ebx);
                m.w32(esi.wrapping_add(0xe4u32), ebx);
                m.w8(esi.wrapping_add(0xbcu32), (0x1u32) as u8);
                pc = 9; continue;
            }
            22 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w8(esi.wrapping_add(0xbcu32), ((ebx & 0xff)) as u8);
                pc = 9; continue;
            }
            24 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 25; continue;
            }
            25 => {
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x18u32), ((ebx & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b04c0, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                return eax;
            }
            26 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 28; continue;
            }
            28 => {
                edi = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edi.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edi.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                return eax;
            }
            29 => {
                ecx = esp.wrapping_add(0x12u32);
                m.w8(esp.wrapping_add(0x12u32), ((ebx & 0xff)) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b04c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xacu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub(edi, 0xfffffffeu32, 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = esp.wrapping_add(0x13u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w8(esp.wrapping_add(0x17u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b04c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 32; continue;
            }
            32 => {
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
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C6D20` button_base_load_layout
pub fn f_004c6d20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66220cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x34);
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
                m.w32(esp.wrapping_add(0x4cu32), ebx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                m.w8(esp.wrapping_add(0x4cu32), (0x1u32) as u8);
                m.w8(esp.wrapping_add(0x18u32), (0x1u32) as u8);
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                fl = lift::sub(eax, 0x5u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = m.r32(0x6beee0u32);
                ecx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x58u32)), 0xffffffffu32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                m.w32(esp.wrapping_add(0x58u32), ebx);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x58u32));
                edx = m.r32(0x6bf6d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                m.w32(esp.wrapping_add(0x20u32), eax);
                fl = lift::sub(eax, ebx, 32).1;
                m.w8(esp.wrapping_add(0x4cu32), (0x2u32) as u8);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4adf00, ecx, &a); }
                pc = 7; continue;
            }
            6 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xa4u32), eax);
                edx = m.r32(0x6beee0u32);
                eax = m.r32(0x6bf778u32);
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
                m.w8(esp.wrapping_add(0x64u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(0x6bf76cu32);
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
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ae030, ecx, &a); }
                esp = esp.wrapping_add(8);
                m.w8(esp.wrapping_add(0x18u32), ((eax & 0xff)) as u8);
                pc = 11; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0xa4u32));
                { let a = []; eax = h.call_at(esp, m, 0x4adfc0, ecx, &a); }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                ebp = esi.wrapping_add(0x94u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 12; continue;
            }
            12 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 23; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(0x6bf760u32);
                ecx = m.r32(edi.wrapping_add(0x6bf6f4u32));
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
                if fl.zf { pc = 23; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(edi.wrapping_add(0x6bf724u32));
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
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 23; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf860u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(0x6beee0u32);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63add0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(m.r32(esp.wrapping_add(0x1cu32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 22; continue; }
                pc = 18; continue;
            }
            18 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x20u32), eax);
                fl = lift::sub(eax, ebx, 32).1;
                m.w8(esp.wrapping_add(0x4cu32), (0x3u32) as u8);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
                pc = 21; continue;
            }
            20 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 21; continue;
            }
            21 => {
                m.w32(ebp, eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x28u32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                m.w8(esp.wrapping_add(0x58u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(ebp);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 23; continue;
            }
            22 => {
                m.w32(ebp, ebx);
                pc = 23; continue;
            }
            23 => {
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(edi, 0x10u32, 32).1;
                if (fl.sf != fl.of) { pc = 12; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                edi = 0x1u32;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esp.wrapping_add(0x2cu32), edi);
                m.w32(esp.wrapping_add(0x30u32), edi);
                m.w32(esp.wrapping_add(0x34u32), edi);
                if fl.zf { pc = 27; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(0x6bf79cu32);
                edx = m.r32(0x6bf784u32);
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
                if fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = esp.wrapping_add(0x30u32);
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf864u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 27; continue;
            }
            27 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x54u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(0x6bf794u32);
                ecx = m.r32(0x6bf78cu32);
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
                m.w32(esp.wrapping_add(0x3cu32), edi);
                m.w32(esp.wrapping_add(0x40u32), edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = esp.wrapping_add(0x28u32);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf86cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                pc = 29; continue;
            }
            29 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x3cu32), eax);
                m.w32(esp.wrapping_add(0x38u32), edx);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x38u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                m.w32(esp.wrapping_add(0x44u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x7cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(0x6bf838u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x46u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xdcu32), eax);
                edx = m.r32(0x6bf848u32);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x17cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xe0u32), eax);
                edx = m.r32(0x6bf7e4u32);
                eax = m.r32(0x6bf7d8u32);
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
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(0x6bf7ccu32);
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
                m.w32(esi.wrapping_add(0xb0u32), 0x3u32);
                m.w32(esi.wrapping_add(0xb4u32), eax);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 30; continue;
            }
            30 => {
                edx = m.r32(edi.wrapping_add(0x6bb5d4u32));
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dab58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x63c47b, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(edi.wrapping_add(0x6714acu32));
                m.w32(esi.wrapping_add(0xb0u32), ecx);
                pc = 32; continue;
            }
            32 => {
                ecx = esp.wrapping_add(0x20u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0xcu32, 32).1;
                if (fl.sf != fl.of) { pc = 30; continue; }
                pc = 33; continue;
            }
            33 => {
                edx = m.r32(esp.wrapping_add(0x54u32));
                eax = m.r32(0x6dd5c0u32);
                ecx = m.r32(0x6bf7f0u32);
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
                m.w32(esi.wrapping_add(0xc0u32), eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(0x6dd5c0u32);
                edx = m.r32(0x6bf7fcu32);
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
                ecx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xc4u32), eax);
                edx = m.r32(0x6dd5c0u32);
                eax = m.r32(0x6bf808u32);
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
                edx = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xc8u32), eax);
                eax = m.r32(0x6bf820u32);
                ecx = m.r32(0x6bf818u32);
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
                edi = esi.wrapping_add(0xd0u32);
                ebp = esi.wrapping_add(0xccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6bf874u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                m.w32(esi.wrapping_add(0xd4u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x639d74, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                m.w32(edi, ebx);
                m.w32(ebp, ebx);
                pc = 35; continue;
            }
            35 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                m.w32(esi.wrapping_add(0xb8u32), ebx);
                ecx = m.r32(0x6dd5bcu32);
                edx = m.r32(0x6bf82cu32);
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
                ecx = m.r32(0x6c7cccu32);
                let r = (ebx & 0xff) ^ (ebx & 0xff); fl = lift::logic(r, 8);
                ebx = (ebx & !0xff) | ((r) & 0xff);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0x50u32), (0x4u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e2f0, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64e1e0, ecx, &a); }
                ecx = esp.wrapping_add(0x14u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f709, ecx, &a); }
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 40; continue; }
                pc = 37; continue;
            }
            37 => {
                edx = m.r32(edi.wrapping_add(0x6bf750u32));
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64d390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 39; continue; }
                pc = 38; continue;
            }
            38 => {
                eax = m.r32(edi.wrapping_add(0x6714b8u32));
                ebx = (ebx & !0xff) | ((0x1u32) & 0xff);
                m.w32(esi.wrapping_add(0xb8u32), eax);
                pc = 39; continue;
            }
            39 => {
                let (r, f) = lift::add(edi, 0x4u32, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, 0xcu32, 32).1;
                if (fl.sf != fl.of) { pc = 36; continue; }
                pc = 40; continue;
            }
            40 => {
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(0x6bf7acu32);
                eax = m.r32(0x6bf7a8u32);
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
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                fl = lift::sub(u32::from(m.r8(0x6dbb58u32)), 0x23u32, 8).1;
                if !fl.zf { pc = 45; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x20u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb59u32);
                m.w8(esp.wrapping_add(0x50u32), (0x5u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = esi.wrapping_add(0xa8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 44; continue; }
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = esi.wrapping_add(0xacu32);
                edx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 44; continue; }
                pc = 43; continue;
            }
            43 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 44; continue;
            }
            44 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x4cu32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 49; continue;
            }
            45 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esi.wrapping_add(0xa8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 48; continue; }
                pc = 46; continue;
            }
            46 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = esi.wrapping_add(0xacu32);
                edx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 48; continue; }
                pc = 47; continue;
            }
            47 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 48; continue;
            }
            48 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 49; continue;
            }
            49 => {
                eax = m.r32(esp.wrapping_add(0x54u32));
                ecx = m.r32(0x6bf7c0u32);
                edx = m.r32(0x6bf7bcu32);
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
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dbb58u32))) & 0xff);
                fl = lift::sub((eax & 0xff), 0x23u32, 8).1;
                if !fl.zf { pc = 51; continue; }
                pc = 50; continue;
            }
            50 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x20u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb59u32);
                m.w8(esp.wrapping_add(0x50u32), (0x6u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x639cda, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0x4cu32), (0x4u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 53; continue;
            }
            51 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 53; continue;
            }
            53 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 55; continue; }
                pc = 54; continue;
            }
            54 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b1ec0, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = esp.wrapping_add(0x10u32);
                eax = esp.wrapping_add(0x54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4b2080, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 55; continue;
            }
            55 => {
                eax = m.r32(esp.wrapping_add(0x58u32));
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
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x14u32);
                m.w8(esp.wrapping_add(0x4cu32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0x4cu32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x54u32);
                m.w32(esp.wrapping_add(0x4cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
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
                esp = esp.wrapping_add(0x40);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C74A0` button_base_slot3_release_string
pub fn f_004c74a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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

/// `0X004C7780` button_base_set_text
pub fn f_004c7780(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = esi.wrapping_add(0xa8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xacu32));
                ecx = esi.wrapping_add(0xacu32);
                edx = m.r32(eax.wrapping_add(0xfffffff8u32));
                fl = lift::logic(edx & edx, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
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

/// `0X004C77D0` button_base_set_text_by_id
pub fn f_004c77d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66221eu32);
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
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 5; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0xa8u32);
                edi = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0xacu32));
                ecx = esi.wrapping_add(0xacu32);
                eax = m.r32(edx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esp.wrapping_add(0x8u32);
                m.w32(esp.wrapping_add(0x14u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = 0x1u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            5 => {
                ecx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004C78A0` button_base_get_text_ref
pub fn f_004c78a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = ecx.wrapping_add(0xa8u32);
    return eax;
    eax
}

/// `0X004C78B0` button_base_poll_repeat
pub fn f_004c78b0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                eax = m.r32(esi);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xbcu32)))) & 0xff);
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                ecx = 0x2u32;
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x4u32)), ecx, 32).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w8(esi.wrapping_add(0xbcu32), ((edx & 0xff)) as u8);
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), ecx, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0xbcu32))), (edx & 0xff), 8).1;
                if fl.zf { pc = 10; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(0x6dccf8u32);
                eax = m.r32(esi.wrapping_add(0xd8u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(eax, edx, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esi.wrapping_add(0xe0u32));
                pc = 9; continue;
            }
            8 => {
                edx = m.r32(esi.wrapping_add(0xdcu32));
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0xd8u32), edx);
                m.w32(esi.wrapping_add(0xe4u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            10 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            11 => {
                m.w8(esi.wrapping_add(0xbcu32), ((edx & 0xff)) as u8);
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

/// `0X004C7940` button_base_is_checked
pub fn f_004c7940(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x10u32)))) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C7970` button_base_set_checked
pub fn f_004c7970(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::sub(m.r32(esi.wrapping_add(0xb8u32)), 0x1u32, 32).1;
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi);
                { let t = m.r32(eax.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            3 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x10u32)))) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xcu32)))) & 0xff);
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xd4u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi);
                ecx = esp.wrapping_add(0xcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b04c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C79D0` button_base_get_toggle_text_ref
pub fn f_004c79d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    eax = ecx.wrapping_add(0xacu32);
    return eax;
    eax
}

/// `0X004C79E0` button_base_set_toggle_text
pub fn f_004c79e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let (r, f) = lift::add(ecx, 0xacu32, 32); fl = f;
    ecx = r;
    { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x64f390, ecx, &a); }
    eax
}

/// `0X004C79F0` button_base_set_toggle_text_by_id
pub fn f_004c79f0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662230u32);
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
                m.w32(esp.wrapping_add(0x10u32), 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x654c58, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                eax = 0x1u32;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            2 => {
                ecx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x8u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
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

/// `0X004C7A90` button_base_copy_to
pub fn f_004c7a90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x662250u32);
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
                esi = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                edi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b1d20, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xbcu32)))) & 0xff);
                ebp = esi;
                m.w8(esi.wrapping_add(0xbcu32), ((eax & 0xff)) as u8);
                ecx = m.r32(edi.wrapping_add(0xc4u32));
                m.w32(esi.wrapping_add(0xc4u32), ecx);
                edx = m.r32(edi.wrapping_add(0xc8u32));
                m.w32(esi.wrapping_add(0xc8u32), edx);
                eax = m.r32(edi.wrapping_add(0xb0u32));
                m.w32(esi.wrapping_add(0xb0u32), eax);
                ecx = m.r32(edi.wrapping_add(0xb8u32));
                m.w32(esi.wrapping_add(0xb8u32), ecx);
                edx = m.r32(edi.wrapping_add(0xb4u32));
                m.w32(esi.wrapping_add(0xb4u32), edx);
                ebx = edi.wrapping_add(0x94u32);
                let (r, f) = lift::sub(ebp, edi, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x24u32), 0x4u32);
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xd8u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x10u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x1cu32), 0x0u32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
                pc = 4; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 4; continue;
            }
            4 => {
                m.w32(ebx.wrapping_add(ebp), eax);
                ecx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ada30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                if !fl.zf { pc = 1; continue; }
                pc = 5; continue;
            }
            5 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(esp.wrapping_add(0x24u32), eax);
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x1cu32), 0x1u32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x4adf00, ecx, &a); }
                pc = 8; continue;
            }
            7 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 8; continue;
            }
            8 => {
                m.w32(esi.wrapping_add(0xa4u32), eax);
                ecx = m.r32(edi.wrapping_add(0xa4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x20u32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4aea50, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = edi;
                ebx = esi.wrapping_add(0xa8u32);
                let (r, f) = lift::sub(ebp, esi, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x24u32), 0x2u32);
                pc = 9; continue;
            }
            9 => {
                eax = ebx.wrapping_add(ebp);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f340, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ebx, 0x4u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                if !fl.zf { pc = 9; continue; }
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(edi.wrapping_add(0xc0u32));
                m.w32(esi.wrapping_add(0xc0u32), ecx);
                edx = m.r32(edi.wrapping_add(0xccu32));
                m.w32(esi.wrapping_add(0xccu32), edx);
                eax = m.r32(edi.wrapping_add(0xd0u32));
                m.w32(esi.wrapping_add(0xd0u32), eax);
                ecx = m.r32(edi.wrapping_add(0xd4u32));
                m.w32(esi.wrapping_add(0xd4u32), ecx);
                edx = m.r32(edi.wrapping_add(0xdcu32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                m.w32(esi.wrapping_add(0xdcu32), edx);
                eax = m.r32(edi.wrapping_add(0xe0u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0xe0u32), eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0xd8u32), eax);
                m.w32(esi.wrapping_add(0xe4u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
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

/// `0X004C7C30` button_base_set_type
pub fn f_004c7c30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                fl = lift::sub(eax, 0x3u32, 32).1;
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w32(ecx.wrapping_add(0xb8u32), eax);
                pc = 2; continue;
            }
            2 => {
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C7C50` button_base_set_mode
pub fn f_004c7c50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x1cu32));
                esi = ecx;
                ecx = m.r32(esi.wrapping_add(edi.wrapping_mul(4)).wrapping_add(0x94u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4ad8d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(edi, 0x3u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 9; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::logic(edi & edi, 32);
                if (fl.sf != fl.of) { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x4u32));
                fl = lift::sub(edi, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            6 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                m.w32(esi.wrapping_add(0x8u32), eax);
                m.w32(esi.wrapping_add(0x4u32), edi);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4b1ea0, ecx, &a); }
                pc = 8; continue;
            }
            8 => {
                eax = m.r32(esi);
                ecx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x3cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0xd8u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                return eax;
            }
            9 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
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

/// `0X004C7CD0` button_base_set_enabled
pub fn f_004c7cd0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(esp.wrapping_add(0x4u32));
                let r = (edx & 0xff) ^ (edx & 0xff); fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), (edx & 0xff), 8).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                m.w8(ecx.wrapping_add(0xbcu32), ((edx & 0xff)) as u8);
                m.w8(ecx.wrapping_add(0xbdu32), ((edx & 0xff)) as u8);
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x4u32), eax);
                { let a = [m.r32(esp.wrapping_add(4))]; return h.call_at(esp, m, 0x4b17f0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004C7D10` button_base_get_effective_state
pub fn f_004c7d10(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edi = m.r32(esi.wrapping_add(0x4u32));
                { let t = m.r32(eax.wrapping_add(0x38u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
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
                return eax;
            }
            2 => {
                fl = lift::logic(edi & edi, 32);
                if !fl.zf { pc = 4; continue; }
                pc = 3; continue;
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
            4 => {
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x14u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esi = m.r32(esi.wrapping_add(0xb8u32));
                let (r, f) = lift::sub(esi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                esi = r;
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x10u32)))) & 0xff);
                eax = 0x2u32;
                fl = lift::sub((ecx & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = edi;
                pc = 8; continue;
            }
            8 => {
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

/// `0X004C7D60` button_base_get_position_with_motion
pub fn f_004c7d60(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, esi);
                esi = m.r32(esp.wrapping_add(0x14u32));
                eax = ecx.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = esi;
                edi = m.r32(eax);
                m.w32(edx, edi);
                edi = m.r32(eax.wrapping_add(0x4u32));
                m.w32(edx.wrapping_add(0x4u32), edi);
                eax = m.r32(eax.wrapping_add(0x8u32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                ecx = m.r32(ecx.wrapping_add(0xa4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                m.w32(esp.wrapping_add(0x8u32), eax);
                m.w32(esp.wrapping_add(0xcu32), eax);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = esp.wrapping_add(0x8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4ae5f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esp.wrapping_add(0x8u32));
                ecx = m.r32(esi);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x4u32));
                m.w32(esi, ecx);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x4u32), eax);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004D1020` viper_state_switch_ctor
pub fn f_004d1020(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x662858u32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6bffa0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x8u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b15f0, ecx, &a); }
    esp = esp.wrapping_add(4);
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), eax);
    m.w32(esi, 0x671d68u32);
    m.w32(esi.wrapping_add(0x98u32), eax);
    m.w32(esi.wrapping_add(0xa4u32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x4d1130, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004D1080` viper_state_switch_ctor_named
pub fn f_004d1080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66286au32);
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
    let r = eax ^ eax; fl = lift::logic(r, 32);
    eax = r;
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), eax);
    m.w32(esi, 0x671d68u32);
    m.w32(esi.wrapping_add(0x98u32), eax);
    m.w32(esi.wrapping_add(0xa4u32), eax);
    { let a = []; eax = h.call_at(esp, m, 0x4d1130, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    eax = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    return eax;
    eax
}

/// `0X004D10E0` viper_state_switch_dtor
pub fn f_004d10e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x66287cu32);
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
    m.w32(esi, 0x671d68u32);
    m.w32(esp.wrapping_add(0x10u32), 0x0u32);
    { let a = []; eax = h.call_at(esp, m, 0x4d1130, ecx, &a); }
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

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x408b10 => f_00408b10(m, h, ecx, esp),
        0x408b70 => f_00408b70(m, h, ecx, esp),
        0x464ad0 => f_00464ad0(m, h, ecx, esp),
        0x464b40 => f_00464b40(m, h, ecx, esp),
        0x464b90 => f_00464b90(m, h, ecx, esp),
        0x464f30 => f_00464f30(m, h, ecx, esp),
        0x464f60 => f_00464f60(m, h, ecx, esp),
        0x464f90 => f_00464f90(m, h, ecx, esp),
        0x46cc20 => f_0046cc20(m, h, ecx, esp),
        0x46cc90 => f_0046cc90(m, h, ecx, esp),
        0x46cd00 => f_0046cd00(m, h, ecx, esp),
        0x46cd10 => f_0046cd10(m, h, ecx, esp),
        0x46cd60 => f_0046cd60(m, h, ecx, esp),
        0x46cd90 => f_0046cd90(m, h, ecx, esp),
        0x46cdc0 => f_0046cdc0(m, h, ecx, esp),
        0x46cde0 => f_0046cde0(m, h, ecx, esp),
        0x4af090 => f_004af090(m, h, ecx, esp),
        0x4af170 => f_004af170(m, h, ecx, esp),
        0x4af250 => f_004af250(m, h, ecx, esp),
        0x4af260 => f_004af260(m, h, ecx, esp),
        0x4af4f0 => f_004af4f0(m, h, ecx, esp),
        0x4af860 => f_004af860(m, h, ecx, esp),
        0x4af910 => f_004af910(m, h, ecx, esp),
        0x4af920 => f_004af920(m, h, ecx, esp),
        0x4af970 => f_004af970(m, h, ecx, esp),
        0x4af9b0 => f_004af9b0(m, h, ecx, esp),
        0x4afa40 => f_004afa40(m, h, ecx, esp),
        0x4afb80 => f_004afb80(m, h, ecx, esp),
        0x4afbb0 => f_004afbb0(m, h, ecx, esp),
        0x4b2480 => f_004b2480(m, h, ecx, esp),
        0x4b2500 => f_004b2500(m, h, ecx, esp),
        0x4b2580 => f_004b2580(m, h, ecx, esp),
        0x4b2680 => f_004b2680(m, h, ecx, esp),
        0x4b2690 => f_004b2690(m, h, ecx, esp),
        0x4b26a0 => f_004b26a0(m, h, ecx, esp),
        0x4b26b0 => f_004b26b0(m, h, ecx, esp),
        0x4b2970 => f_004b2970(m, h, ecx, esp),
        0x4b29a0 => f_004b29a0(m, h, ecx, esp),
        0x4b29f0 => f_004b29f0(m, h, ecx, esp),
        0x4b2a20 => f_004b2a20(m, h, ecx, esp),
        0x4b2aa0 => f_004b2aa0(m, h, ecx, esp),
        0x4b2ad0 => f_004b2ad0(m, h, ecx, esp),
        0x4b2af0 => f_004b2af0(m, h, ecx, esp),
        0x4c6910 => f_004c6910(m, h, ecx, esp),
        0x4c69a0 => f_004c69a0(m, h, ecx, esp),
        0x4c6a30 => f_004c6a30(m, h, ecx, esp),
        0x4c6aa0 => f_004c6aa0(m, h, ecx, esp),
        0x4c6b90 => f_004c6b90(m, h, ecx, esp),
        0x4c6d20 => f_004c6d20(m, h, ecx, esp),
        0x4c74a0 => f_004c74a0(m, h, ecx, esp),
        0x4c7780 => f_004c7780(m, h, ecx, esp),
        0x4c77d0 => f_004c77d0(m, h, ecx, esp),
        0x4c78a0 => f_004c78a0(m, h, ecx, esp),
        0x4c78b0 => f_004c78b0(m, h, ecx, esp),
        0x4c7940 => f_004c7940(m, h, ecx, esp),
        0x4c7970 => f_004c7970(m, h, ecx, esp),
        0x4c79d0 => f_004c79d0(m, h, ecx, esp),
        0x4c79e0 => f_004c79e0(m, h, ecx, esp),
        0x4c79f0 => f_004c79f0(m, h, ecx, esp),
        0x4c7a90 => f_004c7a90(m, h, ecx, esp),
        0x4c7c30 => f_004c7c30(m, h, ecx, esp),
        0x4c7c50 => f_004c7c50(m, h, ecx, esp),
        0x4c7cd0 => f_004c7cd0(m, h, ecx, esp),
        0x4c7d10 => f_004c7d10(m, h, ecx, esp),
        0x4c7d60 => f_004c7d60(m, h, ecx, esp),
        0x4d1020 => f_004d1020(m, h, ecx, esp),
        0x4d1080 => f_004d1080(m, h, ecx, esp),
        0x4d10e0 => f_004d10e0(m, h, ecx, esp),
        _ => return None,
    }))
}
