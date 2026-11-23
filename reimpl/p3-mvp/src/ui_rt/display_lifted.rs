//! Generado por `reimpl/tools/ui_rt_display.py` con `tools/lift.py --nover` (no editar a mano): las dos funciones que colocan
//! la pantalla de juego según la resolución, con los argumentos reales de sus llamadas virtuales.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static DISPLAY: [(u32, super::core_table::CoreFn); 2] = [
    (0x429a40, f_00429a40),
    (0x429fe0, f_00429fe0),
];
/// `game_screen_apply_resolution_layout` (`0x00429a40`).
pub fn f_00429a40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x429a40;
    loop {
        match pc {
            0x429a40 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x9b8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xa90));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xd18));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xdf0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x429a99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0xc40));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x429aa7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x808));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x429ab5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.push(m, 0x429ac0);
            { let args = []; c.r[0] = host.call(m, 0x401160, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            if c.cond(4) { pc = 0x429c32; continue; }
            pc = 0x429ad0; continue;
            }
            0x429ad0 => {
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(4) { pc = 0x429b8a; continue; }
            pc = 0x429adb; continue;
            }
            0x429adb => {
            { let a = c.get(0,32); c.sub(a,0x500u32,32); }
            if c.cond(5) { pc = 0x429ce5; continue; }
            pc = 0x429ae6; continue;
            }
            0x429ae6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa90),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429af9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429aff); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429b08); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b14); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429b1a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429b30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429b38); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429b57); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0xa90));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429b67);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0xdf0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429b77);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696f28u32);
            pc = 0x429cc5; continue;
            }
            0x429b8a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9b8),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b9d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429ba3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429bac); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429bb8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429bbe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429bd4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429be4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429bed); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429bff); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0x9b8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429c0f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0xd18));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429c1f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696f10u32);
            pc = 0x429cc5; continue;
            }
            0x429c32 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429c4b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429c54); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c60); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429c66); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429c7c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429c8c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c95); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429ca7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0x8e0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429cb7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696ef8u32);
            pc = 0x429cc5; continue;
            }
            0x429cc5 => {
            c.push(m, 0x429cca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x429cd5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x429ce5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x429ce5; continue;
            }
            0x429ce5 => {
            c.push(m,c.get(5,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429cef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429cf9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429d03);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429d1b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429d2e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429d38); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc640),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc644),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x429d51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m, 0x429d5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429d76); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429d89); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429d93); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x429d9e);
            { let args = []; c.r[0] = host.call(m, 0x44f640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e51c0u32,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429dae); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429db9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429dc2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429dce); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x429de5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x808),32));
            c.set(1,32,c.r[6].wrapping_add(0x808));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429df6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429e03); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429e0c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x429e18); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x808),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x808));
            c.set(3,32,c.r[6].wrapping_add(0x1078));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x429e35); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x429e45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429e4b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xfa0),32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x429e5a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429e65); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfa0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xfa0));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429e7b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x429e8c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429e9f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xfa0));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429eab); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1150),32));
            c.set(3,32,c.r[6].wrapping_add(0xec8));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x429ec4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429ecf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.r[6].wrapping_add(0x1078));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x429ee3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x1150));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429eed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x429efe); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x429f0e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429f18); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(3,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429f29); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429f34); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429f3d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429f4a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x429f50);
            { let args = []; c.r[0] = host.call(m, 0x441750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2640),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x2640));
            c.set(1,32,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x12u32);
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429f6d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429f73); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffeeu32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x429f88); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x429f9c; continue; }
            pc = 0x429f92; continue;
            }
            0x429f92 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x429f9c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x429f9c; continue;
            }
            0x429f9c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429fa3);
            { let args = []; c.r[0] = host.call(m, 0x429fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x429fcb; continue; }
            pc = 0x429fad; continue;
            }
            0x429fad => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x429fb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x429fcb; continue; }
            pc = 0x429fb9; continue;
            }
            0x429fb9 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x41e860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x429fcb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
/// `reposition_ui_elements_top_bar_patch` (`0x00429fe0`).
pub fn f_00429fe0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x429fe0;
    loop {
        match pc {
            0x429fe0 => {
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x42a031; continue; }
            pc = 0x429ff5; continue;
            }
            0x429ff5 => {
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(4) { pc = 0x42a01a; continue; }
            pc = 0x429ffc; continue;
            }
            0x429ffc => {
            { let a = c.get(0,32); c.sub(a,0x500u32,32); }
            if c.cond(5) { pc = 0x42a048; continue; }
            pc = 0x42a003; continue;
            }
            0x42a003 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa90),32));
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a012); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x42a04c; continue;
            }
            0x42a01a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a029); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x42a04c; continue;
            }
            0x42a031 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a040); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x42a04c; continue;
            }
            0x42a048 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x42a04c; continue;
            }
            0x42a04c => {
            c.set(0,32,0x2aaaaaabu32);
            c.set(7,32,c.r[6].wrapping_add(0x1268));
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,c.get(2,32));
            c.set(3,32,c.r[6].wrapping_add(0x1350));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a071); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1]));
            c.set(1,32,c.get(3,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a083); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            c.set(1,32,c.r[6].wrapping_add(0x1520));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a094); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1438));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a0a1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x42a0c5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a0d2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42a0e4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1438),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a0f5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a100); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x1438));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42a11f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,c.r[6].wrapping_add(0x1520));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a13a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a143); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1350),32));
            c.set(1,32,c.r[6].wrapping_add(0x1350));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a154); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x42a171); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x808),32));
            c.set(1,32,c.r[6].wrapping_add(0x808));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x42a188); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1230),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a198); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1238),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a1a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc40),32));
            c.set(1,32,c.r[6].wrapping_add(0xc40));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x42a1bf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b78),32));
            c.set(1,32,c.r[6].wrapping_add(0x1b78));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a1d3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1e30),32));
            c.set(1,32,c.r[6].wrapping_add(0x1e30));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x42a1eb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            if c.cond(5) { pc = 0x42a1fb; continue; }
            pc = 0x42a1f7; continue;
            }
            0x42a1f7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42a209; continue;
            }
            0x42a1fb => {
            { let a = c.get(0,32); let r = c.sub(a,0x400u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0xf0u32,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1eu32,32); c.set(0,32,r); }
            pc = 0x42a209; continue;
            }
            0x42a209 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d48),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d48));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x42a21d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(7,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x42a236); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a23d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x19a8));
            c.set(3,32,c.get(0,32));
            c.set(5,32,c.r[6].wrapping_add(0x16f0));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a250); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1608),32));
            c.set(1,32,c.r[6].wrapping_add(0x1608));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a261); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x18c0));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a26e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a278); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1a90));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a285); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c60),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c60));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a296); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,0x92492493u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(3,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1c60));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(3,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42a2c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c60),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c60));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a2d6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1608),32));
            c.set(1,32,c.r[6].wrapping_add(0x1608));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a2f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17d8),32));
            c.set(1,32,c.r[6].wrapping_add(0x17d8));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a30a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1608),32));
            c.set(1,32,c.r[6].wrapping_add(0x1608));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a319); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18c0),32));
            c.set(1,32,c.r[6].wrapping_add(0x18c0));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42a339); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2528),32));
            c.set(1,32,c.r[6].wrapping_add(0x2528));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42a34d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18c0),32));
            c.set(1,32,c.r[6].wrapping_add(0x18c0));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a35c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x19a8),32));
            c.set(1,32,c.r[6].wrapping_add(0x19a8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a37c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x19a8),32));
            c.set(1,32,c.r[6].wrapping_add(0x19a8));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a38b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1a90));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42a3a7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a90),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a90));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a3b6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a3cf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a3d7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x42a3f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a3f7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0x1f18));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a406); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2000),32));
            c.set(1,32,c.r[6].wrapping_add(0x2000));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a417); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x20e8));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a424); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x21d0),32));
            c.set(1,32,c.r[6].wrapping_add(0x21d0));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a435); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x22b8));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a442); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a44c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,0x92492493u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(5,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42a479); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a480); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2000),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x2000));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a4a2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a4a9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x20e8),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x20e8));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a4cb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a4d2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x21d0),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x21d0));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a4f4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a4fb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x22b8));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a517); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23a0),32));
            { let a = c.get(6,32); let r = c.add(a,0x23a0u32,32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x42a52d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
