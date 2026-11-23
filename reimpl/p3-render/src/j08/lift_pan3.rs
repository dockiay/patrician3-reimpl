//! Generado por `oracle/lift_x86.py` (no editar a mano): transcripción de funciones del original.
#![allow(unused_assignments, unused_variables, unused_mut, clippy::all)]

use super::lift;
use super::{Host, Mem, Ret};

/// `0X00486D20` ship_side_menu_ctor
pub fn f_00486d20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x65fe5au32);
    eax = m.r32(0x0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(0x0u32, esp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    eax = m.r32(0x6b9e74u32);
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
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x440990, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = 0x3u32;
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ebx = 0x1u32;
    eax = 0x2u32;
    m.w32(esi.wrapping_add(0xc4u32), ecx);
    m.w32(esi.wrapping_add(0xdcu32), ecx);
    ecx = esi.wrapping_add(0xecu32);
    m.w32(esp.wrapping_add(0x18u32), edi);
    m.w32(esi.wrapping_add(0xb8u32), edi);
    m.w32(esi.wrapping_add(0xbcu32), ebx);
    m.w32(esi.wrapping_add(0xc0u32), eax);
    m.w32(esi.wrapping_add(0xc8u32), 0x4u32);
    m.w32(esi.wrapping_add(0xd0u32), edi);
    m.w32(esi.wrapping_add(0xd4u32), ebx);
    m.w32(esi.wrapping_add(0xd8u32), eax);
    m.w32(esi.wrapping_add(0xe0u32), 0x4u32);
    m.w32(esi.wrapping_add(0xe4u32), 0x5u32);
    m.w32(esi.wrapping_add(0xe8u32), 0x6u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1d4u32);
    m.w8(esp.wrapping_add(0x18u32), ((ebx & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2bcu32);
    m.w8(esp.wrapping_add(0x18u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x3a4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x48cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x574u32);
    m.w8(esp.wrapping_add(0x18u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x65cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x744u32);
    m.w8(esp.wrapping_add(0x18u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x82cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x914u32);
    m.w8(esp.wrapping_add(0x18u32), (0x9u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    ecx = esi.wrapping_add(0x9fcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0xau32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x1c1cu32);
    m.w8(esp.wrapping_add(0x18u32), (0xbu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1d04u32);
    m.w8(esp.wrapping_add(0x18u32), (0xcu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1decu32);
    m.w8(esp.wrapping_add(0x18u32), (0xdu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1ed8u32);
    m.w8(esp.wrapping_add(0x18u32), (0xeu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x45c110, ecx, &a); }
    ecx = esi.wrapping_add(0x2068u32);
    m.w8(esp.wrapping_add(0x18u32), (0xfu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2150u32);
    m.w8(esp.wrapping_add(0x18u32), (0x10u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2238u32);
    m.w8(esp.wrapping_add(0x18u32), (0x11u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi.wrapping_add(0x2258u32);
    m.w8(esp.wrapping_add(0x18u32), (0x12u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x46cd00u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x46cc20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x2340u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x13u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    eax = esi.wrapping_add(0x36a0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x14u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x3724u32);
    m.w8(esp.wrapping_add(0x18u32), (0x15u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x387cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x16u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x38b8u32);
    m.w8(esp.wrapping_add(0x18u32), (0x17u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x38f4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x18u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x3930u32);
    m.w8(esp.wrapping_add(0x18u32), (0x19u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x3ce4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1au32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x4170u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1bu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46f760, ecx, &a); }
    ecx = esi.wrapping_add(0x5260u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1cu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x5348u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1du32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    m.w8(esp.wrapping_add(0x18u32), (0x1eu32) as u8);
    ecx = esi.wrapping_add(0x5420u32);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x5834u32), ecx);
    edx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x5894u32), edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    eax = esi.wrapping_add(0x589cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x21u32) as u8);
    m.w16(esi.wrapping_add(0x5898u32), ((edi & 0xffff)) as u16);
    m.w16(esi.wrapping_add(0x589au32), ((ebx & 0xffff)) as u16);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    ecx = esi.wrapping_add(0x6ad0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0x22u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x6d50u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x23u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    eax = esi.wrapping_add(0x7f70u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x24u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x81f0u32);
    m.w8(esp.wrapping_add(0x18u32), (0x25u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    ecx = esi.wrapping_add(0x82d0u32);
    m.w8(esp.wrapping_add(0x18u32), (0x26u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x83c0u32), ecx);
    ecx = esi;
    m.w8(esp.wrapping_add(0x18u32), (0x28u32) as u8);
    m.w32(esi, 0x66f358u32);
    m.w32(esi.wrapping_add(0x82c8u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x4877d0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x10u32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004870E0` ship_side_menu_ctor_named
pub fn f_004870e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6600eeu32);
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
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w32(esp.wrapping_add(0x10u32), esi);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x440990, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = 0x3u32;
    let r = edi ^ edi; fl = lift::logic(r, 32);
    edi = r;
    ebx = 0x1u32;
    eax = 0x2u32;
    m.w32(esi.wrapping_add(0xc4u32), ecx);
    m.w32(esi.wrapping_add(0xdcu32), ecx);
    ecx = esi.wrapping_add(0xecu32);
    m.w32(esp.wrapping_add(0x18u32), edi);
    m.w32(esi.wrapping_add(0xb8u32), edi);
    m.w32(esi.wrapping_add(0xbcu32), ebx);
    m.w32(esi.wrapping_add(0xc0u32), eax);
    m.w32(esi.wrapping_add(0xc8u32), 0x4u32);
    m.w32(esi.wrapping_add(0xd0u32), edi);
    m.w32(esi.wrapping_add(0xd4u32), ebx);
    m.w32(esi.wrapping_add(0xd8u32), eax);
    m.w32(esi.wrapping_add(0xe0u32), 0x4u32);
    m.w32(esi.wrapping_add(0xe4u32), 0x5u32);
    m.w32(esi.wrapping_add(0xe8u32), 0x6u32);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1d4u32);
    m.w8(esp.wrapping_add(0x18u32), ((ebx & 0xff)) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2bcu32);
    m.w8(esp.wrapping_add(0x18u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x3a4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x48cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x574u32);
    m.w8(esp.wrapping_add(0x18u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x65cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x744u32);
    m.w8(esp.wrapping_add(0x18u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x82cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x914u32);
    m.w8(esp.wrapping_add(0x18u32), (0x9u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    ecx = esi.wrapping_add(0x9fcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0xau32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x1c1cu32);
    m.w8(esp.wrapping_add(0x18u32), (0xbu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1d04u32);
    m.w8(esp.wrapping_add(0x18u32), (0xcu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1decu32);
    m.w8(esp.wrapping_add(0x18u32), (0xdu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x1ed8u32);
    m.w8(esp.wrapping_add(0x18u32), (0xeu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x45c110, ecx, &a); }
    ecx = esi.wrapping_add(0x2068u32);
    m.w8(esp.wrapping_add(0x18u32), (0xfu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2150u32);
    m.w8(esp.wrapping_add(0x18u32), (0x10u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x2238u32);
    m.w8(esp.wrapping_add(0x18u32), (0x11u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b03e0, ecx, &a); }
    ecx = esi.wrapping_add(0x2258u32);
    m.w8(esp.wrapping_add(0x18u32), (0x12u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x46cd00u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x46cc20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x2340u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x13u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    eax = esi.wrapping_add(0x36a0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x14u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x3724u32);
    m.w8(esp.wrapping_add(0x18u32), (0x15u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x387cu32);
    m.w8(esp.wrapping_add(0x18u32), (0x16u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x38b8u32);
    m.w8(esp.wrapping_add(0x18u32), (0x17u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x38f4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x18u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a440, ecx, &a); }
    ecx = esi.wrapping_add(0x3930u32);
    m.w8(esp.wrapping_add(0x18u32), (0x19u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x3ce4u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1au32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = esi.wrapping_add(0x4170u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1bu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46f760, ecx, &a); }
    ecx = esi.wrapping_add(0x5260u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1cu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = esi.wrapping_add(0x5348u32);
    m.w8(esp.wrapping_add(0x18u32), (0x1du32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    m.w8(esp.wrapping_add(0x18u32), (0x1eu32) as u8);
    ecx = esi.wrapping_add(0x5420u32);
    { let a = []; eax = h.call_at(esp, m, 0x430f90, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x5834u32), ecx);
    edx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x5894u32), edx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    eax = esi.wrapping_add(0x589cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x21u32) as u8);
    m.w16(esi.wrapping_add(0x5898u32), ((edi & 0xffff)) as u16);
    m.w16(esi.wrapping_add(0x589au32), ((ebx & 0xffff)) as u16);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    ecx = esi.wrapping_add(0x6ad0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x2cu32), (0x22u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6910u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x6d50u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x2cu32), (0x23u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b03e0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    eax = esi.wrapping_add(0x7f70u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x2cu32), (0x24u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
    esp = esp.wrapping_add(20);
    ecx = esi.wrapping_add(0x81f0u32);
    m.w8(esp.wrapping_add(0x18u32), (0x25u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4acdd0, ecx, &a); }
    ecx = esi.wrapping_add(0x82d0u32);
    m.w8(esp.wrapping_add(0x18u32), (0x26u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6910, ecx, &a); }
    ecx = m.r32(0x6c7cccu32);
    m.w32(esi.wrapping_add(0x83c0u32), ecx);
    ecx = esi;
    m.w8(esp.wrapping_add(0x18u32), (0x28u32) as u8);
    m.w32(esi, 0x66f358u32);
    m.w32(esi.wrapping_add(0x82c8u32), edi);
    { let a = []; eax = h.call_at(esp, m, 0x4877d0, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x10u32));
    eax = esi;
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X004874A0` ship_side_menu_dtor
pub fn f_004874a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x660382u32);
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
    m.w32(esi, 0x66f358u32);
    m.w32(esp.wrapping_add(0x10u32), 0x28u32);
    { let a = []; eax = h.call_at(esp, m, 0x4877d0, ecx, &a); }
    ecx = esi.wrapping_add(0x83c0u32);
    m.w8(esp.wrapping_add(0x10u32), (0x27u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0x82d0u32);
    m.w8(esp.wrapping_add(0x10u32), (0x26u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x81f0u32);
    m.w8(esp.wrapping_add(0x10u32), (0x25u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4ace40, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    eax = esi.wrapping_add(0x7f70u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    m.w8(esp.wrapping_add(0x20u32), (0x24u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    ecx = esi.wrapping_add(0x6d50u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x20u32), (0x23u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x6ad0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x20u32), (0x22u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    m.w8(esp.wrapping_add(0x18u32), (0x21u32) as u8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    eax = esi.wrapping_add(0x589cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0x5894u32);
    m.w8(esp.wrapping_add(0x10u32), (0x20u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0x5834u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1fu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
    ecx = esi.wrapping_add(0x5420u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1eu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
    ecx = esi.wrapping_add(0x5348u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1du32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4ace40, ecx, &a); }
    ecx = esi.wrapping_add(0x5260u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1cu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x4170u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1bu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x46f860, ecx, &a); }
    ecx = esi.wrapping_add(0x3ce4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1au32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
    ecx = esi.wrapping_add(0x3930u32);
    m.w8(esp.wrapping_add(0x10u32), (0x19u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x430fe0, ecx, &a); }
    ecx = esi.wrapping_add(0x38f4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x18u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a480, ecx, &a); }
    ecx = esi.wrapping_add(0x38b8u32);
    m.w8(esp.wrapping_add(0x10u32), (0x17u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a480, ecx, &a); }
    ecx = esi.wrapping_add(0x387cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x16u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x40a480, ecx, &a); }
    ecx = esi.wrapping_add(0x3724u32);
    m.w8(esp.wrapping_add(0x10u32), (0x15u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4b0440u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4u32);
    ecx = esi.wrapping_add(0x36a0u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x20u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    m.w8(esp.wrapping_add(0x20u32), (0x14u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x46cd00u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    edx = esi.wrapping_add(0x2340u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xf8u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w8(esp.wrapping_add(0x20u32), (0x13u32) as u8);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0x2258u32);
    m.w8(esp.wrapping_add(0x10u32), (0x12u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x2238u32);
    m.w8(esp.wrapping_add(0x10u32), (0x11u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4b0440, ecx, &a); }
    ecx = esi.wrapping_add(0x2150u32);
    m.w8(esp.wrapping_add(0x10u32), (0x10u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x2068u32);
    m.w8(esp.wrapping_add(0x10u32), (0xfu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x1ed8u32);
    m.w8(esp.wrapping_add(0x10u32), (0xeu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x45c180, ecx, &a); }
    ecx = esi.wrapping_add(0x1decu32);
    m.w8(esp.wrapping_add(0x10u32), (0xdu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x1d04u32);
    m.w8(esp.wrapping_add(0x10u32), (0xcu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x1c1cu32);
    m.w8(esp.wrapping_add(0x10u32), (0xbu32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x4c6a30u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x14u32);
    m.w8(esp.wrapping_add(0x18u32), (0xau32) as u8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xe8u32);
    eax = esi.wrapping_add(0x9fcu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
    esp = esp.wrapping_add(16);
    ecx = esi.wrapping_add(0x914u32);
    m.w8(esp.wrapping_add(0x10u32), (0x9u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x82cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x8u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x744u32);
    m.w8(esp.wrapping_add(0x10u32), (0x7u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x65cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x6u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x574u32);
    m.w8(esp.wrapping_add(0x10u32), (0x5u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x48cu32);
    m.w8(esp.wrapping_add(0x10u32), (0x4u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x3a4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x3u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x2bcu32);
    m.w8(esp.wrapping_add(0x10u32), (0x2u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0x1d4u32);
    m.w8(esp.wrapping_add(0x10u32), (0x1u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi.wrapping_add(0xecu32);
    m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
    { let a = []; eax = h.call_at(esp, m, 0x4c6a30, ecx, &a); }
    ecx = esi;
    m.w32(esp.wrapping_add(0x10u32), 0xffffffffu32);
    { let a = []; eax = h.call_at(esp, m, 0x440a20, ecx, &a); }
    ecx = m.r32(esp.wrapping_add(0x8u32));
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    m.w32(0x0u32, ecx);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X00487C30` ship_side_menu_set_ship_list
pub fn f_00487c30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x28u32));
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                fl = lift::sub(edi, ebx, 32).1;
                esi = ecx;
                if fl.zf { pc = 51; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(edi);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 51; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = m.r32(edi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6cc950u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x477cf0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d500u32);
                fl = lift::sub(edx, eax, 32).1;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if fl.zf { pc = 47; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(0x8u32))) & 0x1u32, 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(m.r32(edi), 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
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
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                edx = m.r32(edi);
                m.w32(esi.wrapping_add(0xacu32), edx);
                eax = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esi.wrapping_add(0xa0u32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, m.r32(0x66d504u32), 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = edx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x3930u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = esi.wrapping_add(0x387cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                m.w32(esp.wrapping_add(0x18u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x3880u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x14u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x14u32));
                ecx = 0x6da5f0u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
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
            10 => {
                fl = lift::sub(eax, m.r32(0x66d4f8u32), 32).1;
                if !fl.zf { pc = 54; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(0x66d4fcu32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                m.w32(esi.wrapping_add(0x94u32), ecx);
                ecx = m.r32(0x6dccf0u32);
                edx = m.r32(0x6cbd98u32);
                edi = ecx;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                fl = lift::sub(edi, edx, 32).1;
                if fl.cf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                m.w32(esi.wrapping_add(0x9cu32), ecx);
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
            13 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            14 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc4u32)), 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 16; continue;
            }
            16 => {
                ebp = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(ebp, ebx, 32).1;
                if fl.zf { pc = 29; continue; }
                pc = 17; continue;
            }
            17 => {
                fl = lift::sub(m.r32(edi), 0x1u32, 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(0x66d4fcu32);
                fl = lift::sub(edx, ecx, 32).1;
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(edx, m.r32(0x66d504u32), 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(edi.wrapping_add(0x4u32));
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(ebp)), 16).1;
                if !fl.zf { pc = 24; continue; }
                pc = 21; continue;
            }
            21 => {
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(0x8u32))) & 0x8u32, 8);
                if !fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 23; continue;
            }
            23 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                m.w32(edi.wrapping_add(0x4u32), 0x0u32);
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
            24 => {
                fl = lift::sub(edx, m.r32(0x66d4f8u32), 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(edi.wrapping_add(0x4u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(esi.wrapping_add(0x3868u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(esi.wrapping_add(0x9cu32));
                m.w32(esi.wrapping_add(0x94u32), ecx);
                ecx = m.r32(0x6dccf0u32);
                edx = m.r32(0x6cbd98u32);
                ebx = ecx;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                fl = lift::sub(ebx, edx, 32).1;
                if !fl.cf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 28; continue;
            }
            28 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0xa0u32), eax);
                edx = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esi.wrapping_add(0xa0u32), edx);
                m.w32(esi.wrapping_add(0xacu32), 0x1u32);
                m.w8(esi.wrapping_add(0xb0u32), ((eax & 0xff)) as u8);
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
            29 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::sub(eax, ebx, 32).1;
                m.w32(esp.wrapping_add(0x28u32), eax);
                if fl.zf { pc = 47; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(0x8u32))) & 0x8u32, 8);
                if fl.zf { pc = 47; continue; }
                pc = 33; continue;
            }
            33 => {
                ecx = m.r32(edi.wrapping_add(0x4u32));
                ebp = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(ebp).wrapping_add(0x8u32)))) & 0xffff);
                ebx = m.r32(0x6dd7a8u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = ebx.wrapping_add(eax.wrapping_mul(4));
                ebx = m.r32(0x66d4fcu32);
                fl = lift::sub(edx, ebx, 32).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub(edx, m.r32(0x66d504u32), 32).1;
                if !fl.zf { pc = 40; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(esi.wrapping_add(0x5238u32));
                ebx = m.r32(esp.wrapping_add(0x28u32));
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub((ebp & 0xffff), u32::from(m.r16(ebx.wrapping_add(edx.wrapping_mul(2)))), 16).1;
                if !fl.zf { pc = 39; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 38; continue;
            }
            38 => {
                m.w32(edi.wrapping_add(0x4u32), 0x0u32);
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
            39 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                pc = 41; continue;
            }
            40 => {
                ebx = m.r32(esp.wrapping_add(0x28u32));
                pc = 41; continue;
            }
            41 => {
                fl = lift::sub(edx, m.r32(0x66d4f8u32), 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 42; continue;
            }
            42 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x10u32)))) & 0xffff);
                eax = m.r32(esi.wrapping_add(0x3868u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 47; continue; }
                pc = 43; continue;
            }
            43 => {
                edx = m.r32(0x66d4fcu32);
                eax = m.r32(esi.wrapping_add(0x9cu32));
                m.w32(esi.wrapping_add(0x94u32), edx);
                ecx = m.r32(0x6dccf0u32);
                edx = m.r32(0x6cbd98u32);
                ebp = ecx;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                fl = lift::sub(ebp, edx, 32).1;
                if !fl.cf { pc = 45; continue; }
                pc = 44; continue;
            }
            44 => {
                let (r, f) = lift::add(ecx, ecx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                pc = 45; continue;
            }
            45 => {
                m.w32(esi.wrapping_add(0x9cu32), ecx);
                edx = m.r32(edi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0xa0u32), edx);
                m.w32(esi.wrapping_add(0xacu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48fe80, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic(ebx & ebx, 32);
                m.w8(esi.wrapping_add(0xb0u32), (0x8u32) as u8);
                if fl.zf { pc = 54; continue; }
                pc = 46; continue;
            }
            46 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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
            47 => {
                eax = m.r32(edi.wrapping_add(0x4u32));
                ebp = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8u32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                m.w8(esi.wrapping_add(0xb0u32), ((ecx & 0xff)) as u8);
                edx = m.r32(edi);
                m.w32(esi.wrapping_add(0xacu32), edx);
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x8u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & 0x8u32, 8);
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x8u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x5228u32), ((ecx & 0xffff)) as u16);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48fe80, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 49; continue;
            }
            49 => {
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 54; continue; }
                pc = 50; continue;
            }
            50 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
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
            51 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x440f70, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, ebx, 32).1;
                if fl.zf { pc = 53; continue; }
                pc = 52; continue;
            }
            52 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 53; continue;
            }
            53 => {
                m.w32(esi.wrapping_add(0xa0u32), ebx);
                m.w32(esi.wrapping_add(0xacu32), ebx);
                pc = 54; continue;
            }
            54 => {
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

/// `0X00488080` ship_side_menu_show
pub fn f_00488080(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66039cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xb0u32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & 0x1u32, 8);
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0xacu32)), 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | (((ebx & 0xff)) & 0xff);
                let r = (eax & 0xff) & 0x8u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), 0x8u32, 8).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                edx = m.r32(esi.wrapping_add(0xdcu32));
                pc = 16; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edi = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 11; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = eax;
                edi = m.r32(0x6dd7a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ecx = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = edi.wrapping_add(ecx.wrapping_mul(4));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 11; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x14u32)))) & 0xff);
                let r = (ecx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                ecx = (ecx & !0xff) | ((r) & 0xff);
                fl = lift::sub((ecx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 11; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::logic((ebx & 0xff) & 0x8u32, 8);
                if fl.zf { pc = 11; continue; }
                pc = 8; continue;
            }
            8 => {
                m.w16(esi.wrapping_add(0x5228u32), ((eax & 0xffff)) as u16);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax)))) & 0xff);
                eax = m.r32(0x6dfc14u32);
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                edx = m.r32(esi.wrapping_add(0xdcu32));
                pc = 16; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0xc8u32));
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                m.w32(esi.wrapping_add(0xb4u32), eax);
                m.w32(esi.wrapping_add(0xccu32), ecx);
                pc = 18; continue;
            }
            11 => {
                ecx = m.r32(0x6dfc14u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx))) & 0xff);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 15; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::logic((ebx & 0xff) & 0x2u32, 8);
                m.w32(esi.wrapping_add(0xb4u32), ecx);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(esi.wrapping_add(0xd8u32));
                pc = 17; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0xd0u32));
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 18; continue;
            }
            15 => {
                ecx = m.r32(esi.wrapping_add(0xc4u32));
                edx = m.r32(esi.wrapping_add(0xe8u32));
                pc = 16; continue;
            }
            16 => {
                m.w32(esi.wrapping_add(0xb4u32), ecx);
                pc = 17; continue;
            }
            17 => {
                m.w32(esi.wrapping_add(0xccu32), edx);
                pc = 18; continue;
            }
            18 => {
                ecx = esi.wrapping_add(0x5894u32);
                { let a = []; eax = h.call_at(esp, m, 0x64f1de, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                m.w8(esi.wrapping_add(0x5832u32), (0x0u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x489b80, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 29; continue; }
                pc = 19; continue;
            }
            19 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 23; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0x4098u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x48fa90, ecx, &a); }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0xcu32), ecx);
                edx = m.r32(0x6998acu32);
                eax = m.r32(0x69988cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9eacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0x10);
                edi = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x699874u32);
                eax = m.r32(0x6998acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9eb8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0x10);
                edi = esi.wrapping_add(0x914u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xcu32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0xcu32);
                m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 29; continue;
            }
            23 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xb8u32)), 32).1;
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                m.w32(esi.wrapping_add(0x4164u32), 0x0u32);
                pc = 29; continue;
            }
            25 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                m.w32(esi.wrapping_add(0x4098u32), 0xffffffffu32);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4906e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                edx = m.r32(0x699908u32);
                eax = m.r32(0x69988cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ec4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x2cu32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x10);
                edi = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x699874u32);
                eax = m.r32(0x699908u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ed0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_add(0x10);
                edi = esi.wrapping_add(0x914u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x130u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x1cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 29; continue;
            }
            27 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                edx = m.r32(esi.wrapping_add(0x5238u32));
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x4098u32), 0xffffffffu32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                ecx = esi;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                edi = (edi & !0xffff) | ((u32::from(m.r8(edx.wrapping_add(eax)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4906e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494f90, ecx, &a); }
                pc = 29; continue;
            }
            29 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440e50, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00488440` ship_side_menu_refresh_and_invalidate
pub fn f_00488440(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    esi = ecx;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esi;
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    { let a = []; return h.call_at(esp, m, 0x441710, ecx, &a); }
    eax
}

/// `0X00488460` ship_side_menu_hide
pub fn f_00488460(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                ecx = m.r32(0x6cbb40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xau32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x42d6d0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cba98u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6cba98u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0x94u32));
                edx = m.r32(0x66d504u32);
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 4; continue;
            }
            4 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                m.w32(esi.wrapping_add(0x3868u32), eax);
                pc = 9; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                m.w32(esi.wrapping_add(0x3868u32), eax);
                pc = 9; continue;
            }
            8 => {
                m.w32(esi.wrapping_add(0x3868u32), 0xffffffffu32);
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(0x6d9584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(0x6cbb10u32);
                fl = lift::sub(m.r32(ecx.wrapping_add(0x5acu32)), 0x4950d0u32, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 11; continue;
            }
            11 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x440ed0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00488540` ship_side_menu_close
pub fn f_00488540(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax | 0xffffffffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x3868u32), eax);
                m.w32(esi.wrapping_add(0x4168u32), eax);
                m.w16(esi.wrapping_add(0x5228u32), (0xffffu32) as u16);
                eax = m.r32(0x6cbd8cu32);
                fl = lift::sub(eax, esi, 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                m.w32(esi.wrapping_add(0xa0u32), 0x0u32);
                m.w32(esi.wrapping_add(0xacu32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x489b80, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 4; continue;
            }
            4 => {
                ecx = esi;
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                { let a = []; return h.call_at(esp, m, 0x4410f0, ecx, &a); }
            }
            _ => unreachable!(),
        }
    }
}

/// `0X004885A0` ship_side_menu_get_ship_list
pub fn f_004885a0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0xb0u32)))) & 0xff);
                m.w8(eax, ((edx & 0xff)) as u8);
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(ecx.wrapping_add(0xa0u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(u32::from(m.r8(ecx.wrapping_add(0xb0u32))), 0x8u32, 8).1;
                if !fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0xcu32)))) & 0xff);
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = m.r32(ecx.wrapping_add(0x5238u32));
                edx = eax.wrapping_add(ecx.wrapping_mul(2));
                eax = m.r32(esp.wrapping_add(0x4u32));
                m.w32(eax, edx);
                eax = 0x1u32;
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                edx = m.r32(esp.wrapping_add(0x4u32));
                m.w32(edx, eax);
                eax = m.r32(ecx.wrapping_add(0xacu32));
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00488600` ship_side_menu_load_layout
pub fn f_00488600(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6603cfu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0xd0);
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
                m.w32(esp.wrapping_add(0xe8u32), 0x0u32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esp.wrapping_add(0xf4u32));
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9edcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0xf4u32), (0x1u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0xfcu32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9eecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x1cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(0x6beee0u32);
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1000u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ef4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0x100u32), (0x2u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4be060, ecx, &a); }
                esp = esp.wrapping_add(24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6dbb58u32);
                ecx = esp.wrapping_add(0x24u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                m.w8(esp.wrapping_add(0xf8u32), (0x3u32) as u8);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                ebp = eax;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xd4u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f2cu32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xdcu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f40u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f54u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x48u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xc8u32), eax);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                ebx = eax;
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f70u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f7cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f84u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x50u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9f90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fa0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xb8u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fb0u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x90u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fc0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x68u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xd8u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fdcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x98u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9fe8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6b9ff8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x70u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xa0u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba008u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x68u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba01cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba028u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xd0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x70u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba034u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xa8u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba044u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba054u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x78u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x80u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba078u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba084u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xc0u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x88u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba094u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xdcu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x44u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0c4u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x3cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0dcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x5cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x54u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x5cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba0f4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba100u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x74u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x6cu32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba110u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x74u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba11cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba128u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0x8cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0x84u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba138u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0x8cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba144u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba154u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xa4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x9cu32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xbcu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba170u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba184u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xbcu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0xb4u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba194u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esp.wrapping_add(0xa4u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esp.wrapping_add(0xd4u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                m.w32(esp.wrapping_add(0xd0u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1c4u32);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w16(esi.wrapping_add(0x582cu32), ((eax & 0xffff)) as u16);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w16(esi.wrapping_add(0x582eu32), ((eax & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x3864u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba1ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esi.wrapping_add(0x3874u32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba200u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba214u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6dd524u32;
                m.w32(esi.wrapping_add(0x386cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esi.wrapping_add(0x3870u32), eax);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba220u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3878u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2bcu32));
                edi = esi.wrapping_add(0x2bcu32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xecu32));
                ebp = esi.wrapping_add(0xecu32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x20u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1d4u32));
                ebx = esi.wrapping_add(0x1d4u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xd0u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2258u32));
                ecx = esi.wrapping_add(0x2258u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xd4u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xdcu32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                ecx = esi.wrapping_add(0x3a4u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x38u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0xc8u32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x65cu32));
                ecx = esi.wrapping_add(0x65cu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3938u32));
                ecx = esi.wrapping_add(0x3938u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                edx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3cecu32));
                ecx = esi.wrapping_add(0x3cecu32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5428u32));
                ecx = esi.wrapping_add(0x5428u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x387cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x90u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x48u32));
                m.w32(esi.wrapping_add(0x3810u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb8u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3814u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3818u32), eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x98u32));
                m.w32(esi.wrapping_add(0x380cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x58u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3824u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x381cu32), eax);
                eax = m.r32(esp.wrapping_add(0xd8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x60u32));
                m.w32(esi.wrapping_add(0x3820u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xa0u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3828u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x382cu32), eax);
                eax = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xc0u32));
                m.w32(esi.wrapping_add(0x3830u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x70u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3834u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3838u32), eax);
                eax = m.r32(esp.wrapping_add(0xa8u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0xf8u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x38b8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x78u32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x38f4u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x40a560, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x80u32));
                m.w32(esi.wrapping_add(0x383cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb0u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x3840u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3844u32), eax);
                eax = m.r32(esp.wrapping_add(0x88u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0x3848u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x384cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x3854u32), eax);
                eax = m.r32(esp.wrapping_add(0x44u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                m.w32(esi.wrapping_add(0x3850u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x54u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5824u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5858u32), eax);
                eax = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x64u32));
                m.w32(esi.wrapping_add(0x5854u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x6cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x585cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5850u32), eax);
                eax = m.r32(esp.wrapping_add(0x74u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                m.w32(esi.wrapping_add(0x5870u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x84u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5868u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5864u32), eax);
                eax = m.r32(esp.wrapping_add(0x8cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x94u32));
                m.w32(esi.wrapping_add(0x586cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x9cu32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5860u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5880u32), eax);
                eax = m.r32(esp.wrapping_add(0xa4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xacu32));
                m.w32(esi.wrapping_add(0x588cu32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xb4u32));
                ecx = 0x6da820u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esi.wrapping_add(0x5884u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x5888u32), eax);
                eax = m.r32(esp.wrapping_add(0xbcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xc4u32));
                m.w32(esi.wrapping_add(0x5890u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da820u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b3dd0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                m.w32(esi.wrapping_add(0x385cu32), eax);
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba238u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(ebp);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba244u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(edi);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                eax = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba250u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(ebx);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba25cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                ecx = esi.wrapping_add(0x3a4u32);
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba270u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba284u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x65cu32));
                ecx = esi.wrapping_add(0x65cu32);
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba294u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp.wrapping_add(0x30u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x5818u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x580cu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2bcu32);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x57ecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi.wrapping_add(0x57e0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esp.wrapping_add(0xf0u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x57f8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bd0e0, ecx, &a); }
                esp = esp.wrapping_add(12);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x744u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx.wrapping_mul(4));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x82cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = esi.wrapping_add(0x914u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x3724u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                eax = esi.wrapping_add(0x36a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                eax = esi.wrapping_add(0x36c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                eax = esi.wrapping_add(0x36e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                eax = esi.wrapping_add(0x3700u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x1d04u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = esi.wrapping_add(0x1decu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2ecu32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2eeu32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7c30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0xccu32));
                edx = esp.wrapping_add(0xf0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1ed8u32));
                edi = esi.wrapping_add(0x1ed8u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                eax = esi.wrapping_add(0x2238u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x10u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                pc = 8; continue;
            }
            2 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x20u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                pc = 8; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x30u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
                pc = 8; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999b4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x60u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                pc = 8; continue;
            }
            8 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x2068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6999b4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x10u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                pc = 16; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x20u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de57u32))) & 0xff);
                pc = 16; continue;
            }
            12 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x30u32, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
                pc = 16; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                edx = m.r32(0x6999c8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c1b20, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x60u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de54u32))) & 0xff);
                pc = 16; continue;
            }
            16 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = eax.wrapping_add(ecx.wrapping_mul(4));
                ebx = eax.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(0x6999c8u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = esi.wrapping_add(0x2150u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x69c98cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba2f0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dd524u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba308u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba318u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xbu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba328u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2340u32));
                edi = esi.wrapping_add(0x2340u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xecu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x2380u32), (0x5u32) as u8);
                ebx = esi.wrapping_add(0x2438u32);
                ebp = 0x13u32;
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46cdc0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xf8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 17; continue; }
                pc = 18; continue;
            }
            18 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edi = esi.wrapping_add(0x9fcu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de55u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x69b338u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0xae4u32);
                ebp = 0x13u32;
                pc = 19; continue;
            }
            19 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                if !fl.zf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esp.wrapping_add(0xf0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba354u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba340u32);
                ecx = 0x6dd524u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x4be0b0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0xf8u32);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64efc8, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x4170u32));
                ecx = esi.wrapping_add(0x4170u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x130u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1du32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba360u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5348u32));
                ecx = esi.wrapping_add(0x5348u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = esi.wrapping_add(0x6d50u32);
                ebp = esi.wrapping_add(0x7f70u32);
                m.w32(esp.wrapping_add(0x1cu32), 0x14u32);
                pc = 21; continue;
            }
            21 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebx = edi.wrapping_add(0xffffeb4cu32);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x66de58u32))) & 0xff);
                ecx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                edx = eax.wrapping_add(ecx.wrapping_mul(4));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6cbe08u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7a90, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                ecx = ebp.wrapping_add(0xffffeb60u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x10u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ebp, 0x20u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 21; continue; }
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1du32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba378u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x81f0u32));
                ecx = esi.wrapping_add(0x81f0u32);
                { let t = m.r32(eax.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(0x66de56u32))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esp;
                m.w32(esp.wrapping_add(0x1cu32), esp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x66de5cu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f2c1, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                let (r, f) = lift::add(esi, 0x82d0u32, 32); fl = f;
                esi = r;
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4e57u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x104u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0x20u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x2u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x18u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x1u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0x10u32);
                m.w8(esp.wrapping_add(0xe8u32), (0x0u32) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = esp.wrapping_add(0xf0u32);
                m.w32(esp.wrapping_add(0xe8u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0xe0u32));
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
                esp = esp.wrapping_add(0xdc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X00489B80` ship_side_menu_show_or_hide_widgets
pub fn f_00489b80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x6603e1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x64);
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
                edi = eax;
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x84u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x3860u32)))) & 0xff);
                if fl.zf { pc = 17; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 12; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = esi.wrapping_add(0x574u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x5260u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x5348u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x82d0u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                m.w32(esi.wrapping_add(0x57d8u32), 0x0u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                fl = lift::logic(u32::from(m.r8(eax.wrapping_add(0x136u32))) & 0x1u32, 8);
                ecx = ebx;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            4 => {
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(esi.wrapping_add(0x38f8u32));
                ebp = m.r32(esi.wrapping_add(0x14u32));
                ebx = esi.wrapping_add(0x38f4u32);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x24u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x84u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), ecx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x84u32));
                ecx = ebx;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x84u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), 0x0u32);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ebp = esi.wrapping_add(0x589cu32);
                m.w32(esp.wrapping_add(0x14u32), eax);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x84u32));
                eax = m.r32(ebp.wrapping_add(0xffffb160u32));
                ebx = ebp.wrapping_add(0xffffb160u32);
                ecx = edx.wrapping_add(ecx).wrapping_add(0x5au32);
                edx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x14u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x84u32));
                ebx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = edx.wrapping_add(ecx).wrapping_add(0x5au32);
                edx = m.r32(esi.wrapping_add(0x9fcu32));
                ecx = esi.wrapping_add(0x9fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x84u32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebp;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp.wrapping_add(0x18u32));
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba390u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_add(0xc);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x7cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x84u32));
                eax = m.r32(esp.wrapping_add(0x14u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                let (r, f) = lift::add(eax, 0xf8u32, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
                eax = 0x14u32;
                let (r, f) = lift::add(ebp, 0xe8u32, 32); fl = f;
                ebp = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                ebp = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                m.w32(esp.wrapping_add(0x84u32), eax);
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                edx = ebx.wrapping_add(ecx).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = ebx.wrapping_add(ebx.wrapping_mul(4));
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x46cd60, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x84u32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x84u32), eax);
                if !fl.zf { pc = 8; continue; }
                pc = 9; continue;
            }
            9 => {
                m.w32(esp.wrapping_add(0x84u32), ebx);
                ebp = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x1cu32), 0x14u32);
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x84u32));
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                eax = ecx.wrapping_add(eax).wrapping_add(0x5au32);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x84u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x84u32), edx);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if !fl.zf { pc = 10; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 0xc8u32, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, 0x5au32, 32); fl = f;
                edx = r;
                ecx = esi.wrapping_add(0x5348u32);
                m.w32(esp.wrapping_add(0x48u32), eax);
                m.w32(esp.wrapping_add(0x4cu32), edx);
                eax = esp.wrapping_add(0x48u32);
                edx = m.r32(ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x54u32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                eax = esp.wrapping_add(0x48u32);
                let (r, f) = lift::add(ebp, 0x4u32, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x4cu32), ebp);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ebp = esi.wrapping_add(0x82d0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = 0xffffffd6u32;
                edx = m.r32(ebp);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x28u32));
                m.w32(esp.wrapping_add(0x48u32), eax);
                eax = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = esi.wrapping_add(0x2bcu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1d4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x3a4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x48cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x65cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                ebp = esi.wrapping_add(0x387cu32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = ebp;
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x58u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                edx = eax;
                eax = m.r32(esp.wrapping_add(0x84u32));
                ecx = m.r32(esi.wrapping_add(0x3880u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x60u32), edx);
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x54u32), ecx);
                eax = eax.wrapping_add(edx).wrapping_add(0xfffffffau32);
                ecx = esp.wrapping_add(0x54u32);
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x64u32), eax);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                ebx = esi.wrapping_add(0x3938u32);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x88u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x30u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                ebp = esi.wrapping_add(0x38b8u32);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebp;
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x58u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = ebp;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x38bcu32));
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x84u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x60u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x54u32), edx);
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffau32);
                eax = esp.wrapping_add(0x54u32);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x64u32), edx);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                ebx = esi.wrapping_add(0x3cecu32);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x88u32));
                edx = esp.wrapping_add(0x30u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x38fcu32));
                ebx = esi.wrapping_add(0x38f4u32);
                m.w32(esp.wrapping_add(0x84u32), ecx);
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                ebp = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x84u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebx;
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x84u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = esi.wrapping_add(0x38f4u32);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x58u32));
                ecx = ebx;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x38f8u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x60u32), ebp);
                m.w32(esp.wrapping_add(0x54u32), edx);
                edx = eax.wrapping_add(ecx).wrapping_add(0xfffffffau32);
                eax = esp.wrapping_add(0x54u32);
                ecx = esp.wrapping_add(0x64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x64u32), edx);
                { let t = m.r32(0x66a4a4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                ebp = esi.wrapping_add(0x5428u32);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebx;
                m.w32(esp.wrapping_add(0x88u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x88u32));
                edx = esp.wrapping_add(0x68u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x460ef0, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ebx = esi.wrapping_add(0x82cu32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                m.w32(esp.wrapping_add(0x44u32), 0x0u32);
                ecx = eax;
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ebx);
                m.w32(esp.wrapping_add(0x40u32), eax);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x84u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld64(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                { let v = lift::fld64(m, 0x66f498u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ebp = m.r32(esp.wrapping_add(0x40u32));
                ecx = esp.wrapping_add(0x3cu32);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x914u32));
                m.w32(esp.wrapping_add(0x40u32), ebp);
                ebp = esi.wrapping_add(0x914u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x84u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld64(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                { let v = lift::fld64(m, 0x66f498u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                edx = m.r32(esp.wrapping_add(0x40u32));
                ecx = esi.wrapping_add(0x744u32);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(ecx);
                m.w32(esp.wrapping_add(0x40u32), edx);
                edx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                m.w32(esp.wrapping_add(0x84u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x84u32)) as i32); fp.push(v); }
                lift::fst64(m, esp.wrapping_add(0x2cu32), fp.st(0)); fp.pop();
                { let v = lift::fld64(m, esp.wrapping_add(0x2cu32)); fp.push(v); }
                { let v = lift::fld64(m, 0x66f490u32); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esi.wrapping_add(0x3724u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = esp.wrapping_add(0x3cu32);
                m.w32(esp.wrapping_add(0x40u32), ecx);
                ecx = esi.wrapping_add(0x3724u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x68u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x744u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x3724u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esp.wrapping_add(0x88u32), eax);
                ebx = esi.wrapping_add(0x2150u32);
                { let t = m.r32(eax.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(ebx);
                ecx = ebx;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                { let t = m.r32(edx.wrapping_add(0x48u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = eax.wrapping_add(ebp).wrapping_add(0xfffffff1u32);
                ebp = esi.wrapping_add(0x1c1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = ebp;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x8cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(ebp.wrapping_add(0x48u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx.wrapping_add(edx).wrapping_add(0xfffffff6u32);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x57fcu32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x57e0u32));
                edx = m.r32(esi.wrapping_add(0x1d04u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                ebx = esi.wrapping_add(0x1d04u32);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                edx = m.r32(esi.wrapping_add(0x57fcu32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                eax = m.r32(ebx);
                ebp = m.r32(esi.wrapping_add(0x1ed8u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x8cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x8cu32));
                ebx = esi.wrapping_add(0x1ed8u32);
                eax = ecx.wrapping_add(eax).wrapping_add(0x2u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x57fcu32));
                edx = m.r32(esi.wrapping_add(0x18u32));
                ebp = m.r32(esi.wrapping_add(0x1decu32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x8cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x8cu32));
                ebx = esi.wrapping_add(0x1decu32);
                eax = ecx.wrapping_add(eax).wrapping_add(0x2u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x57fcu32));
                eax = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(ebx);
                ebp = m.r32(esi.wrapping_add(0x2068u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(8);
                edx = m.r32(ebx);
                ecx = ebx;
                m.w32(esp.wrapping_add(0x8cu32), eax);
                { let t = m.r32(edx.wrapping_add(0x44u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x8cu32));
                ebx = esi.wrapping_add(0x2068u32);
                eax = ecx.wrapping_add(eax).wrapping_add(0x3u32);
                ecx = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(ebp.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1decu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1d04u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1ed8u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1c1cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x2150u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x18u32));
                ebp = m.r32(0x6cbd5cu32);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x2258u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ebx = esi.wrapping_add(0x2258u32);
                ecx = ecx.wrapping_add(eax).wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4e30, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x3860u32), (0x1u32) as u8);
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esi.wrapping_add(0x2258u32));
                edi = esi.wrapping_add(0x2258u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x6cu32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = ecx.wrapping_add(edx).wrapping_add(0xffffffacu32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                edx = m.r32(edi);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(0x6cbd5cu32);
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(0x6cbd5cu32);
                ecx = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(edi);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = ecx.wrapping_add(eax).wrapping_add(0xffffffacu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x64u32)); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 15; continue;
            }
            15 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 21; continue; }
                pc = 16; continue;
            }
            16 => {
                edx = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2bcu32));
                ecx = esi.wrapping_add(0x2bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d4u32));
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xecu32));
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = m.r32(esi.wrapping_add(0x3a4u32));
                edi = esi.wrapping_add(0x3a4u32);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x48fc00, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(ebx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x65cu32));
                let (r, f) = lift::add(esi, 0x65cu32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            17 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 42; continue; }
                pc = 18; continue;
            }
            18 => {
                edx = esi.wrapping_add(0x5260u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x5348u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ebx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x574u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x84u32), 0x14u32);
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(ebx.wrapping_add(0xffffb160u32));
                ecx = ebx.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = ebx.wrapping_add(0xffffb160u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x84u32));
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebx, 0xe8u32, 32); fl = f;
                ebx = r;
                let (r, f) = lift::sub(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x84u32), eax);
                if !fl.zf { pc = 19; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = esi.wrapping_add(0x2bcu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0xecu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x3a4u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x65cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x82cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x914u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x744u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x3724u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x1d04u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x1decu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x1ed8u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x2068u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x1c1cu32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esi.wrapping_add(0x2150u32);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x2258u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b4eb0, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi.wrapping_add(0x3938u32);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                ecx = esi.wrapping_add(0x3cecu32);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                ecx = esi.wrapping_add(0x5428u32);
                { let a = []; eax = h.call_at(esp, m, 0x461480, ecx, &a); }
                m.w8(esi.wrapping_add(0x3860u32), (0x0u32) as u8);
                pc = 42; continue;
            }
            21 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc4u32)), 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 22; continue;
            }
            22 => {
                edx = m.r32(esi.wrapping_add(0x2bcu32));
                ecx = esi.wrapping_add(0x2bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1d4u32));
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xecu32));
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                edi = esi.wrapping_add(0x3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x65cu32));
                edi = esi.wrapping_add(0x65cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 26; continue;
            }
            25 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 26; continue;
            }
            26 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x574u32));
                let (r, f) = lift::add(esi, 0x574u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            27 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 38; continue; }
                pc = 28; continue;
            }
            28 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edx = m.r32(esi.wrapping_add(0x2bcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                ecx = esi.wrapping_add(0x2bcu32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x1d4u32));
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0xecu32));
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32)))) & 0xff);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                let r = (edx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x134u32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(0x48cu32));
                let r = (edx & 0xff) & 0x14u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                ecx = esi.wrapping_add(0x48cu32);
                fl = lift::sub((edx & 0xff), 0x14u32, 8).1;
                edx = (edx & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3a4u32));
                ecx = esi.wrapping_add(0x3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x65cu32));
                edi = esi.wrapping_add(0x65cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x574u32));
                let (r, f) = lift::add(esi, 0x574u32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            31 => {
                eax = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::logic((eax & 0xffff) & (eax & 0xffff), 16);
                if fl.zf { pc = 34; continue; }
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub((eax & 0xffff), 0x1u32, 16).1;
                if fl.zf { pc = 34; continue; }
                pc = 33; continue;
            }
            33 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 35; continue;
            }
            34 => {
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(esi.wrapping_add(0x3a4u32));
                ebx = esi.wrapping_add(0x3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = ebx;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebx;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x65cu32));
                ecx = esi.wrapping_add(0x65cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                let (r, f) = lift::add(esi, 0x574u32, 32); fl = f;
                esi = r;
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                ecx = esi;
                if !fl.cf { pc = 37; continue; }
                pc = 36; continue;
            }
            36 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            37 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            38 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if !fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                edx = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2bcu32));
                ecx = esi.wrapping_add(0x2bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d4u32));
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xecu32));
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x3a4u32));
                edi = esi.wrapping_add(0x3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                edi = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edi).wrapping_add(0x134u32)))) & 0xff);
                let r = (eax & 0xff) & 0x14u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                fl = lift::sub((eax & 0xff), 0x14u32, 8).1;
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x65cu32));
                let (r, f) = lift::add(esi, 0x65cu32, 32); fl = f;
                esi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            40 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                edx = m.r32(esi.wrapping_add(0x574u32));
                ecx = esi.wrapping_add(0x574u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x2bcu32));
                ecx = esi.wrapping_add(0x2bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d4u32));
                ecx = esi.wrapping_add(0x1d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xecu32));
                ecx = esi.wrapping_add(0xecu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x3a4u32));
                edi = esi.wrapping_add(0x3a4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x48cu32));
                ecx = esi.wrapping_add(0x48cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(0x65cu32));
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edi = esi.wrapping_add(0x65cu32);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 42; continue;
            }
            42 => {
                ecx = m.r32(esp.wrapping_add(0x74u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x70);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048AC30` ship_side_menu_fit_table_rows
pub fn f_0048ac30(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0xffffffffu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6603f3u32);
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
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ebx = m.r32(esi.wrapping_add(0x38c0u32));
    edi = esi.wrapping_add(0x38b8u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    ecx = m.r32(esi.wrapping_add(0x30u32));
    let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
    ecx = r;
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    eax = ecx;
    ecx = edi;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    ebx = m.r32(esi.wrapping_add(0x3884u32));
    edi = esi.wrapping_add(0x387cu32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    edx = m.r32(esi.wrapping_add(0x30u32));
    ecx = edi;
    let (r, f) = lift::sub(edx, ebx, 32); fl = f;
    edx = r;
    let (r, f) = lift::sub(edx, eax, 32); fl = f;
    edx = r;
    eax = edx;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    ebx = m.r32(esi.wrapping_add(0x38fcu32));
    edi = esi.wrapping_add(0x38f4u32);
    ecx = edi;
    { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
    ecx = edi;
    ebp = eax;
    { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
    ecx = edi;
    let (r, f) = lift::add(ebp, eax, 32); fl = f;
    ebp = r;
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    ecx = m.r32(esi.wrapping_add(0x30u32));
    let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
    ecx = r;
    let (r, f) = lift::sub(ecx, eax, 32); fl = f;
    ecx = r;
    eax = ecx;
    ecx = edi;
    edx = ((eax as i32) >> 31) as u32;
    { let d = i64::from(ebp as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a900, ecx, &a); }
    esp = esp.wrapping_add(4);
    edx = m.r32(0x6c7cccu32);
    m.w32(esp.wrapping_add(0x10u32), edx);
    eax = m.r32(0x699874u32);
    ecx = m.r32(0x69988cu32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    edx = esp.wrapping_add(0x18u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x6ba394u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w32(esp.wrapping_add(0x2cu32), 0x0u32);
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
    eax = m.r32(esp.wrapping_add(0x20u32));
    esp = esp.wrapping_add(0x10);
    ecx = esi.wrapping_add(0x744u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, eax);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = m.r32(0x699934u32);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    ecx = esi.wrapping_add(0x3724u32);
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    m.w16(esi.wrapping_add(0x57dcu32), (0xffffu32) as u16);
    edx = m.r32(0x6b04f8u32);
    let (r, f) = lift::add(esi, 0x82d0u32, 32); fl = f;
    esi = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4c7780, ecx, &a); }
    esp = esp.wrapping_add(4);
    eax = m.r32(esi);
    esp = esp.wrapping_sub(4);
    m.w32(esp, 0x0u32);
    ecx = esi;
    { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
    esp = esp.wrapping_add(4);
    ecx = esp.wrapping_add(0x10u32);
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
    eax
}

/// `0X0048ADA0` ship_side_menu_prerender
pub fn f_0048ada0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esi = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                edx = m.r32(0x66d504u32);
                edi = m.r32(esi.wrapping_add(0x14u32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(esi.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x94u32));
                m.w8(esp.wrapping_add(0x17u32), ((ecx & 0xff)) as u8);
                esp = esp.wrapping_add(0x4);
                ecx = esi;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esi.wrapping_add(0x94u32), edx);
                m.w8(esi.wrapping_add(0xa4u32), (0x0u32) as u8);
                let (r, f) = lift::sub(0, edi, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                let (r, f) = lift::sub(0, ebx, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                edx = (edx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(ecx)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 3; continue;
            }
            2 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 3; continue;
            }
            3 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(0x6cbd94u32);
                edx = esp;
                ecx = esi.wrapping_add(0x3938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(edx, edi);
                m.w32(edx.wrapping_add(0x4u32), ebp);
                m.w32(edx.wrapping_add(0x8u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(0x6cbd94u32);
                ecx = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(ecx, edi);
                m.w32(ecx.wrapping_add(0x4u32), ebp);
                m.w32(ecx.wrapping_add(0x8u32), ebx);
                ecx = esi.wrapping_add(0x3cecu32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi.wrapping_add(0x5428u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x461fe0, ecx, &a); }
                esp = esp.wrapping_add(16);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x744u32));
                eax = esp;
                ecx = esi.wrapping_add(0x744u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x82cu32));
                eax = esp;
                ecx = esi.wrapping_add(0x82cu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x914u32));
                eax = esp;
                ecx = esi.wrapping_add(0x914u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1decu32));
                eax = esp;
                ecx = esi.wrapping_add(0x1decu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x1d04u32));
                ecx = esi.wrapping_add(0x1d04u32);
                esp = esp.wrapping_sub(0xc);
                eax = esp;
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1ed8u32));
                eax = esp;
                ecx = esi.wrapping_add(0x1ed8u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2068u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2068u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x1c1cu32));
                eax = esp;
                ecx = esi.wrapping_add(0x1c1cu32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2150u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2150u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(0xc);
                edx = m.r32(esi.wrapping_add(0x2258u32));
                eax = esp;
                ecx = esi.wrapping_add(0x2258u32);
                m.w32(eax, edi);
                m.w32(eax.wrapping_add(0x4u32), ebp);
                m.w32(eax.wrapping_add(0x8u32), ebx);
                eax = m.r32(0x6cbd94u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x9cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x18u32));
                ecx = m.r32(esp.wrapping_add(0x14u32));
                edx = (edx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esi.wrapping_add(0x94u32), ecx);
                m.w8(esi.wrapping_add(0xa4u32), ((edx & 0xff)) as u8);
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
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048B060` ship_side_menu_draw
pub fn f_0048b060(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0x28u32));
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ebp = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esi);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 33; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb7c0, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 33; continue; }
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(0xc);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                edx = esp;
                ecx = esi;
                m.w32(edx, edi);
                edi = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(edx.wrapping_add(0x4u32), ebx);
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x440b20, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0x94u32));
                eax = m.r32(0x66d504u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 33; continue; }
                pc = 3; continue;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = m.r32(esi.wrapping_add(0xb4u32));
                esp = esp.wrapping_add(0x8);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 7; continue; }
                pc = 4; continue;
            }
            4 => {
                eax = m.r32(esi.wrapping_add(0x4164u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x3cd4u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 6; continue;
            }
            6 => {
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 29; continue;
            }
            7 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48dd70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            9 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if !fl.zf { pc = 18; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = esp.wrapping_add(0x10u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4904b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 29; continue;
            }
            12 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xd0u32)), 32).1;
                if !fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f510, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            14 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xe4u32)), 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                eax = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 17; continue; }
                pc = 16; continue;
            }
            16 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = esp.wrapping_add(0x10u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 17; continue;
            }
            17 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f2c0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            18 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc4u32)), 32).1;
                if !fl.zf { pc = 24; continue; }
                pc = 19; continue;
            }
            19 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 23; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = m.r32(0x6dd7a4u32);
                ebx = esi.wrapping_add(0x5834u32);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edi = ebp;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4953c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(ebx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495450, ecx, &a); }
                edx = edi.wrapping_add(0x160u32);
                m.w32(ebx.wrapping_add(0x4u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4954c0, ecx, &a); }
                m.w32(ebx.wrapping_add(0x10u32), eax);
                eax = m.r32(edi.wrapping_add(0x10u32));
                m.w32(ebx.wrapping_add(0xcu32), eax);
                ecx = m.r32(edi.wrapping_add(0x11cu32));
                m.w32(ebx.wrapping_add(0x14u32), ecx);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                esp = esp.wrapping_add(0x18);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                ecx = 0xc8u32;
                eax = (eax & !0xff) | ((u32::from(fl.cf)) & 0xff);
                m.w8(ebx.wrapping_add(0x1au32), ((eax & 0xff)) as u8);
                eax = m.r32(edi.wrapping_add(0x11cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = ecx;
                ecx = m.r32(ebx.wrapping_add(0x14u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(ebx.wrapping_add(0x14u32), ecx);
                pc = 22; continue;
            }
            22 => {
                m.w16(ebx.wrapping_add(0x18u32), ((ebp & 0xffff)) as u16);
                pc = 23; continue;
            }
            23 => {
                ecx = esi.wrapping_add(0x5834u32);
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x491240, ecx, &a); }
                esp = esp.wrapping_add(8);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = esp.wrapping_add(0x10u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48d8c0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 30; continue;
            }
            24 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = esp.wrapping_add(0x10u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4904b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xe8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 27; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48d8c0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 30; continue;
            }
            27 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xdcu32)), 32).1;
                if !fl.zf { pc = 30; continue; }
                pc = 28; continue;
            }
            28 => {
                edx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 29; continue;
            }
            29 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48cf50, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 30; continue;
            }
            30 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0xa4u32)))) & 0xff);
                esp = esp.wrapping_add(0x4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(0x6cbd64u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(0x6cbd5cu32);
                edx = m.r32(0x6cbd58u32);
                eax = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x18u32));
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
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9b0, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 33; continue;
            }
            33 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048B3E0` ship_side_menu_update
pub fn f_0048b3e0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                esp = esp.wrapping_sub(0x50);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esi = ecx;
                { let a = []; eax = h.call_at(esp, m, 0x4415d0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(0x66d504u32);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 279; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x3720u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 50; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x914u32));
                ebp = esi.wrapping_add(0x914u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x82cu32);
                m.w8(esp.wrapping_add(0x11u32), ((eax & 0xff)) as u8);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edx = m.r32(esi.wrapping_add(0x744u32));
                edi = esi.wrapping_add(0x744u32);
                ecx = edi;
                m.w8(esp.wrapping_add(0x10u32), ((eax & 0xff)) as u8);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = esi.wrapping_add(0x3724u32);
                m.w8(esp.wrapping_add(0x12u32), ((eax & 0xff)) as u8);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x11u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x13u32), ((eax & 0xff)) as u8);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 17; continue; }
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(0x6d9584u32);
                eax = m.r32(ecx.wrapping_add(0xf0u32));
                ecx = m.r32(0x66fe88u32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(eax, m.r32(0x66fe94u32), 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 8; continue;
            }
            8 => {
                fl = lift::sub(eax, m.r32(0x66fe8cu32), 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::sub(eax, m.r32(0x66fe98u32), 32).1;
                if fl.zf { pc = 14; continue; }
                pc = 10; continue;
            }
            10 => {
                fl = lift::sub(eax, m.r32(0x66fe90u32), 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 17; continue;
            }
            12 => {
                fl = lift::sub(eax, m.r32(0x66fe9cu32), 32).1;
                if !fl.zf { pc = 17; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                pc = 16; continue;
            }
            14 => {
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 17; continue;
            }
            15 => {
                eax = m.r32(ebp);
                ecx = ebp;
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(0x6d9584u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 44; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xe0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 44; continue; }
                pc = 19; continue;
            }
            19 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x134u32))), 0x4u32, 16).1;
                if !fl.cf { pc = 44; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(0x6d9584u32);
                eax = m.r32(0x66fe90u32);
                edx = m.r32(ecx.wrapping_add(0xf0u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x11u32)))) & 0xff);
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 26; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            25 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x13u32)))) & 0xff);
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x11u32)))) & 0xff);
                pc = 26; continue;
            }
            26 => {
                fl = lift::sub(edx, m.r32(0x66fe88u32), 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(edx, m.r32(0x66fe94u32), 32).1;
                if !fl.zf { pc = 32; continue; }
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 31; continue; }
                pc = 29; continue;
            }
            29 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 31; continue; }
                pc = 30; continue;
            }
            30 => {
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if fl.zf { pc = 33; continue; }
                pc = 31; continue;
            }
            31 => {
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            32 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x12u32)))) & 0xff);
                pc = 33; continue;
            }
            33 => {
                fl = lift::sub(edx, m.r32(0x66fe8cu32), 32).1;
                if fl.zf { pc = 35; continue; }
                pc = 34; continue;
            }
            34 => {
                fl = lift::sub(edx, m.r32(0x66fe98u32), 32).1;
                if !fl.zf { pc = 38; continue; }
                pc = 35; continue;
            }
            35 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 43; continue; }
                pc = 36; continue;
            }
            36 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 43; continue; }
                pc = 37; continue;
            }
            37 => {
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                if !fl.zf { pc = 43; continue; }
                pc = 38; continue;
            }
            38 => {
                fl = lift::sub(edx, m.r32(0x66fe9cu32), 32).1;
                if !fl.zf { pc = 50; continue; }
                pc = 39; continue;
            }
            39 => {
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                if !fl.zf { pc = 42; continue; }
                pc = 40; continue;
            }
            40 => {
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 42; continue; }
                pc = 41; continue;
            }
            41 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x10u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 50; continue; }
                pc = 42; continue;
            }
            42 => {
                eax = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            43 => {
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 50; continue;
            }
            44 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xe0u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 46; continue; }
                pc = 45; continue;
            }
            45 => {
                edx = m.r32(esi.wrapping_add(0xd0u32));
                m.w32(esi.wrapping_add(0xccu32), edx);
                pc = 46; continue;
            }
            46 => {
                ecx = m.r32(0x6d9584u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 49; continue; }
                pc = 47; continue;
            }
            47 => {
                ecx = m.r32(0x6d9584u32);
                eax = m.r32(0x66fe90u32);
                edx = m.r32(ecx.wrapping_add(0xf0u32));
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 49; continue; }
                pc = 48; continue;
            }
            48 => {
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x118u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 49; continue;
            }
            49 => {
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                m.w32(esi.wrapping_add(0x3720u32), 0xffffffffu32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x124u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 50; continue;
            }
            50 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc4u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 65; continue; }
                pc = 51; continue;
            }
            51 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                ebx = m.r32(0x6dd7a4u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = ecx;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x13u32, 16).1;
                if fl.zf { pc = 64; continue; }
                pc = 52; continue;
            }
            52 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 64; continue; }
                pc = 53; continue;
            }
            53 => {
                fl = lift::sub((eax & 0xffff), 0xcu32, 16).1;
                if fl.zf { pc = 64; continue; }
                pc = 54; continue;
            }
            54 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x494740, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), (ebx & 0xff), 8).1;
                if !fl.zf { pc = 56; continue; }
                pc = 55; continue;
            }
            55 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                edx = (edx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esi.wrapping_add(0x5832u32), ((edx & 0xff)) as u8);
                pc = 56; continue;
            }
            56 => {
                ecx = esi.wrapping_add(0x65cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 59; continue; }
                pc = 57; continue;
            }
            57 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 63; continue; }
                pc = 58; continue;
            }
            58 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x28u32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w8(esp.wrapping_add(0x30u32), (0x8u32) as u8);
                esp = esp.wrapping_add(0x4);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = esp.wrapping_add(0x24u32);
                m.w16(eax, ((edx & 0xffff)) as u16);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 59; continue;
            }
            59 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 63; continue; }
                pc = 60; continue;
            }
            60 => {
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 62; continue; }
                pc = 61; continue;
            }
            61 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                edx = m.r32(esi.wrapping_add(0x3864u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x24u32), edx);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x34u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 62; continue;
            }
            62 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                pc = 63; continue;
            }
            63 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x5850u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 157; continue;
            }
            64 => {
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            65 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 107; continue; }
                pc = 66; continue;
            }
            66 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ebp = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x14u32), ebp);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), 0x13u32, 16).1;
                if fl.zf { pc = 106; continue; }
                pc = 67; continue;
            }
            67 => {
                fl = lift::sub((eax & 0xffff), 0xdu32, 16).1;
                if fl.zf { pc = 106; continue; }
                pc = 68; continue;
            }
            68 => {
                fl = lift::sub((eax & 0xffff), 0xcu32, 16).1;
                if fl.zf { pc = 106; continue; }
                pc = 69; continue;
            }
            69 => {
                ecx = esi.wrapping_add(0x65cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 71; continue; }
                pc = 70; continue;
            }
            70 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x28u32), 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x2cu32), eax);
                m.w8(esp.wrapping_add(0x30u32), (0x8u32) as u8);
                esp = esp.wrapping_add(0x4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                m.w16(eax, ((ecx & 0xffff)) as u16);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x24u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            71 => {
                ecx = esi.wrapping_add(0x574u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 74; continue; }
                pc = 72; continue;
            }
            72 => {
                ecx = m.r32(esi.wrapping_add(0xe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esi.wrapping_add(0xccu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 73; continue;
            }
            73 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 96; continue;
            }
            74 => {
                ecx = esi.wrapping_add(0x3a4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 77; continue; }
                pc = 75; continue;
            }
            75 => {
                ecx = m.r32(0x6cba98u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 96; continue; }
                pc = 76; continue;
            }
            76 => {
                ecx = m.r32(0x6cba98u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 96; continue;
            }
            77 => {
                ecx = esi.wrapping_add(0x48cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 79; continue; }
                pc = 78; continue;
            }
            78 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494dc0, ecx, &a); }
                pc = 96; continue;
            }
            79 => {
                ecx = esi.wrapping_add(0x2bcu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 83; continue; }
                pc = 80; continue;
            }
            80 => {
                ecx = m.r32(esi.wrapping_add(0xd4u32));
                m.w32(esi.wrapping_add(0xccu32), ecx);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0x2060u32));
                edi = esi.wrapping_add(0x1ed8u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x40u32)))) & 0xffff);
                fl = lift::sub(eax, ecx, 32).1;
                m.w32(edi.wrapping_add(0x180u32), eax);
                if (fl.sf == fl.of) { pc = 82; continue; }
                pc = 81; continue;
            }
            81 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 82; continue;
            }
            82 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x45c930, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 96; continue;
            }
            83 => {
                ecx = esi.wrapping_add(0x1d4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 87; continue; }
                pc = 84; continue;
            }
            84 => {
                edx = m.r32(esi.wrapping_add(0xb4u32));
                eax = m.r32(esi.wrapping_add(0xbcu32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 96; continue; }
                pc = 85; continue;
            }
            85 => {
                eax = m.r32(esi.wrapping_add(0xd8u32));
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 96; continue; }
                pc = 86; continue;
            }
            86 => {
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 73; continue;
            }
            87 => {
                eax = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 90; continue; }
                pc = 88; continue;
            }
            88 => {
                edi = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe8cu32);
                fl = lift::sub(m.r32(edi.wrapping_add(0xf0u32)), edx, 32).1;
                if fl.zf { pc = 90; continue; }
                pc = 89; continue;
            }
            89 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                ebp = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ecx, 0x7u32, 32, fl); fl = f;
                ecx = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(ebp).wrapping_add(0x39u32)))) & 0xff);
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4a9690, ecx, &a); }
                esp = esp.wrapping_add(12);
                ebp = m.r32(esp.wrapping_add(0x14u32));
                pc = 96; continue;
            }
            90 => {
                edx = m.r32(esi.wrapping_add(0x914u32));
                ecx = esi.wrapping_add(0x914u32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 92; continue; }
                pc = 91; continue;
            }
            91 => {
                edi = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe88u32);
                fl = lift::sub(m.r32(edi.wrapping_add(0xf0u32)), edx, 32).1;
                if !fl.zf { pc = 89; continue; }
                pc = 92; continue;
            }
            92 => {
                edx = m.r32(esi.wrapping_add(0x3724u32));
                ecx = esi.wrapping_add(0x3724u32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 94; continue; }
                pc = 93; continue;
            }
            93 => {
                edi = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe9cu32);
                fl = lift::sub(m.r32(edi.wrapping_add(0xf0u32)), edx, 32).1;
                if !fl.zf { pc = 89; continue; }
                pc = 94; continue;
            }
            94 => {
                ecx = esi.wrapping_add(0xecu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 96; continue; }
                pc = 95; continue;
            }
            95 => {
                edx = m.r32(esi.wrapping_add(0xd0u32));
                ecx = esi;
                m.w32(esi.wrapping_add(0xccu32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                ecx = esi.wrapping_add(0x3ce4u32);
                m.w32(esi.wrapping_add(0x4098u32), 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 96; continue;
            }
            96 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x48fa90, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 99; continue; }
                pc = 97; continue;
            }
            97 => {
                ecx = esi.wrapping_add(0x3ce4u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 99; continue; }
                pc = 98; continue;
            }
            98 => {
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                edi = esi.wrapping_add(0x38b8u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38bcu32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = edi;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x24u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 99; continue;
            }
            99 => {
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xd4u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 101; continue; }
                pc = 100; continue;
            }
            100 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493ed0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                pc = 158; continue;
            }
            101 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xd8u32)), 32).1;
                if !fl.zf { pc = 103; continue; }
                pc = 102; continue;
            }
            102 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4942c0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                pc = 158; continue;
            }
            103 => {
                fl = lift::sub(u32::from(m.r16(ebp.wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 105; continue; }
                pc = 104; continue;
            }
            104 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 105; continue;
            }
            105 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                pc = 158; continue;
            }
            106 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            107 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xb8u32)), 32).1;
                if !fl.zf { pc = 117; continue; }
                pc = 108; continue;
            }
            108 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490b10, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 279; continue; }
                pc = 109; continue;
            }
            109 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490600, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x493d10, ecx, &a); }
                ecx = esi.wrapping_add(0x3930u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 111; continue; }
                pc = 110; continue;
            }
            110 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = esi.wrapping_add(0x387cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(esi.wrapping_add(0x3880u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 111; continue;
            }
            111 => {
                eax = m.r32(esi.wrapping_add(0x4164u32));
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 114; continue; }
                pc = 112; continue;
            }
            112 => {
                edx = m.r32(esi.wrapping_add(0x3cd4u32));
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 114; continue; }
                pc = 113; continue;
            }
            113 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 114; continue;
            }
            114 => {
                ecx = esi.wrapping_add(0x3a4u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 158; continue; }
                pc = 115; continue;
            }
            115 => {
                ecx = m.r32(0x6cba98u32);
                edx = m.r32(ecx);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 158; continue; }
                pc = 116; continue;
            }
            116 => {
                ecx = m.r32(0x6cba98u32);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                pc = 158; continue;
            }
            117 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if !fl.zf { pc = 142; continue; }
                pc = 118; continue;
            }
            118 => {
                ecx = esi.wrapping_add(0xecu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 121; continue; }
                pc = 119; continue;
            }
            119 => {
                eax = m.r32(esi.wrapping_add(0xd0u32));
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 121; continue; }
                pc = 120; continue;
            }
            120 => {
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                edi = esi.wrapping_add(0x38b8u32);
                ecx = edi;
                m.w32(esi.wrapping_add(0xccu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = edi;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38bcu32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = edi;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = 0x6da5f0u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x4098u32), 0xffffffffu32);
                pc = 133; continue;
            }
            121 => {
                ecx = esi.wrapping_add(0x65cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 124; continue; }
                pc = 122; continue;
            }
            122 => {
                eax = m.r32(esi.wrapping_add(0xdcu32));
                ecx = m.r32(esi.wrapping_add(0xccu32));
                fl = lift::sub(ecx, eax, 32).1;
                if fl.zf { pc = 124; continue; }
                pc = 123; continue;
            }
            123 => {
                ecx = esi;
                m.w32(esi.wrapping_add(0xccu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 133; continue;
            }
            124 => {
                ecx = esi.wrapping_add(0x48cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 126; continue; }
                pc = 125; continue;
            }
            125 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494dc0, ecx, &a); }
                pc = 133; continue;
            }
            126 => {
                ecx = esi.wrapping_add(0x574u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 128; continue; }
                pc = 127; continue;
            }
            127 => {
                eax = m.r32(esi.wrapping_add(0xe4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esi.wrapping_add(0xccu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x462280, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_add(0x4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                pc = 133; continue;
            }
            128 => {
                edx = m.r32(esi.wrapping_add(0x82cu32));
                ecx = esi.wrapping_add(0x82cu32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 130; continue; }
                pc = 129; continue;
            }
            129 => {
                ecx = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe98u32);
                fl = lift::sub(m.r32(ecx.wrapping_add(0xf0u32)), edx, 32).1;
                if !fl.zf { pc = 132; continue; }
                pc = 130; continue;
            }
            130 => {
                edx = m.r32(esi.wrapping_add(0x914u32));
                ecx = esi.wrapping_add(0x914u32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 133; continue; }
                pc = 131; continue;
            }
            131 => {
                ecx = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe94u32);
                fl = lift::sub(m.r32(ecx.wrapping_add(0xf0u32)), edx, 32).1;
                if fl.zf { pc = 133; continue; }
                pc = 132; continue;
            }
            132 => {
                edi = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi))) & 0xffff);
                edi = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(edi).wrapping_add(0x39u32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x5228u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4a9690, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 133; continue;
            }
            133 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48fe80, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 279; continue; }
                pc = 134; continue;
            }
            134 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4906e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = (edx & !0xffff) | ((u32::from(m.r16(0x6dfc14u32))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 137; continue; }
                pc = 135; continue;
            }
            135 => {
                ecx = esi.wrapping_add(0x3ce4u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 141; continue; }
                pc = 136; continue;
            }
            136 => {
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                edi = esi.wrapping_add(0x38b8u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(esi.wrapping_add(0x38bcu32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                pc = 158; continue;
            }
            137 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xdcu32)), 32).1;
                if !fl.zf { pc = 141; continue; }
                pc = 138; continue;
            }
            138 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490b10, ecx, &a); }
                ecx = esi.wrapping_add(0x3930u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 140; continue; }
                pc = 139; continue;
            }
            139 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = esi.wrapping_add(0x387cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x3880u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x24u32));
                ecx = 0x6da5f0u32;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x2cu32), eax);
                eax = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 140; continue;
            }
            140 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490600, ecx, &a); }
                pc = 141; continue;
            }
            141 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492bc0, ecx, &a); }
                pc = 158; continue;
            }
            142 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 158; continue; }
                pc = 143; continue;
            }
            143 => {
                ecx = m.r32(esi.wrapping_add(0x5238u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x494740, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebx = (ebx & !0xff) | (((eax & 0xff)) & 0xff);
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::sub((ebx & 0xff), (eax & 0xff), 8).1;
                if !fl.zf { pc = 145; continue; }
                pc = 144; continue;
            }
            144 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                m.w8(esi.wrapping_add(0x5832u32), ((ecx & 0xff)) as u8);
                pc = 145; continue;
            }
            145 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 148; continue; }
                pc = 146; continue;
            }
            146 => {
                edx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 148; continue; }
                pc = 147; continue;
            }
            147 => {
                eax = m.r32(esi.wrapping_add(0xe8u32));
                m.w32(esi.wrapping_add(0xccu32), eax);
                pc = 148; continue;
            }
            148 => {
                ecx = esi.wrapping_add(0x65cu32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 151; continue; }
                pc = 149; continue;
            }
            149 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 151; continue; }
                pc = 150; continue;
            }
            150 => {
                ecx = m.r32(esi.wrapping_add(0xdcu32));
                m.w32(esi.wrapping_add(0xccu32), ecx);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492580, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 151; continue;
            }
            151 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48fe80, ecx, &a); }
                esp = esp.wrapping_add(4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 279; continue; }
                pc = 152; continue;
            }
            152 => {
                edx = m.r32(esi.wrapping_add(0x5238u32));
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                ebx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(edx.wrapping_mul(2))))) & 0xffff);
                ecx = esi;
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebx, 32); fl = f;
                edi = r;
                ebx = (ebx & !0xffff) | ((u32::from(m.r8(edi))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4906e0, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x492150, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494f90, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 156; continue; }
                pc = 153; continue;
            }
            153 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490b10, ecx, &a); }
                ecx = esi.wrapping_add(0x3930u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 155; continue; }
                pc = 154; continue;
            }
            154 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = esi.wrapping_add(0x387cu32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = edi;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), eax);
                eax = m.r32(esi.wrapping_add(0x3880u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x30u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 155; continue;
            }
            155 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x490600, ecx, &a); }
                pc = 158; continue;
            }
            156 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0xeu32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x5860u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 157; continue;
            }
            157 => {
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48db50, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 158; continue;
            }
            158 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 202; continue; }
                pc = 159; continue;
            }
            159 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x4934b0, ecx, &a); }
                ecx = esi.wrapping_add(0x5420u32);
                { let a = []; eax = h.call_at(esp, m, 0x431260, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 161; continue; }
                pc = 160; continue;
            }
            160 => {
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), edx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                edx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 161; continue;
            }
            161 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dd7a4u32);
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                fl = lift::sub(u32::from(m.r16(edx.wrapping_add(eax).wrapping_add(0x3cu32))), 0x0u32, 16).1;
                if fl.zf { pc = 163; continue; }
                pc = 162; continue;
            }
            162 => {
                eax = m.r32(esi.wrapping_add(0x14u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(0x6cbd5cu32);
                m.w32(esp.wrapping_add(0x2cu32), edx);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x18u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x34u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 163; continue;
            }
            163 => {
                ecx = esi.wrapping_add(0x9fcu32);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 164; continue;
            }
            164 => {
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 181; continue; }
                pc = 165; continue;
            }
            165 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 167; continue; }
                pc = 166; continue;
            }
            166 => {
                edi = m.r32(0x6dd7a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 167; continue;
            }
            167 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                edi = m.r32(0x6dd72cu32);
                eax = edx;
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                pc = 168; continue;
            }
            168 => {
                ecx = eax;
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                ebp = ecx.wrapping_add(ecx.wrapping_mul(4));
                ecx = ecx.wrapping_add(ebp.wrapping_mul(2));
                ecx = ecx.wrapping_add(ecx.wrapping_mul(4));
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x3u32))) & 0x4u32, 8);
                ecx = edi.wrapping_add(ecx.wrapping_mul(4));
                if !fl.zf { pc = 171; continue; }
                pc = 169; continue;
            }
            169 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                fl = lift::sub((edx & 0xffff), (eax & 0xffff), 16).1;
                if !fl.zf { pc = 168; continue; }
                pc = 170; continue;
            }
            170 => {
                pc = 172; continue;
            }
            171 => {
                edx = eax;
                pc = 172; continue;
            }
            172 => {
                ecx = m.r32(esp.wrapping_add(0x14u32));
                pc = 173; continue;
            }
            173 => {
                fl = lift::sub(ebx, ecx, 32).1;
                if fl.zf { pc = 175; continue; }
                pc = 174; continue;
            }
            174 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                ebp = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ebp.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(eax.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((edx & 0xffff), (eax & 0xffff), 16).1;
                if !fl.zf { pc = 173; continue; }
                pc = 175; continue;
            }
            175 => {
                ecx = m.r32(esi.wrapping_add(0xb4u32));
                edx = m.r32(esi.wrapping_add(0xc0u32));
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 177; continue; }
                pc = 176; continue;
            }
            176 => {
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 178; continue;
            }
            177 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 178; continue;
            }
            178 => {
                ecx = m.r32(0x6cba74u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x405a20, ecx, &a); }
                esp = esp.wrapping_add(12);
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 179; continue;
            }
            179 => {
                edx = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 179; continue; }
                pc = 180; continue;
            }
            180 => {
                eax = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x525du32), (0x1u32) as u8);
                pc = 181; continue;
            }
            181 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(ecx, 0xe8u32, 32); fl = f;
                ecx = r;
                fl = lift::sub(eax, 0x14u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                if (fl.sf != fl.of) { pc = 164; continue; }
                pc = 182; continue;
            }
            182 => {
                eax = esi.wrapping_add(0x589cu32);
                m.w32(esp.wrapping_add(0x14u32), 0x0u32);
                m.w32(esp.wrapping_add(0x18u32), eax);
                ebp = 0x6du32;
                pc = 183; continue;
            }
            183 => {
                ecx = m.r32(esp.wrapping_add(0x18u32));
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 201; continue; }
                pc = 184; continue;
            }
            184 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 186; continue; }
                pc = 185; continue;
            }
            185 => {
                edi = m.r32(0x6dd7a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 186; continue;
            }
            186 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                ecx = m.r32(0x6dd72cu32);
                ebx = edi;
                pc = 187; continue;
            }
            187 => {
                eax = edi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(edx.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                fl = lift::logic((edx & 0xff) & 0x4u32, 8);
                if !fl.zf { pc = 190; continue; }
                pc = 188; continue;
            }
            188 => {
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (edi & 0xffff), 16).1;
                if !fl.zf { pc = 187; continue; }
                pc = 189; continue;
            }
            189 => {
                pc = 191; continue;
            }
            190 => {
                ebx = edi;
                pc = 191; continue;
            }
            191 => {
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                pc = 192; continue;
            }
            192 => {
                fl = lift::sub(edx, m.r32(esp.wrapping_add(0x14u32)), 32).1;
                if fl.zf { pc = 194; continue; }
                pc = 193; continue;
            }
            193 => {
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                eax = edi.wrapping_add(edi.wrapping_mul(4));
                eax = edi.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edi = (edi & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (edi & 0xffff), 16).1;
                if !fl.zf { pc = 192; continue; }
                pc = 194; continue;
            }
            194 => {
                edx = m.r32(esp.wrapping_add(0x14u32));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(esi).wrapping_add(0x6abcu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 196; continue; }
                pc = 195; continue;
            }
            195 => {
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                eax = edi.wrapping_add(edi.wrapping_mul(4));
                eax = edi.wrapping_add(eax.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                ecx = esp.wrapping_add(0x24u32);
                let r = (eax & 0xff) & 0xf6u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w8(esp.wrapping_add(0x1cu32), ((eax & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = esp.wrapping_add(0x24u32);
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ebp);
                m.w32(esp.wrapping_add(0x28u32), edi);
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 200; continue;
            }
            196 => {
                fl = lift::sub((eax & 0xff), 0x1u32, 8).1;
                if !fl.zf { pc = 198; continue; }
                pc = 197; continue;
            }
            197 => {
                ecx = esp.wrapping_add(0x38u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                edx = m.r32(0x6dd72cu32);
                m.w32(esp.wrapping_add(0x38u32), ebp);
                eax = edi.wrapping_add(edi.wrapping_mul(4));
                eax = edi.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                edx = esp.wrapping_add(0x38u32);
                let r = (eax & 0xff) & 0xf7u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                m.w32(esp.wrapping_add(0x3cu32), edi);
                let r = (eax & 0xff) | 0x1u32; fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w8(esp.wrapping_add(0x24u32), ((eax & 0xff)) as u8);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let r = ecx & 0xffu32; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x44u32), ecx);
                pc = 200; continue;
            }
            198 => {
                fl = lift::sub((eax & 0xff), 0x2u32, 8).1;
                if !fl.zf { pc = 201; continue; }
                pc = 199; continue;
            }
            199 => {
                ecx = esp.wrapping_add(0x4cu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x4cu32), ebp);
                m.w32(esp.wrapping_add(0x50u32), edi);
                eax = edi.wrapping_add(edi.wrapping_mul(4));
                eax = edi.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd72cu32);
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x3u32)))) & 0xff);
                ecx = esp.wrapping_add(0x4cu32);
                let r = edx | 0x8u32; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                m.w32(esp.wrapping_add(0x58u32), edx);
                pc = 200; continue;
            }
            200 => {
                ecx = 0x6df2f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 201; continue;
            }
            201 => {
                eax = m.r32(esp.wrapping_add(0x14u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0xe8u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, 0x14u32, 32).1;
                m.w32(esp.wrapping_add(0x14u32), eax);
                m.w32(esp.wrapping_add(0x18u32), edx);
                if (fl.sf != fl.of) { pc = 183; continue; }
                pc = 202; continue;
            }
            202 => {
                edx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 212; continue; }
                pc = 203; continue;
            }
            203 => {
                ecx = esi.wrapping_add(0x82d0u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 212; continue; }
                pc = 204; continue;
            }
            204 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esi.wrapping_add(0x83b8u32), (0xffu32) as u8);
                edx = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 206; continue; }
                pc = 205; continue;
            }
            205 => {
                edi = m.r32(0x6dd7a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 206; continue;
            }
            206 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                edi = esi.wrapping_add(0x83c0u32);
                fl = lift::sub((eax & 0xffff), 0xffffu32, 16).1;
                if fl.zf { pc = 208; continue; }
                pc = 207; continue;
            }
            207 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3a0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 209; continue;
            }
            208 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ce6d4u32);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f390, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 209; continue;
            }
            209 => {
                edx = m.r32(esi.wrapping_add(0x4170u32));
                eax = esi.wrapping_add(0x83bcu32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w16(esi.wrapping_add(0x4214u32), (0x4u32) as u16);
                m.w32(eax, ecx);
                m.w32(esi.wrapping_add(0x421cu32), ecx);
                m.w16(esi.wrapping_add(0x4216u32), ((ecx & 0xffff)) as u16);
                m.w32(esi.wrapping_add(0x4234u32), ecx);
                ecx = esi.wrapping_add(0x4170u32);
                m.w32(esi.wrapping_add(0x423cu32), edi);
                m.w32(esi.wrapping_add(0x4238u32), eax);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 210; continue;
            }
            210 => {
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 210; continue; }
                pc = 211; continue;
            }
            211 => {
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 212; continue;
            }
            212 => {
                edx = m.r32(esi.wrapping_add(0x744u32));
                ecx = esi.wrapping_add(0x744u32);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 215; continue; }
                pc = 213; continue;
            }
            213 => {
                edi = m.r32(0x6d9584u32);
                edx = m.r32(0x66fe90u32);
                fl = lift::sub(m.r32(edi.wrapping_add(0xf0u32)), edx, 32).1;
                if fl.zf { pc = 215; continue; }
                pc = 214; continue;
            }
            214 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = edi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4a9690, ecx, &a); }
                esp = esp.wrapping_add(12);
                pc = 215; continue;
            }
            215 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x489b80, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x9fcu32));
                ecx = esi.wrapping_add(0x9fcu32);
                { let t = m.r32(edx.wrapping_add(0xc8u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                edi = m.r32(esi.wrapping_add(0x57d8u32));
                let (r, f) = lift::sub(0, (eax & 0xff), 8); fl = f;
                eax = (eax & !0xff) | ((r) & 0xff);
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::logic(edi & edi, 32);
                ecx = (ecx & !0xff) | ((u32::from(fl.zf)) & 0xff);
                fl = lift::logic(ecx & eax, 32);
                if fl.zf { pc = 217; continue; }
                pc = 216; continue;
            }
            216 => {
                edx = m.r32(esi.wrapping_add(0x5348u32));
                ecx = esi.wrapping_add(0x5348u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 217; continue;
            }
            217 => {
                eax = m.r32(esi.wrapping_add(0x5250u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 223; continue; }
                pc = 218; continue;
            }
            218 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 219; continue;
            }
            219 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 219; continue; }
                pc = 220; continue;
            }
            220 => {
                eax = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5250u32));
                fl = lift::logic(eax & eax, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 222; continue; }
                pc = 221; continue;
            }
            221 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492ca0, ecx, &a); }
                pc = 222; continue;
            }
            222 => {
                m.w32(esi.wrapping_add(0x5250u32), 0x0u32);
                pc = 223; continue;
            }
            223 => {
                eax = m.r32(esi.wrapping_add(0x83bcu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 234; continue; }
                pc = 224; continue;
            }
            224 => {
                fl = lift::sub(eax, 0x1u32, 32).1;
                if !fl.zf { pc = 226; continue; }
                pc = 225; continue;
            }
            225 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = esi.wrapping_add(0x83c0u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(0x6dfc24u32, ecx);
                ecx = 0x6dd728u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4d5ee0, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(0x6dfc20u32, eax);
                ebx = m.r32(esi.wrapping_add(0x3884u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 231; continue;
            }
            226 => {
                fl = lift::sub(eax, 0x2u32, 32).1;
                if !fl.zf { pc = 231; continue; }
                pc = 227; continue;
            }
            227 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd894u32)), 16).1;
                if !fl.cf { pc = 229; continue; }
                pc = 228; continue;
            }
            228 => {
                edi = m.r32(0x6dd7a8u32);
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 229; continue;
            }
            229 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                fl = lift::sub(ecx, 0xffffu32, 32).1;
                if fl.zf { pc = 231; continue; }
                pc = 230; continue;
            }
            230 => {
                edx = esi.wrapping_add(0x83c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6dd728u32;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4d5c50, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 231; continue;
            }
            231 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 232; continue;
            }
            232 => {
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 232; continue; }
                pc = 233; continue;
            }
            233 => {
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x83bcu32), 0x0u32);
                pc = 234; continue;
            }
            234 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x525cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 239; continue; }
                pc = 235; continue;
            }
            235 => {
                eax = m.r32(esi.wrapping_add(0x5258u32));
                ecx = eax;
                let (r, f) = lift::shl(ecx, 0x5u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                ecx = esi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x2340u32);
                { let a = []; eax = h.call_at(esp, m, 0x4b1bf0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 259; continue; }
                pc = 236; continue;
            }
            236 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x492ee0, ecx, &a); }
                m.w8(esi.wrapping_add(0x525cu32), (0x0u32) as u8);
                edi = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 237; continue;
            }
            237 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                let (r, f) = lift::add(edi, 0xf8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 237; continue; }
                pc = 238; continue;
            }
            238 => {
                pc = 259; continue;
            }
            239 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ebx = esi.wrapping_add(0x2340u32);
                m.w32(esp.wrapping_add(0x18u32), edi);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                pc = 240; continue;
            }
            240 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4b1bf0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 258; continue; }
                pc = 241; continue;
            }
            241 => {
                ecx = ebx;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 256; continue; }
                pc = 242; continue;
            }
            242 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 243; continue;
            }
            243 => {
                edx = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 243; continue; }
                pc = 244; continue;
            }
            244 => {
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xeu32)))) & 0xff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                ebp = ecx;
                if !fl.cf { pc = 249; continue; }
                pc = 245; continue;
            }
            245 => {
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(0x6dd894u32))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0xau32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if !fl.cf { pc = 249; continue; }
                pc = 246; continue;
            }
            246 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0xeu32)))) & 0xff);
                fl = lift::sub(ecx, ebp, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 248; continue; }
                pc = 247; continue;
            }
            247 => {
                ebp = ecx;
                pc = 248; continue;
            }
            248 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (edi & 0xffff), 16).1;
                if fl.cf { pc = 246; continue; }
                pc = 249; continue;
            }
            249 => {
                ebx = 0x1u32;
                m.w16(esi.wrapping_add(0x4214u32), ((ebx & 0xffff)) as u16);
                eax = m.r32(0x6de4b0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                m.w32(esi.wrapping_add(0x421cu32), eax);
                edx = m.r32(0x6de4b0u32);
                esp = esp.wrapping_add(0x4);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::logic(edx & edx, 32);
                if (fl.cf || fl.zf) { pc = 255; continue; }
                pc = 250; continue;
            }
            250 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                pc = 251; continue;
            }
            251 => {
                fl = lift::sub(ebp, 0x2u32, 32).1;
                if (fl.sf != fl.of) { pc = 253; continue; }
                pc = 252; continue;
            }
            252 => {
                edx = m.r32(0x6de508u32);
                edx = m.r32(edx.wrapping_add(edi).wrapping_add(0x2c8u32));
                let r = edx & ebx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::sub((edx & 0xff), 0x1u32, 8).1;
                if fl.zf { pc = 254; continue; }
                pc = 253; continue;
            }
            253 => {
                edx = m.r32(esi.wrapping_add(0x421cu32));
                let (r, f) = lift::add(ecx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ecx = r;
                m.w8(ecx.wrapping_add(edx).wrapping_add(0xffffffffu32), ((eax & 0xff)) as u8);
                pc = 254; continue;
            }
            254 => {
                edx = m.r32(0x6de4b0u32);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edi, 0x9f8u32, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 251; continue; }
                pc = 255; continue;
            }
            255 => {
                edx = m.r32(esi.wrapping_add(0x4170u32));
                m.w16(esi.wrapping_add(0x4216u32), ((ecx & 0xffff)) as u16);
                eax = esi.wrapping_add(0x5254u32);
                ecx = esi.wrapping_add(0x4170u32);
                m.w32(esi.wrapping_add(0x423cu32), eax);
                eax = esi.wrapping_add(0x5250u32);
                m.w32(esi.wrapping_add(0x4234u32), ebx);
                m.w32(esi.wrapping_add(0x4238u32), eax);
                { let t = m.r32(edx.wrapping_add(0x120u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                eax = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esi.wrapping_add(0x5258u32), eax);
                edi = eax;
                pc = 256; continue;
            }
            256 => {
                let (r, f) = lift::add(edi, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edi = r;
                let (r, f) = lift::add(ebx, 0xf8u32, 32); fl = f;
                ebx = r;
                fl = lift::sub(edi, 0x14u32, 32).1;
                m.w32(esp.wrapping_add(0x18u32), edi);
                m.w32(esp.wrapping_add(0x1cu32), ebx);
                if (fl.sf != fl.of) { pc = 240; continue; }
                pc = 257; continue;
            }
            257 => {
                pc = 259; continue;
            }
            258 => {
                m.w8(esi.wrapping_add(0x525cu32), (0x1u32) as u8);
                m.w32(esi.wrapping_add(0x5258u32), edi);
                pc = 259; continue;
            }
            259 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x525du32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 264; continue; }
                pc = 260; continue;
            }
            260 => {
                ecx = m.r32(0x6cba74u32);
                eax = (eax & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x5448u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 264; continue; }
                pc = 261; continue;
            }
            261 => {
                edi = esi.wrapping_add(0x589cu32);
                ebp = esi.wrapping_add(0x2340u32);
                ebx = 0x14u32;
                pc = 262; continue;
            }
            262 => {
                edx = m.r32(edi.wrapping_add(0xffffb160u32));
                ecx = edi.wrapping_add(0xffffb160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(ebp, 0xf8u32, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(edi, 0xe8u32, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 262; continue; }
                pc = 263; continue;
            }
            263 => {
                eax = m.r32(esi.wrapping_add(0x5260u32));
                ecx = esi.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(eax.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esi.wrapping_add(0x82d0u32));
                ecx = esi.wrapping_add(0x82d0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let t = m.r32(edx.wrapping_add(0x34u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w8(esi.wrapping_add(0x525du32), (0x0u32) as u8);
                pc = 264; continue;
            }
            264 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = m.r32(0x6dd7a4u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                ecx = esi.wrapping_add(0x5260u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                { let a = []; eax = h.call_at(esp, m, 0x4c78b0, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 276; continue; }
                pc = 265; continue;
            }
            265 => {
                ebp = m.r32(esi.wrapping_add(0x3884u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                m.w32(esp.wrapping_add(0x24u32), eax);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x24u32);
                ecx = 0x6da5f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x30u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 267; continue; }
                pc = 266; continue;
            }
            266 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd7a4u32);
                edi = (edi & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                pc = 267; continue;
            }
            267 => {
                fl = lift::logic(u32::from(m.r8(edi.wrapping_add(0x136u32))) & 0x1u32, 8);
                if fl.zf { pc = 269; continue; }
                pc = 268; continue;
            }
            268 => {
                ecx = esp.wrapping_add(0x4cu32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                edx = m.r32(esi.wrapping_add(0x5348u32));
                m.w32(esp.wrapping_add(0x4cu32), 0x68u32);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                m.w32(esp.wrapping_add(0x54u32), ecx);
                ecx = esi.wrapping_add(0x5348u32);
                m.w32(esp.wrapping_add(0x58u32), 0x0u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x4cu32);
                pc = 275; continue;
            }
            269 => {
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x39u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6cbdd8u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x454f80, ecx, &a); }
                esp = esp.wrapping_add(8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 276; continue; }
                pc = 270; continue;
            }
            270 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                let (r, f) = lift::sbb(eax, eax, fl.cf, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                let (r, f) = lift::sbb(edx, edx, fl.cf, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                let r = eax | edx; fl = lift::logic(r, 32);
                eax = r;
                if fl.zf { pc = 276; continue; }
                pc = 271; continue;
            }
            271 => {
                ecx = esp.wrapping_add(0x24u32);
                { let a = []; eax = h.call_at(esp, m, 0x535740, ecx, &a); }
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x8u32)))) & 0xffff);
                m.w32(esp.wrapping_add(0x24u32), 0x68u32);
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 273; continue; }
                pc = 272; continue;
            }
            272 => {
                edx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                pc = 274; continue;
            }
            273 => {
                eax = m.r32(esi.wrapping_add(0xa0u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                pc = 274; continue;
            }
            274 => {
                edx = m.r32(esi.wrapping_add(0x5348u32));
                m.w32(esp.wrapping_add(0x28u32), ecx);
                ecx = esi.wrapping_add(0x5348u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x30u32), 0x1u32);
                { let t = m.r32(edx.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = esp.wrapping_add(0x24u32);
                pc = 275; continue;
            }
            275 => {
                ecx = 0x6df2f0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x54aa70, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 276; continue;
            }
            276 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xe4u32));
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 279; continue; }
                pc = 277; continue;
            }
            277 => {
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x57dcu32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(edi.wrapping_add(0x132u32))), 16).1;
                if fl.zf { pc = 279; continue; }
                pc = 278; continue;
            }
            278 => {
                ebp = m.r32(esi.wrapping_add(0x3884u32));
                ebx = m.r32(esi.wrapping_add(0x18u32));
                ecx = esi.wrapping_add(0x387cu32);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                edx = esp.wrapping_add(0x24u32);
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), eax);
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x132u32)))) & 0xffff);
                m.w16(esi.wrapping_add(0x57dcu32), ((eax & 0xffff)) as u16);
                pc = 279; continue;
            }
            279 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048CF50` ship_side_menu_draw_ship_list_table
pub fn f_0048cf50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x660405u32);
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
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = m.r32(esi.wrapping_add(0xacu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 13; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esp.wrapping_add(0x58u32));
                eax = m.r32(esi.wrapping_add(0x3880u32));
                ebx = m.r32(ecx);
                edi = m.r32(ecx.wrapping_add(0x4u32));
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                eax = m.r32(esi.wrapping_add(0x3884u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x2cu32), ebx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                m.w32(esp.wrapping_add(0x54u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a4u32));
                esp = esp.wrapping_add(0x4);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                edx = m.r32(0x6998acu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(0x6998f0u32);
                esp = esp.wrapping_add(0x10);
                ebp = esi.wrapping_add(0x387cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x1cu32));
                ecx = ebx;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = []; eax = h.call_at(esp, m, 0x4bb770, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                ecx = ebp;
                m.w32(esp.wrapping_add(0x3cu32), edi);
                m.w32(esp.wrapping_add(0x38u32), ebx);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x38u32));
                ecx = ebp;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x40u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                ebx = edi;
                ecx = ebp;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                m.w32(esp.wrapping_add(0x48u32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                edx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = m.r32(esi.wrapping_add(0x1acu32));
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                ebx = m.r32(esi.wrapping_add(0x3cdcu32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(edi, ebx, 32); fl = f;
                edi = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                ebx = m.r32(esi.wrapping_add(0x3cd4u32));
                eax = m.r32(esi.wrapping_add(0x4164u32));
                ecx = m.r32(esi.wrapping_add(0x3cd8u32));
                esp = esp.wrapping_add(0x4);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(ebx, ecx, 32).1;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf == fl.of) { pc = 12; continue; }
                pc = 4; continue;
            }
            4 => {
                pc = 6; continue;
            }
            5 => {
                eax = m.r32(esp.wrapping_add(0x18u32));
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(ebx, eax, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = m.r32(0x66e104u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 9; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                pc = 9; continue;
            }
            9 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x4168u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ebx, 32).1;
                if !fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(0x66e108u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(ebx.wrapping_mul(2))))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                edx = ebx.wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x24u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xeu32)))) & 0xff);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0cb4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ebp = esi.wrapping_add(0x387cu32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                ebx = eax;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x2cu32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x14u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(ebx.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(0xc);
                ecx = esp;
                m.w32(ecx, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(ecx.wrapping_add(0x4u32), edi);
                m.w32(ecx.wrapping_add(0x8u32), eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490f80, ecx, &a); }
                esp = esp.wrapping_add(16);
                eax = m.r32(esi.wrapping_add(0x3cacu32));
                edx = m.r32(esp.wrapping_add(0x58u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esi.wrapping_add(0x3880u32));
                ecx = m.r32(edx);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x3cd8u32));
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                fl = lift::sub(ebx, eax, 32).1;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                if (fl.sf != fl.of) { pc = 5; continue; }
                pc = 12; continue;
            }
            12 => {
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x28u32);
                m.w32(esp.wrapping_add(0x50u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                pc = 13; continue;
            }
            13 => {
                ecx = m.r32(esp.wrapping_add(0x48u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x44);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048D230` ship_side_menu_on_mouse_event
pub fn f_0048d230(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                ecx = m.r32(0x66d504u32);
                eax = m.r32(esi.wrapping_add(0x94u32));
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xb8u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc0u32)), 32).1;
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xc8u32)), 32).1;
                if !fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                edx = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(ecx, edx, 32).1;
                if fl.zf { pc = 27; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xbcu32)), 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xd8u32));
                fl = lift::sub(edx, eax, 32).1;
                if !fl.zf { pc = 28; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(0x6e51acu32);
                edx = m.r32(0x6de4b0u32);
                eax = m.r32(ecx.wrapping_add(0xc324u32));
                fl = lift::sub(eax, edx, 32).1;
                if fl.cf { pc = 28; continue; }
                pc = 8; continue;
            }
            8 => {
                { let a = []; eax = h.call_at(esp, m, 0x4b2200, ecx, &a); }
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 28; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = m.r32(esi.wrapping_add(0x5824u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                ecx = m.r32(eax.wrapping_add(0xcu32));
                edx = m.r32(ecx);
                m.w32(esp.wrapping_add(0x10u32), edx);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), edx);
                edi = m.r32(ecx.wrapping_add(0x8u32));
                ecx = m.r32(ecx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                pc = 13; continue;
            }
            12 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ebp = m.r32(edx.wrapping_add(0x8u32));
                edx = m.r32(edx.wrapping_add(0xcu32));
                pc = 17; continue;
            }
            16 => {
                ebp = m.r32(esp.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                pc = 17; continue;
            }
            17 => {
                ecx = m.r32(esi.wrapping_add(0x5810u32));
                eax = m.r32(esi.wrapping_add(0x18u32));
                ebx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x580cu32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5828u32)))) & 0xff);
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x24u32));
                m.w32(esp.wrapping_add(0x1cu32), edx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = eax.wrapping_add(ebp);
                eax = m.r32(edi.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(edi);
                m.w32(esp.wrapping_add(0x18u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(0x66a4f4u32); let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(12);
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 19; continue; }
                pc = 18; continue;
            }
            18 => {
                eax = ebp;
                ebp = m.r32(esp.wrapping_add(0x10u32));
                edx = ((eax as i32) >> 31) as u32;
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                let (r, f) = lift::sar(ecx, 0x2u32, 32, fl); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                m.w8(esi.wrapping_add(0x5828u32), ((eax & 0xff)) as u8);
                pc = 20; continue;
            }
            19 => {
                m.w8(esi.wrapping_add(0x5828u32), (0xffu32) as u8);
                pc = 20; continue;
            }
            20 => {
                eax = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                fl = lift::sub(eax, 0x1u32, 32).1;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                if !fl.zf { pc = 25; continue; }
                pc = 21; continue;
            }
            21 => {
                eax = m.r32(esi.wrapping_add(0x3858u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 25; continue; }
                pc = 22; continue;
            }
            22 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5828u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x5829u32))), 8).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48d420, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            25 => {
                fl = lift::sub(u32::from(m.r8(esi.wrapping_add(0x5828u32))), (ebx & 0xff), 8).1;
                if fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x494580, ecx, &a); }
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            27 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48d4d0, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 28; continue;
            }
            28 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048D4D0` ship_side_menu_on_list_row_click
pub fn f_0048d4d0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(0xc);
                fl = lift::sub(eax, 0xfffffffeu32, 32).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                if !fl.zf { pc = 6; continue; }
                pc = 1; continue;
            }
            1 => {
                fl = lift::sub(m.r32(esi.wrapping_add(0x4164u32)), 0xffffffffu32, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6dcbd1u32))) & 0xff);
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
                m.w8(0x6dcbd1u32, ((edx & 0xff)) as u8);
                if fl.zf { pc = 5; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0xacu32));
                eax = m.r32(esi.wrapping_add(0xa0u32));
                edx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x4u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x3cd4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esi.wrapping_add(0x4164u32));
                m.w8(esp.wrapping_add(0x10u32), (0x0u32) as u8);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = (edx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x48fdf0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = esp.wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = esi;
                { let t = m.r32(eax.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0x3cd4u32));
                ecx = m.r32(esi.wrapping_add(0x4164u32));
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esi.wrapping_add(0x4168u32), edx);
                eax = (eax & !0xff) | ((u32::from(m.r8(0x6e59ccu32))) & 0xff);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                m.w32(esp.wrapping_add(0x8u32), 0x1u32);
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let (r, f) = lift::sub((eax & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                eax = (eax & !0xff) | ((r) & 0xff);
                let r = eax & 0x2u32; fl = lift::logic(r, 32);
                eax = r;
                m.w8(esp.wrapping_add(0x10u32), ((eax & 0xff)) as u8);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4168u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0xcu32), eax);
                esp = esp.wrapping_add(0x4);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edx.wrapping_add(ecx.wrapping_mul(2))))) & 0xffff);
                m.w16(eax, ((ecx & 0xffff)) as u16);
                edx = m.r32(esi);
                eax = esp.wrapping_add(0x4u32);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x4168u32), 0xffffffffu32);
                m.w32(esi.wrapping_add(0x4164u32), 0xffffffffu32);
                pc = 6; continue;
            }
            6 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048D8C0` ship_side_menu_draw_ship_picture
pub fn f_0048d8c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6dd7a4u32);
                esp = esp.wrapping_sub(0x28);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(ebx, 0x7u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, edx, 32); fl = f;
                ebx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebx.wrapping_add(0x8u32)))) & 0xffff);
                esi = ecx;
                fl = lift::sub((eax & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(0x6dd7a8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x14u32)))) & 0xff);
                let r = (edx & 0xff) & 0x10u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x10u32, 8).1;
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = 0x4u32;
                pc = 4; continue;
            }
            3 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                pc = 4; continue;
            }
            4 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0xeu32)))) & 0xff);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x5870u32));
                edx = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x5850u32));
                ebp = m.r32(eax.wrapping_add(0x4u32));
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x40u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x44u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                eax = m.r32(edi.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x40u32), eax);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x38u32), eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                m.w32(esp.wrapping_add(0x34u32), ecx);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x3cu32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                m.w32(esp.wrapping_add(0x48u32), eax);
                eax = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x50u32));
                edx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(esp.wrapping_add(0x58u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x40u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                eax = ecx;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x5878u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x5874u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(edi);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x50u32));
                edx = m.r32(edi.wrapping_add(0xcu32));
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebp = eax;
                eax = ecx;
                ecx = m.r32(edi);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ebp, 0x1u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ebp, eax, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x54);
                fl = lift::sub(u32::from(m.r16(ebx.wrapping_add(0x134u32))), 0x12u32, 16).1;
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(ebx.wrapping_add(0x15cu32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64ffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5880u32));
                ecx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ebx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                eax = ecx;
                ebp = m.r32(edi.wrapping_add(0xcu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x28);
                pc = 7; continue;
            }
            7 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc8u32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esi.wrapping_add(0x5894u32));
                eax = m.r32(ecx.wrapping_add(0xfffffff8u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                esp = esp.wrapping_add(0x4);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5894u32));
                edx = m.r32(esi.wrapping_add(0x3864u32));
                esi = m.r32(edi);
                ecx = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(edx, esi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x14);
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
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048DB50` ship_side_menu_animate_ship_picture
pub fn f_0048db50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    let mut pc = 0usize;
    loop {
        match pc {
            0 => {
                eax = m.r32(0x6dccf8u32);
                esp = esp.wrapping_sub(0x28);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0x587cu32)), 32).1;
                if fl.cf { pc = 27; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5832u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if !fl.zf { pc = 27; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(esi.wrapping_add(0x5870u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ecx = esp.wrapping_add(0xcu32);
                eax = m.r32(edx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x30u32));
                edi = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x3cu32), eax);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edi = m.r32(esp.wrapping_add(0x18u32));
                m.w32(esp.wrapping_add(0x34u32), eax);
                eax = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x8);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = eax;
                eax = edi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::sar(ecx, 0x1u32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x28u32), ecx);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x30u32), ecx);
                ecx = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6da5f0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                ebp = m.r32(esi.wrapping_add(0x5874u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x8);
                let (r, f) = lift::sar(edi, 0x1u32, 32, fl); fl = f;
                edi = r;
                ebx = eax;
                eax = edi;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(ebx, 0x1u32, 32, fl); fl = f;
                ebx = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = ebx;
                edx = ((eax as i32) >> 31) as u32;
                let r = edx & 0x3u32; fl = lift::logic(r, 32);
                edx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = lift::sx8(u32::from(m.r8(0x6b9e78u32)));
                let (r, f) = lift::sar(eax, 0x2u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ebp, edx, 32); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esi.wrapping_add(0x5874u32), ebp);
                ebp = edi;
                eax = lift::sx8(u32::from(m.r8(0x6b9e79u32)));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x5878u32), ecx);
                let (r, f) = lift::sub(0, ebp, 32); fl = f;
                ebp = r;
                eax = m.r32(esi.wrapping_add(0x5874u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub(eax, edi, 32).1;
                if (!fl.zf && fl.sf == fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = ebx;
                let (r, f) = lift::sub(0, edx, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                if (fl.sf != fl.of) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 6; continue;
            }
            6 => {
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e78u32, ((edx & 0xff)) as u8);
                { let a = []; eax = h.call_at(esp, m, 0x639c03, ecx, &a); }
                edx = ((eax as i32) >> 31) as u32;
                ecx = 0x3u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                let (r, f) = lift::sub((edx & 0xff), 1, 8); fl = lift::Fl { cf: fl.cf, ..f };
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e79u32, ((edx & 0xff)) as u8);
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                m.w8(0x6b9e79u32, (0x1u32) as u8);
                pc = 8; continue;
            }
            8 => {
                eax = edi;
                ecx = m.r32(esi.wrapping_add(0x5874u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = edi;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6b9e78u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::sub(eax, edi, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (fl.sf == fl.of) { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                let (r, f) = lift::sub(0, (edx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e78u32, ((edx & 0xff)) as u8);
                pc = 13; continue;
            }
            13 => {
                eax = ebx;
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ebx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                fl = lift::sub(ecx, edx, 32).1;
                edx = (edx & !0xff) | ((u32::from(m.r8(0x6b9e79u32))) & 0xff);
                if (fl.zf || fl.sf != fl.of) { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (!fl.zf && fl.sf == fl.of) { pc = 17; continue; }
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                fl = lift::logic((edx & 0xff) & (edx & 0xff), 8);
                if (fl.sf == fl.of) { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                let (r, f) = lift::sub(0, (edx & 0xff), 8); fl = f;
                edx = (edx & !0xff) | ((r) & 0xff);
                m.w8(0x6b9e79u32, ((edx & 0xff)) as u8);
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x5874u32));
                fl = lift::sub(eax, ebp, 32).1;
                if (fl.sf == fl.of) { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                m.w32(esi.wrapping_add(0x5874u32), ebp);
                pc = 22; continue;
            }
            20 => {
                fl = lift::sub(eax, edi, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x5874u32), edi);
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(esi.wrapping_add(0x5878u32));
                eax = ebx;
                let (r, f) = lift::sub(0, eax, 32); fl = f;
                eax = r;
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.sf == fl.of) { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                m.w32(esi.wrapping_add(0x5878u32), eax);
                pc = 26; continue;
            }
            24 => {
                fl = lift::sub(ecx, ebx, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                m.w32(esi.wrapping_add(0x5878u32), ebx);
                pc = 26; continue;
            }
            26 => {
                eax = m.r32(0x6dccf8u32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let (r, f) = lift::add(eax, 0x96u32, 32); fl = f;
                eax = r;
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(esi.wrapping_add(0x587cu32), eax);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 27; continue;
            }
            27 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x28);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048DD70` ship_side_menu_draw_ship_panel
pub fn f_0048dd70(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = m.r32(0x6dd7a4u32);
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::shl(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(ecx).wrapping_add(0xeu32)))) & 0xff);
                eax = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x5884u32));
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x10u32), eax);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                ebp = m.r32(edx.wrapping_add(0xcu32));
                pc = 4; continue;
            }
            3 => {
                ebx = m.r32(esp.wrapping_add(0x18u32));
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                pc = 4; continue;
            }
            4 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edi = m.r32(esp.wrapping_add(0x2cu32));
                eax = m.r32(esi.wrapping_add(0x2cu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                edx = m.r32(edi.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(edi);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ecx, ebp, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0xccu32));
                ecx = m.r32(esi.wrapping_add(0xd0u32));
                esp = esp.wrapping_add(0x20);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = esi;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f510, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            6 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xd4u32)), 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48df90, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            8 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xd8u32)), 32).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48e700, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            10 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xe0u32)), 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            12 => {
                fl = lift::sub(eax, m.r32(esi.wrapping_add(0xe4u32)), 32).1;
                if !fl.zf { pc = 16; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 15; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f2c0, ecx, &a); }
                esp = esp.wrapping_add(4);
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
                esp = esp.wrapping_add(0x10);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048DF20` ship_side_menu_draw_header_and_cargo
pub fn f_0048df20(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    edi = m.r32(esp.wrapping_add(0xcu32));
    eax = m.r32(esi.wrapping_add(0xa0u32));
    ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ecx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
    esp = esp.wrapping_add(8);
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ecx = esi;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f510, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X0048DF50` ship_side_menu_draw_header_and_route
pub fn f_0048df50(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(esp.wrapping_add(0xcu32));
                eax = m.r32(esi.wrapping_add(0xb4u32));
                ecx = m.r32(esi.wrapping_add(0xc0u32));
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                edx = (edx & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = esi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x48ee90, ecx, &a); }
                esp = esp.wrapping_add(8);
                pc = 2; continue;
            }
            2 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x48f2c0, ecx, &a); }
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

/// `0X0048DF90` ship_side_menu_draw_crew_panel
pub fn f_0048df90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66042au32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x48);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                ecx = m.r32(0x6c7cccu32);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(0x6dd7a4u32);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ebp = ebp.wrapping_add(ebp.wrapping_mul(2));
                let (r, f) = lift::shl(ebp, 0x7u32, 32, fl); fl = f;
                ebp = r;
                let (r, f) = lift::add(ebp, edi, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                eax = m.r32(esp.wrapping_add(0x68u32));
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                m.w32(esp.wrapping_add(0x60u32), 0x0u32);
                edx = m.r32(eax);
                eax = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x28u32), eax);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0x3fu32)))) & 0xff);
                esp = esp.wrapping_add(0x8);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if (fl.sf == fl.of) { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                pc = 4; continue;
            }
            2 => {
                fl = lift::sub((eax & 0xff), 0x4u32, 8).1;
                if (fl.zf || fl.sf != fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                eax = (eax & !0xff) | ((0x4u32) & 0xff);
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x3848u32));
                eax = lift::sx8((eax & 0xff));
                fl = lift::sub(eax, m.r32(ecx.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 7; continue; }
                pc = 6; continue;
            }
            6 => {
                m.w32(esp.wrapping_add(0x14u32), edx);
                edx = m.r32(ecx.wrapping_add(0xcu32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(eax);
                m.w32(esp.wrapping_add(0x48u32), edx);
                ecx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                edi = m.r32(eax.wrapping_add(0x8u32));
                ebx = m.r32(eax.wrapping_add(0xcu32));
                pc = 8; continue;
            }
            7 => {
                edi = m.r32(esp.wrapping_add(0x50u32));
                ebx = m.r32(esp.wrapping_add(0x54u32));
                pc = 8; continue;
            }
            8 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x57e4u32));
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x57e0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x24);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x5185c0, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(ebp.wrapping_add(0xeu32)))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x673660u32)))) & 0xff);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x40u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3bcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                esp = esp.wrapping_add(0x14);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(edx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                edx = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let t = m.r32(eax.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esp.wrapping_add(0x10u32));
                edx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = ebx;
                ecx = m.r32(esi.wrapping_add(0x57e4u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x57e0u32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3848u32));
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x5u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 11; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 11; continue; }
                pc = 10; continue;
            }
            10 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                let (r, f) = lift::add(edx, 0x50u32, 32); fl = f;
                edx = r;
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x48u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                edi = m.r32(edx.wrapping_add(0x8u32));
                ebx = m.r32(edx.wrapping_add(0xcu32));
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x57f0u32));
                edx = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esi.wrapping_add(0x57ecu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x58u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esp.wrapping_add(0x5cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(0x699c78u32);
                esp = esp.wrapping_add(0x24);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x517f70, ecx, &a); }
                edx = eax.wrapping_mul(8);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                eax = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3ccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                esp = esp.wrapping_add(0x10);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = m.r32(ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(esp.wrapping_add(0x10u32));
                eax = ebx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x57f0u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0x57ecu32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, edi, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x74u32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                esp = esp.wrapping_add(0xc);
                edx = m.r32(eax);
                eax = m.r32(0x6cbd5cu32);
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, m.r32(edx.wrapping_add(0x4u32)), 32); fl = f;
                eax = r;
                edx = m.r32(esi.wrapping_add(0xa0u32));
                m.w32(esp.wrapping_add(0x24u32), ecx);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                m.w32(esp.wrapping_add(0x28u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(0xc);
                edx = esp;
                m.w32(edx, ecx);
                ecx = esi;
                m.w32(edx.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x3cu32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490f80, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = ecx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                edx = ebp.wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                esp = esp.wrapping_add(0x14);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3850u32));
                esp = esp.wrapping_add(0x4);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 14; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), edx);
                edi = m.r32(eax.wrapping_add(0x8u32));
                ebx = m.r32(eax.wrapping_add(0xcu32));
                pc = 14; continue;
            }
            14 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = esp.wrapping_add(0x10u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x154u32)))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                esp = esp.wrapping_add(0x14);
                let (r, f) = lift::sub(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                ecx = m.r32(edx);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = esi.wrapping_add(0x2068u32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                eax = m.r32(ecx);
                { let t = m.r32(eax.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                edx = m.r32(ecx.wrapping_add(0x4u32));
                ecx = m.r32(esi.wrapping_add(0x18u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                eax = eax.wrapping_add(edx).wrapping_add(0x21u32);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x30u32));
                eax = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
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
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x34u32));
                eax = m.r32(esp.wrapping_add(0x48u32));
                edx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi.wrapping_add(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x42u32)))) & 0xffff);
                esp = esp.wrapping_add(0x30);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                if !fl.cf { pc = 24; continue; }
                pc = 15; continue;
            }
            15 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4103c0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                eax = esp.wrapping_add(0x3cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16))]; eax = h.call_at(esp, m, 0x639e9c, ecx, &a); }
                esp = esp.wrapping_add(20);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x51eb851fu32;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5809u32)))) & 0xff);
                m.w8(esp.wrapping_add(0x60u32), (0x1u32) as u8);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = esp.wrapping_add(0x34u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3d8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x51eb851fu32;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5808u32)))) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                ecx = esp.wrapping_add(0x40u32);
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3dcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                eax = 0x51eb851fu32;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x580au32)))) & 0xff);
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x4u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                ecx = esp.wrapping_add(0x50u32);
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3e0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x38a8u32));
                esp = esp.wrapping_add(0x24);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x384cu32));
                esp = esp.wrapping_add(0x4);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 18; continue; }
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), edx);
                edi = m.r32(eax.wrapping_add(0x8u32));
                ebx = m.r32(eax.wrapping_add(0xcu32));
                pc = 18; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x1c1cu32));
                ecx = esi.wrapping_add(0x1c1cu32);
                { let t = m.r32(eax.wrapping_add(0x70u32)); let a = []; eax = h.call_at(esp, m, t, ecx, &a); esp = esp.wrapping_add(4 * h.callee_pop(t)); }
                ecx = m.r32(esp.wrapping_add(0x68u32));
                ebp = m.r32(esi.wrapping_add(0x18u32));
                edx = m.r32(ecx.wrapping_add(0xcu32));
                ecx = m.r32(esi.wrapping_add(0x30u32));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                ecx = m.r32(esp.wrapping_add(0x38u32));
                let (r, f) = lift::sub(edx, ebp, 32); fl = f;
                edx = r;
                ebp = m.r32(esp.wrapping_add(0x34u32));
                let (r, f) = lift::sub(edx, ebx, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x18u32), ecx);
                eax = eax.wrapping_add(edx).wrapping_add(0xfffffffbu32);
                edx = m.r32(esp.wrapping_add(0x30u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x2cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x2cu32));
                ecx = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::sub(edx, ecx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0xc);
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                let (r, f) = lift::sub(ecx, 0x1cu32, 32); fl = f;
                ecx = r;
                eax = ecx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                ebp = eax;
                eax = m.r32(esp.wrapping_add(0x68u32));
                let (r, f) = lift::sar(ebp, 0x1u32, 32, fl); fl = f;
                ebp = r;
                ecx = m.r32(eax);
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(esi.wrapping_add(0x384cu32));
                ecx = m.r32(esp.wrapping_add(0x18u32));
                fl = lift::sub(ecx, m.r32(eax.wrapping_add(0x14u32)), 32).1;
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 20; continue;
            }
            20 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 22; continue; }
                pc = 21; continue;
            }
            21 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x48u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x4cu32), ecx);
                edi = m.r32(edx.wrapping_add(0x8u32));
                ebx = m.r32(edx.wrapping_add(0xcu32));
                pc = 22; continue;
            }
            22 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x18u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                ecx = m.r32(esp.wrapping_add(0x54u32));
                edx = m.r32(esp.wrapping_add(0x50u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::add(ebp, edi, 32); fl = f;
                ebp = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                eax = m.r32(esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x48u32));
                eax = m.r32(esp.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x60u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                ebp = eax.wrapping_add(ebp).wrapping_add(0xeu32);
                eax = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_add(0x34);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x18u32), eax);
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, 0x10u32, 32); fl = f;
                eax = r;
                fl = lift::sub(eax, 0x30u32, 32).1;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                if (fl.sf != fl.of) { pc = 19; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(esp.wrapping_add(0x20u32));
                ebp = m.r32(0x6dd7a0u32);
                edx = m.r32(0x6999e0u32);
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x42u32)))) & 0xffff);
                eax = m.r32(esi.wrapping_add(0x5804u32));
                ecx = 0xfffffffcu32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                let (r, f) = lift::sub(ecx, ebx, 32); fl = f;
                ecx = r;
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::shl(edi, 0x4u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(ebx, ecx, 32); fl = f;
                ebx = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = 0x6ddaa0u32;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512ae0, ecx, &a); }
                esp = esp.wrapping_add(4);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x2u32)))) & 0xff);
                ecx = 0x6ddaa0u32;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x512a60, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(0x6999d4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esp.wrapping_add(0x24u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3e4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba90, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x30u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x88u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                esi = m.r32(edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x2c);
                edx = esp.wrapping_add(0x30u32);
                m.w8(esp.wrapping_add(0x60u32), (0x0u32) as u8);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x64f253u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x4u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x639da8, ecx, &a); }
                esp = esp.wrapping_add(16);
                pc = 24; continue;
            }
            24 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x60u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x58u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x54);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048E700` ship_side_menu_draw_load_panel
pub fn f_0048e700(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66043cu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x74);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ebp = m.r32(0x6dd7a4u32);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::add(edi, ebp, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x18u32), edi);
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(0x6cbd5cu32);
                ebx = m.r32(esp.wrapping_add(0x9cu32));
                ecx = m.r32(esi.wrapping_add(0x3864u32));
                esp = esp.wrapping_add(0x8);
                ebp = m.r32(ebx.wrapping_add(0x4u32));
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(ebx);
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, edx, 32); fl = f;
                ecx = r;
                edx = m.r32(esi.wrapping_add(0xa0u32));
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), ecx);
                m.w32(esp.wrapping_add(0x20u32), eax);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(0xc);
                edx = esp;
                m.w32(edx, ecx);
                ecx = esi;
                m.w32(edx.wrapping_add(0x4u32), eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                m.w32(edx.wrapping_add(0x8u32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x490f80, ecx, &a); }
                esp = esp.wrapping_add(16);
                ebp = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::add(ebp, eax, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x20u32), ebp);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                let (r, f) = lift::add(edi, 0x160u32, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3844u32));
                esp = esp.wrapping_add(0x10);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x58u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x5cu32), edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x60u32), ecx);
                edi = m.r32(eax.wrapping_add(0xcu32));
                pc = 4; continue;
            }
            3 => {
                edi = m.r32(esp.wrapping_add(0x64u32));
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(esi.wrapping_add(0x3874u32));
                edx = m.r32(esi.wrapping_add(0x3864u32));
                ebp = m.r32(ebx);
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::add(edx, ebp, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(eax.wrapping_mul(8).wrapping_add(0x6dcd28u32));
                ecx = eax.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                eax = esp.wrapping_add(0x28u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let t = m.r32(edx.wrapping_add(0x40u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x2cu32));
                ebp = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(0x6c7cccu32);
                let (r, f) = lift::add(ecx, edi, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(ebp, ecx, 32); fl = f;
                ebp = r;
                m.w32(esp.wrapping_add(0x68u32), edx);
                m.w32(esp.wrapping_add(0x20u32), ebp);
                eax = m.r32(esi.wrapping_add(0xa0u32));
                ecx = m.r32(0x6dd7a4u32);
                m.w32(esp.wrapping_add(0x8cu32), 0x0u32);
                ebp = (ebp & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                eax = ebp;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dfc14u32);
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x40u32)))) & 0xffff);
                pc = 11; continue;
            }
            6 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x51a3a0, ecx, &a); }
                ecx = m.r32(0x66e0f4u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x40u32)))) & 0xffff);
                ebx = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(ebx, eax, 32); fl = f;
                ebx = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(edx, ebx); fl = f;
                edx = r;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                ebx = ecx;
                fl = lift::sub(ebx, 0x1u32, 32).1;
                if (fl.sf == fl.of) { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ebx = 0x1u32;
                pc = 10; continue;
            }
            8 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5185c0, ecx, &a); }
                fl = lift::sub(ebx, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x5185c0, ecx, &a); }
                ebx = eax;
                pc = 10; continue;
            }
            10 => {
                eax = ebx;
                ebx = m.r32(esp.wrapping_add(0x94u32));
                pc = 11; continue;
            }
            11 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x74u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x495450, ecx, &a); }
                ecx = edi.wrapping_add(0x160u32);
                edx = esp.wrapping_add(0x6cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                m.w32(esp.wrapping_add(0x7cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4954c0, ecx, &a); }
                m.w32(esp.wrapping_add(0x8cu32), eax);
                eax = m.r32(edi.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x88u32), eax);
                ecx = m.r32(edi.wrapping_add(0x11cu32));
                m.w32(esp.wrapping_add(0x90u32), ecx);
                edx = (edx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x42u32)))) & 0xffff);
                esp = esp.wrapping_add(0x14);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                ecx = 0xc8u32;
                eax = (eax & !0xff) | ((u32::from(fl.cf)) & 0xff);
                m.w8(esp.wrapping_add(0x82u32), ((eax & 0xff)) as u8);
                eax = m.r32(edi.wrapping_add(0x11cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x7cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x7cu32), ecx);
                pc = 13; continue;
            }
            13 => {
                ecx = esp.wrapping_add(0x68u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                edx = esp.wrapping_add(0x20u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = esi;
                m.w16(esp.wrapping_add(0x8cu32), ((ebp & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x491ca0, ecx, &a); }
                esp = esp.wrapping_add(12);
                eax = m.r32(esi.wrapping_add(0x580cu32));
                ecx = m.r32(ebx);
                ebp = m.r32(ebx.wrapping_add(0x4u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esi.wrapping_add(0x5810u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                let (r, f) = lift::add(ecx, ebp, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x20u32), eax);
                m.w32(esp.wrapping_add(0x24u32), ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x5824u32));
                esp = esp.wrapping_add(0x4);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 14; continue;
            }
            14 => {
                edi = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(edi & edi, 32);
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x28u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x2cu32), ecx);
                ebx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x30u32), ebx);
                edx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x34u32), edx);
                pc = 17; continue;
            }
            16 => {
                edi = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x30u32));
                pc = 17; continue;
            }
            17 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x38u32));
                eax = m.r32(esi.wrapping_add(0x5824u32));
                let (r, f) = lift::add(edx, ebx, 32); fl = f;
                edx = r;
                esp = esp.wrapping_add(0x1c);
                m.w32(esp.wrapping_add(0x1cu32), edx);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::sub(ecx, 0x1u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 20; continue; }
                pc = 18; continue;
            }
            18 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 20; continue; }
                pc = 19; continue;
            }
            19 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                edi = ecx;
                let (r, f) = lift::add(edx, 0x10u32, 32); fl = f;
                edx = r;
                ebx = m.r32(edx);
                ebp = m.r32(edx.wrapping_add(0x4u32));
                eax = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x50u32), eax);
                ecx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x54u32), ecx);
                pc = 21; continue;
            }
            20 => {
                ebx = m.r32(esp.wrapping_add(0x48u32));
                ebp = m.r32(esp.wrapping_add(0x4cu32));
                pc = 21; continue;
            }
            21 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x58u32));
                eax = m.r32(esp.wrapping_add(0x54u32));
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x3820u32));
                esp = esp.wrapping_add(0x1c);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 24; continue; }
                pc = 22; continue;
            }
            22 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 24; continue; }
                pc = 23; continue;
            }
            23 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                edi = ecx;
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x38u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x3cu32), edx);
                ebx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x40u32), ebx);
                ebp = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x44u32), ebp);
                pc = 25; continue;
            }
            24 => {
                ebp = m.r32(esp.wrapping_add(0x44u32));
                ebx = m.r32(esp.wrapping_add(0x40u32));
                pc = 25; continue;
            }
            25 => {
                ecx = m.r32(esp.wrapping_add(0x50u32));
                eax = ebx.wrapping_add(ebx.wrapping_mul(4));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5829u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                { let v = f64::from(m.r32(esp.wrapping_add(0x10u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x10u32)); fp.push(v); }
                { let v = lift::fld32(m, 0x66f4a0u32); let d = fp.st(0); fp.set(0, d / v); }
                lift::fst32(m, esp.wrapping_add(0x10u32), fp.st(0)); fp.pop();
                if fl.zf { pc = 28; continue; }
                pc = 26; continue;
            }
            26 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x582au32))), 8).1;
                if fl.zf { pc = 28; continue; }
                pc = 27; continue;
            }
            27 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x80ffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x5cu32));
                ecx = m.r32(esp.wrapping_add(0x28u32));
                { let v = f64::from(m.r32(esp.wrapping_add(0x48u32)) as i32); fp.push(v); }
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                edx = ((eax as i32) >> 31) as u32;
                lift::fst32(m, esp.wrapping_add(0x20u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d + v); }
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5829u32)))) & 0xff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                m.w32(esp.wrapping_add(0x24u32), edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x24u32)) as i32); fp.push(v); }
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); let d = fp.st(0); fp.set(0, d * v); }
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x4cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x54u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x24);
                pc = 28; continue;
            }
            28 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5828u32)))) & 0xff);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 29; continue;
            }
            29 => {
                fl = lift::sub((eax & 0xff), u32::from(m.r8(esi.wrapping_add(0x5829u32))), 8).1;
                if fl.zf { pc = 35; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esi.wrapping_add(0x5824u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x3u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 33; continue; }
                pc = 31; continue;
            }
            31 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 33; continue; }
                pc = 32; continue;
            }
            32 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                edi = ecx;
                let (r, f) = lift::add(edx, 0x30u32, 32); fl = f;
                edx = r;
                ebp = m.r32(edx);
                eax = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x5cu32), eax);
                ecx = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x60u32), ecx);
                ebx = m.r32(edx.wrapping_add(0xcu32));
                pc = 34; continue;
            }
            33 => {
                ebp = m.r32(esp.wrapping_add(0x58u32));
                ebx = m.r32(esp.wrapping_add(0x64u32));
                pc = 34; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x5cu32));
                edx = m.r32(esp.wrapping_add(0x68u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let (r, f) = lift::sub(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let v = f64::from(m.r32(esp.wrapping_add(0x70u32)) as i32); fp.push(v); }
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                lift::fst32(m, esp.wrapping_add(0x24u32), fp.st(0)); fp.pop();
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                { let v = lift::fld32(m, esp.wrapping_add(0x24u32)); fp.push(v); }
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                { let v = lift::fld32(m, esp.wrapping_add(0x20u32)); let d = fp.st(0); fp.set(0, d + v); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x5828u32)))) & 0xff);
                m.w32(esp.wrapping_add(0x28u32), eax);
                { let v = f64::from(m.r32(esp.wrapping_add(0x28u32)) as i32); fp.push(v); }
                lift::fst32(m, esp.wrapping_add(0x28u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x28u32)); let d = fp.st(0); fp.set(0, d * v); }
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ecx = m.r32(esp.wrapping_add(0x30u32));
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                ecx = m.r32(esp.wrapping_add(0x70u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ebp = m.r32(esp.wrapping_add(0x64u32));
                ebx = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_add(0x20);
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(esp.wrapping_add(0x50u32));
                eax = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x5824u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x2u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 38; continue; }
                pc = 36; continue;
            }
            36 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 38; continue; }
                pc = 37; continue;
            }
            37 => {
                eax = m.r32(eax.wrapping_add(0xcu32));
                edi = ecx;
                let (r, f) = lift::add(eax, 0x20u32, 32); fl = f;
                eax = r;
                ecx = m.r32(eax);
                m.w32(esp.wrapping_add(0x28u32), ecx);
                edx = m.r32(eax.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x2cu32), edx);
                ecx = m.r32(eax.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x30u32), ecx);
                edx = m.r32(eax.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x34u32), edx);
                pc = 38; continue;
            }
            38 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x38u32));
                ecx = m.r32(esp.wrapping_add(0x34u32));
                edx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                edx = m.r32(esp.wrapping_add(0x38u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x582au32)))) & 0xff);
                esp = esp.wrapping_add(0x1c);
                fl = lift::sub((eax & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 40; continue; }
                pc = 39; continue;
            }
            39 => {
                eax = m.r32(esi.wrapping_add(0x3820u32));
                eax = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                { let v = f64::from(m.r32(esp.wrapping_add(0x44u32)) as i32); fp.push(v); }
                eax = m.r32(esp.wrapping_add(0x58u32));
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi.wrapping_add(0x582au32)))) & 0xff);
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                lift::fst32(m, esp.wrapping_add(0x18u32), fp.st(0)); fp.pop();
                { let v = lift::fld32(m, esp.wrapping_add(0x18u32)); fp.push(v); }
                { let v = lift::fld32(m, esp.wrapping_add(0x14u32)); let d = fp.st(0); fp.set(0, d + v); }
                m.w32(esp.wrapping_add(0x18u32), ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let v = f64::from(m.r32(esp.wrapping_add(0x1cu32)) as i32); fp.push(v); }
                edx = ((eax as i32) >> 31) as u32;
                lift::fst32(m, esp.wrapping_add(0x1cu32), fp.st(0)); fp.pop();
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                { let v = lift::fld32(m, esp.wrapping_add(0x1cu32)); let d = fp.st(0); fp.set(0, d * v); }
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                if let Some(v) = h.ftol_hook(m, ecx) { eax = v; } else { let t = fp.st(0).trunc(); fp.pop(); let q: i64 = if t.is_nan() || t.abs() >= 9.223372036854775807e18 { i64::MIN } else { t as i64 }; eax = q as u32; edx = (q >> 32) as u32; }
                ebp = m.r32(esp.wrapping_add(0x60u32));
                ebx = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(eax, ebp, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                pc = 40; continue;
            }
            40 => {
                edx = m.r32(esp.wrapping_add(0x94u32));
                ecx = m.r32(esi.wrapping_add(0x581cu32));
                eax = m.r32(esi.wrapping_add(0x5818u32));
                ebp = m.r32(edx);
                ebx = m.r32(edx.wrapping_add(0x4u32));
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(ecx, ebx, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(eax, ebp, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi;
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x495210, ecx, &a); }
                esp = esp.wrapping_add(12);
                ecx = esp.wrapping_add(0x68u32);
                m.w32(esp.wrapping_add(0x8cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x84u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x80);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048ED80` ship_side_menu_draw_load_icon
pub fn f_0048ed80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                fl = lift::sub((edx & 0xff), 0x6u32, 8).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                if !fl.cf { pc = 4; continue; }
                pc = 1; continue;
            }
            1 => {
                ebx = (ebx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x3cu32)))) & 0xff);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic((ebx & 0xff) & (ebx & 0xff), 8);
                ecx = m.r32(ecx.wrapping_add(0x5824u32));
                eax = (eax & !0xff) | ((u32::from(fl.zf)) & 0xff);
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let r = edx & 0xffu32; fl = lift::logic(r, 32);
                edx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = edx.wrapping_add(eax.wrapping_mul(2));
                edx = m.r32(ecx.wrapping_add(0x14u32));
                fl = lift::sub(eax, edx, 32).1;
                if (fl.sf == fl.of) { pc = 8; continue; }
                pc = 2; continue;
            }
            2 => {
                edx = m.r32(ecx.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 3; continue;
            }
            3 => {
                esi = m.r32(ecx.wrapping_add(0xcu32));
                let (r, f) = lift::shl(eax, 0x4u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                edi = m.r32(eax);
                ebp = m.r32(eax.wrapping_add(0x4u32));
                ebx = m.r32(eax.wrapping_add(0x8u32));
                esi = m.r32(eax.wrapping_add(0xcu32));
                pc = 9; continue;
            }
            4 => {
                fl = lift::sub((edx & 0xff), 0xffu32, 8).1;
                if !fl.zf { pc = 16; continue; }
                pc = 5; continue;
            }
            5 => {
                eax = m.r32(ecx.wrapping_add(0x5824u32));
                fl = lift::sub(m.r32(eax.wrapping_add(0x14u32)), 0x12u32, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                edx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(eax.wrapping_add(0xcu32));
                let (r, f) = lift::add(ecx, 0x120u32, 32); fl = f;
                ecx = r;
                edi = m.r32(ecx);
                ebp = m.r32(ecx.wrapping_add(0x4u32));
                ebx = m.r32(ecx.wrapping_add(0x8u32));
                esi = m.r32(ecx.wrapping_add(0xcu32));
                pc = 9; continue;
            }
            8 => {
                edi = m.r32(esp.wrapping_add(0x14u32));
                ebp = m.r32(esp.wrapping_add(0x18u32));
                ebx = m.r32(esp.wrapping_add(0x1cu32));
                esi = m.r32(esp.wrapping_add(0x20u32));
                edx = m.r32(esp.wrapping_add(0x3cu32));
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x40u32)))) & 0xff);
                esp = esp.wrapping_add(0x4);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 12; continue; }
                pc = 10; continue;
            }
            10 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x30u32)))) & 0xff);
                let (r, f) = lift::sub(eax, esi, 32); fl = f;
                eax = r;
                fl = lift::sub((ecx & 0xff), 0xffu32, 8).1;
                if fl.zf { pc = 15; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(ecx.wrapping_add(0x5830u32)))) & 0xff);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                pc = 15; continue;
            }
            12 => {
                fl = lift::sub(u32::from(m.r8(esp.wrapping_add(0x30u32))), 0xffu32, 8).1;
                if fl.zf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = m.r32(esp.wrapping_add(0x10u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                edx = (edx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x5831u32)))) & 0xff);
                eax = m.r32(esp.wrapping_add(0x2cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                pc = 15; continue;
            }
            14 => {
                eax = m.r32(esp.wrapping_add(0x2cu32));
                pc = 15; continue;
            }
            15 => {
                ecx = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                esp = esp.wrapping_add(0x18);
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
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048EE90` ship_side_menu_draw_ship_info
pub fn f_0048ee90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x66044eu32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x1c);
                eax = m.r32(0x6c7cccu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ebx = ecx;
                m.w32(esp.wrapping_add(0x10u32), eax);
                ebp = m.r32(esp.wrapping_add(0x40u32));
                ecx = m.r32(0x6dd7a4u32);
                eax = ebp;
                m.w32(esp.wrapping_add(0x34u32), 0x0u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                esi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(0x6dfc14u32);
                let (r, f) = lift::shl(esi, 0x7u32, 32, fl); fl = f;
                esi = r;
                let (r, f) = lift::add(esi, ecx, 32); fl = f;
                esi = r;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = edi ^ edi; fl = lift::logic(r, 32);
                edi = r;
                edi = (edi & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x40u32)))) & 0xffff);
                pc = 6; continue;
            }
            2 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x51a3a0, ecx, &a); }
                ecx = m.r32(0x66e0f4u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ecx;
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x40u32)))) & 0xffff);
                edi = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                edi = ecx;
                fl = lift::sub(edi, 0x1u32, 32).1;
                if (fl.sf == fl.of) { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                edi = 0x1u32;
                pc = 6; continue;
            }
            4 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5185c0, ecx, &a); }
                fl = lift::sub(edi, eax, 32).1;
                if (fl.cf || fl.zf) { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x5185c0, ecx, &a); }
                edi = eax;
                pc = 6; continue;
            }
            6 => {
                eax = m.r32(0x6dfc14u32);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                m.w32(esp.wrapping_add(0x18u32), edi);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                ecx = m.r32(esi.wrapping_add(0x120u32));
                pc = 10; continue;
            }
            8 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x51a3a0, ecx, &a); }
                ecx = m.r32(0x66e0f8u32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x120u32));
                edi = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = ecx;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                if !fl.sf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 10; continue;
            }
            10 => {
                m.w32(esp.wrapping_add(0x14u32), ecx);
                ecx = esi.wrapping_add(0x160u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                edx = esp.wrapping_add(0x14u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3f8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(0x6dfc14u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(esi))) & 0xff);
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(eax, ecx, 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0x118u32));
                pc = 16; continue;
            }
            12 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x51a3a0, ecx, &a); }
                ecx = m.r32(0x66e0ecu32);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                eax = ecx;
                ecx = m.r32(esi.wrapping_add(0x118u32));
                edi = edx;
                edx = ((eax as i32) >> 31) as u32;
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::sar(eax, 0x1u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                eax = 0x51eb851fu32;
                let (r, f) = lift::shl(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::imul(edx, edi); fl = f;
                edx = r;
                { let r = i64::from(eax as i32) * i64::from(edx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x5u32, 32, fl); fl = f;
                edx = r;
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                let (r, f) = lift::sar(edx, 0x10u32, 32, fl); fl = f;
                edx = r;
                eax = edx.wrapping_add(edx.wrapping_mul(4));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                let (r, f) = lift::shl(edx, 0x3u32, 32, fl); fl = f;
                edx = r;
                let (r, f) = lift::sub(ecx, edx, 32); fl = f;
                ecx = r;
                if !fl.sf { pc = 14; continue; }
                pc = 13; continue;
            }
            13 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                pc = 16; continue;
            }
            14 => {
                eax = m.r32(esi.wrapping_add(0x10u32));
                fl = lift::sub(ecx, eax, 32).1;
                if (fl.zf || fl.sf != fl.of) { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                ecx = eax;
                pc = 16; continue;
            }
            16 => {
                m.w32(esp.wrapping_add(0x20u32), ecx);
                eax = m.r32(esi.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x1cu32), eax);
                ecx = m.r32(esi.wrapping_add(0x11cu32));
                m.w32(esp.wrapping_add(0x24u32), ecx);
                edx = (edx & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x42u32)))) & 0xffff);
                fl = lift::sub((edx & 0xffff), u32::from(m.r16(0x6dd892u32)), 16).1;
                ecx = 0xc8u32;
                eax = (eax & !0xff) | ((u32::from(fl.cf)) & 0xff);
                m.w8(esp.wrapping_add(0x2au32), ((eax & 0xff)) as u8);
                eax = m.r32(esi.wrapping_add(0x11cu32));
                edx = ((eax as i32) >> 31) as u32;
                { let d = i64::from(ecx as i32); let n = ((u64::from(edx) << 32) | u64::from(eax)) as i64; eax = (n / d) as u32; edx = (n % d) as u32; }
                fl = lift::logic(edx & edx, 32);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = ecx;
                ecx = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x24u32), ecx);
                pc = 18; continue;
            }
            18 => {
                edx = m.r32(esp.wrapping_add(0x3cu32));
                ecx = esp.wrapping_add(0x10u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = ebx;
                m.w16(esp.wrapping_add(0x30u32), ((ebp & 0xffff)) as u16);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4916e0, ecx, &a); }
                esp = esp.wrapping_add(8);
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x34u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
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
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048F2C0` ship_side_menu_draw_route_list
pub fn f_0048f2c0(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x660460u32);
                eax = m.r32(0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x18);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ebp = ecx;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x3cu32));
                eax = m.r32(ebp.wrapping_add(0x38f8u32));
                esi = ebp.wrapping_add(0x38f4u32);
                esp = esp.wrapping_add(0x4);
                edx = m.r32(ecx);
                edi = m.r32(ecx.wrapping_add(0x4u32));
                ecx = esi;
                eax = edx.wrapping_add(eax).wrapping_add(0x4u32);
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(ebp.wrapping_add(0x38fcu32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                m.w32(esp.wrapping_add(0x20u32), edi);
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = esi;
                ebx = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(0x6c7cccu32);
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                m.w32(esp.wrapping_add(0x18u32), ebx);
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = m.r32(ebp.wrapping_add(0xa0u32));
                esi = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                m.w32(esp.wrapping_add(0x30u32), edx);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if !fl.cf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = ecx & 0xffffu32; fl = lift::logic(r, 32);
                ecx = r;
                eax = ecx.wrapping_add(ecx.wrapping_mul(2));
                ecx = m.r32(0x6dd7a8u32);
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x10u32)))) & 0xffff);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, esi, 32); fl = f;
                eax = r;
                pc = 2; continue;
            }
            2 => {
                ebx = (ebx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x132u32)))) & 0xffff);
                ecx = 0x6dcd28u32;
                esi = ebx;
                m.w32(esp.wrapping_add(0x14u32), ebx);
                m.w32(esp.wrapping_add(0x38u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                edx = m.r32(0x6b04f4u32);
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(0x6a4510u32);
                edx = m.r32(ebp.wrapping_add(0x5260u32));
                esp = esp.wrapping_add(0x14);
                edi = ebp.wrapping_add(0x5260u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = edi;
                { let t = m.r32(edx.wrapping_add(0x70u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(edi);
                ecx = edi;
                { let t = m.r32(eax.wrapping_add(0x6cu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                let (r, f) = lift::sub(eax, 0x2u32, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                ecx = 0x6dcdd0u32;
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::sub((ebx & 0xffff), u32::from(m.r16(0x6dd72au32)), 16).1;
                if !fl.cf { pc = 14; continue; }
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6dd72cu32);
                edx = (edx & !0xff) | ((0x4u32) & 0xff);
                pc = 4; continue;
            }
            4 => {
                eax = esi;
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                edi = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(edi.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                fl = lift::logic(u32::from(m.r8(ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x3u32))) & (edx & 0xff), 8);
                eax = ecx.wrapping_add(eax.wrapping_mul(4));
                if !fl.zf { pc = 7; continue; }
                pc = 5; continue;
            }
            5 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (esi & 0xffff), 16).1;
                if !fl.zf { pc = 4; continue; }
                pc = 6; continue;
            }
            6 => {
                pc = 8; continue;
            }
            7 => {
                ebx = esi;
                pc = 8; continue;
            }
            8 => {
                ecx = m.r32(esp.wrapping_add(0x38u32));
                edx = esp.wrapping_add(0x10u32);
                edi = ecx.wrapping_add(0x1u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba3fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                fl = lift::sub(u32::from(m.r16(esp.wrapping_add(0x14u32))), (esi & 0xffff), 16).1;
                if !fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffff0000u32);
                pc = 11; continue;
            }
            10 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                pc = 11; continue;
            }
            11 => {
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x3cu32));
                ecx = m.r32(ebp.wrapping_add(0x57d8u32));
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_add(0x4);
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                fl = lift::sub(eax, 0x6u32, 32).1;
                ecx = (ecx & !0xff) | ((u32::from((fl.sf != fl.of))) & 0xff);
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                fl = lift::logic(eax & eax, 32);
                edx = (edx & !0xff) | ((u32::from((fl.sf == fl.of))) & 0xff);
                fl = lift::logic(edx & ecx, 32);
                if fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                ecx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::imul(eax, m.r32(esp.wrapping_add(0x18u32))); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x1cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::add(eax, m.r32(esp.wrapping_add(0x24u32)), 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 13; continue;
            }
            13 => {
                eax = esi;
                m.w32(esp.wrapping_add(0x38u32), edi);
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                ecx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = eax.wrapping_add(ecx.wrapping_mul(2));
                edx = eax.wrapping_add(eax.wrapping_mul(4));
                eax = m.r32(0x6dd72cu32);
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(edx.wrapping_mul(4))))) & 0xffff);
                fl = lift::sub((ebx & 0xffff), (esi & 0xffff), 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 14; continue;
            }
            14 => {
                ecx = esp.wrapping_add(0x10u32);
                m.w32(esp.wrapping_add(0x30u32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x28u32));
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x24);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048F510` ship_side_menu_draw_cargo_list
pub fn f_0048f510(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                m.w32(esp, 0x660472u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                m.w32(0x0u32, esp);
                esp = esp.wrapping_sub(0x44);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                eax = m.r32(esi.wrapping_add(0x4098u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 39; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(esi.wrapping_add(0x38bcu32));
                ecx = m.r32(esi.wrapping_add(0x4090u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                ebp = m.r32(esp.wrapping_add(0x60u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(ebp);
                ebx = m.r32(ebp.wrapping_add(0x4u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                let (r, f) = lift::sub(ebx, ecx, 32); fl = f;
                ebx = r;
                let (r, f) = lift::add(ebx, eax, 32); fl = f;
                ebx = r;
                { let a = []; eax = h.call_at(esp, m, 0x4bb770, ecx, &a); }
                fl = lift::logic(eax & eax, 32);
                m.w32(esp.wrapping_add(0x24u32), eax);
                if fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                eax = m.r32(esi.wrapping_add(0x38c0u32));
                ecx = m.r32(ebp.wrapping_add(0x4u32));
                edx = m.r32(ebp);
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                eax = m.r32(esi.wrapping_add(0x38bcu32));
                ebp = esi.wrapping_add(0x38b8u32);
                m.w32(esp.wrapping_add(0x48u32), ecx);
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                ecx = ebp;
                m.w32(esp.wrapping_add(0x44u32), edx);
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x44u32));
                ecx = ebp;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x4cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x3u32);
                let (r, f) = lift::add(eax, ecx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x54u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                eax = esp.wrapping_add(0x48u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb820, ecx, &a); }
                esp = esp.wrapping_add(0x8);
                pc = 3; continue;
            }
            3 => {
                ecx = m.r32(0x6c7cccu32);
                m.w32(esp.wrapping_add(0x14u32), ecx);
                eax = m.r32(esi.wrapping_add(0x3834u32));
                m.w32(esp.wrapping_add(0x5cu32), 0x0u32);
                ecx = m.r32(eax.wrapping_add(0x14u32));
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 6; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = m.r32(eax.wrapping_add(0x4u32));
                fl = lift::logic(ecx & ecx, 32);
                if fl.zf { pc = 6; continue; }
                pc = 5; continue;
            }
            5 => {
                edx = m.r32(eax.wrapping_add(0xcu32));
                eax = m.r32(edx);
                m.w32(esp.wrapping_add(0x34u32), eax);
                ecx = m.r32(edx.wrapping_add(0x4u32));
                m.w32(esp.wrapping_add(0x38u32), ecx);
                eax = m.r32(edx.wrapping_add(0x8u32));
                m.w32(esp.wrapping_add(0x3cu32), eax);
                ecx = m.r32(edx.wrapping_add(0xcu32));
                m.w32(esp.wrapping_add(0x40u32), ecx);
                pc = 6; continue;
            }
            6 => {
                ecx = m.r32(esi.wrapping_add(0x38e4u32));
                eax = ecx.wrapping_mul(8);
                let (r, f) = lift::sub(eax, ecx, 32); fl = f;
                eax = r;
                edx = eax.wrapping_add(eax.wrapping_mul(2));
                ecx = edx.wrapping_mul(8).wrapping_add(0x6dcd28u32);
                { let a = []; eax = h.call_at(esp, m, 0x4c1b90, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb8f0, ecx, &a); }
                eax = m.r32(0x6cc37cu32);
                ecx = esp.wrapping_add(0x2cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                eax = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4))]; eax = h.call_at(esp, m, 0x4bbb20, ecx, &a); }
                ebp = m.r32(esi.wrapping_add(0x4088u32));
                esp = esp.wrapping_add(0xc);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 7; continue;
            }
            7 => {
                fl = lift::sub(edx, ebp, 32).1;
                if fl.zf { pc = 12; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x698538u32)))) & 0xff);
                fl = lift::sub(m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x409cu32)), 0x0u32, 32).1;
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                pc = 10; continue;
            }
            10 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x18u32, 32).1;
                if (fl.sf != fl.of) { pc = 7; continue; }
                pc = 11; continue;
            }
            11 => {
                m.w32(esp.wrapping_add(0x10u32), edx);
                pc = 29; continue;
            }
            12 => {
                fl = lift::sub(eax, 0x18u32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf == fl.of) { pc = 29; continue; }
                pc = 13; continue;
            }
            13 => {
                eax = eax.wrapping_add(0x698538u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                pc = 14; continue;
            }
            14 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                m.w32(esp.wrapping_add(0x18u32), ecx);
                ebp = m.r32(esi.wrapping_add(ecx.wrapping_mul(4)).wrapping_add(0x409cu32));
                fl = lift::logic(ebp & ebp, 32);
                if fl.zf { pc = 26; continue; }
                pc = 15; continue;
            }
            15 => {
                let (r, f) = lift::add(edx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                edx = r;
                fl = lift::sub(eax, 0x69854cu32, 32).1;
                m.w32(esp.wrapping_add(0x10u32), edx);
                if (fl.sf == fl.of) { pc = 22; continue; }
                pc = 16; continue;
            }
            16 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ebp = esi.wrapping_add(0x38b8u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = (eax & !0xff) | ((u32::from(m.r8(edx.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 18; continue; }
                pc = 17; continue;
            }
            17 => {
                eax = m.r32(esi.wrapping_add(0x3834u32));
                pc = 19; continue;
            }
            18 => {
                eax = m.r32(esi.wrapping_add(0x3838u32));
                pc = 19; continue;
            }
            19 => {
                eax = m.r32(eax.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x44u32));
                eax = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(edi, eax, 32); fl = f;
                edi = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x34u32));
                esp = esp.wrapping_add(0x1c);
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax.wrapping_add(0x672c14u32)))) & 0xff);
                fl = lift::logic((ecx & 0xff) & (ecx & 0xff), 8);
                ecx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x409cu32));
                if fl.zf { pc = 21; continue; }
                pc = 20; continue;
            }
            20 => {
                eax = 0x51eb851fu32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x6u32, 32, fl); fl = f;
                edx = r;
                pc = 23; continue;
            }
            21 => {
                eax = 0x10624dd3u32;
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x7u32, 32, fl); fl = f;
                edx = r;
                ecx = edx;
                let (r, f) = lift::shr(ecx, 0x1fu32, 32, fl); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, ecx, 32); fl = f;
                edx = r;
                pc = 24; continue;
            }
            22 => {
                ebp = esi.wrapping_add(0x38b8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                let (r, f) = lift::sub(eax, edx, 32); fl = f;
                eax = r;
                edx = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = 0x66666667u32;
                ecx = m.r32(esi.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x409cu32));
                { let r = i64::from(eax as i32) * i64::from(ecx as i32); eax = r as u32; edx = (r >> 32) as u32; fl.cf = r != i64::from(eax as i32); fl.of = fl.cf; }
                let (r, f) = lift::sar(edx, 0x2u32, 32, fl); fl = f;
                edx = r;
                pc = 23; continue;
            }
            23 => {
                eax = edx;
                let (r, f) = lift::shr(eax, 0x1fu32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(edx, eax, 32); fl = f;
                edx = r;
                pc = 24; continue;
            }
            24 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                m.w32(esp.wrapping_add(0x20u32), edx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                edx = esp.wrapping_add(0x1cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba400u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x20);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = m.r32(esp.wrapping_add(0x3cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(ecx.wrapping_mul(4).wrapping_add(0x6c0bb0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                ecx = ebp;
                m.w32(esp.wrapping_add(0x20u32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                let (r, f) = lift::add(eax, edi, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                edi = eax.wrapping_add(ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ebp = esi.wrapping_add(0x38b8u32);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x28u32));
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                eax = m.r32(esp.wrapping_add(0x18u32));
                let (r, f) = lift::sub(edi, ecx, 32); fl = f;
                edi = r;
                edx = esp.wrapping_add(0x14u32);
                ecx = m.r32(esi.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x40fcu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba404u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(0x6cc37cu32);
                eax = m.r32(ecx.wrapping_add(0x4u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb9c0, ecx, &a); }
                edx = m.r32(esp.wrapping_add(0x4cu32));
                eax = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12)), m.r32(esp.wrapping_add(16)), m.r32(esp.wrapping_add(20))]; eax = h.call_at(esp, m, 0x4bb330, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x48u32));
                esp = esp.wrapping_add(0x38);
                fl = lift::sub(ecx, m.r32(esi.wrapping_add(0x408cu32)), 32).1;
                if fl.zf { pc = 28; continue; }
                pc = 25; continue;
            }
            25 => {
                edx = m.r32(esp.wrapping_add(0x64u32));
                eax = m.r32(esi.wrapping_add(0x38bcu32));
                ecx = ebp;
                edi = m.r32(edx);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x1cu32));
                edx = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = eax.wrapping_add(ecx);
                eax = m.r32(esp.wrapping_add(0x20u32));
                pc = 26; continue;
            }
            26 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                fl = lift::sub(eax, 0x698550u32, 32).1;
                m.w32(esp.wrapping_add(0x20u32), eax);
                if (fl.sf != fl.of) { pc = 14; continue; }
                pc = 27; continue;
            }
            27 => {
                pc = 29; continue;
            }
            28 => {
                edx = m.r32(esp.wrapping_add(0x10u32));
                pc = 29; continue;
            }
            29 => {
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0x408cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 38; continue; }
                pc = 30; continue;
            }
            30 => {
                eax = m.r32(esi.wrapping_add(0x4160u32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 32; continue; }
                pc = 31; continue;
            }
            31 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x38bcu32));
                esp = esp.wrapping_add(0x4);
                ebp = esi.wrapping_add(0x38b8u32);
                m.w32(esp.wrapping_add(0x20u32), eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                ecx = m.r32(esp.wrapping_add(0x64u32));
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x20u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                let (r, f) = lift::add(edi, m.r32(ecx), 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x4160u32));
                eax = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba408u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                ecx = m.r32(esp.wrapping_add(0x24u32));
                eax = m.r32(esp.wrapping_add(0x4cu32));
                edx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                let (r, f) = lift::sub(edx, eax, 32); fl = f;
                edx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x4160u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::sub(ecx, 0x1u32, 32).1;
                eax = (eax & !0xff) | ((u32::from(!fl.zf)) & 0xff);
                ecx = m.r32(eax.wrapping_mul(4).wrapping_add(0x699c8cu32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x10);
                ecx = ebp;
                { let a = []; eax = h.call_at(esp, m, 0x40abf0, ecx, &a); }
                ecx = ebp;
                edi = eax;
                { let a = []; eax = h.call_at(esp, m, 0x40abe0, ecx, &a); }
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = eax.wrapping_add(edi);
                eax = m.r32(esp.wrapping_add(0x10u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esp.wrapping_add(0x10u32), eax);
                edx = eax;
                pc = 32; continue;
            }
            32 => {
                fl = lift::sub(edx, m.r32(esi.wrapping_add(0x408cu32)), 32).1;
                if (fl.sf == fl.of) { pc = 38; continue; }
                pc = 33; continue;
            }
            33 => {
                eax = m.r32(esi.wrapping_add(0x415cu32));
                fl = lift::logic(eax & eax, 32);
                if fl.zf { pc = 38; continue; }
                pc = 34; continue;
            }
            34 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x2u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x38bcu32));
                esp = esp.wrapping_add(0x4);
                ebp = esi.wrapping_add(0x38b8u32);
                m.w32(esp.wrapping_add(0x20u32), edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = ebp;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a940, ecx, &a); }
                esp = esp.wrapping_add(4);
                edi = eax;
                eax = m.r32(esp.wrapping_add(0x64u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xff000000u32);
                ecx = m.r32(eax);
                eax = m.r32(esp.wrapping_add(0x24u32));
                let (r, f) = lift::add(edi, ecx, 32); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb870, ecx, &a); }
                ecx = m.r32(esi.wrapping_add(0x415cu32));
                edx = esp.wrapping_add(0x18u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x6ba40cu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x64df80, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x24u32));
                ecx = edi;
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                eax = m.r32(esp.wrapping_add(0x50u32));
                let (r, f) = lift::sub(ecx, eax, 32); fl = f;
                ecx = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0x1c);
                ecx = ebp;
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x40a960, ecx, &a); }
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x1u32);
                let (r, f) = lift::add(edi, eax, 32); fl = f;
                edi = r;
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bba10, ecx, &a); }
                eax = m.r32(esi.wrapping_add(0x415cu32));
                esp = esp.wrapping_add(0x4);
                fl = lift::sub(eax, 0x1u32, 32).1;
                if fl.zf { pc = 36; continue; }
                pc = 35; continue;
            }
            35 => {
                edx = m.r32(0x6b2c90u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                pc = 37; continue;
            }
            36 => {
                eax = m.r32(0x69c9e8u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                pc = 37; continue;
            }
            37 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))]; eax = h.call_at(esp, m, 0x4bb3e0, ecx, &a); }
                esp = esp.wrapping_add(0xc);
                pc = 38; continue;
            }
            38 => {
                ecx = m.r32(esp.wrapping_add(0x24u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4bb760, ecx, &a); }
                esp = esp.wrapping_add(0x4);
                ecx = esp.wrapping_add(0x14u32);
                m.w32(esp.wrapping_add(0x5cu32), 0xffffffffu32);
                { let a = []; eax = h.call_at(esp, m, 0x64f253, ecx, &a); }
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                pc = 39; continue;
            }
            39 => {
                ecx = m.r32(esp.wrapping_add(0x48u32));
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                m.w32(0x0u32, ecx);
                esp = esp.wrapping_add(0x50);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048FA90` ship_side_menu_refresh_cargo_cache
pub fn f_0048fa90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(0x6dd7a4u32);
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                eax = (eax & !0xffff) | ((u32::from(m.r16(ecx))) & 0xffff);
                edi = eax.wrapping_add(eax.wrapping_mul(2));
                eax = m.r32(esi.wrapping_add(0x4098u32));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                fl = lift::sub(eax, 0xffffffffu32, 32).1;
                if fl.zf { pc = 2; continue; }
                pc = 1; continue;
            }
            1 => {
                edx = (edx & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x3cu32)))) & 0xff);
                let r = (edx & 0xff) & 0x4u32; fl = lift::logic(r, 8);
                edx = (edx & !0xff) | ((r) & 0xff);
                fl = lift::sub((edx & 0xff), 0x4u32, 8).1;
                if !fl.zf { pc = 13; continue; }
                pc = 2; continue;
            }
            2 => {
                m.w32(esp.wrapping_add(0x8u32), eax);
                eax = esi.wrapping_add(0x40fcu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = edi;
                m.w32(esi.wrapping_add(0x4098u32), 0x0u32);
                m.w32(esi.wrapping_add(0x415cu32), 0x0u32);
                m.w32(esi.wrapping_add(0x4160u32), 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x5187f0, ecx, &a); }
                esp = esp.wrapping_add(4);
                edx = esi.wrapping_add(0x409cu32);
                ecx = edi.wrapping_add(0x54u32);
                ebx = 0x18u32;
                pc = 3; continue;
            }
            3 => {
                eax = m.r32(ecx);
                fl = lift::logic(eax & eax, 32);
                m.w32(edx, eax);
                if fl.zf { pc = 5; continue; }
                pc = 4; continue;
            }
            4 => {
                let (r, f) = lift::add(m.r32(esi.wrapping_add(0x4098u32)), 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                m.w32(esi.wrapping_add(0x4098u32), r);
                pc = 5; continue;
            }
            5 => {
                let (r, f) = lift::add(ecx, 0x4u32, 32); fl = f;
                ecx = r;
                let (r, f) = lift::add(edx, 0x4u32, 32); fl = f;
                edx = r;
                let (r, f) = lift::sub(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                if !fl.zf { pc = 3; continue; }
                pc = 6; continue;
            }
            6 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x157u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 8; continue; }
                pc = 7; continue;
            }
            7 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x415cu32), eax);
                eax = m.r32(esi.wrapping_add(0x4098u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x4098u32), eax);
                pc = 8; continue;
            }
            8 => {
                eax = (eax & !0xff) | ((u32::from(m.r8(edi.wrapping_add(0x156u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                if fl.zf { pc = 10; continue; }
                pc = 9; continue;
            }
            9 => {
                let r = eax & 0xffu32; fl = lift::logic(r, 32);
                eax = r;
                m.w32(esi.wrapping_add(0x4160u32), eax);
                eax = m.r32(esi.wrapping_add(0x4098u32));
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                m.w32(esi.wrapping_add(0x4098u32), eax);
                pc = 10; continue;
            }
            10 => {
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                edi = esi.wrapping_add(0x38b8u32);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x18u32));
                ecx = edi;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ebx = m.r32(esi.wrapping_add(0x38c0u32));
                m.w32(esp.wrapping_add(0x14u32), eax);
                { let a = []; eax = h.call_at(esp, m, 0x40abd0, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0x18u32));
                ecx = m.r32(esi.wrapping_add(0x38bcu32));
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                eax = m.r32(esi.wrapping_add(0x14u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esp.wrapping_add(0x10u32), ecx);
                ecx = edi;
                { let a = []; eax = h.call_at(esp, m, 0x40abc0, ecx, &a); }
                ebx = m.r32(esp.wrapping_add(0x10u32));
                edx = esp.wrapping_add(0x10u32);
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                ecx = 0x6da5f0u32;
                m.w32(esp.wrapping_add(0x1cu32), eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x4098u32));
                ecx = m.r32(esp.wrapping_add(0xcu32));
                fl = lift::sub(ecx, eax, 32).1;
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 13; continue; }
                pc = 11; continue;
            }
            11 => {
                ecx = m.r32(esi.wrapping_add(0xccu32));
                edx = m.r32(esi.wrapping_add(0xd0u32));
                fl = lift::sub(ecx, edx, 32).1;
                if !fl.zf { pc = 13; continue; }
                pc = 12; continue;
            }
            12 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                ecx = esi.wrapping_add(0x3ce4u32);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 13; continue;
            }
            13 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0x14);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

/// `0X0048FC00` ship_side_menu_can_group_selected_ships
pub fn f_0048fc00(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                edx = m.r32(ecx.wrapping_add(0xa0u32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ebx = m.r32(0x6dd7a4u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                edi = m.r32(ecx.wrapping_add(0xacu32));
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let r = ebp ^ ebp; fl = lift::logic(r, 32);
                ebp = r;
                eax = (eax & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(ebx).wrapping_add(0x39u32)))) & 0xffff);
                fl = lift::logic(edi & edi, 32);
                m.w16(esp.wrapping_add(0x10u32), ((eax & 0xffff)) as u16);
                if (fl.zf || fl.sf != fl.of) { pc = 7; continue; }
                pc = 1; continue;
            }
            1 => {
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(edx))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, ebx, 32); fl = f;
                eax = r;
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x8u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(0x6dd896u32)), 16).1;
                if fl.cf { pc = 8; continue; }
                pc = 2; continue;
            }
            2 => {
                esi = (esi & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x134u32)))) & 0xffff);
                fl = lift::logic((esi & 0xffff) & (esi & 0xffff), 16);
                if fl.zf { pc = 4; continue; }
                pc = 3; continue;
            }
            3 => {
                fl = lift::sub((esi & 0xffff), 0x1u32, 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 4; continue;
            }
            4 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r8(eax.wrapping_add(0x39u32)))) & 0xffff);
                fl = lift::sub((ecx & 0xffff), u32::from(m.r16(esp.wrapping_add(0x10u32))), 16).1;
                if !fl.zf { pc = 8; continue; }
                pc = 5; continue;
            }
            5 => {
                let r = ecx ^ ecx; fl = lift::logic(r, 32);
                ecx = r;
                ecx = (ecx & !0xff) | ((u32::from(m.r8(eax))) & 0xff);
                eax = m.r32(0x6dfc14u32);
                fl = lift::sub(ecx, eax, 32).1;
                if !fl.zf { pc = 8; continue; }
                pc = 6; continue;
            }
            6 => {
                let (r, f) = lift::add(ebp, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebp = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(ebp, edi, 32).1;
                if (fl.sf != fl.of) { pc = 1; continue; }
                pc = 7; continue;
            }
            7 => {
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
            8 => {
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
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

/// `0X0048FD90` ship_side_menu_invalidate_list_area
pub fn f_0048fd90(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
    let (mut eax, mut edx, mut ebx, mut ebp, mut esi, mut edi) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut ecx, mut esp) = (ecx_in, esp_in);
    let mut fl = lift::Fl::default();
    let mut fp = lift::Fpu::default();
    let _ = (&mut eax, &mut edx, &mut ebx, &mut ebp, &mut esi, &mut edi, &mut ecx, &mut esp, &mut fl, &mut fp);
    esp = esp.wrapping_sub(0x10);
    esp = esp.wrapping_sub(4);
    m.w32(esp, ebx);
    esp = esp.wrapping_sub(4);
    m.w32(esp, esi);
    esi = ecx;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edi);
    ebx = m.r32(esi.wrapping_add(0x3884u32));
    edi = m.r32(esi.wrapping_add(0x18u32));
    ecx = esi.wrapping_add(0x387cu32);
    { let a = []; eax = h.call_at(esp, m, 0x40ac00, ecx, &a); }
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    ecx = m.r32(esi.wrapping_add(0x2cu32));
    let (r, f) = lift::add(eax, ebx, 32); fl = f;
    eax = r;
    edx = esp.wrapping_add(0xcu32);
    m.w32(esp.wrapping_add(0x10u32), eax);
    eax = m.r32(esi.wrapping_add(0x30u32));
    let (r, f) = lift::add(eax, edi, 32); fl = f;
    eax = r;
    esp = esp.wrapping_sub(4);
    m.w32(esp, edx);
    m.w32(esp.wrapping_add(0x1cu32), eax);
    eax = m.r32(esi.wrapping_add(0x14u32));
    let (r, f) = lift::add(ecx, eax, 32); fl = f;
    ecx = r;
    m.w32(esp.wrapping_add(0x10u32), eax);
    m.w32(esp.wrapping_add(0x18u32), ecx);
    ecx = 0x6da5f0u32;
    { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4b9650, ecx, &a); }
    esp = esp.wrapping_add(4);
    edi = m.r32(esp);
    esp = esp.wrapping_add(4);
    esi = m.r32(esp);
    esp = esp.wrapping_add(4);
    ebx = m.r32(esp);
    esp = esp.wrapping_add(4);
    esp = esp.wrapping_add(0x10);
    lift::set_fp(fp.depth(), fp.st(0)); return eax;
    eax
}

/// `0X0048FE80` ship_side_menu_rebuild_convoy_ship_list
pub fn f_0048fe80(m: &mut dyn Mem, h: &mut dyn Host, ecx_in: u32, esp_in: u32) -> u32 {
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
                esp = esp.wrapping_sub(4);
                m.w32(esp, esi);
                esi = ecx;
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                ecx = m.r32(0x6dd7a8u32);
                eax = (eax & !0xffff) | ((u32::from(m.r16(esi.wrapping_add(0x5228u32)))) & 0xffff);
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                eax = eax.wrapping_add(eax.wrapping_mul(4));
                ebp = ecx.wrapping_add(eax.wrapping_mul(4));
                eax = (eax & !0xff) | ((u32::from(m.r8(esp.wrapping_add(0x18u32)))) & 0xff);
                fl = lift::logic((eax & 0xff) & (eax & 0xff), 8);
                m.w32(esp.wrapping_add(0xcu32), ebp);
                if !fl.zf { pc = 3; continue; }
                pc = 1; continue;
            }
            1 => {
                eax = m.r32(ebp.wrapping_add(0x18u32));
                fl = lift::logic(eax & eax, 32);
                if !fl.zf { pc = 3; continue; }
                pc = 2; continue;
            }
            2 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            3 => {
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                let r = ebx ^ ebx; fl = lift::logic(r, 32);
                ebx = r;
                m.w32(esi.wrapping_add(0x5244u32), ebx);
                eax = (eax & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0xau32)))) & 0xffff);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd894u32))) & 0xffff);
                edx = (edx & !0xffff) | ((u32::from(m.r16(ebp.wrapping_add(0x10u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                m.w32(esp.wrapping_add(0x20u32), ebx);
                m.w16(esp.wrapping_add(0x18u32), ((edx & 0xffff)) as u16);
                m.w32(esp.wrapping_add(0x10u32), eax);
                if !fl.cf { pc = 14; continue; }
                pc = 4; continue;
            }
            4 => {
                edx = m.r32(0x6dd7a4u32);
                pc = 5; continue;
            }
            5 => {
                edi = eax;
                let r = edi & 0xffffu32; fl = lift::logic(r, 32);
                edi = r;
                edi = edi.wrapping_add(edi.wrapping_mul(2));
                let (r, f) = lift::shl(edi, 0x7u32, 32, fl); fl = f;
                edi = r;
                let (r, f) = lift::add(edi, edx, 32); fl = f;
                edi = r;
                fl = lift::sub(u32::from(m.r16(edi.wrapping_add(0x134u32))), 0x17u32, 16).1;
                if !fl.cf { pc = 13; continue; }
                pc = 6; continue;
            }
            6 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0xacu32)), 32).1;
                if (fl.sf != fl.of) { pc = 10; continue; }
                pc = 7; continue;
            }
            7 => {
                eax = (eax & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if !fl.cf { pc = 9; continue; }
                pc = 8; continue;
            }
            8 => {
                let r = eax & 0xffffu32; fl = lift::logic(r, 32);
                eax = r;
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                eax = eax.wrapping_add(eax.wrapping_mul(2));
                let (r, f) = lift::shl(eax, 0x7u32, 32, fl); fl = f;
                eax = r;
                let (r, f) = lift::add(eax, edx, 32); fl = f;
                eax = r;
                eax = (eax & !0xffff) | ((u32::from(m.r16(eax.wrapping_add(0x6u32)))) & 0xffff);
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if fl.cf { pc = 8; continue; }
                pc = 9; continue;
            }
            9 => {
                eax = ebx.wrapping_add(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                ebp = eax;
                eax = ebx.wrapping_add(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0xffffffffu32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, v, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)) as u8, m.r32(esp.wrapping_add(8))); crate::j07::memset(m, d, v, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esp.wrapping_add(0x30u32));
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ecx = eax.wrapping_add(eax);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                eax = m.r32(esp.wrapping_add(0x30u32));
                m.w32(esi.wrapping_add(0xa0u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x34u32));
                m.w32(esi.wrapping_add(0xacu32), ebx);
                ebx = m.r32(esp.wrapping_add(0x40u32));
                esp = esp.wrapping_add(0x20);
                pc = 10; continue;
            }
            10 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                let r = edx ^ edx; fl = lift::logic(r, 32);
                edx = r;
                m.w16(ecx.wrapping_add(ebx.wrapping_mul(2)), ((eax & 0xffff)) as u16);
                edx = (edx & !0xff) | ((u32::from(m.r8(edi))) & 0xff);
                fl = lift::sub(edx, m.r32(0x6dfc14u32), 32).1;
                if !fl.zf { pc = 12; continue; }
                pc = 11; continue;
            }
            11 => {
                eax = m.r32(edi.wrapping_add(0x10u32));
                ecx = m.r32(esi.wrapping_add(0x5244u32));
                let (r, f) = lift::add(ecx, eax, 32); fl = f;
                ecx = r;
                m.w32(esi.wrapping_add(0x5244u32), ecx);
                pc = 12; continue;
            }
            12 => {
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(edi.wrapping_add(0x6u32)))) & 0xffff);
                edx = m.r32(0x6dd7a4u32);
                let (r, f) = lift::add(ebx, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                ebx = r;
                m.w16(esp.wrapping_add(0x10u32), ((ecx & 0xffff)) as u16);
                ecx = (ecx & !0xffff) | ((u32::from(m.r16(0x6dd894u32))) & 0xffff);
                eax = m.r32(esp.wrapping_add(0x10u32));
                m.w32(esp.wrapping_add(0x20u32), ebx);
                pc = 13; continue;
            }
            13 => {
                fl = lift::sub((eax & 0xffff), (ecx & 0xffff), 16).1;
                if fl.cf { pc = 5; continue; }
                pc = 14; continue;
            }
            14 => {
                fl = lift::sub(ebx, m.r32(esi.wrapping_add(0xacu32)), 32).1;
                if fl.zf { pc = 16; continue; }
                pc = 15; continue;
            }
            15 => {
                edi = ebx.wrapping_add(ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7b9, ecx, &a); }
                edx = m.r32(esi.wrapping_add(0xa0u32));
                ebp = eax;
                esp = esp.wrapping_sub(4);
                m.w32(esp, edi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, edx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebp);
                { let (d, sr, n) = (m.r32(esp), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8))); crate::j07::memcpy(m, d, sr, n); eax = d; ecx = u32::MAX; }
                eax = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, eax);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x64f7e2, ecx, &a); }
                m.w32(esi.wrapping_add(0xa0u32), ebp);
                ebp = m.r32(esp.wrapping_add(0x28u32));
                esp = esp.wrapping_add(0x14);
                pc = 16; continue;
            }
            16 => {
                ecx = m.r32(esi.wrapping_add(0xa0u32));
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ecx);
                ecx = m.r32(0x6cc950u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let a = [m.r32(esp.wrapping_add(0)), m.r32(esp.wrapping_add(4)), m.r32(esp.wrapping_add(8)), m.r32(esp.wrapping_add(12))]; eax = h.call_at(esp, m, 0x477cf0, ecx, &a); }
                esp = esp.wrapping_add(16);
                ecx = m.r32(esi.wrapping_add(0xacu32));
                let r = eax ^ eax; fl = lift::logic(r, 32);
                eax = r;
                fl = lift::logic(ecx & ecx, 32);
                if (fl.zf || fl.sf != fl.of) { pc = 22; continue; }
                pc = 17; continue;
            }
            17 => {
                edx = m.r32(esi.wrapping_add(0xa0u32));
                edi = m.r32(esp.wrapping_add(0x18u32));
                pc = 18; continue;
            }
            18 => {
                fl = lift::sub(u32::from(m.r16(edx)), (edi & 0xffff), 16).1;
                if fl.zf { pc = 21; continue; }
                pc = 19; continue;
            }
            19 => {
                let (r, f) = lift::add(eax, 1, 32); fl = lift::Fl { cf: fl.cf, ..f };
                eax = r;
                let (r, f) = lift::add(edx, 0x2u32, 32); fl = f;
                edx = r;
                fl = lift::sub(eax, ecx, 32).1;
                if (fl.sf != fl.of) { pc = 18; continue; }
                pc = 20; continue;
            }
            20 => {
                pc = 22; continue;
            }
            21 => {
                m.w32(esi.wrapping_add(0x5238u32), eax);
                pc = 22; continue;
            }
            22 => {
                ecx = esi;
                { let a = []; eax = h.call_at(esp, m, 0x441710, ecx, &a); }
                esp = esp.wrapping_sub(4);
                m.w32(esp, ebx);
                ecx = esi.wrapping_add(0x3930u32);
                m.w32(esi.wrapping_add(0xacu32), ebx);
                { let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, 0x4311b0, ecx, &a); }
                esp = esp.wrapping_add(4);
                eax = m.r32(esi.wrapping_add(0x94u32));
                ecx = m.r32(0x66d4fcu32);
                fl = lift::sub(eax, ecx, 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 23; continue;
            }
            23 => {
                fl = lift::sub(eax, m.r32(0x66d4f8u32), 32).1;
                if fl.zf { pc = 25; continue; }
                pc = 24; continue;
            }
            24 => {
                edx = m.r32(esi.wrapping_add(0xccu32));
                eax = m.r32(esi.wrapping_add(0xdcu32));
                fl = lift::sub(edx, eax, 32).1;
                if fl.zf { pc = 26; continue; }
                pc = 25; continue;
            }
            25 => {
                eax = m.r32(esi.wrapping_add(0x3938u32));
                ecx = esi.wrapping_add(0x3938u32);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                { let t = m.r32(eax.wrapping_add(0xccu32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                pc = 26; continue;
            }
            26 => {
                fl = lift::logic(ebx & ebx, 32);
                edi = m.r32(esp);
                esp = esp.wrapping_add(4);
                ebx = m.r32(esp);
                esp = esp.wrapping_add(4);
                if fl.zf { pc = 29; continue; }
                pc = 27; continue;
            }
            27 => {
                fl = lift::sub(u32::from(m.r16(ebp.wrapping_add(0x12u32))), 0x13u32, 16).1;
                if fl.zf { pc = 29; continue; }
                pc = 28; continue;
            }
            28 => {
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                eax = (eax & !0xff) | ((0x1u32) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            29 => {
                edx = m.r32(esi);
                esp = esp.wrapping_sub(4);
                m.w32(esp, 0x0u32);
                ecx = esi;
                { let t = m.r32(edx.wrapping_add(0x134u32)); let a = [m.r32(esp.wrapping_add(0))]; eax = h.call_at(esp, m, t, ecx, &a); }
                esp = esp.wrapping_add(4);
                esi = m.r32(esp);
                esp = esp.wrapping_add(4);
                let r = (eax & 0xff) ^ (eax & 0xff); fl = lift::logic(r, 8);
                eax = (eax & !0xff) | ((r) & 0xff);
                ebp = m.r32(esp);
                esp = esp.wrapping_add(4);
                esp = esp.wrapping_add(0xc);
                lift::set_fp(fp.depth(), fp.st(0)); return eax;
            }
            _ => unreachable!(),
        }
    }
}

pub fn run(f: u32, m: &mut dyn Mem, h: &mut dyn Host, ecx: u32, _a: &[u32], esp: u32) -> Option<Ret> {
    Some(Ret::U32(match f {
        0x486d20 => f_00486d20(m, h, ecx, esp),
        0x4870e0 => f_004870e0(m, h, ecx, esp),
        0x4874a0 => f_004874a0(m, h, ecx, esp),
        0x487c30 => f_00487c30(m, h, ecx, esp),
        0x488080 => f_00488080(m, h, ecx, esp),
        0x488440 => f_00488440(m, h, ecx, esp),
        0x488460 => f_00488460(m, h, ecx, esp),
        0x488540 => f_00488540(m, h, ecx, esp),
        0x4885a0 => f_004885a0(m, h, ecx, esp),
        0x488600 => f_00488600(m, h, ecx, esp),
        0x489b80 => f_00489b80(m, h, ecx, esp),
        0x48ac30 => f_0048ac30(m, h, ecx, esp),
        0x48ada0 => f_0048ada0(m, h, ecx, esp),
        0x48b060 => f_0048b060(m, h, ecx, esp),
        0x48b3e0 => f_0048b3e0(m, h, ecx, esp),
        0x48cf50 => f_0048cf50(m, h, ecx, esp),
        0x48d230 => f_0048d230(m, h, ecx, esp),
        0x48d4d0 => f_0048d4d0(m, h, ecx, esp),
        0x48d8c0 => f_0048d8c0(m, h, ecx, esp),
        0x48db50 => f_0048db50(m, h, ecx, esp),
        0x48dd70 => f_0048dd70(m, h, ecx, esp),
        0x48df20 => f_0048df20(m, h, ecx, esp),
        0x48df50 => f_0048df50(m, h, ecx, esp),
        0x48df90 => f_0048df90(m, h, ecx, esp),
        0x48e700 => f_0048e700(m, h, ecx, esp),
        0x48ed80 => f_0048ed80(m, h, ecx, esp),
        0x48ee90 => f_0048ee90(m, h, ecx, esp),
        0x48f2c0 => f_0048f2c0(m, h, ecx, esp),
        0x48f510 => f_0048f510(m, h, ecx, esp),
        0x48fa90 => f_0048fa90(m, h, ecx, esp),
        0x48fc00 => f_0048fc00(m, h, ecx, esp),
        0x48fd90 => f_0048fd90(m, h, ecx, esp),
        0x48fe80 => f_0048fe80(m, h, ecx, esp),
        _ => return None,
    }))
}
