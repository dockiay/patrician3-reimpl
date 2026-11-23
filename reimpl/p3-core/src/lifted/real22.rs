//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn render_window_title(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420c70;
    loop {
        match pc {
            0x420c70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c65au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x791eu32);
            c.set(1,32,0x6da820u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.push(m, 0x420ca3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x420cc1; continue; }
            pc = 0x420caa; continue;
            }
            0x420caa => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x420cc1; continue; }
            pc = 0x420cb1; continue;
            }
            0x420cb1 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[0],32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x420cd5; continue;
            }
            0x420cc1 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x420cd5; continue;
            }
            0x420cd5 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x420cdb);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x420ce2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x420cf2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x420cfd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x420d0b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff6e));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x420d1a);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x420d21);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcf20u32);
            c.push(m, 0x420d2e);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x420d34);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x420d40);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x420d4f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0xdu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x420d5a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x420d68); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xa));
            c.push(m,c.get(1,32));
            c.push(m, 0x420d72);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x420d7e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x420d8d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x420d98); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x420da6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x9));
            c.push(m,c.get(0,32));
            c.push(m, 0x420db0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x420dc4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42ecc0;
    loop {
        match pc {
            0x42ecc0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d457u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x42ecfd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x42ed04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,0x66e094u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x94),32); c.push(m, 0x42ed21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1f40u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697118u32);
            c.push(m, 0x42ed36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42ed41);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4439b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1770u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69712cu32);
            c.push(m, 0x42ed56);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42ed61);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4439b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[5].wrapping_add(0xffffffec));
            c.set(3,32,0x1u32);
            c.push(m,0x697140u32);
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x42ed81);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x42ed94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x42ed9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x3e8eu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x42edad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3e9eu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc37cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edc1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3e9fu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc380u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xcu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc384u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42ede6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xeu32);
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0x274); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edf8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x278); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42ee0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x19c),32));
            c.set(1,32,c.r[6].wrapping_add(0x19c));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42ee1e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x69714cu32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb60u32);
            c.push(m,0x697160u32);
            c.push(m,0x697168u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x42ee46);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42ee57; continue; }
            pc = 0x42ee4a; continue;
            }
            0x42ee4a => {
            c.push(m,0x697174u32);
            c.push(m, 0x42ee54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x42ee57; continue;
            }
            0x42ee57 => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[6].wrapping_add(0x288));
            c.push(m, 0x42ee67);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2698u32);
            c.push(m, 0x42ee71);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x42ee88; continue; }
            pc = 0x42ee7f; continue;
            }
            0x42ee7f => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42ee86);
            { let args = []; c.r[0] = host.call(m, 0x496c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ee8a; continue;
            }
            0x42ee88 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42ee8a; continue;
            }
            0x42ee8a => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d1c7cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42eea3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42eeae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42eebf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x42eec9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x49ac90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697180u32);
            c.push(m, 0x42eedb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x42eee5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x49bc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x5d28u32);
            c.push(m, 0x42eeef);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x42ef06; continue; }
            pc = 0x42eefd; continue;
            }
            0x42eefd => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42ef04);
            { let args = []; c.r[0] = host.call(m, 0x436ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ef08; continue;
            }
            0x42ef06 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42ef08; continue;
            }
            0x42ef08 => {
            c.push(m,0x0u32);
            { let a = 0x6cbb94u32; wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x42ef22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x42ef27);
            { let args = []; c.r[0] = host.call(m, 0x4406a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x42ef89; continue;
            }
            0x42ef2c => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef34);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x42ef42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42ef53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42ef5e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef66);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            { let a = c.r[1].wrapping_add(0x218); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef7e);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x42ef84u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x42ef84 => {
            c.set(3,32,0x1u32);
            pc = 0x42ef89; continue;
            }
            0x42ef89 => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42efdf; continue; }
            pc = 0x42ef95; continue;
            }
            0x42ef95 => {
            c.push(m,0x2c98u32);
            c.push(m, 0x42ef9f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            if c.cond(4) { pc = 0x42efb6; continue; }
            pc = 0x42efad; continue;
            }
            0x42efad => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42efb4);
            { let args = []; c.r[0] = host.call(m, 0x45cac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42efb8; continue;
            }
            0x42efb6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42efb8; continue;
            }
            0x42efb8 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbdf0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42efd1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42efdc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[5].wrapping_add(0xf),8); let r = c.and(a,c.get(0,8),8); { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,r); } }
            pc = 0x42efdf; continue;
            }
            0x42efdf => {
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f02f; continue; }
            pc = 0x42efe8; continue;
            }
            0x42efe8 => {
            c.push(m,0x1210u32);
            c.push(m, 0x42eff2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            if c.cond(4) { pc = 0x42f009; continue; }
            pc = 0x42f000; continue;
            }
            0x42f000 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f007);
            { let args = []; c.r[0] = host.call(m, 0x4313b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f00b; continue;
            }
            0x42f009 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f00b; continue;
            }
            0x42f00b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb7cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f024);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f02f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x42f02f; continue;
            }
            0x42f02f => {
            c.set(0,32,rm(m,0x6cbaa4u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f07f; continue; }
            pc = 0x42f038; continue;
            }
            0x42f038 => {
            c.push(m,0xfa8u32);
            c.push(m, 0x42f042);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x9u32); }
            if c.cond(4) { pc = 0x42f059; continue; }
            pc = 0x42f050; continue;
            }
            0x42f050 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f057);
            { let args = []; c.r[0] = host.call(m, 0x40d7b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f05b; continue;
            }
            0x42f059 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f05b; continue;
            }
            0x42f05b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbaa4u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f074);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f07f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x42f07f; continue;
            }
            0x42f07f => {
            c.set(0,32,rm(m,0x6d9594u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f0cf; continue; }
            pc = 0x42f088; continue;
            }
            0x42f088 => {
            c.push(m,0xc24u32);
            c.push(m, 0x42f092);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xau32); }
            if c.cond(4) { pc = 0x42f0a9; continue; }
            pc = 0x42f0a0; continue;
            }
            0x42f0a0 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f0a7);
            { let args = []; c.r[0] = host.call(m, 0x4ab1a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f0ab; continue;
            }
            0x42f0a9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f0ab; continue;
            }
            0x42f0ab => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d9594u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f0c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f0cf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x42f0cf; continue;
            }
            0x42f0cf => {
            c.set(0,32,rm(m,0x6cbb6cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f11f; continue; }
            pc = 0x42f0d8; continue;
            }
            0x42f0d8 => {
            c.push(m,0xd0cu32);
            c.push(m, 0x42f0e2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xbu32); }
            if c.cond(4) { pc = 0x42f0f9; continue; }
            pc = 0x42f0f0; continue;
            }
            0x42f0f0 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f0f7);
            { let args = []; c.r[0] = host.call(m, 0x42fbf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f0fb; continue;
            }
            0x42f0f9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f0fb; continue;
            }
            0x42f0fb => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb6cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f114);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f11f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x42f11f; continue;
            }
            0x42f11f => {
            c.set(0,32,rm(m,0x6cbb30u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f16f; continue; }
            pc = 0x42f128; continue;
            }
            0x42f128 => {
            c.push(m,0x350u32);
            c.push(m, 0x42f132);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xcu32); }
            if c.cond(4) { pc = 0x42f149; continue; }
            pc = 0x42f140; continue;
            }
            0x42f140 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f147);
            { let args = []; c.r[0] = host.call(m, 0x4215d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f14b; continue;
            }
            0x42f149 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f14b; continue;
            }
            0x42f14b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb30u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f164);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb30u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f16f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x42f16f; continue;
            }
            0x42f16f => {
            c.set(0,32,rm(m,0x6d95a0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f1bf; continue; }
            pc = 0x42f178; continue;
            }
            0x42f178 => {
            c.push(m,0x434u32);
            c.push(m, 0x42f182);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xdu32); }
            if c.cond(4) { pc = 0x42f199; continue; }
            pc = 0x42f190; continue;
            }
            0x42f190 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f197);
            { let args = []; c.r[0] = host.call(m, 0x4ac5d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f19b; continue;
            }
            0x42f199 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f19b; continue;
            }
            0x42f19b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d95a0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f1b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f1bf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f1bf; continue;
            }
            0x42f1bf => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x42f1cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42f1da);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_delete_savegame(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433cc0;
    loop {
        match pc {
            0x433cc0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d8fau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x433cef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4348c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.push(m, 0x433d0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433d17);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697428u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x433d28);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697430u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433d35);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m, 0x433d3d);
            { let args = []; c.r[0] = host.call(m, 0x653705, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x433d4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653f49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x433d5b; continue;
            }
            0x433d4c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x433d58u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x433d58 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            pc = 0x433d5b; continue;
            }
            0x433d5b => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.push(m, 0x433d67);
            { let args = []; c.r[0] = host.call(m, 0x653743, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x433d6f);
            { let args = []; c.r[0] = host.call(m, 0x653705, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m,c.get(1,32));
            c.push(m, 0x433d7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653f49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x433d8d; continue;
            }
            0x433d7e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(0,32,0x433d8au32);
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(2,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x433d8a => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            pc = 0x433d8d; continue;
            }
            0x433d8d => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.push(m, 0x433d99);
            { let args = []; c.r[0] = host.call(m, 0x653743, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x433db6);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            if c.cond(4) { pc = 0x433de1; continue; }
            pc = 0x433dc4; continue;
            }
            0x433dc4 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x433ddc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(3,32)); }
            pc = 0x433de8; continue;
            }
            0x433de1 => {
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,0x0u32); }
            pc = 0x433de8; continue;
            }
            0x433de8 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x433e00);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x9u32); }
            if c.cond(4) { pc = 0x433e2b; continue; }
            pc = 0x433e0e; continue;
            }
            0x433e0e => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x433e26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(3,32)); }
            pc = 0x433e32; continue;
            }
            0x433e2b => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x0u32); }
            pc = 0x433e32; continue;
            }
            0x433e32 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x433e95; continue; }
            pc = 0x433e46; continue;
            }
            0x433e46 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(1,32)); }
            pc = 0x433e51; continue;
            }
            0x433e51 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x8),32),32); }
            if c.cond(4) { pc = 0x433e86; continue; }
            pc = 0x433e56; continue;
            }
            0x433e56 => {
            c.set(7,32,c.r[0].wrapping_mul(4));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x433e6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x433e7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            pc = 0x433e86; continue;
            }
            0x433e86 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x433e51; continue; }
            pc = 0x433e95; continue;
            }
            0x433e95 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x433ebc; continue; }
            pc = 0x433e9f; continue;
            }
            0x433e9f => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x433eb3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x433eb9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x433ebc; continue;
            }
            0x433ebc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x433eed; continue; }
            pc = 0x433ed0; continue;
            }
            0x433ed0 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x433ee4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x433eea);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x433eed; continue;
            }
            0x433eed => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x433f0b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x433f1a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_main_window_run(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x463d20;
    loop {
        match pc {
            0x463d20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef0du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1e4u32);
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x1u32); }
            c.push(m, 0x463d4d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x463d67; continue; }
            pc = 0x463d5e; continue;
            }
            0x463d5e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x463d65);
            { let args = []; c.r[0] = host.call(m, 0x4419a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463d69; continue;
            }
            0x463d67 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x463d69; continue;
            }
            0x463d69 => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            { let a = 0x6cbdb8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x463d7d);
            { let args = []; c.r[0] = host.call(m, 0x441cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x463d88);
            { let args = []; c.r[0] = host.call(m, 0x441e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698a28u32);
            c.push(m, 0x463d9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,c.r[5].wrapping_add(0x44));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x463da3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x463dab; continue;
            }
            0x463dab => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x463db5);
            { let args = []; c.r[0] = host.call(m, 0x54a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x463dbf);
            { let args = []; c.r[0] = host.call(m, 0x42f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463df1; continue; }
            pc = 0x463dc9; continue;
            }
            0x463dc9 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x463dd3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463ddc; continue;
            }
            0x463ddc => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x463dec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x463e7a; continue;
            }
            0x463df1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463e34; continue; }
            pc = 0x463df6; continue;
            }
            0x463df6 => {
            c.set(1,32,c.r[5].wrapping_add(0x308));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x463e06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x463e23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x463e34);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463e34; continue;
            }
            0x463e34 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x304),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x300),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x463e54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4381e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x463e62); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(4) { pc = 0x463e7a; continue; }
            pc = 0x463e67; continue;
            }
            0x463e67 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x463e71); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463e7a; continue;
            }
            0x463e7a => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,0x2u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,8,0x3u32); }
            c.push(m, 0x463ea3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463eb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x463ebe);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xac44u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x10u32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6dcb70u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x4u32); }
            c.push(m, 0x463f0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dcb74u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463f19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcb8cu32,32));
            c.set(0,32,rm(m,0x6dcb90u32,32));
            c.set(1,32,0x5u32);
            c.set(6,32,0x6dcb78u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            c.rep_movsd(m);
            c.set(1,32,rm(m,0x6dcb94u32,32));
            c.set(6,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcb98u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcb9cu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcba0u32,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcba4u32,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcba8u32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcbacu32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcbb0u32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcbb4u32,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x46405c; continue; }
            pc = 0x463fbb; continue;
            }
            0x463fbb => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463fd1; continue; }
            pc = 0x463fbf; continue;
            }
            0x463fbf => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x320u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x258u32); }
            pc = 0x463ffd; continue;
            }
            0x463fd1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463fe8; continue; }
            pc = 0x463fd6; continue;
            }
            0x463fd6 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x400u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x300u32); }
            pc = 0x463ffd; continue;
            }
            0x463fe8 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x463ffd; continue; }
            pc = 0x463fed; continue;
            }
            0x463fed => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x500u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x400u32); }
            pc = 0x463ffd; continue;
            }
            0x463ffd => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x464011; continue; }
            pc = 0x464007; continue;
            }
            0x464007 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x10u32); }
            pc = 0x46401e; continue;
            }
            0x464011 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x46401e; continue; }
            pc = 0x464016; continue;
            }
            0x464016 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x20u32); }
            pc = 0x46401e; continue;
            }
            0x46401e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x46402c);
            { let args = []; c.r[0] = host.call(m, 0x4bccc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x46405c; continue; }
            pc = 0x464033; continue;
            }
            0x464033 => {
            c.push(m,c.get(3,32));
            c.push(m,0x698a40u32);
            c.push(m,0x698a68u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x464045);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x651b5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[2].wrapping_add(0x168); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[0].wrapping_add(0x164); wm(m,a,32,c.get(3,32)); }
            pc = 0x46405c; continue;
            }
            0x46405c => {
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x464066);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x46406d);
            { let args = []; c.r[0] = host.call(m, 0x6534f6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x168),8));
            { let a = c.r[0].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x164),8));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x180),8));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x184),8));
            { let a = c.r[0].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x178),8));
            { let a = c.r[0].wrapping_add(0x22); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x174),8));
            { let a = c.r[0].wrapping_add(0x23); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x170),8));
            { let a = c.r[0].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x16c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x21),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x464124);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x23),8));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46413b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x188),32));
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x1dd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x464164);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x46417a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x464188); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x464193);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x4641a1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(5) { pc = 0x4641aa; continue; }
            pc = 0x4641a6; continue;
            }
            0x4641a6 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            pc = 0x4641aa; continue;
            }
            0x4641aa => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4641b5);
            { let args = []; c.r[0] = host.call(m, 0x427960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4641c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4641d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0x2fc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0x5u32); }
            c.push(m, 0x4641ee);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4641ff);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x13),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x463dab; continue; }
            pc = 0x464209; continue;
            }
            0x464209 => {
            { let a = rm(m,0x6cbb94u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x464211; continue;
            }
            0x464211 => {
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698ab8u32);
            c.push(m, 0x464225);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x464230);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x464230; continue;
            }
            0x464230 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x464245 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x46424c);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x464230; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn draw_interior_window_ornaments(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46b960;
    loop {
        match pc {
            0x46b960 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0xffffffffu32);
            c.push(m, 0x46b96e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,0x7925u32);
            c.push(m, 0x46b980);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x46b9ae; continue; }
            pc = 0x46b98b; continue;
            }
            0x46b98b => {
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x46b9ae; continue; }
            pc = 0x46b992; continue;
            }
            0x46b992 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46b9b6; continue;
            }
            0x46b9ae => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x46b9b6; continue;
            }
            0x46b9b6 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46b9bc);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46b9d4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x46b9e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x46b9f4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46ba07);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x46ba3a; continue; }
            pc = 0x46ba14; continue;
            }
            0x46ba14 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46ba3a; continue; }
            pc = 0x46ba1b; continue;
            }
            0x46ba1b => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46ba3a; continue;
            }
            0x46ba3a => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46ba40);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46ba54); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x98));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46ba6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46ba79); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46ba8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,0x7926u32);
            c.push(m, 0x46ba9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x46bacc; continue; }
            pc = 0x46baa9; continue;
            }
            0x46baa9 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46bacc; continue; }
            pc = 0x46bab0; continue;
            }
            0x46bab0 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,c.get(1,32));
            c.set(3,32,rm(m,c.r[2],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x46bacc; continue;
            }
            0x46bacc => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46bad2);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x46bae6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46bafa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46bb08); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46bb1b);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x46bb4a; continue; }
            pc = 0x46bb28; continue;
            }
            0x46bb28 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46bb4a; continue; }
            pc = 0x46bb2f; continue;
            }
            0x46bb2f => {
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46bb4a; continue;
            }
            0x46bb4a => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46bb50);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46bb64); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46bb78); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46bb86); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x84));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46bb95);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class75_load_texture_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b3ea0;
    loop {
        match pc {
            0x4b3ea0 => {
            c.push(m,0xffffffffu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0x661ac2u32);
            c.push(m,c.get(0,32));
            c.set(0,32,0x1014u32);
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m, 0x4b3ebf);
            { let args = []; c.r[0] = host.call(m, 0x63b920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x4b3f26; continue; }
            pc = 0x4b3eea; continue;
            }
            0x4b3eea => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            pc = 0x4b3eed; continue;
            }
            0x4b3eed => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1038),32));
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b3f04; continue; }
            pc = 0x4b3efa; continue;
            }
            0x4b3efa => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4b3eed; continue; }
            pc = 0x4b3f02; continue;
            }
            0x4b3f02 => {
            pc = 0x4b3f2d; continue;
            }
            0x4b3f04 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b3f2d; continue; }
            pc = 0x4b3f09; continue;
            }
            0x4b3f09 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f11; continue;
            }
            0x4b3f11 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f19; continue;
            }
            0x4b3f19 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x4b41a2; continue;
            }
            0x4b3f26 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1038),32));
            pc = 0x4b3f2d; continue;
            }
            0x4b3f2d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x4b3f60; continue; }
            pc = 0x4b3f31; continue;
            }
            0x4b3f31 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b3f42);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1034));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b3f59);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x4b41cc; continue;
            }
            0x4b3f60 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f69; continue;
            }
            0x4b3f69 => {
            c.set(2,32,rm(m,0x6bb910u32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b3f7b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1040),32));
            c.set(2,32,rm(m,0x6bb91cu32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4b3f9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1034),32));
            c.set(2,32,rm(m,0x6bb938u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6bb928u32,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dab58u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4b3fd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1034),32));
            c.set(0,32,rm(m,0x6bb948u32,32));
            c.set(1,32,rm(m,0x6bb940u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4b4000);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4b41a2; continue; }
            pc = 0x4b4008; continue;
            }
            0x4b4008 => {
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4011);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xffu32);
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4b4023);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,0x6bb94cu32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4b4034);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4040);
            { let args = []; c.r[0] = host.call(m, 0x639f83, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4b408b; continue; }
            pc = 0x4b4053; continue;
            }
            0x4b4053 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b408b; continue; }
            pc = 0x4b4057; continue;
            }
            0x4b4057 => {
            c.push(m,0x0u32);
            c.push(m,0x6bb950u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b4065);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b4081; continue; }
            pc = 0x4b406e; continue;
            }
            0x4b406e => {
            c.push(m,0xau32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4078);
            { let args = []; c.r[0] = host.call(m, 0x639f83, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4082; continue;
            }
            0x4b4081 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            pc = 0x4b4082; continue;
            }
            0x4b4082 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b4053; continue; }
            pc = 0x4b408b; continue;
            }
            0x4b408b => {
            c.push(m,0x1u32);
            c.push(m,0x6bb954u32);
            c.push(m,0x6dab58u32);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m, 0x4b409e);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b40ab);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x4b4125; continue; }
            pc = 0x4b40be; continue;
            }
            0x4b40be => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b4125; continue; }
            pc = 0x4b40c2; continue;
            }
            0x4b40c2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4b40d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b40df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4b40f6; continue; }
            pc = 0x4b40e3; continue;
            }
            0x4b40e3 => {
            c.push(m,0x1u32);
            c.push(m,0x6bb958u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b40f1);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            pc = 0x4b40f6; continue;
            }
            0x4b40f6 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b410e; continue; }
            pc = 0x4b40fa; continue;
            }
            0x4b40fa => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4b410e; continue; }
            pc = 0x4b40fe; continue;
            }
            0x4b40fe => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b4109);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x4b4114; continue;
            }
            0x4b410e => {
            c.set(3,8,0x1u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            pc = 0x4b4114; continue;
            }
            0x4b4114 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4b40be; continue; }
            pc = 0x4b4125; continue;
            }
            0x4b4125 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(3,32,c.r[7].wrapping_mul(4));
            c.push(m,c.get(3,32));
            c.push(m, 0x4b4136);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4b413e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4b4151);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4b4162);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b416b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b4174);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1060),32));
            { let a = c.r[5].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0x1u32); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4b419f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            pc = 0x4b41a2; continue;
            }
            0x4b41a2 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b41b3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1034));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b41ca);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            pc = 0x4b41cc; continue;
            }
            0x4b41cc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1024),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1020u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn battle_publish_player_side_ships(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58a240;
    loop {
        match pc {
            0x58a240 => {
            c.set(0,32,rm(m,0x6e59ccu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x35a),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58a275; continue; }
            pc = 0x58a26a; continue;
            }
            0x58a26a => {
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x658),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x58a280; continue;
            }
            0x58a275 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x658),16));
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x14),16));
            pc = 0x58a280; continue;
            }
            0x58a280 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[3]));
            c.push(m,c.get(2,32));
            c.push(m, 0x58a299);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x58a2c0; continue; }
            pc = 0x58a2a2; continue;
            }
            0x58a2a2 => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            pc = 0x58a2a9; continue;
            }
            0x58a2a9 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            c.set(7,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.r[2].wrapping_add(0xfffffffe); wm(m,a,16,c.get(7,16)); }
            if c.cond(12) { pc = 0x58a2a9; continue; }
            pc = 0x58a2c0; continue;
            }
            0x58a2c0 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58a2dd; continue; }
            pc = 0x58a2d4; continue;
            }
            0x58a2d4 => {
            c.set(0,8,rm(m,0x66f352u32,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            pc = 0x58a2dd; continue;
            }
            0x58a2dd => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x58a2f0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x58a304; continue; }
            pc = 0x58a2f9; continue;
            }
            0x58a2f9 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x58a304);
            { let args = []; c.r[0] = host.call(m, 0x495150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a304; continue;
            }
            0x58a304 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_place_and_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637e20;
    loop {
        match pc {
            0x637e20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e34); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x290),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e55); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(4));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x284),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x637e81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x637e88); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x280),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x637ea2; continue; }
            pc = 0x637e9d; continue;
            }
            0x637e9d => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea2 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea5 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6dcce4u32,32));
            c.set(0,32,0x92492493u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(3,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x637ee0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637ef8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x637f09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x637f28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x198),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(7,32,c.r[6].wrapping_add(0x198));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x637f4e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637f5f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x637f7a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x13c),32); c.push(m, 0x637f84); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x637f8f);
            { let args = []; c.r[0] = host.call(m, 0x4438a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fc4; continue; }
            pc = 0x637f97; continue;
            }
            0x637f97 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[6].wrapping_add(0x2a0); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x298),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x29c); wm(m,a,32,c.get(0,32)); }
            pc = 0x637fc4; continue;
            }
            0x637fc4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x637fcf);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fe4; continue; }
            pc = 0x637fd7; continue;
            }
            0x637fd7 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x637fe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x637fe4; continue;
            }
            0x637fe4 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637fef);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637ffe; continue; }
            pc = 0x637ff3; continue;
            }
            0x637ff3 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637ffe);
            { let args = []; c.r[0] = host.call(m, 0x453ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637ffe; continue;
            }
            0x637ffe => {
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "render_window_title" => render_window_title(m, host, regs),
        "main_menu_init" => main_menu_init(m, host, regs),
        "load_save_screen_delete_savegame" => load_save_screen_delete_savegame(m, host, regs),
        "game_main_window_run" => game_main_window_run(m, host, regs),
        "draw_interior_window_ornaments" => draw_interior_window_ornaments(m, host, regs),
        "class75_load_texture_list" => class75_load_texture_list(m, host, regs),
        "battle_publish_player_side_ships" => battle_publish_player_side_ships(m, host, regs),
        "tutorial_window_place_and_layout" => tutorial_window_place_and_layout(m, host, regs),
        _ => return None,
    })
}
