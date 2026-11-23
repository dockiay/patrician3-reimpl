//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn convoy_choice_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40bd20;
    loop {
        match pc {
            0x40bd20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b4f0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x40bd45);
            { let args = []; c.r[0] = host.call(m, 0x41c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xa8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x40bd56);
            { let args = []; c.r[0] = host.call(m, 0x46f760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x40bd66);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x1540));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x40bd78);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x1634));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40bd8a);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            { let a = c.r[6]; wm(m,a,32,0x66ace0u32); }
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x40bda5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x171c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1720); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1724); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1160); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            if c.cond(4) { pc = 0x40bdf0; continue; }
            pc = 0x40bde1; continue;
            }
            0x40bde1 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40bde7);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(3,32)); }
            pc = 0x40bdf0; continue;
            }
            0x40bdf0 => {
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x40be01);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x1534); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1535); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1538); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x40be1f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x1628); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x153c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1520); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1524); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1528); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_hide(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40bfb0;
    loop {
        match pc {
            0x40bfb0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1160); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            if c.cond(4) { pc = 0x40bfec; continue; }
            pc = 0x40bfdd; continue;
            }
            0x40bfdd => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40bfe3);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(3,32)); }
            pc = 0x40bfec; continue;
            }
            0x40bfec => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x40bffd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40c004);
            { let args = []; c.r[0] = host.call(m, 0x41ccf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1174));
            c.push(m, 0x40c00f);
            { let args = []; c.r[0] = host.call(m, 0x461480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x40c01b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x1540));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x40c02d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x1634));
            c.push(m,c.get(2,32));
            c.push(m, 0x40c03f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c060;
    loop {
        match pc {
            0x40c060 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1634),32));
            c.set(1,32,c.r[6].wrapping_add(0x1634));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x40c076); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x171c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1720); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1724); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1160); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            if c.cond(4) { pc = 0x40c0c3; continue; }
            pc = 0x40c0b4; continue;
            }
            0x40c0b4 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40c0ba);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c0c3; continue;
            }
            0x40c0c3 => {
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x40c0d4);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1540),32));
            c.set(1,32,c.r[6].wrapping_add(0x1540));
            { let a = c.r[6].wrapping_add(0x1534); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1535); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1538); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x40c0f9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1628); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x153c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1520); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1524); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1528); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_clear_options(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c120;
    loop {
        match pc {
            0x40c120 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1160); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            if c.cond(4) { pc = 0x40c15c; continue; }
            pc = 0x40c14d; continue;
            }
            0x40c14d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40c153);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c15c; continue;
            }
            0x40c15c => {
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_build_option_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c170;
    loop {
        match pc {
            0x40c170 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x153c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,8,c.get(3,8)); }
            if c.cond(14) { pc = 0x40c2ab; continue; }
            pc = 0x40c192; continue;
            }
            0x40c192 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1538),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m, 0x40c1b1);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x40c1c5; continue; }
            pc = 0x40c1b5; continue;
            }
            0x40c1b5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x153c),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40c192; continue; }
            pc = 0x40c1c0; continue;
            }
            0x40c1c0 => {
            pc = 0x40c2ab; continue;
            }
            0x40c1c5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40c1da; continue; }
            pc = 0x40c1cf; continue;
            }
            0x40c1cf => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = rm(m,c.r[2],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x40c206; continue; }
            pc = 0x40c1da; continue;
            }
            0x40c1da => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x40c1e8);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,0xffffffffu32); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,8,0x1u32); }
            pc = 0x40c206; continue;
            }
            0x40c206 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(3,32,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x40c220);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1534),8));
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(2,32); let r = c.and(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x80u32,8); }
            if c.cond(5) { pc = 0x40c255; continue; }
            pc = 0x40c249; continue;
            }
            0x40c249 => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x468),32); c.and(a,c.get(0,32),32); }
            pc = 0x40c25d; continue;
            }
            0x40c255 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x468),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            pc = 0x40c25d; continue;
            }
            0x40c25d => {
            if c.cond(4) { pc = 0x40c2ab; continue; }
            pc = 0x40c25f; continue;
            }
            0x40c25f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(7,32,0xfffffffeu32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x40c279; continue; }
            pc = 0x40c26e; continue;
            }
            0x40c26e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40c2a2; continue; }
            pc = 0x40c279; continue;
            }
            0x40c279 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x40c287);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,0x2u32); }
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,8,0x1u32); }
            pc = 0x40c2a2; continue;
            }
            0x40c2a2 => {
            c.set(3,32,0x2u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c2ab; continue;
            }
            0x40c2ab => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x153c),32));
            c.push(m,c.get(5,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x40c304; continue; }
            pc = 0x40c2bd; continue;
            }
            0x40c2bd => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1538),32));
            pc = 0x40c2c3; continue;
            }
            0x40c2c3 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x40c2f5; continue; }
            pc = 0x40c2ea; continue;
            }
            0x40c2ea => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(15) { pc = 0x40c2ff; continue; }
            pc = 0x40c2f5; continue;
            }
            0x40c2f5 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x40c2c3; continue; }
            pc = 0x40c2fd; continue;
            }
            0x40c2fd => {
            pc = 0x40c304; continue;
            }
            0x40c2ff => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x40c304; continue;
            }
            0x40c304 => {
            c.set(7,32,rm(m,0x6dd88cu32,32));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x40c43f; continue; }
            pc = 0x40c317; continue;
            }
            0x40c317 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x1534),8));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),16); c.sub(a,c.get(1,16),16); }
            c.set(5,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            if c.cond(5) { pc = 0x40c42e; continue; }
            pc = 0x40c33e; continue;
            }
            0x40c33e => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            if c.cond(4) { pc = 0x40c35d; continue; }
            pc = 0x40c348; continue;
            }
            0x40c348 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x40c35d; continue; }
            pc = 0x40c34d; continue;
            }
            0x40c34d => {
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(4) { pc = 0x40c35d; continue; }
            pc = 0x40c353; continue;
            }
            0x40c353 => {
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x40c42e; continue; }
            pc = 0x40c35d; continue;
            }
            0x40c35d => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x14),8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40c371; continue; }
            pc = 0x40c369; continue;
            }
            0x40c369 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x40c3ad; continue; }
            pc = 0x40c371; continue;
            }
            0x40c371 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1538),32));
            c.set(1,16,rm(m,c.r[0],16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x40c382);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5039f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x40c3a9; continue; }
            pc = 0x40c386; continue;
            }
            0x40c386 => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x14),8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x40c42a; continue; }
            pc = 0x40c396; continue;
            }
            0x40c396 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x40c42a; continue; }
            pc = 0x40c3a9; continue;
            }
            0x40c3a9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x40c3ad; continue;
            }
            0x40c3ad => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1535),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40c3e2; continue; }
            pc = 0x40c3b7; continue;
            }
            0x40c3b7 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1f),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x40c3e2; continue; }
            pc = 0x40c3c6; continue;
            }
            0x40c3c6 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40c42e; continue; }
            pc = 0x40c3e2; continue;
            }
            0x40c3e2 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x1530),32),32); }
            if c.cond(13) { pc = 0x40c3f5; continue; }
            pc = 0x40c3ea; continue;
            }
            0x40c3ea => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40c423; continue; }
            pc = 0x40c3f5; continue;
            }
            0x40c3f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            c.set(2,32,c.r[3].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40c409);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x152c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[3].wrapping_add(0x1));
            { let a = c.r[0].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            pc = 0x40c423; continue;
            }
            0x40c423 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c42e; continue;
            }
            0x40c42a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x40c42e; continue;
            }
            0x40c42e => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x40c317; continue; }
            pc = 0x40c43f; continue;
            }
            0x40c43f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40c459; continue; }
            pc = 0x40c44a; continue;
            }
            0x40c44a => {
            { let a = c.r[6].wrapping_add(0x1530); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40c459 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xe),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c470;
    loop {
        match pc {
            0x40c470 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x1538));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x40c48f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x153c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,16,rm(m,c.r[7],16));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x39),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1534); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1535); wm(m,a,8,c.get(3,8)); }
            if c.cond(14) { pc = 0x40c4e7; continue; }
            pc = 0x40c4be; continue;
            }
            0x40c4be => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x40c4c4; continue;
            }
            0x40c4c4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7],16));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[5]).wrapping_add(0xe),8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x40c4e0; continue; }
            pc = 0x40c4d6; continue;
            }
            0x40c4d6 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40c4c4; continue; }
            pc = 0x40c4de; continue;
            }
            0x40c4de => {
            pc = 0x40c4e7; continue;
            }
            0x40c4e0 => {
            { let a = c.r[6].wrapping_add(0x1535); wm(m,a,8,0x1u32); }
            pc = 0x40c4e7; continue;
            }
            0x40c4e7 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40c4ee);
            { let args = []; c.r[0] = host.call(m, 0x40c170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x1530),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x40c551; continue; }
            pc = 0x40c4f6; continue;
            }
            0x40c4f6 => {
            c.set(2,32,rm(m,0x699b48u32,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,8,0xffu32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(3,16)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x3fb00000u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40c549);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40c551 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x40c557);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x40c566);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x1540));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x40c578);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x1634));
            c.push(m,c.get(2,32));
            c.push(m, 0x40c58a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40c591);
            { let args = []; c.r[0] = host.call(m, 0x40c5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x40c59d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40c5a4);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c5f0;
    loop {
        match pc {
            0x40c5f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x40c609); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40c619); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x1174));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40c626);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x40c65f);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,0x6cbb00u32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),8); c.fop('*',v); }
            c.push(m, 0x40c675);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c68b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x40c6cf);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,0x6cbb00u32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),8); c.fop('*',v); }
            c.push(m, 0x40c6e5);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x40c701; continue; }
            pc = 0x40c6f0; continue;
            }
            0x40c6f0 => {
            c.set(3,32,rm(m,0x6cbb00u32,32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x40c701; continue;
            }
            0x40c701 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x40c749; continue; }
            pc = 0x40c734; continue;
            }
            0x40c734 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0x80000003u32,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x40c743; continue; }
            pc = 0x40c73e; continue;
            }
            0x40c73e => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xfffffffcu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x40c743; continue;
            }
            0x40c743 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c749; continue;
            }
            0x40c749 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x40c755); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40c776); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c789); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x3c),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40c7c6);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c7ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c800); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40c81f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            c.push(m,c.get(0,32));
            c.push(m, 0x40c831);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14f0),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x40c84d; continue; }
            pc = 0x40c841; continue;
            }
            0x40c841 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x40c84d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40c84d; continue;
            }
            0x40c84d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40c860; continue; }
            pc = 0x40c857; continue;
            }
            0x40c857 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40c85d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40c860; continue;
            }
            0x40c860 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,0x0u32); }
            c.push(m, 0x40c87b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x41c5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1634),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x171c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,c.r[6].wrapping_add(0x1634));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x40c8b2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1540),32));
            c.set(3,32,c.r[6].wrapping_add(0x1540));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x40c8c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x40c8d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x40c8e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c8f0;
    loop {
        match pc {
            0x40c8f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1168),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40c949; continue; }
            pc = 0x40c901; continue;
            }
            0x40c901 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1164),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40cb7c; continue; }
            pc = 0x40c90f; continue;
            }
            0x40c90f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1160),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x40c921; continue; }
            pc = 0x40c919; continue;
            }
            0x40c919 => {
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            pc = 0x40ca73; continue;
            }
            0x40c921 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1540),32));
            c.set(1,32,c.r[6].wrapping_add(0x1540));
            c.push(m,0x1u32);
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x40c93c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40c949 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x40c95d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1538),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x40cb74; continue; }
            pc = 0x40c971; continue;
            }
            0x40c971 => {
            c.push(m, 0x40c976);
            { let args = []; c.r[0] = host.call(m, 0x40c170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40c9df; continue; }
            pc = 0x40c97a; continue;
            }
            0x40c97a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40c9a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40c9b0);
            { let args = []; c.r[0] = host.call(m, 0x40c5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40c9df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40c9df; continue;
            }
            0x40c9df => {
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            c.push(m, 0x40c9ea);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40ca1d; continue; }
            pc = 0x40c9ee; continue;
            }
            0x40c9ee => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40ca1d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40ca1d; continue;
            }
            0x40ca1d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40ca24);
            { let args = []; c.r[0] = host.call(m, 0x40d170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x162c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1540),32));
            c.set(5,32,c.r[6].wrapping_add(0x1540));
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x40ca42); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x1634));
            c.push(m, 0x40ca4d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x40cb72; continue; }
            pc = 0x40ca55; continue;
            }
            0x40ca55 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x40ca5c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40cb7c; continue; }
            pc = 0x40ca64; continue;
            }
            0x40ca64 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x162c),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x40ca8e; continue; }
            pc = 0x40ca6f; continue;
            }
            0x40ca6f => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            pc = 0x40ca73; continue;
            }
            0x40ca73 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x40ca7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x494b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x40ca88); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40ca8e => {
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x40cb5f; continue; }
            pc = 0x40ca97; continue;
            }
            0x40ca97 => {
            { let a = c.r[6].wrapping_add(0x14c); wm(m,a,16,0x1u32); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40caab);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x154); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x40cb0d; continue; }
            pc = 0x40cac2; continue;
            }
            0x40cac2 => {
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x40cac5; continue;
            }
            0x40cac5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1534),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x40cafb; continue; }
            pc = 0x40cad1; continue;
            }
            0x40cad1 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1535),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x40caf0; continue; }
            pc = 0x40cadb; continue;
            }
            0x40cadb => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x2c8),32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40cafb; continue; }
            pc = 0x40caf0; continue;
            }
            0x40caf0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x154),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            pc = 0x40cafb; continue;
            }
            0x40cafb => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x9f8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x40cac5; continue; }
            pc = 0x40cb0c; continue;
            }
            0x40cb0c => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x40cb0d; continue;
            }
            0x40cb0d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.r[6].wrapping_add(0x14e); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[6].wrapping_add(0x1164));
            c.set(0,32,c.r[6].wrapping_add(0x1160));
            { let a = c.r[6].wrapping_add(0x170); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0xa8));
            { let a = c.r[6].wrapping_add(0x16c); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x174); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x40cb48); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,0x1u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x40cb59); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40cb5f => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x162c),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x40cb72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x494af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40cb72; continue;
            }
            0x40cb72 => {
            c.set(1,32,c.get(6,32));
            pc = 0x40cb74; continue;
            }
            0x40cb74 => {
            c.set(2,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x40cb7c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x40cb7c; continue;
            }
            0x40cb7c => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40cda0;
    loop {
        match pc {
            0x40cda0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b56eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x40cdf4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40d13f; continue; }
            pc = 0x40cdfc; continue;
            }
            0x40cdfc => {
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(1,32));
            c.push(m, 0x40ce06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40d13f; continue; }
            pc = 0x40ce11; continue;
            }
            0x40ce11 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40ce1b);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x40ce22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40ce31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41caa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m, 0x40ce38);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x699b68u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcf20u32);
            c.push(m, 0x40ce4c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,0x6dcf20u32);
            c.set(0,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m, 0x40ce69);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x40ce6f);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40ce7a);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbb00u32,32));
            c.set(1,32,rm(m,0x699b68u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(0x1));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x1));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x40ce9c);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40cea8);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x699b68u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cbb00u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x40cec4);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14e4),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x40cf04);
            { let args = []; c.r[0] = host.call(m, 0x4bb820, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x40cf0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x40cf1c);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x40cf22);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1518),32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1510),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1514),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            if c.cond(3) { pc = 0x40d121; continue; }
            pc = 0x40cf62; continue;
            }
            0x40cf62 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x162c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40cf7d; continue; }
            pc = 0x40cf75; continue;
            }
            0x40cf75 => {
            c.set(0,32,rm(m,0x66e108u32,32));
            c.push(m,c.get(0,32));
            pc = 0x40cf9d; continue;
            }
            0x40cf7d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1628),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1510),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x40cf98; continue; }
            pc = 0x40cf8f; continue;
            }
            0x40cf8f => {
            c.set(2,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(2,32));
            pc = 0x40cf9d; continue;
            }
            0x40cf98 => {
            c.push(m,0xff000000u32);
            pc = 0x40cf9d; continue;
            }
            0x40cf9d => {
            c.push(m, 0x40cfa2);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x40cfc6; continue; }
            pc = 0x40cfb3; continue;
            }
            0x40cfb3 => {
            c.set(1,32,rm(m,0x699b80u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x40cfc1);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x40d0f2; continue;
            }
            0x40cfc6 => {
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x40cfde; continue; }
            pc = 0x40cfcb; continue;
            }
            0x40cfcb => {
            c.set(2,32,rm(m,0x699b98u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x40cfd9);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x40d0f2; continue;
            }
            0x40cfde => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40cff5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x503ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40d00d; continue; }
            pc = 0x40cff9; continue;
            }
            0x40cff9 => {
            c.set(0,32,rm(m,0x6b0d88u32,32));
            c.set(1,32,rm(m,0x699bacu32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x40d035; continue;
            }
            0x40d00d => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.set(0,32,rm(m,0x699bacu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x40d035; continue;
            }
            0x40d035 => {
            c.push(m, 0x40d03a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40d0de; continue; }
            pc = 0x40d04d; continue;
            }
            0x40d04d => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x16),16));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,8,0x1u32); }
            c.push(m, 0x40d083);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1f),8));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x78));
            c.push(m,0x6930fcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x40d0bf);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m, 0x40d0d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x68));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x0u32); }
            c.push(m, 0x40d0de);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x40d0de; continue;
            }
            0x40d0de => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x40d0ee);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x40d0f2; continue;
            }
            0x40d0f2 => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d108); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1514),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x40cf62; continue; }
            pc = 0x40d121; continue;
            }
            0x40d121 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40d12b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x40d13f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x40d13f; continue;
            }
            0x40d13f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_slot54_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d160;
    loop {
        match pc {
            0x40d160 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_update_hover_row(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d170;
    loop {
        match pc {
            0x40d170 => {
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4]);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x40d187);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x40d18e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x40d200; continue; }
            pc = 0x40d1a5; continue;
            }
            0x40d1a5 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x40d200; continue; }
            pc = 0x40d1b3; continue;
            }
            0x40d1b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x40d200; continue; }
            pc = 0x40d1c7; continue;
            }
            0x40d1c7 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14e4),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x40d200; continue; }
            pc = 0x40d1d5; continue;
            }
            0x40d1d5 => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d1e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x18),32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            pc = 0x40d200; continue;
            }
            0x40d200 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1628),32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40d216; continue; }
            pc = 0x40d20a; continue;
            }
            0x40d20a => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(12) { pc = 0x40d326; continue; }
            pc = 0x40d216; continue;
            }
            0x40d216 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x40d298; continue; }
            pc = 0x40d21a; continue;
            }
            0x40d21a => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(13) { pc = 0x40d298; continue; }
            pc = 0x40d222; continue;
            }
            0x40d222 => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d235); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d260); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40d298);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40d298; continue;
            }
            0x40d298 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1628),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x40d320; continue; }
            pc = 0x40d2a2; continue;
            }
            0x40d2a2 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(13) { pc = 0x40d320; continue; }
            pc = 0x40d2aa; continue;
            }
            0x40d2aa => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d2bd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d2e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40d320);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40d320; continue;
            }
            0x40d320 => {
            { let a = c.r[6].wrapping_add(0x1628); wm(m,a,32,c.get(3,32)); }
            pc = 0x40d326; continue;
            }
            0x40d326 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_refresh_marked_row(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d330;
    loop {
        match pc {
            0x40d330 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1168),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x40d58d; continue; }
            pc = 0x40d347; continue;
            }
            0x40d347 => {
            { let a = rm(m,c.r[4].wrapping_add(0x34),32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x40d58d; continue; }
            pc = 0x40d352; continue;
            }
            0x40d352 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1628),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x162c),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x40d442; continue; }
            pc = 0x40d367; continue;
            }
            0x40d367 => {
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(4) { pc = 0x40d58d; continue; }
            pc = 0x40d370; continue;
            }
            0x40d370 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x40d42e; continue; }
            pc = 0x40d380; continue;
            }
            0x40d380 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            pc = 0x40d386; continue;
            }
            0x40d386 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40d3a6; continue; }
            pc = 0x40d38a; continue;
            }
            0x40d38a => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x40d386; continue; }
            pc = 0x40d392; continue;
            }
            0x40d392 => {
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x40d3a6 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x1510),32),32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            if c.cond(8) { pc = 0x40d42e; continue; }
            pc = 0x40d3b0; continue;
            }
            0x40d3b0 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(13) { pc = 0x40d42e; continue; }
            pc = 0x40d3b8; continue;
            }
            0x40d3b8 => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d3cb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d3f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40d42e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40d42e; continue;
            }
            0x40d42e => {
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,0xfffffffdu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x40d442 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1510),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x152c),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[0]));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40d58d; continue; }
            pc = 0x40d45c; continue;
            }
            0x40d45c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x40d4ff; continue; }
            pc = 0x40d46c; continue;
            }
            0x40d46c => {
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40d47d; continue; }
            pc = 0x40d470; continue;
            }
            0x40d470 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x40d46c; continue; }
            pc = 0x40d478; continue;
            }
            0x40d478 => {
            pc = 0x40d4ff; continue;
            }
            0x40d47d => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            if c.cond(8) { pc = 0x40d4ff; continue; }
            pc = 0x40d483; continue;
            }
            0x40d483 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(13) { pc = 0x40d4ff; continue; }
            pc = 0x40d48b; continue;
            }
            0x40d48b => {
            c.set(0,32,rm(m,0x6dcdd0u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40d49d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6dcdd0u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40d4c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40d4ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40d4ff; continue;
            }
            0x40d4ff => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1628),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x40d587; continue; }
            pc = 0x40d509; continue;
            }
            0x40d509 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14f0),32),32); }
            if c.cond(13) { pc = 0x40d587; continue; }
            pc = 0x40d511; continue;
            }
            0x40d511 => {
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d524); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40d54f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40d587);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40d587; continue;
            }
            0x40d587 => {
            { let a = c.r[6].wrapping_add(0x162c); wm(m,a,32,c.get(5,32)); }
            pc = 0x40d58d; continue;
            }
            0x40d58d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d730;
    loop {
        match pc {
            0x40d730 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x40d738);
            { let args = []; c.r[0] = host.call(m, 0x40be60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40d748; continue; }
            pc = 0x40d73f; continue;
            }
            0x40d73f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x40d745);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40d748; continue;
            }
            0x40d748 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn select_next_player_ship_in_port(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4290a0;
    loop {
        match pc {
            0x4290a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4290c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x429198; continue; }
            pc = 0x4290d6; continue;
            }
            0x4290d6 => {
            { let a = rm(m,0x6dd8a0u32,16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x429198; continue; }
            pc = 0x4290e4; continue;
            }
            0x4290e4 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4290f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x429198; continue; }
            pc = 0x429108; continue;
            }
            0x429108 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            c.set(1,16,rm(m,c.r[2],16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            pc = 0x42911a; continue;
            }
            0x42911a => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x429143; continue; }
            pc = 0x429137; continue;
            }
            0x429137 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x429198; continue; }
            pc = 0x42913f; continue;
            }
            0x42913f => {
            c.set(0,8,0x1u32);
            pc = 0x42918b; continue;
            }
            0x429143 => {
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x42918b; continue; }
            pc = 0x42914d; continue;
            }
            0x42914d => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x28),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.or(a,c.get(3,8),8); c.set(2,8,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            if c.cond(4) { pc = 0x42918b; continue; }
            pc = 0x42915b; continue;
            }
            0x42915b => {
            c.set(2,16,rm(m,0x6dd896u32,16));
            { let a = rm(m,c.r[1].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x429174; continue; }
            pc = 0x429168; continue;
            }
            0x429168 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x42918b; continue; }
            pc = 0x429174; continue;
            }
            0x429174 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x429185; continue; }
            pc = 0x429180; continue;
            }
            0x429180 => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[1].wrapping_add(0x39),8),8); }
            if c.cond(5) { pc = 0x42918b; continue; }
            pc = 0x429185; continue;
            }
            0x429185 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(5,32,c.get(6,32));
            if c.cond(5) { pc = 0x429198; continue; }
            pc = 0x42918b; continue;
            }
            0x42918b => {
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x42911a; continue; }
            pc = 0x429198; continue;
            }
            0x429198 => {
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x42922c; continue; }
            pc = 0x4291a5; continue;
            }
            0x4291a5 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4291b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(7,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x429227; continue; }
            pc = 0x4291c4; continue;
            }
            0x4291c4 => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            pc = 0x4291ca; continue;
            }
            0x4291ca => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x42921e; continue; }
            pc = 0x4291ea; continue;
            }
            0x4291ea => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x429203; continue; }
            pc = 0x4291f7; continue;
            }
            0x4291f7 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x42921e; continue; }
            pc = 0x429203; continue;
            }
            0x429203 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x429214; continue; }
            pc = 0x42920f; continue;
            }
            0x42920f => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[1].wrapping_add(0x39),8),8); }
            if c.cond(5) { pc = 0x42921e; continue; }
            pc = 0x429214; continue;
            }
            0x429214 => {
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x429227; continue; }
            pc = 0x42921e; continue;
            }
            0x42921e => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x4291ca; continue; }
            pc = 0x429227; continue;
            }
            0x429227 => {
            { let a = c.get(5,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x42928f; continue; }
            pc = 0x42922c; continue;
            }
            0x42922c => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x42924f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x476a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m, 0x429256);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,16,c.get(5,16)); }
            c.set(1,8,rm(m,0x66f353u32,8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x429285); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m, 0x42928c);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x42928f; continue;
            }
            0x42928f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn select_next_player_ship_at_sea(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4292a0;
    loop {
        match pc {
            0x4292a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4292cd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x429395; continue; }
            pc = 0x4292d6; continue;
            }
            0x4292d6 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4292e8; continue; }
            pc = 0x4292de; continue;
            }
            0x4292de => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6dd8a6u32,16));
            pc = 0x4292ea; continue;
            }
            0x4292e8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4292ea; continue;
            }
            0x4292ea => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd8a2u32,16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x429395; continue; }
            pc = 0x4292fe; continue;
            }
            0x4292fe => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x42930f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x429395; continue; }
            pc = 0x42931e; continue;
            }
            0x42931e => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(5) { pc = 0x429348; continue; }
            pc = 0x42933c; continue;
            }
            0x42933c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x429395; continue; }
            pc = 0x429344; continue;
            }
            0x429344 => {
            c.set(3,8,0x1u32);
            pc = 0x429388; continue;
            }
            0x429348 => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x429388; continue; }
            pc = 0x429352; continue;
            }
            0x429352 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(1,8,c.get(3,8));
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            if c.cond(4) { pc = 0x429388; continue; }
            pc = 0x42935c; continue;
            }
            0x42935c => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x429378; continue; }
            pc = 0x429369; continue;
            }
            0x429369 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429370);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x429384; continue; }
            pc = 0x429378; continue;
            }
            0x429378 => {
            c.set(5,32,c.get(7,32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x429395; continue; }
            pc = 0x429382; continue;
            }
            0x429382 => {
            pc = 0x429388; continue;
            }
            0x429384 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x429388; continue;
            }
            0x429388 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x42931e; continue; }
            pc = 0x429395; continue;
            }
            0x429395 => {
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x429417; continue; }
            pc = 0x42939e; continue;
            }
            0x42939e => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4293af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x42940e; continue; }
            pc = 0x4293bc; continue;
            }
            0x4293bc => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x429401; continue; }
            pc = 0x4293db; continue;
            }
            0x4293db => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4293f7; continue; }
            pc = 0x4293e8; continue;
            }
            0x4293e8 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4293ef);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x429401; continue; }
            pc = 0x4293f7; continue;
            }
            0x4293f7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(5,32,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x42940e; continue; }
            pc = 0x429401; continue;
            }
            0x429401 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4293bc; continue; }
            pc = 0x42940e; continue;
            }
            0x42940e => {
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x429479; continue; }
            pc = 0x429417; continue;
            }
            0x429417 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x42943a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x476a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m, 0x429441);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,16,c.get(5,16)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x66f353u32,8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x42946f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m, 0x429476);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x429479; continue;
            }
            0x429479 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn select_next_player_convoy_at_sea(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x429490;
    loop {
        match pc {
            0x429490 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4294bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4295ba; continue; }
            pc = 0x4294c5; continue;
            }
            0x4294c5 => {
            { let a = rm(m,0x6dd8a6u32,16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4295ba; continue; }
            pc = 0x4294d2; continue;
            }
            0x4294d2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4295c0; continue; }
            pc = 0x4294f9; continue;
            }
            0x4294f9 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x429523);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x4295c6; continue; }
            pc = 0x42953e; continue;
            }
            0x42953e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,16,rm(m,c.r[1],16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            pc = 0x42954a; continue;
            }
            0x42954a => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),16),16); }
            if c.cond(5) { pc = 0x42956d; continue; }
            pc = 0x429561; continue;
            }
            0x429561 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4295c6; continue; }
            pc = 0x429569; continue;
            }
            0x429569 => {
            c.set(2,8,0x1u32);
            pc = 0x4295af; continue;
            }
            0x42956d => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x4295af; continue; }
            pc = 0x429577; continue;
            }
            0x429577 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.or(a,c.get(3,8),8); c.set(1,8,r); }
            if c.cond(4) { pc = 0x4295af; continue; }
            pc = 0x429581; continue;
            }
            0x429581 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4295af; continue; }
            pc = 0x42958e; continue;
            }
            0x42958e => {
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4295af; continue; }
            pc = 0x4295a7; continue;
            }
            0x4295a7 => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4295c6; continue; }
            pc = 0x4295af; continue;
            }
            0x4295af => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x42954a; continue; }
            pc = 0x4295b8; continue;
            }
            0x4295b8 => {
            pc = 0x4295c6; continue;
            }
            0x4295ba => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x4295c0; continue;
            }
            0x4295c0 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            pc = 0x4295c6; continue;
            }
            0x4295c6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x429657; continue; }
            pc = 0x4295d3; continue;
            }
            0x4295d3 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4295e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(3,32,rm(m,0x6dd894u32,32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x42964e; continue; }
            pc = 0x4295f9; continue;
            }
            0x4295f9 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            pc = 0x4295ff; continue;
            }
            0x4295ff => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x429645; continue; }
            pc = 0x429619; continue;
            }
            0x429619 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x429645; continue; }
            pc = 0x429626; continue;
            }
            0x429626 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x429645; continue; }
            pc = 0x429639; continue;
            }
            0x429639 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x42964e; continue; }
            pc = 0x429645; continue;
            }
            0x429645 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(2) { pc = 0x4295ff; continue; }
            pc = 0x42964e; continue;
            }
            0x42964e => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x4296b4; continue; }
            pc = 0x429657; continue;
            }
            0x429657 => {
            c.set(0,32,c.get(6,32));
            c.set(1,32,rm(m,0x6cc950u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x429674);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x476a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m, 0x42967b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            c.set(1,8,rm(m,0x66f354u32,8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x4296aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m, 0x4296b1);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4296b4; continue;
            }
            0x4296b4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn select_next_player_convoy_in_port(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4296c0;
    loop {
        match pc {
            0x4296c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4296ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4297ea; continue; }
            pc = 0x4296f5; continue;
            }
            0x4296f5 => {
            { let a = rm(m,0x6dd8a4u32,16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4297ea; continue; }
            pc = 0x429702; continue;
            }
            0x429702 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4297f0; continue; }
            pc = 0x429729; continue;
            }
            0x429729 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x429753);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x4297f6; continue; }
            pc = 0x42976e; continue;
            }
            0x42976e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,16,rm(m,c.r[1],16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            pc = 0x42977a; continue;
            }
            0x42977a => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),16),16); }
            if c.cond(5) { pc = 0x42979d; continue; }
            pc = 0x429791; continue;
            }
            0x429791 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4297f6; continue; }
            pc = 0x429799; continue;
            }
            0x429799 => {
            c.set(2,8,0x1u32);
            pc = 0x4297df; continue;
            }
            0x42979d => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4297df; continue; }
            pc = 0x4297a7; continue;
            }
            0x4297a7 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.or(a,c.get(3,8),8); c.set(1,8,r); }
            if c.cond(4) { pc = 0x4297df; continue; }
            pc = 0x4297b1; continue;
            }
            0x4297b1 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4297df; continue; }
            pc = 0x4297be; continue;
            }
            0x4297be => {
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4297df; continue; }
            pc = 0x4297d7; continue;
            }
            0x4297d7 => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4297f6; continue; }
            pc = 0x4297df; continue;
            }
            0x4297df => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x42977a; continue; }
            pc = 0x4297e8; continue;
            }
            0x4297e8 => {
            pc = 0x4297f6; continue;
            }
            0x4297ea => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x4297f0; continue;
            }
            0x4297f0 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            pc = 0x4297f6; continue;
            }
            0x4297f6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x429887; continue; }
            pc = 0x429803; continue;
            }
            0x429803 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x429814);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(3,32,rm(m,0x6dd894u32,32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x42987e; continue; }
            pc = 0x429829; continue;
            }
            0x429829 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            pc = 0x42982f; continue;
            }
            0x42982f => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x429875; continue; }
            pc = 0x429849; continue;
            }
            0x429849 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x429875; continue; }
            pc = 0x429856; continue;
            }
            0x429856 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x429875; continue; }
            pc = 0x429869; continue;
            }
            0x429869 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x42987e; continue; }
            pc = 0x429875; continue;
            }
            0x429875 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(2) { pc = 0x42982f; continue; }
            pc = 0x42987e; continue;
            }
            0x42987e => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x4298e4; continue; }
            pc = 0x429887; continue;
            }
            0x429887 => {
            c.set(0,32,c.get(6,32));
            c.set(1,32,rm(m,0x6cc950u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4298a4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x476a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m, 0x4298ab);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            c.set(1,8,rm(m,0x66f354u32,8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x4298da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m, 0x4298e1);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4298e4; continue;
            }
            0x4298e4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_planner_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x444ba0;
    loop {
        match pc {
            0x444ba0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x66d744u32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x444bc6);
            { let args = []; c.r[0] = host.call(m, 0x444c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_planner_ctor_clear_town_count(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x444bd0;
    loop {
        match pc {
            0x444bd0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x66d744u32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x444bfa);
            { let args = []; c.r[0] = host.call(m, 0x444c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_planner_free_nav_data(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x444c10;
    loop {
        match pc {
            0x444c10 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x1c),16); c.sub(a,c.get(3,16),16); }
            if c.cond(14) { pc = 0x444c42; continue; }
            pc = 0x444c1f; continue;
            }
            0x444c1f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x444c32; continue; }
            pc = 0x444c29; continue;
            }
            0x444c29 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x444c2f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x444c32; continue;
            }
            0x444c32 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x1c),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x444c1f; continue; }
            pc = 0x444c42; continue;
            }
            0x444c42 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x444c56; continue; }
            pc = 0x444c4d; continue;
            }
            0x444c4d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x444c53);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x444c56; continue;
            }
            0x444c56 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x444c69; continue; }
            pc = 0x444c60; continue;
            }
            0x444c60 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x444c66);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x444c69; continue;
            }
            0x444c69 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x444c7c; continue; }
            pc = 0x444c73; continue;
            }
            0x444c73 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x444c79);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x444c7c; continue;
            }
            0x444c7c => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x444c86);
            { let args = []; c.r[0] = host.call(m, 0x444c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_planner_free_town_route_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x444c90;
    loop {
        match pc {
            0x444c90 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x444cdd; continue; }
            pc = 0x444c9b; continue;
            }
            0x444c9b => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            { let a = c.get(7,32); let r = c.imul(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x444cca; continue; }
            pc = 0x444ca8; continue;
            }
            0x444ca8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x444cb7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x444cc3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x444ca8; continue; }
            pc = 0x444cca; continue;
            }
            0x444cca => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x444cd3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x444cdd; continue;
            }
            0x444cdd => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x444d1b; continue; }
            pc = 0x444ce4; continue;
            }
            0x444ce4 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x1e),16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x444d06; continue; }
            pc = 0x444cec; continue;
            }
            0x444cec => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x444cf8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x444cec; continue; }
            pc = 0x444d06; continue;
            }
            0x444d06 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x444d0f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x444d18);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x444d1b; continue;
            }
            0x444d1b => {
            { let a = c.r[6].wrapping_add(0x1e); wm(m,a,16,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_planner_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x448c40;
    loop {
        match pc {
            0x448c40 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x66d744u32); }
            c.push(m, 0x448c4e);
            { let args = []; c.r[0] = host.call(m, 0x444c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x448c5e; continue; }
            pc = 0x448c55; continue;
            }
            0x448c55 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x448c5b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x448c5e; continue;
            }
            0x448c5e => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x448fb0;
    loop {
        match pc {
            0x448fb0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e73au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x697e3cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x448fd9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x2u32);
            c.set(1,32,c.r[6].wrapping_add(0xd8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x448ffc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0xau32);
            c.set(2,32,c.r[6].wrapping_add(0x288));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x44901e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x4u32);
            c.set(0,32,c.r[6].wrapping_add(0xaf8));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x2u32); }
            c.push(m, 0x449040);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0xe58));
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            c.push(m, 0x449062);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x4u32); }
            c.push(m, 0x449084);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x17a0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x449094);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1878));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x4490a4);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1950));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7u32); }
            c.push(m, 0x4490b4);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x11u32);
            c.set(0,32,c.r[6].wrapping_add(0x1a28));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x8u32); }
            c.push(m, 0x4490d6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x9u32); }
            c.push(m,0x4u32);
            c.set(1,32,c.r[6].wrapping_add(0x2884));
            c.push(m,c.get(1,32));
            c.push(m, 0x4490f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x2904); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66da48u32); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xbu32); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x2d18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x2b22); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x44912d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x449157; continue; }
            pc = 0x449142; continue;
            }
            0x449142 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x449142; continue; }
            pc = 0x449157; continue;
            }
            0x449157 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x28a4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28a8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28ac); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b26); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4491b9);
            { let args = []; c.r[0] = host.call(m, 0x449560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_ctor_named(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4491d0;
    loop {
        match pc {
            0x4491d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e83eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4491f8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x2u32);
            c.set(1,32,c.r[6].wrapping_add(0xd8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x44921b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0xau32);
            c.set(2,32,c.r[6].wrapping_add(0x288));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x44923d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x4u32);
            c.set(0,32,c.r[6].wrapping_add(0xaf8));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x2u32); }
            c.push(m, 0x44925f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0xe58));
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            c.push(m, 0x449281);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x4u32); }
            c.push(m, 0x4492a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x17a0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x4492b3);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1878));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x4492c3);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1950));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7u32); }
            c.push(m, 0x4492d3);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x11u32);
            c.set(0,32,c.r[6].wrapping_add(0x1a28));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x8u32); }
            c.push(m, 0x4492f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x9u32); }
            c.push(m,0x4u32);
            c.set(1,32,c.r[6].wrapping_add(0x2884));
            c.push(m,c.get(1,32));
            c.push(m, 0x449314);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x2904); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66da48u32); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xbu32); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x2d18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x2b22); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x44934c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x449376; continue; }
            pc = 0x449361; continue;
            }
            0x449361 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x449361; continue; }
            pc = 0x449376; continue;
            }
            0x449376 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x28a4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28a8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28ac); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b26); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4493d8);
            { let args = []; c.r[0] = host.call(m, 0x449560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4493f0;
    loop {
        match pc {
            0x4493f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e942u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66da48u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xbu32); }
            c.push(m, 0x449420);
            { let args = []; c.r[0] = host.call(m, 0x449560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44942c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x2904));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0xau32); }
            c.push(m, 0x44943f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x64f253u32);
            c.push(m,0x8u32);
            c.set(0,32,c.r[6].wrapping_add(0x2884));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x9u32); }
            c.push(m, 0x449459);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x11u32);
            c.set(1,32,c.r[6].wrapping_add(0x1a28));
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x8u32); }
            c.push(m, 0x449476);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x1950));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x7u32); }
            c.push(m, 0x449486);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1878));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x449496);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x17a0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x4494a6);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4ace40u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x4u32); }
            c.push(m, 0x4494c3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0xe58));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3u32); }
            c.push(m, 0x4494e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x4u32);
            c.set(1,32,c.r[6].wrapping_add(0xaf8));
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x2u32); }
            c.push(m, 0x4494fd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0xau32);
            c.set(2,32,c.r[6].wrapping_add(0x288));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x44951a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x2u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m,0xd8u32);
            c.set(0,32,c.r[6].wrapping_add(0xd8));
            c.push(m,c.get(0,32));
            c.push(m, 0x449537);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x449546);
            { let args = []; c.r[0] = host.call(m, 0x4b1640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_reset_state(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x449560;
    loop {
        match pc {
            0x449560 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x44956c);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xd8));
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x29e4); wm(m,a,16,0x32au32); }
            { let a = c.r[6].wrapping_add(0x29e6); wm(m,a,16,0x3b6u32); }
            c.push(m, 0x449595);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1b0));
            c.push(m, 0x4495a0);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x288));
            c.set(5,32,0xau32);
            pc = 0x4495ab; continue;
            }
            0x4495ab => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4495b2);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4495ab; continue; }
            pc = 0x4495bb; continue;
            }
            0x4495bb => {
            c.set(7,32,c.r[6].wrapping_add(0xe58));
            c.set(5,32,0x3u32);
            pc = 0x4495c6; continue;
            }
            0x4495c6 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4495cd);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4495c6; continue; }
            pc = 0x4495d6; continue;
            }
            0x4495d6 => {
            c.set(7,32,c.r[6].wrapping_add(0xaf8));
            c.set(5,32,0x4u32);
            pc = 0x4495e1; continue;
            }
            0x4495e1 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4495e8);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4495e1; continue; }
            pc = 0x4495f1; continue;
            }
            0x4495f1 => {
            c.set(7,32,c.r[6].wrapping_add(0x10e0));
            c.set(5,32,0x8u32);
            pc = 0x4495fc; continue;
            }
            0x4495fc => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x449603);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4495fc; continue; }
            pc = 0x44960c; continue;
            }
            0x44960c => {
            c.set(1,32,c.r[6].wrapping_add(0x17a0));
            c.push(m, 0x449617);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1878));
            c.push(m, 0x449622);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1950));
            c.push(m, 0x44962d);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x1a28));
            c.set(5,32,0x3u32);
            pc = 0x449638; continue;
            }
            0x449638 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x44963f);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x449638; continue; }
            pc = 0x449648; continue;
            }
            0x449648 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44965b; continue; }
            pc = 0x449652; continue;
            }
            0x449652 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449658);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44965b; continue;
            }
            0x44965b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449674; continue; }
            pc = 0x44966b; continue;
            }
            0x44966b => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449671);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449674; continue;
            }
            0x449674 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44968d; continue; }
            pc = 0x449684; continue;
            }
            0x449684 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44968a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44968d; continue;
            }
            0x44968d => {
            c.set(2,32,0xffffd588u32);
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x2a78));
            c.set(1,32,c.r[6].wrapping_add(0x2a28));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            pc = 0x4496a6; continue;
            }
            0x4496a6 => {
            { let a = c.r[1].wrapping_add(0xffffffc0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0xfffffff0); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            { let a = c.get(7,32); c.sub(a,0x10u32,32); }
            if c.cond(12) { pc = 0x4496a6; continue; }
            pc = 0x4496c3; continue;
            }
            0x4496c3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c4),32));
            { let a = c.r[6].wrapping_add(0x2b0c); wm(m,a,16,0xau32); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x2b1c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b1d); wm(m,a,8,0x1eu32); }
            { let a = c.r[6].wrapping_add(0x2b18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c0); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4496f8; continue; }
            pc = 0x4496ef; continue;
            }
            0x4496ef => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4496f5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4496f8; continue;
            }
            0x4496f8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c8),32));
            { let a = c.r[6].wrapping_add(0x28c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449711; continue; }
            pc = 0x449708; continue;
            }
            0x449708 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44970e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449711; continue;
            }
            0x449711 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28cc),32));
            { let a = c.r[6].wrapping_add(0x28c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44972a; continue; }
            pc = 0x449721; continue;
            }
            0x449721 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449727);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44972a; continue;
            }
            0x44972a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28a4),32));
            { let a = c.r[6].wrapping_add(0x28cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449743; continue; }
            pc = 0x44973a; continue;
            }
            0x44973a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449740);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449743; continue;
            }
            0x449743 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28a8),32));
            { let a = c.r[6].wrapping_add(0x28a4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44975c; continue; }
            pc = 0x449753; continue;
            }
            0x449753 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449759);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44975c; continue;
            }
            0x44975c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28ac),32));
            { let a = c.r[6].wrapping_add(0x28a8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449775; continue; }
            pc = 0x44976c; continue;
            }
            0x44976c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449772);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449775; continue;
            }
            0x449775 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b0),32));
            { let a = c.r[6].wrapping_add(0x28ac); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44978e; continue; }
            pc = 0x449785; continue;
            }
            0x449785 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44978b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44978e; continue;
            }
            0x44978e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.r[6].wrapping_add(0x28b0); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x2908); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4497ad; continue; }
            pc = 0x4497a4; continue;
            }
            0x4497a4 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4497aa);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4497ad; continue;
            }
            0x4497ad => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            if c.cond(6) { pc = 0x4497f1; continue; }
            pc = 0x4497bf; continue;
            }
            0x4497bf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4497e6; continue; }
            pc = 0x4497cc; continue;
            }
            0x4497cc => {
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4497d4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4497da);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(4)); wm(m,a,32,c.get(3,32)); }
            pc = 0x4497e6; continue;
            }
            0x4497e6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2d18),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4497bf; continue; }
            pc = 0x4497f1; continue;
            }
            0x4497f1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28d0),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449804; continue; }
            pc = 0x4497fb; continue;
            }
            0x4497fb => {
            c.push(m,c.get(0,32));
            c.push(m, 0x449801);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449804; continue;
            }
            0x449804 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[6].wrapping_add(0x28d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44981d; continue; }
            pc = 0x449814; continue;
            }
            0x449814 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44981a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x44981d; continue;
            }
            0x44981d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x4498a4; continue; }
            pc = 0x449831; continue;
            }
            0x449831 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x449833; continue;
            }
            0x449833 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4),16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x44988e; continue; }
            pc = 0x449842; continue;
            }
            0x449842 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449852; continue; }
            pc = 0x449849; continue;
            }
            0x449849 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44984f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449852; continue;
            }
            0x449852 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x449872; continue; }
            pc = 0x449869; continue;
            }
            0x449869 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x44986f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x449872; continue;
            }
            0x449872 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(c.r[2]).wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4),16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x449842; continue; }
            pc = 0x44988e; continue;
            }
            0x44988e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x449833; continue; }
            pc = 0x4498a4; continue;
            }
            0x4498a4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4498b7; continue; }
            pc = 0x4498ae; continue;
            }
            0x4498ae => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4498b4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4498b7; continue;
            }
            0x4498b7 => {
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x4498d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_reset_interaction_state(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4498d0;
    loop {
        match pc {
            0x4498d0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x60u32);
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x2b1f); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x2914); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x29e0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x28f8); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x28fa); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[6].wrapping_add(0x2b54));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x290c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x2918); wm(m,a,32,0x5dcu32); }
            { let a = c.r[6].wrapping_add(0x291c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b22); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b1e); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b21); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b23); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2920); wm(m,a,32,0xff687888u32); }
            { let a = c.r[6].wrapping_add(0x2924); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b24); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2928); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b25); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b26); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b27); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x29dc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b50); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x4499a0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xbu32);
            c.set(1,32,c.r[6].wrapping_add(0x2b44));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x2d14); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4499b5);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x2b29); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2b10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2b2b); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_slot3_return_zero(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44ab60;
    loop {
        match pc {
            0x44ab60 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x44ab69);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_select_ships_in_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44d780;
    loop {
        match pc {
            0x44d780 => {
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x44d79f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x44dcd9; continue; }
            pc = 0x44d7b9; continue;
            }
            0x44d7b9 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x44d7cf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            if c.cond(14) { pc = 0x44d80e; continue; }
            pc = 0x44d7eb; continue;
            }
            0x44d7eb => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44d80e; continue; }
            pc = 0x44d7f0; continue;
            }
            0x44d7f0 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            pc = 0x44d80e; continue;
            }
            0x44d80e => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x44d81e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x44db57; continue; }
            pc = 0x44d83e; continue;
            }
            0x44d83e => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x44d842; continue;
            }
            0x44d842 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x44d872; continue; }
            pc = 0x44d861; continue;
            }
            0x44d861 => {
            { let a = c.get(0,16); c.sub(a,0x16u32,16); }
            if c.cond(4) { pc = 0x44d872; continue; }
            pc = 0x44d867; continue;
            }
            0x44d867 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            pc = 0x44db3d; continue;
            }
            0x44d872 => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x50),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x52),16));
            { let a = c.get(2,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(0,16)); }
            if c.cond(4) { pc = 0x44d95b; continue; }
            pc = 0x44d88a; continue;
            }
            0x44d88a => {
            c.set(0,32,c.get(2,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,0x16u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,16); c.sub(a,0x12u32,16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,16); c.sub(a,0xfu32,16); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(3,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(3,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44d957; continue; }
            pc = 0x44d916; continue;
            }
            0x44d916 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x44d933; continue; }
            pc = 0x44d920; continue;
            }
            0x44d920 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)),16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x44d933; continue; }
            pc = 0x44d92a; continue;
            }
            0x44d92a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x44d920; continue; }
            pc = 0x44d933; continue;
            }
            0x44d933 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x44d95b; continue; }
            pc = 0x44d93f; continue;
            }
            0x44d93f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d95b; continue;
            }
            0x44d957 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x44d95b; continue;
            }
            0x44d95b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x44da46; continue; }
            pc = 0x44d969; continue;
            }
            0x44d969 => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(5,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,16); c.sub(a,0x16u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,0x12u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,0xfu32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(3,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x44da46; continue; }
            pc = 0x44da01; continue;
            }
            0x44da01 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x44da23; continue; }
            pc = 0x44da0b; continue;
            }
            0x44da0b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x24),16),16); }
            if c.cond(4) { pc = 0x44da23; continue; }
            pc = 0x44da1a; continue;
            }
            0x44da1a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x44da0b; continue; }
            pc = 0x44da23; continue;
            }
            0x44da23 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); }
            if c.cond(5) { pc = 0x44da46; continue; }
            pc = 0x44da29; continue;
            }
            0x44da29 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x24),16));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x44da46; continue;
            }
            0x44da46 => {
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x1e),16));
            c.set(7,16,rm(m,c.r[3].wrapping_add(0x22),16));
            c.set(3,16,rm(m,c.r[3].wrapping_add(0x134),16));
            { let a = c.get(7,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(7,16,r); }
            { let a = c.get(3,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x44da6d; continue; }
            pc = 0x44da5f; continue;
            }
            0x44da5f => {
            { let a = c.get(3,16); c.sub(a,0x16u32,16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x44da75; continue; }
            pc = 0x44da6d; continue;
            }
            0x44da6d => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            pc = 0x44da75; continue;
            }
            0x44da75 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(5,32,sx(c.get(5,16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(7,32,sx(c.get(7,16),16)&0xffffffffu32);
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.and(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.and(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.and(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44db2b; continue; }
            pc = 0x44dad9; continue;
            }
            0x44dad9 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x44db00; continue; }
            pc = 0x44dae7; continue;
            }
            0x44dae7 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44db04; continue; }
            pc = 0x44daf5; continue;
            }
            0x44daf5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x44dae7; continue; }
            pc = 0x44dafe; continue;
            }
            0x44dafe => {
            pc = 0x44db04; continue;
            }
            0x44db00 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x44db04; continue;
            }
            0x44db04 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); }
            if c.cond(5) { pc = 0x44db2b; continue; }
            pc = 0x44db0a; continue;
            }
            0x44db0a => {
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[7].wrapping_add(0x2));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44db15);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.r[7]; wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x44db2b; continue;
            }
            0x44db2b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            pc = 0x44db3d; continue;
            }
            0x44db3d => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x44d842; continue; }
            pc = 0x44db55; continue;
            }
            0x44db55 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x44db57; continue;
            }
            0x44db57 => {
            { let a = rm(m,c.r[4].wrapping_add(0x30),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x44db8c; continue; }
            pc = 0x44db5d; continue;
            }
            0x44db5d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x44db8c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x44db8c; continue;
            }
            0x44db8c => {
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x44dc83; continue; }
            pc = 0x44db95; continue;
            }
            0x44db95 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x44dc04; continue; }
            pc = 0x44dbba; continue;
            }
            0x44dbba => {
            c.set(1,8,rm(m,0x66f354u32,8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x1u32);
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x44dc04; continue; }
            pc = 0x44dbcd; continue;
            }
            0x44dbcd => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x8),16));
            pc = 0x44dbe4; continue;
            }
            0x44dbe4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x8),16),16); }
            if c.cond(5) { pc = 0x44dbff; continue; }
            pc = 0x44dbf8; continue;
            }
            0x44dbf8 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x44dbe4; continue; }
            pc = 0x44dbfd; continue;
            }
            0x44dbfd => {
            pc = 0x44dc04; continue;
            }
            0x44dbff => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            pc = 0x44dc04; continue;
            }
            0x44dc04 => {
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x44dc0c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x44dc16);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44dc75; continue; }
            pc = 0x44dc21; continue;
            }
            0x44dc21 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x10),16));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.set(0,8,rm(m,0x66ecf5u32,8));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x50));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc950u32,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0x2u32); }
            c.push(m, 0x44dc64);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x475e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.push(m, 0x44dc6a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x44dc75 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            pc = 0x44dcbb; continue;
            }
            0x44dc83 => {
            if c.cond(5) { pc = 0x44dcae; continue; }
            pc = 0x44dc85; continue;
            }
            0x44dc85 => {
            c.push(m,0x2u32);
            c.push(m, 0x44dc8c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[5],16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,0x66f353u32,8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            pc = 0x44dcbb; continue;
            }
            0x44dcae => {
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            pc = 0x44dcbb; continue;
            }
            0x44dcbb => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x134),32); c.push(m, 0x44dcce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.push(m, 0x44dcd4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x44dcd9; continue;
            }
            0x44dcd9 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_slot54_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44f670;
    loop {
        match pc {
            0x44f670 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_free_cached_route(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44f940;
    loop {
        match pc {
            0x44f940 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xa4),32));
            { let a = c.r[1].wrapping_add(0x2b22); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x44f973; continue; }
            pc = 0x44f95c; continue;
            }
            0x44f95c => {
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x44f96a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x44f970);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x44f973; continue;
            }
            0x44f973 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_select_player_ships_in_town_at_point(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44f980;
    loop {
        match pc {
            0x44f980 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x44fa44; continue; }
            pc = 0x44f99d; continue;
            }
            0x44f99d => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x44fa44; continue; }
            pc = 0x44f9b1; continue;
            }
            0x44f9b1 => {
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x44f9d6; continue;
            }
            0x44f9d6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.set(0,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x6ddba8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x6ddbac),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x37));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,c.r[0].wrapping_add(0x19));
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.and(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(6,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x44fa50; continue; }
            pc = 0x44fa31; continue;
            }
            0x44fa31 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x44f9d6; continue; }
            pc = 0x44fa44; continue;
            }
            0x44fa44 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x44fa50 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x44fa63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x6dd894u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x44fb32; continue; }
            pc = 0x44fa7f; continue;
            }
            0x44fa7f => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x44fa89; continue;
            }
            0x44fa89 => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x44faa9; continue; }
            pc = 0x44fa9a; continue;
            }
            0x44fa9a => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x44faa9; continue; }
            pc = 0x44faa4; continue;
            }
            0x44faa4 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x44faa9; continue;
            }
            0x44faa9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x44fa89; continue; }
            pc = 0x44fab3; continue;
            }
            0x44fab3 => {
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x44faf5; continue; }
            pc = 0x44fab8; continue;
            }
            0x44fab8 => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            { let a = c.r[4].wrapping_add(0x31); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,0x66ecf6u32,8));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            c.push(m, 0x44fae9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x475e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x44faf5 => {
            if c.cond(5) { pc = 0x44fb32; continue; }
            pc = 0x44faf7; continue;
            }
            0x44faf7 => {
            c.push(m,0x2u32);
            c.push(m, 0x44fafe);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x34),16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,0x66f353u32,8));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x134),32); c.push(m, 0x44fb32); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x44fb32; continue;
            }
            0x44fb32 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_handle_point_click(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44fb40;
    loop {
        match pc {
            0x44fb40 => {
            { let a = c.get(4,32); let r = c.sub(a,0xa8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x6e); wm(m,a,16,c.get(6,16)); }
            c.push(m, 0x44fb6d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x5c));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,0x0u32); }
            c.push(m, 0x44fb8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x4501c4; continue; }
            pc = 0x44fba5; continue;
            }
            0x44fba5 => {
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x451057; continue; }
            pc = 0x44fbae; continue;
            }
            0x44fbae => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x15c),8));
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x451057; continue; }
            pc = 0x44fbe1; continue;
            }
            0x44fbe1 => {
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x50),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x52),16));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x44febb; continue; }
            pc = 0x44fbff; continue;
            }
            0x44fbff => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x44fc3a; continue; }
            pc = 0x44fc1a; continue;
            }
            0x44fc1a => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            pc = 0x44fc3a; continue;
            }
            0x44fc3a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,0x12u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15c),8));
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x44febb; continue; }
            pc = 0x44fc71; continue;
            }
            0x44fc71 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x44fd33; continue; }
            pc = 0x44fc8a; continue;
            }
            0x44fc8a => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451078)); continue;
            }
            0x44fc91 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb4),32));
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x44fd33; continue; }
            pc = 0x44fca0; continue;
            }
            0x44fca0 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x44fd33; continue; }
            pc = 0x44fcab; continue;
            }
            0x44fcab => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            pc = 0x44fd33; continue;
            }
            0x44fcd2 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb0),32));
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x44fd33; continue; }
            pc = 0x44fcdd; continue;
            }
            0x44fcdd => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x44fd33; continue; }
            pc = 0x44fce4; continue;
            }
            0x44fce4 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            pc = 0x44fd33; continue;
            }
            0x44fd0b => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xac),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x44fd2d; continue;
            }
            0x44fd1d => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            c.push(m,c.get(2,32));
            pc = 0x44fd2d; continue;
            }
            0x44fd2d => {
            c.push(m,c.get(7,32));
            c.push(m, 0x44fd33);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45c080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44fd33; continue;
            }
            0x44fd33 => {
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[3].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44febb; continue; }
            pc = 0x44fdb7; continue;
            }
            0x44fdb7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x44fe12; continue; }
            pc = 0x44fdcb; continue;
            }
            0x44fdcb => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451088)); continue;
            }
            0x44fdd2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x44fe16; continue;
            }
            0x44fde2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x44fe16; continue;
            }
            0x44fdf2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x44fe16; continue;
            }
            0x44fe02 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x44fe16; continue;
            }
            0x44fe12 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x44fe16; continue;
            }
            0x44fe16 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,0x0u32); }
            c.push(m, 0x44fe2b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,0x38u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44fe96; continue; }
            pc = 0x44fe54; continue;
            }
            0x44fe54 => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x80000003u32,32); c.set(7,32,r); }
            if c.cond(9) { pc = 0x44fe7c; continue; }
            pc = 0x44fe77; continue;
            }
            0x44fe77 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.or(a,0xfffffffcu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x44fe7c; continue;
            }
            0x44fe7c => {
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x44fe96; continue; }
            pc = 0x44fe91; continue;
            }
            0x44fe91 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x44fe96; continue;
            }
            0x44fe96 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x44fe9f);
            { let args = []; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44febb; continue; }
            pc = 0x44feae; continue;
            }
            0x44feae => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            pc = 0x44febb; continue;
            }
            0x44febb => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1b),8));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45016b; continue; }
            pc = 0x44fedd; continue;
            }
            0x44fedd => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x44ff1e; continue; }
            pc = 0x44fefe; continue;
            }
            0x44fefe => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            pc = 0x44ff1e; continue;
            }
            0x44ff1e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0x12u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15c),8));
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x45016b; continue; }
            pc = 0x44ff55; continue;
            }
            0x44ff55 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x450005; continue; }
            pc = 0x44ff6e; continue;
            }
            0x44ff6e => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451098)); continue;
            }
            0x44ff75 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb4),32));
            pc = 0x44ffbc; continue;
            }
            0x44ff7d => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb0),32));
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x450005; continue; }
            pc = 0x44ff88; continue;
            }
            0x44ff88 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x450005; continue; }
            pc = 0x44ff8f; continue;
            }
            0x44ff8f => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            pc = 0x450005; continue;
            }
            0x44ffb6 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xac),32));
            pc = 0x44ffbc; continue;
            }
            0x44ffbc => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x450005; continue; }
            pc = 0x44ffc1; continue;
            }
            0x44ffc1 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x450005; continue; }
            pc = 0x44ffc8; continue;
            }
            0x44ffc8 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            pc = 0x450005; continue;
            }
            0x44ffef => {
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x450005);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45c080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x450005; continue;
            }
            0x450005 => {
            c.set(2,32,sx(rm(m,c.r[3].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45016b; continue; }
            pc = 0x450089; continue;
            }
            0x450089 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x4500ce; continue; }
            pc = 0x45009d; continue;
            }
            0x45009d => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4510a8)); continue;
            }
            0x4500a4 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb4),32));
            pc = 0x4500ca; continue;
            }
            0x4500ac => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x4500ce; continue;
            }
            0x4500b8 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4500ce; continue;
            }
            0x4500c4 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            pc = 0x4500ca; continue;
            }
            0x4500ca => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4500ce; continue;
            }
            0x4500ce => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x7c));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4500e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,0x38u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45014c; continue; }
            pc = 0x45010d; continue;
            }
            0x45010d => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x80000003u32,32); c.set(7,32,r); }
            if c.cond(9) { pc = 0x450135; continue; }
            pc = 0x450130; continue;
            }
            0x450130 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.or(a,0xfffffffcu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x450135; continue;
            }
            0x450135 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x45014c; continue; }
            pc = 0x45014a; continue;
            }
            0x45014a => {
            c.set(3,8,0x1u32);
            pc = 0x45014c; continue;
            }
            0x45014c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x450159);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45016b; continue; }
            pc = 0x450160; continue;
            }
            0x450160 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            pc = 0x45017b; continue;
            }
            0x45016b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1b),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x451057; continue; }
            pc = 0x450177; continue;
            }
            0x450177 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x45017b; continue;
            }
            0x45017b => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x450184);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xeu32); }
            c.set(2,32,c.get(6,32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4501b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[1].wrapping_add(0x2b14); wm(m,a,16,c.get(6,16)); }
            pc = 0x451057; continue;
            }
            0x4501c4 => {
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x451057; continue; }
            pc = 0x4501cd; continue;
            }
            0x4501cd => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2b1e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x450712; continue; }
            pc = 0x4501db; continue;
            }
            0x4501db => {
            { let a = rm(m,c.r[3].wrapping_add(0x291c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x45021b; continue; }
            pc = 0x4501e3; continue;
            }
            0x4501e3 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4501ec);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xbu32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x45021b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45021b; continue;
            }
            0x45021b => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x451057; continue; }
            pc = 0x450231; continue;
            }
            0x450231 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x450245);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            if c.cond(14) { pc = 0x4502a6; continue; }
            pc = 0x45025a; continue;
            }
            0x45025a => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45026a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[5]),8));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x450299; continue; }
            pc = 0x450294; continue;
            }
            0x450294 => {
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x450299; continue;
            }
            0x450299 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x45025a; continue; }
            pc = 0x4502a2; continue;
            }
            0x4502a2 => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            pc = 0x4502a6; continue;
            }
            0x4502a6 => {
            c.set(3,32,c.r[6].wrapping_add(c.r[6]));
            c.push(m,c.get(3,32));
            c.push(m, 0x4502af);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4502b9);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,0x66f351u32,8));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4502df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x450698; continue; }
            pc = 0x4502f7; continue;
            }
            0x4502f7 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x450338; continue; }
            pc = 0x450318; continue;
            }
            0x450318 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(6,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            pc = 0x450338; continue;
            }
            0x450338 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450350; continue; }
            pc = 0x450345; continue;
            }
            0x450345 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x4),16));
            pc = 0x450684; continue;
            }
            0x450350 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            if c.cond(7) { pc = 0x4503fe; continue; }
            pc = 0x45036b; continue;
            }
            0x45036b => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4510b8)); continue;
            }
            0x450372 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xb4),32));
            pc = 0x450394; continue;
            }
            0x45037e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xb0),32));
            pc = 0x4503cf; continue;
            }
            0x45038a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xac),32));
            pc = 0x450394; continue;
            }
            0x450394 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x4503fe; continue; }
            pc = 0x450399; continue;
            }
            0x450399 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4503fe; continue; }
            pc = 0x4503a0; continue;
            }
            0x4503a0 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4503fe; continue;
            }
            0x4503c5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xb8),32));
            pc = 0x4503cf; continue;
            }
            0x4503cf => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x4503fe; continue; }
            pc = 0x4503d4; continue;
            }
            0x4503d4 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4503fe; continue; }
            pc = 0x4503db; continue;
            }
            0x4503db => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4503fe; continue;
            }
            0x4503fe => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(6,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x450456; continue; }
            pc = 0x450448; continue;
            }
            0x450448 => {
            { let a = c.get(6,16); c.sub(a,0x16u32,16); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x45045e; continue; }
            pc = 0x450456; continue;
            }
            0x450456 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x1u32); }
            pc = 0x45045e; continue;
            }
            0x45045e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x450676; continue; }
            pc = 0x4504b5; continue;
            }
            0x4504b5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x45050a; continue; }
            pc = 0x4504c9; continue;
            }
            0x4504c9 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4510c8)); continue;
            }
            0x4504d0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xb4),32));
            pc = 0x450506; continue;
            }
            0x4504dc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x45050a; continue;
            }
            0x4504ec => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x45050a; continue;
            }
            0x4504fc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xb8),32));
            pc = 0x450506; continue;
            }
            0x450506 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x45050a; continue;
            }
            0x45050a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x7c));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x0u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x450523);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x38u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450593; continue; }
            pc = 0x45054c; continue;
            }
            0x45054c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x80000003u32,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x450577; continue; }
            pc = 0x450572; continue;
            }
            0x450572 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xfffffffcu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x450577; continue;
            }
            0x450577 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x450593; continue; }
            pc = 0x45058e; continue;
            }
            0x45058e => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x450593; continue;
            }
            0x450593 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4505a0);
            { let args = []; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x450676; continue; }
            pc = 0x4505af; continue;
            }
            0x4505af => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x450637; continue; }
            pc = 0x4505c0; continue;
            }
            0x4505c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            pc = 0x4505c4; continue;
            }
            0x4505c4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),16));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4505d5; continue; }
            pc = 0x4505ce; continue;
            }
            0x4505ce => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4505c4; continue; }
            pc = 0x4505d3; continue;
            }
            0x4505d3 => {
            pc = 0x450637; continue;
            }
            0x4505d5 => {
            { let a = rm(m,c.r[4].wrapping_add(0x44),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x45062f; continue; }
            pc = 0x4505dc; continue;
            }
            0x4505dc => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4505e2);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(c.r[7]).wrapping_add(0xfffffffe));
            c.push(m,c.get(0,32));
            c.push(m, 0x4505ec);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6]));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4505fd);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,c.r[6].wrapping_add(0x1));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[5].wrapping_add(c.r[6].wrapping_mul(2)));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x450619);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,0x66f351u32,8));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[7].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(2,8)); }
            pc = 0x450633; continue;
            }
            0x45062f => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            pc = 0x450633; continue;
            }
            0x450633 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            pc = 0x450637; continue;
            }
            0x450637 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x450676; continue; }
            pc = 0x45063b; continue;
            }
            0x45063b => {
            c.push(m,c.get(5,32));
            c.push(m, 0x450641);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[7].wrapping_add(0x1));
            c.set(0,32,c.r[6].wrapping_add(c.r[6]));
            c.push(m,c.get(0,32));
            c.push(m, 0x45064d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45065d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(c.r[5]); wm(m,a,16,c.get(3,16)); }
            c.set(2,8,rm(m,0x66f351u32,8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(2,8)); }
            pc = 0x450676; continue;
            }
            0x450676 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x4),16));
            pc = 0x450684; continue;
            }
            0x450684 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4502f7; continue; }
            pc = 0x450698; continue;
            }
            0x450698 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4506c3; continue; }
            pc = 0x4506a2; continue;
            }
            0x4506a2 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x4506b5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.push(m, 0x4506bb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x451057; continue;
            }
            0x4506c3 => {
            { let a = rm(m,c.r[4].wrapping_add(0x44),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450704; continue; }
            pc = 0x4506c9; continue;
            }
            0x4506c9 => {
            c.set(1,8,rm(m,0x66f351u32,8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x4506ee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.push(m, 0x4506f4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[1].wrapping_add(0x2b0c); wm(m,a,16,0xau32); }
            pc = 0x450704; continue;
            }
            0x450704 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x45070a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x451057; continue;
            }
            0x450712 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x451057; continue; }
            pc = 0x450728; continue;
            }
            0x450728 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x450733);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,0x6dd894u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(6,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x451022; continue; }
            pc = 0x450755; continue;
            }
            0x450755 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x45077e; continue; }
            pc = 0x450774; continue;
            }
            0x450774 => {
            { let a = c.get(0,16); c.sub(a,0x16u32,16); }
            if c.cond(5) { pc = 0x450f26; continue; }
            pc = 0x45077e; continue;
            }
            0x45077e => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x50),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x52),16));
            { let a = c.get(1,16); c.sub(a,c.get(6,16),16); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x450a0d; continue; }
            pc = 0x450798; continue;
            }
            0x450798 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4507d3; continue; }
            pc = 0x4507b3; continue;
            }
            0x4507b3 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            pc = 0x4507d3; continue;
            }
            0x4507d3 => {
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x134),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,16); c.sub(a,0x16u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,16); c.sub(a,0x12u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x450a0d; continue; }
            pc = 0x4507ff; continue;
            }
            0x4507ff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(5,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x45087a; continue; }
            pc = 0x450814; continue;
            }
            0x450814 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4510d8)); continue;
            }
            0x45081b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xb4),32));
            pc = 0x450849; continue;
            }
            0x450827 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xb0),32));
            pc = 0x450849; continue;
            }
            0x450833 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xac),32));
            pc = 0x450849; continue;
            }
            0x45083f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xb8),32));
            pc = 0x450849; continue;
            }
            0x450849 => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x45087a; continue; }
            pc = 0x45084e; continue;
            }
            0x45084e => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45087a; continue; }
            pc = 0x450855; continue;
            }
            0x450855 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x45087a; continue;
            }
            0x45087a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[3].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x450a09; continue; }
            pc = 0x450902; continue;
            }
            0x450902 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x450947; continue; }
            pc = 0x450916; continue;
            }
            0x450916 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4510e8)); continue;
            }
            0x45091d => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb4),32));
            pc = 0x450943; continue;
            }
            0x450925 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x450947; continue;
            }
            0x450931 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x450947; continue;
            }
            0x45093d => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            pc = 0x450943; continue;
            }
            0x450943 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x450947; continue;
            }
            0x450947 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x7c));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x45095d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x38u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4509c6; continue; }
            pc = 0x450986; continue;
            }
            0x450986 => {
            c.set(0,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0x80000003u32,32); c.set(5,32,r); }
            if c.cond(9) { pc = 0x4509ad; continue; }
            pc = 0x4509a8; continue;
            }
            0x4509a8 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xfffffffcu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x4509ad; continue;
            }
            0x4509ad => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4509c6; continue; }
            pc = 0x4509c4; continue;
            }
            0x4509c4 => {
            c.set(3,8,0x1u32);
            pc = 0x4509c6; continue;
            }
            0x4509c6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4509d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x450a09; continue; }
            pc = 0x4509da; continue;
            }
            0x4509da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x4509fc; continue; }
            pc = 0x4509f1; continue;
            }
            0x4509f1 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)),16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x4509fc; continue; }
            pc = 0x4509f7; continue;
            }
            0x4509f7 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4509f1; continue; }
            pc = 0x4509fc; continue;
            }
            0x4509fc => {
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x450a09; continue; }
            pc = 0x450a00; continue;
            }
            0x450a00 => {
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x450a09; continue;
            }
            0x450a09 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x450a0d; continue;
            }
            0x450a0d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x450cb3; continue; }
            pc = 0x450a1e; continue;
            }
            0x450a1e => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x450a5f; continue; }
            pc = 0x450a3f; continue;
            }
            0x450a3f => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            pc = 0x450a5f; continue;
            }
            0x450a5f => {
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x134),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,16); c.sub(a,0x16u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,16); c.sub(a,0x12u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x450cb3; continue; }
            pc = 0x450a8b; continue;
            }
            0x450a8b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(5,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x450b08; continue; }
            pc = 0x450aa0; continue;
            }
            0x450aa0 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4510f8)); continue;
            }
            0x450aa7 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            pc = 0x450ad5; continue;
            }
            0x450ab3 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            pc = 0x450ad5; continue;
            }
            0x450abf => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            pc = 0x450ad5; continue;
            }
            0x450acb => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            pc = 0x450ad5; continue;
            }
            0x450ad5 => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x450b0c; continue; }
            pc = 0x450ada; continue;
            }
            0x450ada => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x450b0c; continue; }
            pc = 0x450ae1; continue;
            }
            0x450ae1 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x450b0c; continue;
            }
            0x450b08 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x450b0c; continue;
            }
            0x450b0c => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0xbc),32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450caf; continue; }
            pc = 0x450b86; continue;
            }
            0x450b86 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x450be1; continue; }
            pc = 0x450b9a; continue;
            }
            0x450b9a => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451108)); continue;
            }
            0x450ba1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450be5; continue;
            }
            0x450bb1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450be5; continue;
            }
            0x450bc1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450be5; continue;
            }
            0x450bd1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450be5; continue;
            }
            0x450be1 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x450be5; continue;
            }
            0x450be5 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x90));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,0x0u32); }
            c.push(m, 0x450bfd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x38u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450c6c; continue; }
            pc = 0x450c26; continue;
            }
            0x450c26 => {
            c.set(0,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0x80000003u32,32); c.set(5,32,r); }
            if c.cond(9) { pc = 0x450c4d; continue; }
            pc = 0x450c48; continue;
            }
            0x450c48 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xfffffffcu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x450c4d; continue;
            }
            0x450c4d => {
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x90),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x450c6c; continue; }
            pc = 0x450c67; continue;
            }
            0x450c67 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x450c6c; continue;
            }
            0x450c6c => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x450c75);
            { let args = []; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x450caf; continue; }
            pc = 0x450c80; continue;
            }
            0x450c80 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x450ca2; continue; }
            pc = 0x450c97; continue;
            }
            0x450c97 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)),16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x450ca2; continue; }
            pc = 0x450c9d; continue;
            }
            0x450c9d => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x450c97; continue; }
            pc = 0x450ca2; continue;
            }
            0x450ca2 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x450caf; continue; }
            pc = 0x450ca6; continue;
            }
            0x450ca6 => {
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x450caf; continue;
            }
            0x450caf => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x450cb3; continue;
            }
            0x450cb3 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x450cce; continue; }
            pc = 0x450cc4; continue;
            }
            0x450cc4 => {
            { let a = c.get(0,16); c.sub(a,0x16u32,16); }
            if c.cond(5) { pc = 0x450f1c; continue; }
            pc = 0x450cce; continue;
            }
            0x450cce => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x450d06; continue; }
            pc = 0x450cdd; continue;
            }
            0x450cdd => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x450d06; continue;
            }
            0x450d06 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1),8));
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x450d71; continue; }
            pc = 0x450d1b; continue;
            }
            0x450d1b => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451118)); continue;
            }
            0x450d22 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            pc = 0x450d40; continue;
            }
            0x450d2a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            pc = 0x450d40; continue;
            }
            0x450d32 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            pc = 0x450d40; continue;
            }
            0x450d3a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            pc = 0x450d40; continue;
            }
            0x450d40 => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x450d71; continue; }
            pc = 0x450d45; continue;
            }
            0x450d45 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x450d71; continue; }
            pc = 0x450d4c; continue;
            }
            0x450d4c => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x450d71; continue;
            }
            0x450d71 => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0xbc),32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450f18; continue; }
            pc = 0x450deb; continue;
            }
            0x450deb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x450e46; continue; }
            pc = 0x450dff; continue;
            }
            0x450dff => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x451128)); continue;
            }
            0x450e06 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450e4a; continue;
            }
            0x450e16 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450e4a; continue;
            }
            0x450e26 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450e4a; continue;
            }
            0x450e36 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xb8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x450e4a; continue;
            }
            0x450e46 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x450e4a; continue;
            }
            0x450e4a => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0xa4));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,0x0u32); }
            c.push(m, 0x450e62);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x38u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,0x34u32,32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x450ed1; continue; }
            pc = 0x450e8b; continue;
            }
            0x450e8b => {
            c.set(0,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0x80000003u32,32); c.set(5,32,r); }
            if c.cond(9) { pc = 0x450eb2; continue; }
            pc = 0x450ead; continue;
            }
            0x450ead => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xfffffffcu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x450eb2; continue;
            }
            0x450eb2 => {
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xa4),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x450ed1; continue; }
            pc = 0x450ecc; continue;
            }
            0x450ecc => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x450ed1; continue;
            }
            0x450ed1 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x450eda);
            { let args = []; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x450f18; continue; }
            pc = 0x450ee5; continue;
            }
            0x450ee5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x450f0b; continue; }
            pc = 0x450efc; continue;
            }
            0x450efc => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x450f0b; continue; }
            pc = 0x450f06; continue;
            }
            0x450f06 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x450efc; continue; }
            pc = 0x450f0b; continue;
            }
            0x450f0b => {
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x450f18; continue; }
            pc = 0x450f0f; continue;
            }
            0x450f0f => {
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x450f18; continue;
            }
            0x450f18 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x450f1c; continue;
            }
            0x450f1c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,0x6dd894u32,32));
            pc = 0x450f26; continue;
            }
            0x450f26 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x450755; continue; }
            pc = 0x450f40; continue;
            }
            0x450f40 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x450fa5; continue; }
            pc = 0x450f49; continue;
            }
            0x450f49 => {
            c.set(6,32,c.r[5].wrapping_add(c.r[5]));
            c.push(m,c.get(6,32));
            c.push(m, 0x450f53);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x450f61);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,8,rm(m,0x66ecf5u32,8));
            c.set(0,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x68));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x6e); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x450fa0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x475e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x451057; continue;
            }
            0x450fa5 => {
            if c.cond(5) { pc = 0x451022; continue; }
            pc = 0x450fa7; continue;
            }
            0x450fa7 => {
            c.push(m,0x2u32);
            c.push(m, 0x450fae);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,16,rm(m,c.r[6],16));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            c.set(0,16,rm(m,c.r[6],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x450ff1; continue; }
            pc = 0x450fe5; continue;
            }
            0x450fe5 => {
            c.set(2,8,rm(m,0x66f354u32,8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(2,8)); }
            pc = 0x450ffa; continue;
            }
            0x450ff1 => {
            c.set(0,8,rm(m,0x66f353u32,8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(0,8)); }
            pc = 0x450ffa; continue;
            }
            0x450ffa => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x45100d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x451020);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x451057; continue;
            }
            0x451022 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x451057; continue; }
            pc = 0x45102d; continue;
            }
            0x45102d => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x45104e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[3].wrapping_add(0x2b0c); wm(m,a,16,0xau32); }
            pc = 0x451057; continue;
            }
            0x451057 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x451061);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xa8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_hit_test_battle_marker(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x451140;
    loop {
        match pc {
            0x451140 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4511f2; continue; }
            pc = 0x45115f; continue;
            }
            0x45115f => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            c.set(5,32,c.r[2].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4511a3; continue;
            }
            0x45119b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4511a3; continue;
            }
            0x4511a3 => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0xffffffec));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.and(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[7].wrapping_add(0xffffffec));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.and(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4511fe; continue; }
            pc = 0x4511dc; continue;
            }
            0x4511dc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x45119b; continue; }
            pc = 0x4511f2; continue;
            }
            0x4511f2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4511fe => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x451216);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611fc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_get_hint_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4528a0;
    loop {
        match pc {
            0x4528a0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x2904u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4528bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_hover_town_status_icon(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4528d0;
    loop {
        match pc {
            0x4528d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,0x698528u32,8));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x4528ec; continue; }
            pc = 0x4528e3; continue;
            }
            0x4528e3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4528ec => {
            c.push(m, 0x4528f1);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4528fe; continue; }
            pc = 0x4528f5; continue;
            }
            0x4528f5 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4528fe => {
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x452914); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.push(m, 0x45292b);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x452932);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,c.r[6].wrapping_add(0x13));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            c.set(1,32,c.r[0].wrapping_add(0x13));
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            c.set(3,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x452f51; continue; }
            pc = 0x452973; continue;
            }
            0x452973 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(15) { pc = 0x452aa2; continue; }
            pc = 0x452982; continue;
            }
            0x452982 => {
            if c.cond(4) { pc = 0x452a84; continue; }
            pc = 0x452988; continue;
            }
            0x452988 => {
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(15) { pc = 0x452a25; continue; }
            pc = 0x452993; continue;
            }
            0x452993 => {
            if c.cond(4) { pc = 0x452a08; continue; }
            pc = 0x452995; continue;
            }
            0x452995 => {
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(4) { pc = 0x4529ea; continue; }
            pc = 0x45299a; continue;
            }
            0x45299a => {
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(4) { pc = 0x4529cc; continue; }
            pc = 0x45299f; continue;
            }
            0x45299f => {
            { let a = c.get(0,32); c.sub(a,0x200u32,32); }
            if c.cond(5) { pc = 0x452f45; continue; }
            pc = 0x4529aa; continue;
            }
            0x4529aa => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x6b2a28u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(0x2904));
            c.push(m, 0x4529c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4529cc => {
            c.set(1,32,rm(m,0x69b6a4u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m, 0x4529de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4529ea => {
            c.set(2,32,rm(m,0x69b69cu32,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m,c.get(2,32));
            c.push(m, 0x4529fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452a08 => {
            c.set(0,32,rm(m,0x6b2a2cu32,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m,c.get(0,32));
            c.push(m, 0x452a19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452a25 => {
            { let a = c.get(0,32); c.sub(a,0x800u32,32); }
            if c.cond(4) { pc = 0x452a66; continue; }
            pc = 0x452a2c; continue;
            }
            0x452a2c => {
            { let a = c.get(0,32); c.sub(a,0x1000u32,32); }
            if c.cond(4) { pc = 0x452a49; continue; }
            pc = 0x452a33; continue;
            }
            0x452a33 => {
            { let a = c.get(0,32); c.sub(a,0x8000u32,32); }
            if c.cond(5) { pc = 0x452f45; continue; }
            pc = 0x452a3e; continue;
            }
            0x452a3e => {
            c.set(1,32,rm(m,0x69b698u32,32));
            pc = 0x452f35; continue;
            }
            0x452a49 => {
            c.set(0,32,rm(m,0x69b688u32,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m,c.get(0,32));
            c.push(m, 0x452a5a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452a66 => {
            c.set(1,32,rm(m,0x69b6a0u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m, 0x452a78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452a84 => {
            c.set(2,32,rm(m,0x69b694u32,32));
            c.set(1,32,c.r[5].wrapping_add(0x2904));
            c.push(m,c.get(2,32));
            c.push(m, 0x452a96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452aa2 => {
            { let a = c.get(0,32); c.sub(a,0x200000u32,32); }
            if c.cond(15) { pc = 0x452e1c; continue; }
            pc = 0x452aad; continue;
            }
            0x452aad => {
            if c.cond(4) { pc = 0x452d0c; continue; }
            pc = 0x452ab3; continue;
            }
            0x452ab3 => {
            { let a = c.get(0,32); c.sub(a,0x20000u32,32); }
            if c.cond(4) { pc = 0x452c87; continue; }
            pc = 0x452abe; continue;
            }
            0x452abe => {
            { let a = c.get(0,32); c.sub(a,0x40000u32,32); }
            if c.cond(4) { pc = 0x452ba9; continue; }
            pc = 0x452ac9; continue;
            }
            0x452ac9 => {
            { let a = c.get(0,32); c.sub(a,0x80000u32,32); }
            if c.cond(5) { pc = 0x452f45; continue; }
            pc = 0x452ad4; continue;
            }
            0x452ad4 => {
            { let a = rm(m,0x6de4aau32,16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x452b88; continue; }
            pc = 0x452aea; continue;
            }
            0x452aea => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,0x4u32);
            pc = 0x452af3; continue;
            }
            0x452af3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x452b02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x452b56; continue; }
            pc = 0x452b09; continue;
            }
            0x452b09 => {
            c.set(6,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x452b56; continue; }
            pc = 0x452b1f; continue;
            }
            0x452b1f => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x452b25; continue;
            }
            0x452b25 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,c.get(3,16),16); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x452b4c; continue; }
            pc = 0x452b48; continue;
            }
            0x452b48 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x452b4c; continue;
            }
            0x452b4c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x452b25; continue; }
            pc = 0x452b56; continue;
            }
            0x452b56 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x452af3; continue; }
            pc = 0x452b6c; continue;
            }
            0x452b6c => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x452b88; continue; }
            pc = 0x452b73; continue;
            }
            0x452b73 => {
            c.set(0,32,rm(m,0x69b6b4u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x698024u32);
            pc = 0x452c47; continue;
            }
            0x452b88 => {
            c.set(0,32,rm(m,0x69b6b0u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x2904u32,32); c.set(1,32,r); }
            c.push(m, 0x452b9d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452ba9 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x452c66; continue; }
            pc = 0x452bb8; continue;
            }
            0x452bb8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x452bbc; continue;
            }
            0x452bbc => {
            { let a = c.get(3,32); c.sub(a,rm(m,0x6dfc14u32,32),32); }
            if c.cond(4) { pc = 0x452c24; continue; }
            pc = 0x452bc4; continue;
            }
            0x452bc4 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x452bcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x452c24; continue; }
            pc = 0x452bd6; continue;
            }
            0x452bd6 => {
            c.set(6,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x452c24; continue; }
            pc = 0x452bec; continue;
            }
            0x452bec => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x452bf2; continue;
            }
            0x452bf2 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x452c1a; continue; }
            pc = 0x452c16; continue;
            }
            0x452c16 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x452c1a; continue;
            }
            0x452c1a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x452bf2; continue; }
            pc = 0x452c24; continue;
            }
            0x452c24 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x452bbc; continue; }
            pc = 0x452c31; continue;
            }
            0x452c31 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x452c66; continue; }
            pc = 0x452c3a; continue;
            }
            0x452c3a => {
            c.set(1,32,rm(m,0x69b6acu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x69801cu32);
            pc = 0x452c47; continue;
            }
            0x452c47 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,0x2904u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x452c57);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452c66 => {
            c.set(0,32,rm(m,0x69b6a8u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x2904u32,32); c.set(1,32,r); }
            c.push(m, 0x452c7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452c87 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,0x6de588u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x452c96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x535300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(7,32,c.r[5].wrapping_add(0x2904));
            { let a = c.get(6,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0x1fu32,32); c.set(6,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x452cb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x69801au32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x452cbf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x66e0d4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x452cd2; continue; }
            pc = 0x452cc9; continue;
            }
            0x452cc9 => {
            c.set(1,32,rm(m,0x69b68cu32,32));
            c.push(m,c.get(1,32));
            pc = 0x452cd9; continue;
            }
            0x452cd2 => {
            c.set(2,32,rm(m,0x69b690u32,32));
            c.push(m,c.get(2,32));
            pc = 0x452cd9; continue;
            }
            0x452cd9 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x452ce0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x2930),32),32); }
            if c.cond(4) { pc = 0x452f45; continue; }
            pc = 0x452cec; continue;
            }
            0x452cec => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.r[1].wrapping_add(0x2930); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x114),32); c.push(m, 0x452d00); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452d0c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x452dfe; continue; }
            pc = 0x452d1f; continue;
            }
            0x452d1f => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x452d2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x452db3; continue; }
            pc = 0x452d35; continue;
            }
            0x452d35 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x452db3; continue; }
            pc = 0x452d4b; continue;
            }
            0x452d4b => {
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            pc = 0x452d50; continue;
            }
            0x452d50 => {
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x452d96; continue; }
            pc = 0x452d79; continue;
            }
            0x452d79 => {
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x452da2; continue; }
            pc = 0x452d96; continue;
            }
            0x452d96 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x452d50; continue; }
            pc = 0x452da0; continue;
            }
            0x452da0 => {
            pc = 0x452db3; continue;
            }
            0x452da2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x452db3; continue;
            }
            0x452db3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x452d1f; continue; }
            pc = 0x452dcd; continue;
            }
            0x452dcd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x452dfe; continue; }
            pc = 0x452dd6; continue;
            }
            0x452dd6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,0x69b6bcu32,32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.add(a,0x2904u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x452def);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452dfe => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x69b6c0u32,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x452c47; continue;
            }
            0x452e1c => {
            { let a = c.get(0,32); c.sub(a,0x400000u32,32); }
            if c.cond(4) { pc = 0x452e4f; continue; }
            pc = 0x452e23; continue;
            }
            0x452e23 => {
            { let a = c.get(0,32); c.sub(a,0x4000000u32,32); }
            if c.cond(5) { pc = 0x452f45; continue; }
            pc = 0x452e2e; continue;
            }
            0x452e2e => {
            c.set(0,32,rm(m,0x6b2a24u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x2904u32,32); c.set(1,32,r); }
            c.push(m, 0x452e43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452e4f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x452f2f; continue; }
            pc = 0x452e62; continue;
            }
            0x452e62 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x452e6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x452ee6; continue; }
            pc = 0x452e74; continue;
            }
            0x452e74 => {
            c.set(7,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x452ee6; continue; }
            pc = 0x452e8a; continue;
            }
            0x452e8a => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(1,16,rm(m,0x6dd896u32,16));
            pc = 0x452e97; continue;
            }
            0x452e97 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(1,16),16); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x452ec5; continue; }
            pc = 0x452eb9; continue;
            }
            0x452eb9 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x452e97; continue; }
            pc = 0x452ec3; continue;
            }
            0x452ec3 => {
            pc = 0x452ee6; continue;
            }
            0x452ec5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x452ee6; continue; }
            pc = 0x452ee2; continue;
            }
            0x452ee2 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x452ee6; continue;
            }
            0x452ee6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x452e62; continue; }
            pc = 0x452f00; continue;
            }
            0x452f00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x452f2f; continue; }
            pc = 0x452f09; continue;
            }
            0x452f09 => {
            c.set(2,32,rm(m,0x69b6bcu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.add(a,0x2904u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x452f20);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452f2f => {
            c.set(1,32,rm(m,0x69b6b8u32,32));
            pc = 0x452f35; continue;
            }
            0x452f35 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(0x2904));
            c.push(m, 0x452f45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x452f45; continue;
            }
            0x452f45 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x452f51 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_cancel_route_drag(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4538c0;
    loop {
        match pc {
            0x4538c0 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2b23),8));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[1].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x4538da; continue; }
            pc = 0x4538cf; continue;
            }
            0x4538cf => {
            { let a = c.r[1].wrapping_add(0x2b23); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4538da);
            { let args = []; c.r[0] = host.call(m, 0x451eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4538da; continue;
            }
            0x4538da => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_invalidate_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x453df0;
    loop {
        match pc {
            0x453df0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453e23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_add_dirty_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x453e30;
    loop {
        match pc {
            0x453e30 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x453e45);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x28d0),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x453e5e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(3,32));
            { let a = c.r[2].wrapping_add(c.r[7]); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[7]).wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[7]).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[7]).wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2928),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x28d0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x2928); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453eb9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_clear_scroll_blit_flag(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x453ed0;
    loop {
        match pc {
            0x453ed0 => {
            { let a = c.r[1].wrapping_add(0x2b2b); wm(m,a,8,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_validate_selected_ships_for_departure(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x454f80;
    loop {
        match pc {
            0x454f80 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x292c),32));
            c.push(m,0x0u32);
            { let a = c.r[7].wrapping_add(0x2b2c); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(3,8,0x1u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x454fb3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x454ff5; continue; }
            pc = 0x454fc4; continue;
            }
            0x454fc4 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            if c.cond(4) { pc = 0x454ff1; continue; }
            pc = 0x454fec; continue;
            }
            0x454fec => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x454fc4; continue; }
            pc = 0x454ff1; continue;
            }
            0x454ff1 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x454ff5; continue;
            }
            0x454ff5 => {
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x455000; continue; }
            pc = 0x454ff9; continue;
            }
            0x454ff9 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x455535; continue;
            }
            0x455000 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x455535; continue; }
            pc = 0x455010; continue;
            }
            0x455010 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.get(5,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x45506e; continue; }
            pc = 0x455040; continue;
            }
            0x455040 => {
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(5,16),16); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(5) { pc = 0x45551c; continue; }
            pc = 0x455062; continue;
            }
            0x455062 => {
            { let a = rm(m,c.r[1].wrapping_add(0x14),8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x45551c; continue; }
            pc = 0x45506c; continue;
            }
            0x45506c => {
            c.set(1,8,0x1u32);
            pc = 0x45506e; continue;
            }
            0x45506e => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x11u32,16); }
            if c.cond(4) { pc = 0x45551c; continue; }
            pc = 0x45507c; continue;
            }
            0x45507c => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x455306; continue; }
            pc = 0x455084; continue;
            }
            0x455084 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x1c),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4550d0; continue; }
            pc = 0x4550af; continue;
            }
            0x4550af => {
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(5,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x2c9),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4550d0; continue; }
            pc = 0x4550c6; continue;
            }
            0x4550c6 => {
            c.push(m,0x1u32);
            c.push(m,0xau32);
            c.push(m,c.get(6,32));
            pc = 0x455513; continue;
            }
            0x4550d0 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45551c; continue; }
            pc = 0x4550d8; continue;
            }
            0x4550d8 => {
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x14),8));
            { let a = c.get(1,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.set(1,8,!a & 0xffu32); }
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4550ee; continue; }
            pc = 0x4550e4; continue;
            }
            0x4550e4 => {
            c.push(m,0x1u32);
            c.push(m,0x6u32);
            c.push(m,c.get(6,32));
            pc = 0x455513; continue;
            }
            0x4550ee => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x45551c; continue; }
            pc = 0x455100; continue;
            }
            0x455100 => {
            pc = 0x455108; continue;
            }
            0x455102 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            pc = 0x455108; continue;
            }
            0x455108 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.set(1,8,rm(m,c.r[5],8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4552e7; continue; }
            pc = 0x45512c; continue;
            }
            0x45512c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x45516f; continue; }
            pc = 0x455139; continue;
            }
            0x455139 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2cb),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x45516f; continue; }
            pc = 0x455150; continue;
            }
            0x455150 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,0xcu32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x455168);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x455700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4552e7; continue;
            }
            0x45516f => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x45517a; continue; }
            pc = 0x455176; continue;
            }
            0x455176 => {
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4551b1; continue; }
            pc = 0x45517a; continue;
            }
            0x45517a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4551b1; continue; }
            pc = 0x455187; continue;
            }
            0x455187 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4551b1; continue; }
            pc = 0x45519e; continue;
            }
            0x45519e => {
            c.push(m,0x1u32);
            c.push(m,0x8u32);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4551aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x455700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4552e7; continue;
            }
            0x4551b1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4551d7);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x18),32); c.sub(a,c.get(2,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45521d; continue; }
            pc = 0x455214; continue;
            }
            0x455214 => {
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            pc = 0x4552d9; continue;
            }
            0x45521d => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xe),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[5].wrapping_add(0x40),16); c.sub(a,c.get(1,16),16); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x455271; continue; }
            pc = 0x45526b; continue;
            }
            0x45526b => {
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            pc = 0x4552d9; continue;
            }
            0x455271 => {
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45527c);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45529b; continue; }
            pc = 0x455295; continue;
            }
            0x455295 => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            pc = 0x4552d9; continue;
            }
            0x45529b => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x3f),8));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4552bc; continue; }
            pc = 0x4552b6; continue;
            }
            0x4552b6 => {
            c.push(m,0x1u32);
            c.push(m,0x3u32);
            pc = 0x4552d9; continue;
            }
            0x4552bc => {
            c.set(5,16,rm(m,c.r[5].wrapping_add(0x134),16));
            { let a = c.get(5,16); c.sub(a,0x6u32,16); }
            if c.cond(5) { pc = 0x4552cf; continue; }
            pc = 0x4552c9; continue;
            }
            0x4552c9 => {
            c.push(m,0x1u32);
            c.push(m,0x4u32);
            pc = 0x4552d9; continue;
            }
            0x4552cf => {
            { let a = c.get(5,16); c.sub(a,0x7u32,16); }
            if c.cond(5) { pc = 0x4552e3; continue; }
            pc = 0x4552d5; continue;
            }
            0x4552d5 => {
            c.push(m,0x1u32);
            c.push(m,0x5u32);
            pc = 0x4552d9; continue;
            }
            0x4552d9 => {
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4552e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x455700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4552e3; continue;
            }
            0x4552e3 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4552e7; continue;
            }
            0x4552e7 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x455102; continue; }
            pc = 0x455301; continue;
            }
            0x455301 => {
            pc = 0x45551c; continue;
            }
            0x455306 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45551c; continue; }
            pc = 0x455317; continue;
            }
            0x455317 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x45534c; continue; }
            pc = 0x45532a; continue;
            }
            0x45532a => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2cb),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x45534c; continue; }
            pc = 0x455342; continue;
            }
            0x455342 => {
            c.push(m,0x0u32);
            c.push(m,0xcu32);
            c.push(m,c.get(5,32));
            pc = 0x455513; continue;
            }
            0x45534c => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x455359; continue; }
            pc = 0x455354; continue;
            }
            0x455354 => {
            { let a = c.get(1,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x455391; continue; }
            pc = 0x455359; continue;
            }
            0x455359 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x45538d; continue; }
            pc = 0x45536c; continue;
            }
            0x45536c => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x45538d; continue; }
            pc = 0x455383; continue;
            }
            0x455383 => {
            c.push(m,0x0u32);
            c.push(m,0x8u32);
            c.push(m,c.get(5,32));
            pc = 0x455513; continue;
            }
            0x45538d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x455391; continue;
            }
            0x455391 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[6].wrapping_add(0x40),16); c.sub(a,c.get(1,16),16); }
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4553c1; continue; }
            pc = 0x4553b7; continue;
            }
            0x4553b7 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            pc = 0x455513; continue;
            }
            0x4553c1 => {
            c.set(5,32,rm(m,0x6de4b0u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4553ce);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4553ec; continue; }
            pc = 0x4553e3; continue;
            }
            0x4553e3 => {
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            pc = 0x455506; continue;
            }
            0x4553ec => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.sbb(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[6].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45543b; continue; }
            pc = 0x455425; continue;
            }
            0x455425 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x2u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x45543b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3f),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(5,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x455474; continue; }
            pc = 0x455449; continue;
            }
            0x455449 => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x8u32,16); }
            if c.cond(5) { pc = 0x45545e; continue; }
            pc = 0x455453; continue;
            }
            0x455453 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x455474; continue; }
            pc = 0x45545e; continue;
            }
            0x45545e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x455474 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x6u32,16); }
            if c.cond(5) { pc = 0x455494; continue; }
            pc = 0x455481; continue;
            }
            0x455481 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x455494 => {
            { let a = c.get(0,16); c.sub(a,0x7u32,16); }
            if c.cond(5) { pc = 0x4554ad; continue; }
            pc = 0x45549a; continue;
            }
            0x45549a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x5u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x4554ad => {
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(5) { pc = 0x4554c6; continue; }
            pc = 0x4554b3; continue;
            }
            0x4554b3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x7u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x4554c6 => {
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(5) { pc = 0x4554df; continue; }
            pc = 0x4554cc; continue;
            }
            0x4554cc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x0u32);
            c.push(m,0x9u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            pc = 0x455513; continue;
            }
            0x4554df => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x45551c; continue; }
            pc = 0x4554e8; continue;
            }
            0x4554e8 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(4,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x45551c; continue; }
            pc = 0x455502; continue;
            }
            0x455502 => {
            c.push(m,0x0u32);
            c.push(m,0xau32);
            pc = 0x455506; continue;
            }
            0x455506 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            pc = 0x455513; continue;
            }
            0x455513 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x45551a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x455700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x45551c; continue;
            }
            0x45551c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x455010; continue; }
            pc = 0x455531; continue;
            }
            0x455531 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x455535; continue;
            }
            0x455535 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x292c),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4556d0; continue; }
            pc = 0x455543; continue;
            }
            0x455543 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            if c.cond(13) { pc = 0x4556d0; continue; }
            pc = 0x45554d; continue;
            }
            0x45554d => {
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x455558; continue;
            }
            0x455558 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2880),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xe),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x45566a; continue; }
            pc = 0x455573; continue;
            }
            0x455573 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x45561c; continue; }
            pc = 0x45558d; continue;
            }
            0x45558d => {
            c.set(0,32,rm(m,c.r[1],32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x455599; continue;
            }
            0x455599 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,16,rm(m,c.r[2],16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x8),16));
            c.set(5,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16),16); }
            if c.cond(4) { pc = 0x4555e5; continue; }
            pc = 0x4555c7; continue;
            }
            0x4555c7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x455599; continue; }
            pc = 0x4555e3; continue;
            }
            0x4555e3 => {
            pc = 0x45561c; continue;
            }
            0x4555e5 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(2,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x4555f7; continue; }
            pc = 0x4555f1; continue;
            }
            0x4555f1 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            pc = 0x45561c; continue;
            }
            0x4555f7 => {
            { let a = c.get(2,16); c.sub(a,0xcu32,16); }
            if c.cond(5) { pc = 0x455603; continue; }
            pc = 0x4555fd; continue;
            }
            0x4555fd => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            pc = 0x45561c; continue;
            }
            0x455603 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16); c.sub(a,0x6u32,16); }
            if c.cond(4) { pc = 0x455616; continue; }
            pc = 0x45560e; continue;
            }
            0x45560e => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,0x3u32); }
            pc = 0x45561c; continue;
            }
            0x455616 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,0x6u32); }
            pc = 0x45561c; continue;
            }
            0x45561c => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2880),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4556b2; continue; }
            pc = 0x455636; continue;
            }
            0x455636 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,16,rm(m,c.r[2],16));
            { let a = c.get(1,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x455648; continue; }
            pc = 0x455642; continue;
            }
            0x455642 => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(1,16)); }
            pc = 0x4556b2; continue;
            }
            0x455648 => {
            { let a = c.get(1,16); c.sub(a,0xcu32,16); }
            if c.cond(5) { pc = 0x455654; continue; }
            pc = 0x45564e; continue;
            }
            0x45564e => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(1,16)); }
            pc = 0x4556b2; continue;
            }
            0x455654 => {
            { let a = c.get(1,16); c.sub(a,0x6u32,16); }
            if c.cond(4) { pc = 0x455662; continue; }
            pc = 0x45565a; continue;
            }
            0x45565a => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,0x2u32); }
            pc = 0x4556b2; continue;
            }
            0x455662 => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,0x6u32); }
            pc = 0x4556b2; continue;
            }
            0x45566a => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(5) { pc = 0x45567c; continue; }
            pc = 0x455676; continue;
            }
            0x455676 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            pc = 0x4556b2; continue;
            }
            0x45567c => {
            { let a = c.get(0,16); c.sub(a,0x7u32,16); }
            if c.cond(5) { pc = 0x455688; continue; }
            pc = 0x455682; continue;
            }
            0x455682 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            pc = 0x4556b2; continue;
            }
            0x455688 => {
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(5) { pc = 0x455694; continue; }
            pc = 0x45568e; continue;
            }
            0x45568e => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            pc = 0x4556b2; continue;
            }
            0x455694 => {
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x4556a0; continue; }
            pc = 0x45569a; continue;
            }
            0x45569a => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            pc = 0x4556b2; continue;
            }
            0x4556a0 => {
            { let a = c.get(0,16); c.sub(a,0xcu32,16); }
            if c.cond(5) { pc = 0x4556ac; continue; }
            pc = 0x4556a6; continue;
            }
            0x4556a6 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            pc = 0x4556b2; continue;
            }
            0x4556ac => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,0x1u32); }
            pc = 0x4556b2; continue;
            }
            0x4556b2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x455558; continue; }
            pc = 0x4556d0; continue;
            }
            0x4556d0 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4556e1; continue; }
            pc = 0x4556da; continue;
            }
            0x4556da => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4556e1);
            { let args = []; c.r[0] = host.call(m, 0x456f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4556e1; continue;
            }
            0x4556e1 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,c.get(3,8));
            if c.cond(4) { pc = 0x4556ee; continue; }
            pc = 0x4556e7; continue;
            }
            0x4556e7 => {
            { let a = c.r[7].wrapping_add(0x2b2c); wm(m,a,8,0x0u32); }
            pc = 0x4556ee; continue;
            }
            0x4556ee => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_add_ship_problem(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x455700;
    loop {
        match pc {
            0x455700 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x455870; continue; }
            pc = 0x455719; continue;
            }
            0x455719 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45572f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x24),8));
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x20),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x455785; continue; }
            pc = 0x455749; continue;
            }
            0x455749 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x8),16));
            c.set(5,32,rm(m,0x6dd7a8u32,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,16,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            pc = 0x4557cb; continue;
            }
            0x455785 => {
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x4557b6; continue; }
            pc = 0x45578a; continue;
            }
            0x45578a => {
            { let a = c.get(3,16); c.sub(a,0x5u32,16); }
            if c.cond(4) { pc = 0x4557b6; continue; }
            pc = 0x455790; continue;
            }
            0x455790 => {
            { let a = c.get(3,16); c.sub(a,0x8u32,16); }
            if c.cond(4) { pc = 0x4557b6; continue; }
            pc = 0x455796; continue;
            }
            0x455796 => {
            { let a = c.get(3,16); c.sub(a,0xcu32,16); }
            if c.cond(4) { pc = 0x4557b6; continue; }
            pc = 0x45579c; continue;
            }
            0x45579c => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x1));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            pc = 0x4557cb; continue;
            }
            0x4557b6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,16,rm(m,0x6dd894u32,16));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            pc = 0x4557cb; continue;
            }
            0x4557cb => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4557df);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(7,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x455838);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(3,16)); }
            pc = 0x455b23; continue;
            }
            0x455870 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4558e2; continue; }
            pc = 0x45587c; continue;
            }
            0x45587c => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4559e4; continue; }
            pc = 0x4558a1; continue;
            }
            0x4558a1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x8),16));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(2,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(0,32,c.r[3].wrapping_add(0xe));
            pc = 0x4558bb; continue;
            }
            0x4558bb => {
            { let a = rm(m,c.r[0],16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x4558cd; continue; }
            pc = 0x4558c0; continue;
            }
            0x4558c0 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4558bb; continue; }
            pc = 0x4558c8; continue;
            }
            0x4558c8 => {
            pc = 0x4559e4; continue;
            }
            0x4558cd => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x4),16));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[3]),32));
            pc = 0x4559b7; continue;
            }
            0x4558e2 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x455943; continue; }
            pc = 0x4558ec; continue;
            }
            0x4558ec => {
            { let a = c.get(0,16); c.sub(a,0x5u32,16); }
            if c.cond(4) { pc = 0x455943; continue; }
            pc = 0x4558f2; continue;
            }
            0x4558f2 => {
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(4) { pc = 0x455943; continue; }
            pc = 0x4558f8; continue;
            }
            0x4558f8 => {
            { let a = c.get(0,16); c.sub(a,0xcu32,16); }
            if c.cond(4) { pc = 0x455943; continue; }
            pc = 0x4558fe; continue;
            }
            0x4558fe => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4559e4; continue; }
            pc = 0x45590e; continue;
            }
            0x45590e => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xeu32,32); c.set(3,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x1));
            pc = 0x455929; continue;
            }
            0x455929 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[3],16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4559e4; continue; }
            pc = 0x455936; continue;
            }
            0x455936 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x455929; continue; }
            pc = 0x45593e; continue;
            }
            0x45593e => {
            pc = 0x4559e4; continue;
            }
            0x455943 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4559e4; continue; }
            pc = 0x455953; continue;
            }
            0x455953 => {
            c.set(0,32,c.r[3].wrapping_add(0xe));
            pc = 0x455956; continue;
            }
            0x455956 => {
            c.set(7,16,rm(m,c.r[0],16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(4) { pc = 0x45596c; continue; }
            pc = 0x455962; continue;
            }
            0x455962 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x455956; continue; }
            pc = 0x45596a; continue;
            }
            0x45596a => {
            pc = 0x4559e4; continue;
            }
            0x45596c => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x4),16));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4559a2; continue; }
            pc = 0x45597f; continue;
            }
            0x45597f => {
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[3]),32));
            pc = 0x455982; continue;
            }
            0x455982 => {
            { let a = rm(m,c.r[3],16); c.sub(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x455991; continue; }
            pc = 0x455987; continue;
            }
            0x455987 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x455982; continue; }
            pc = 0x45598f; continue;
            }
            0x45598f => {
            pc = 0x4559a2; continue;
            }
            0x455991 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[7].wrapping_mul(2)); wm(m,a,16,c.get(3,16)); }
            pc = 0x4559a2; continue;
            }
            0x4559a2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2]));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4559e4; continue; }
            pc = 0x4559b5; continue;
            }
            0x4559b5 => {
            c.set(7,32,rm(m,c.r[3],32));
            pc = 0x4559b7; continue;
            }
            0x4559b7 => {
            { let a = c.r[7].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[7].wrapping_mul(2)); wm(m,a,16,c.get(3,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4));
            pc = 0x4559e4; continue;
            }
            0x4559e4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x455b44; continue; }
            pc = 0x4559f2; continue;
            }
            0x4559f2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x455a3a; continue; }
            pc = 0x4559fa; continue;
            }
            0x4559fa => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x8),16));
            c.set(7,32,rm(m,0x6dd7a8u32,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            pc = 0x455a85; continue;
            }
            0x455a3a => {
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.get(7,16); c.and(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x455a70; continue; }
            pc = 0x455a44; continue;
            }
            0x455a44 => {
            { let a = c.get(7,16); c.sub(a,0x5u32,16); }
            if c.cond(4) { pc = 0x455a70; continue; }
            pc = 0x455a4a; continue;
            }
            0x455a4a => {
            { let a = c.get(7,16); c.sub(a,0x8u32,16); }
            if c.cond(4) { pc = 0x455a70; continue; }
            pc = 0x455a50; continue;
            }
            0x455a50 => {
            { let a = c.get(7,16); c.sub(a,0xcu32,16); }
            if c.cond(4) { pc = 0x455a70; continue; }
            pc = 0x455a56; continue;
            }
            0x455a56 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x1));
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            pc = 0x455a85; continue;
            }
            0x455a70 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,16,rm(m,0x6dd894u32,16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            pc = 0x455a85; continue;
            }
            0x455a85 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x455a97);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x455af0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(7,16)); }
            pc = 0x455b23; continue;
            }
            0x455b23 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,r); } }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4));
            { let a = rm(m,c.r[6].wrapping_add(0x292c),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,r); } }
            pc = 0x455b44; continue;
            }
            0x455b44 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_handle_order_click(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x455b50;
    loop {
        match pc {
            0x455b50 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e9afu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xd8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m, 0x455b7a);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf8),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(2,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(2,16,r); }
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(1,16,r); }
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1e),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffbcu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x88));
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x8a); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x455bd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(3,8,c.get(0,8));
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x3b); wm(m,a,8,0x0u32); }
            c.push(m, 0x455bf4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x455c0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x455c4a; continue; }
            pc = 0x455c11; continue;
            }
            0x455c11 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(7,32,rm(m,0x6dfc14u32,32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x455c21; continue;
            }
            0x455c21 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x455c41; continue; }
            pc = 0x455c36; continue;
            }
            0x455c36 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x455c21; continue; }
            pc = 0x455c3f; continue;
            }
            0x455c3f => {
            pc = 0x455c46; continue;
            }
            0x455c41 => {
            { let a = c.r[4].wrapping_add(0x2f); wm(m,a,8,0x1u32); }
            pc = 0x455c46; continue;
            }
            0x455c46 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            pc = 0x455c4a; continue;
            }
            0x455c4a => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x455c9d; continue; }
            pc = 0x455c51; continue;
            }
            0x455c51 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2920),32));
            { let a = c.r[6].wrapping_add(0x2920); wm(m,a,32,0xff687888u32); }
            { let a = c.get(0,32); c.sub(a,0xff687888u32,32); }
            if c.cond(4) { pc = 0x455c9d; continue; }
            pc = 0x455c68; continue;
            }
            0x455c68 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x455c9d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x455c9d; continue;
            }
            0x455c9d => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b23),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x456f54; continue; }
            pc = 0x455cab; continue;
            }
            0x455cab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x456259; continue; }
            pc = 0x455cb7; continue;
            }
            0x455cb7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x455ccc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,16,rm(m,c.r[4].wrapping_add(0x1e),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,16); c.sub(a,0x6c8u32,16); }
            c.set(7,32,c.get(0,32));
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,0x5cbu32,16); }
            c.set(5,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,16); c.sub(a,0x76u32,16); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.and(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,0x44u32,16); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4562bc; continue; }
            pc = 0x455d0a; continue;
            }
            0x455d0a => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x455d1e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x698030u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0xf4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x455d3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6a1278u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m, 0x455d4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x455e75; continue; }
            pc = 0x455d58; continue;
            }
            0x455d58 => {
            c.set(0,8,0xffu32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x53); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x5e); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x3ecccccdu32); }
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x455da9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x455dfc; continue; }
            pc = 0x455daf; continue;
            }
            0x455daf => {
            c.set(3,32,0x186a0u32);
            pc = 0x455db4; continue;
            }
            0x455db4 => {
            c.set(1,32,c.r[4].wrapping_add(0x94));
            c.push(m, 0x455dc0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x94));
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,0xbu32); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0xa0); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x455df5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x455db4; continue; }
            pc = 0x455dfa; continue;
            }
            0x455dfa => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x455dfc; continue;
            }
            0x455dfc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x455e6a; continue; }
            pc = 0x455e08; continue;
            }
            0x455e08 => {
            c.set(7,32,c.r[6].wrapping_add(0x28d6));
            pc = 0x455e0e; continue;
            }
            0x455e0e => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7],16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffff5));
            { let a = c.get(0,32); let r = c.add(a,0x16u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x16u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x455e5c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x455e0e; continue; }
            pc = 0x455e6a; continue;
            }
            0x455e6a => {
            { let a = c.r[6].wrapping_add(0x291c); wm(m,a,32,c.get(3,32)); }
            pc = 0x456245; continue;
            }
            0x455e75 => {
            if c.cond(5) { pc = 0x4560f5; continue; }
            pc = 0x455e7b; continue;
            }
            0x455e7b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,16,rm(m,c.r[1],16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(6) { pc = 0x455fbb; continue; }
            pc = 0x455ea6; continue;
            }
            0x455ea6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x44));
            c.set(0,8,0xffu32);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x5e); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x57); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x3ecccccdu32); }
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x455efa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xd4));
            c.push(m, 0x455f06);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xd4); wm(m,a,32,0xbu32); }
            c.set(2,32,c.r[4].wrapping_add(0xd4));
            c.set(0,16,rm(m,c.r[1],16));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0xdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xe0); wm(m,a,32,0x186a0u32); }
            c.push(m, 0x455f3e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x455fac; continue; }
            pc = 0x455f4a; continue;
            }
            0x455f4a => {
            c.set(7,32,c.r[6].wrapping_add(0x28d6));
            pc = 0x455f50; continue;
            }
            0x455f50 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7],16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xfffffff5));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x16u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x78));
            { let a = c.get(0,32); let r = c.add(a,0x16u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x455f9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x455f50; continue; }
            pc = 0x455fac; continue;
            }
            0x455fac => {
            { let a = c.r[6].wrapping_add(0x291c); wm(m,a,32,0x0u32); }
            pc = 0x456245; continue;
            }
            0x455fbb => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x4560f5; continue; }
            pc = 0x455fd7; continue;
            }
            0x455fd7 => {
            c.set(0,8,0xffu32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x53); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x5e); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x3ecccccdu32); }
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x45602b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc0));
            c.push(m, 0x456037);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc0));
            { let a = c.r[4].wrapping_add(0xc0); wm(m,a,32,0xbu32); }
            c.set(2,16,rm(m,c.r[0],16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0xc8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0xcc); wm(m,a,32,0x186a0u32); }
            c.push(m, 0x45606f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4560e6; continue; }
            pc = 0x45607b; continue;
            }
            0x45607b => {
            c.set(7,32,c.r[6].wrapping_add(0x28d6));
            pc = 0x456081; continue;
            }
            0x456081 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7],16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffff5));
            { let a = c.get(0,32); let r = c.add(a,0x16u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x98); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x16u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x94));
            { let a = c.r[4].wrapping_add(0xa0); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4560d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x456081; continue; }
            pc = 0x4560e6; continue;
            }
            0x4560e6 => {
            { let a = c.r[6].wrapping_add(0x291c); wm(m,a,32,0x0u32); }
            pc = 0x456245; continue;
            }
            0x4560f5 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4560fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x454f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            if c.cond(14) { pc = 0x45613f; continue; }
            pc = 0x45610f; continue;
            }
            0x45610f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x456111; continue;
            }
            0x456111 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x10); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x1e),16));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x12); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x456111; continue; }
            pc = 0x45613f; continue;
            }
            0x45613f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[7]),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[7]));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x456245; continue; }
            pc = 0x45616e; continue;
            }
            0x45616e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x456260; continue; }
            pc = 0x456195; continue;
            }
            0x456195 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(5,32,rm(m,0x6dd894u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x4561b6; continue;
            }
            0x4561b6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x3c),16));
            { let a = rm(m,c.r[0].wrapping_add(0xe),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x456223; continue; }
            pc = 0x4561c5; continue;
            }
            0x4561c5 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x456223; continue; }
            pc = 0x4561d3; continue;
            }
            0x4561d3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,16,c.get(0,16)); }
            pc = 0x4561e0; continue;
            }
            0x4561e0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x456217; continue; }
            pc = 0x4561e9; continue;
            }
            0x4561e9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),16));
            pc = 0x4561f3; continue;
            }
            0x4561f3 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x456212; continue; }
            pc = 0x456207; continue;
            }
            0x456207 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x4561f3; continue; }
            pc = 0x456210; continue;
            }
            0x456210 => {
            pc = 0x456217; continue;
            }
            0x456212 => {
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x45621e; continue; }
            pc = 0x456217; continue;
            }
            0x456217 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4561e0; continue; }
            pc = 0x45621c; continue;
            }
            0x45621c => {
            pc = 0x456223; continue;
            }
            0x45621e => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x456223; continue;
            }
            0x456223 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4561b6; continue; }
            pc = 0x45623d; continue;
            }
            0x45623d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x456260; continue; }
            pc = 0x456245; continue;
            }
            0x456245 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0xf0); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x456259);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x456259; continue;
            }
            0x456259 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x456f56; continue;
            }
            0x456260 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x14),8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4562a3; continue; }
            pc = 0x456289; continue;
            }
            0x456289 => {
            c.set(0,32,rm(m,0x6e5544u32,32));
            { let a = c.r[0].wrapping_add(0x94); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6e5544u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x4562a3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4562a3; continue;
            }
            0x4562a3 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0xf0); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4562b7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x456f54; continue;
            }
            0x4562bc => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2b1e),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x28u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.or(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4565b4; continue; }
            pc = 0x4562d4; continue;
            }
            0x4562d4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x456f54; continue; }
            pc = 0x4562eb; continue;
            }
            0x4562eb => {
            c.push(m,0x2u32);
            c.push(m, 0x4562f2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x45632d; continue; }
            pc = 0x456309; continue;
            }
            0x456309 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            pc = 0x45630f; continue;
            }
            0x45630f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45632d; continue; }
            pc = 0x456328; continue;
            }
            0x456328 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x45630f; continue; }
            pc = 0x45632d; continue;
            }
            0x45632d => {
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4563c2; continue; }
            pc = 0x456335; continue;
            }
            0x456335 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x28d4); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1e),16));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28d6); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x456362);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4563bc; continue; }
            pc = 0x456366; continue;
            }
            0x456366 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,0x64u32);
            c.set(3,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x20),16),16)&0xffffffffu32);
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xfffffff7));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1950),32));
            { let a = c.r[6].wrapping_add(0x29d8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffff7u32,32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,c.r[6].wrapping_add(0x1950));
            { let a = c.r[6].wrapping_add(0x29d4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x4563ad); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4563b8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4ad8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4563bc; continue;
            }
            0x4563bc => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x4563c2; continue;
            }
            0x4563c2 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x456512; continue; }
            pc = 0x4563d0; continue;
            }
            0x4563d0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(2)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x456436; continue; }
            pc = 0x4563f4; continue;
            }
            0x4563f4 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,0x6dd894u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x456436; continue; }
            pc = 0x456415; continue;
            }
            0x456415 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x45642d; continue; }
            pc = 0x45642b; continue;
            }
            0x45642b => {
            c.set(2,32,c.get(1,32));
            pc = 0x45642d; continue;
            }
            0x45642d => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(2) { pc = 0x456415; continue; }
            pc = 0x456436; continue;
            }
            0x456436 => {
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x1c),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x456460);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x45bdc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4564f7; continue; }
            pc = 0x456468; continue;
            }
            0x456468 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,0x13u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,16); c.sub(a,0x11u32,16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,16); c.sub(a,0x6u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4564f7; continue; }
            pc = 0x456490; continue;
            }
            0x456490 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x456501; continue; }
            pc = 0x4564b8; continue;
            }
            0x4564b8 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x1c),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xbu32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4564f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4564fb; continue;
            }
            0x4564f7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x4564fb; continue;
            }
            0x4564fb => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x456501; continue;
            }
            0x456501 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x4563d0; continue; }
            pc = 0x456512; continue;
            }
            0x456512 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x698038u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x456529);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x291c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2884));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x291c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x456546);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x1c),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(0xfffffff6));
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xb0); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffff6));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0xb4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xb0));
            c.push(m,c.get(0,32));
            c.push(m, 0x4565a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xb0));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4565af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x456f54; continue;
            }
            0x4565b4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(7,32,c.r[6].wrapping_add(0x28fc));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4565cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4565eb; continue; }
            pc = 0x4565d4; continue;
            }
            0x4565d4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4565e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,8); let r = c.or(a,c.get(0,8),8); c.set(3,8,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x4565eb; continue;
            }
            0x4565eb => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x2f),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.or(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x456f54; continue; }
            pc = 0x456601; continue;
            }
            0x456601 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,16); c.sub(a,0x2u32,16); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x456664; continue; }
            pc = 0x456654; continue;
            }
            0x456654 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x45665f);
            { let args = []; c.r[0] = host.call(m, 0x492430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x456f54; continue;
            }
            0x456664 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45666b);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4566c1; continue; }
            pc = 0x45666f; continue;
            }
            0x45666f => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x28fe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,0x64u32);
            c.set(3,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7],16),16)&0xffffffffu32);
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xfffffff7));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1950),32));
            { let a = c.r[6].wrapping_add(0x29d8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffff7u32,32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,c.r[6].wrapping_add(0x1950));
            { let a = c.r[6].wrapping_add(0x29d4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x4566b6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4566c1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4ad8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4566c1; continue;
            }
            0x4566c1 => {
            c.set(2,32,c.r[5].wrapping_add(c.r[5]));
            c.push(m,c.get(2,32));
            c.push(m, 0x4566cb);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            if c.cond(14) { pc = 0x45674a; continue; }
            pc = 0x4566dc; continue;
            }
            0x4566dc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(3,32,rm(m,0x6dfc14u32,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x456745; continue; }
            pc = 0x456707; continue;
            }
            0x456707 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x456745; continue; }
            pc = 0x456714; continue;
            }
            0x456714 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x456745; continue; }
            pc = 0x45672c; continue;
            }
            0x45672c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            pc = 0x456745; continue;
            }
            0x456745 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4566dc; continue; }
            pc = 0x45674a; continue;
            }
            0x45674a => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x456b81; continue; }
            pc = 0x456758; continue;
            }
            0x456758 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x456b70; continue; }
            pc = 0x456789; continue;
            }
            0x456789 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            if c.cond(14) { pc = 0x4567cb; continue; }
            pc = 0x456797; continue;
            }
            0x456797 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            pc = 0x45679b; continue;
            }
            0x45679b => {
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4567c6; continue; }
            pc = 0x4567a4; continue;
            }
            0x4567a4 => {
            c.set(0,32,c.get(2,32));
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)),16),16); }
            if c.cond(4) { pc = 0x4567cb; continue; }
            pc = 0x4567c6; continue;
            }
            0x4567c6 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x45679b; continue; }
            pc = 0x4567cb; continue;
            }
            0x4567cb => {
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x456b6c; continue; }
            pc = 0x4567d3; continue;
            }
            0x4567d3 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x456b6c; continue; }
            pc = 0x4567e4; continue;
            }
            0x4567e4 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x38),16));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(0,16),16); }
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(4) { pc = 0x456b6c; continue; }
            pc = 0x456808; continue;
            }
            0x456808 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x456976; continue; }
            pc = 0x456816; continue;
            }
            0x456816 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x45682c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x456874);
            { let args = []; c.r[0] = host.call(m, 0x5038f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45687c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.push(m, 0x4568eb);
            { let args = []; c.r[0] = host.call(m, 0x5038f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4568f3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,0x4u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x12),16); c.sub(a,0x16u32,16); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            if c.cond(5) { pc = 0x456b4d; continue; }
            pc = 0x456954; continue;
            }
            0x456954 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,0xbu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(0,32)); }
            pc = 0x456b70; continue;
            }
            0x456976 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x456a04; continue; }
            pc = 0x456986; continue;
            }
            0x456986 => {
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            pc = 0x456989; continue;
            }
            0x456989 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x2),16); c.sub(a,c.get(2,16),16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1],16); c.sub(a,c.get(2,16),16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4569b6; continue; }
            pc = 0x4569a2; continue;
            }
            0x4569a2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x456989; continue; }
            pc = 0x4569b0; continue;
            }
            0x4569b0 => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x456a04; continue;
            }
            0x4569b6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x38),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x4),16));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(3,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2]));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,0x4u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x456a04; continue;
            }
            0x456a04 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x456b6c; continue; }
            pc = 0x456a12; continue;
            }
            0x456a12 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.push(m, 0x456a51);
            { let args = []; c.r[0] = host.call(m, 0x5038f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x456a59);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.push(m, 0x456ac9);
            { let args = []; c.r[0] = host.call(m, 0x5038f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x456ad1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,0x4u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x12),16); c.sub(a,0x16u32,16); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            if c.cond(5) { pc = 0x456b4d; continue; }
            pc = 0x456b2e; continue;
            }
            0x456b2e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,0xbu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(0,32)); }
            pc = 0x456b70; continue;
            }
            0x456b4d => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(0,32)); }
            pc = 0x456b70; continue;
            }
            0x456b6c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x456b70; continue;
            }
            0x456b70 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x456758; continue; }
            pc = 0x456b81; continue;
            }
            0x456b81 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x456b8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x454f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x456bcf; continue; }
            pc = 0x456b9b; continue;
            }
            0x456b9b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x456b9d; continue;
            }
            0x456b9d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x28fc),16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x10); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x28fe),16));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x12); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x456b9d; continue; }
            pc = 0x456bcf; continue;
            }
            0x456bcf => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x456f43; continue; }
            pc = 0x456bdf; continue;
            }
            0x456bdf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x13u32,16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0x11u32,16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0x6u32,16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x456f2e; continue; }
            pc = 0x456c3c; continue;
            }
            0x456c3c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x456f2e; continue; }
            pc = 0x456c51; continue;
            }
            0x456c51 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x456d3d; continue; }
            pc = 0x456c6a; continue;
            }
            0x456c6a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x456d90; continue; }
            pc = 0x456c92; continue;
            }
            0x456c92 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0xac); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            pc = 0x456cb1; continue;
            }
            0x456caa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x90),32));
            pc = 0x456cb1; continue;
            }
            0x456cb1 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xac),16));
            { let a = rm(m,c.r[5].wrapping_add(0xe),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x456d29; continue; }
            pc = 0x456cbf; continue;
            }
            0x456cbf => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x456d29; continue; }
            pc = 0x456ccb; continue;
            }
            0x456ccb => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0xa8); wm(m,a,16,c.get(1,16)); }
            pc = 0x456cd7; continue;
            }
            0x456cd7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xa8),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x456d1d; continue; }
            pc = 0x456ce7; continue;
            }
            0x456ce7 => {
            c.set(1,32,rm(m,c.r[5],32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            pc = 0x456cee; continue;
            }
            0x456cee => {
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x456d14; continue; }
            pc = 0x456cf3; continue;
            }
            0x456cf3 => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x456cee; continue; }
            pc = 0x456d12; continue;
            }
            0x456d12 => {
            pc = 0x456d1d; continue;
            }
            0x456d14 => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x456d24; continue; }
            pc = 0x456d1d; continue;
            }
            0x456d1d => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x456cd7; continue; }
            pc = 0x456d22; continue;
            }
            0x456d22 => {
            pc = 0x456d29; continue;
            }
            0x456d24 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x456d29; continue;
            }
            0x456d29 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x456caa; continue; }
            pc = 0x456d3b; continue;
            }
            0x456d3b => {
            pc = 0x456d84; continue;
            }
            0x456d3d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x456d90; continue; }
            pc = 0x456d47; continue;
            }
            0x456d47 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(3,32,c.get(0,32));
            pc = 0x456d4f; continue;
            }
            0x456d4f => {
            c.set(1,16,rm(m,0x6dd896u32,16));
            { let a = rm(m,c.r[7].wrapping_add(0xe),16); c.sub(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x456d7e; continue; }
            pc = 0x456d5c; continue;
            }
            0x456d5c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x456d7e; continue; }
            pc = 0x456d68; continue;
            }
            0x456d68 => {
            c.set(2,32,rm(m,c.r[7],32));
            pc = 0x456d6a; continue;
            }
            0x456d6a => {
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[2],16),16); }
            if c.cond(4) { pc = 0x456d79; continue; }
            pc = 0x456d6f; continue;
            }
            0x456d6f => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x456d6a; continue; }
            pc = 0x456d77; continue;
            }
            0x456d77 => {
            pc = 0x456d7e; continue;
            }
            0x456d79 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x456d7e; continue;
            }
            0x456d7e => {
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x456d4f; continue; }
            pc = 0x456d84; continue;
            }
            0x456d84 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x456f2e; continue; }
            pc = 0x456d90; continue;
            }
            0x456d90 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            if c.cond(14) { pc = 0x456de8; continue; }
            pc = 0x456d9e; continue;
            }
            0x456d9e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x456da2; continue;
            }
            0x456da2 => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x456de3; continue; }
            pc = 0x456dab; continue;
            }
            0x456dab => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x8c),16),16); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x456de8; continue; }
            pc = 0x456ddf; continue;
            }
            0x456ddf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x456de3; continue;
            }
            0x456de3 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x456da2; continue; }
            pc = 0x456de8; continue;
            }
            0x456de8 => {
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x456f2e; continue; }
            pc = 0x456df0; continue;
            }
            0x456df0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            if c.cond(3) { pc = 0x456e4e; continue; }
            pc = 0x456e06; continue;
            }
            0x456e06 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x456e4e; continue; }
            pc = 0x456e27; continue;
            }
            0x456e27 => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x456e2d; continue;
            }
            0x456e2d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x456e45; continue; }
            pc = 0x456e43; continue;
            }
            0x456e43 => {
            c.set(2,32,c.get(1,32));
            pc = 0x456e45; continue;
            }
            0x456e45 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x456e2d; continue; }
            pc = 0x456e4e; continue;
            }
            0x456e4e => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[4].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[4].wrapping_add(0x1c),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x456e79);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45bcf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x456e98; continue; }
            pc = 0x456e7d; continue;
            }
            0x456e7d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x456e93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x456f2e; continue;
            }
            0x456e98 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x456ea1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x45bdc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x456f2e; continue; }
            pc = 0x456ea9; continue;
            }
            0x456ea9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x456ed8; continue; }
            pc = 0x456eba; continue;
            }
            0x456eba => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x456f2e; continue; }
            pc = 0x456ed8; continue;
            }
            0x456ed8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            if c.cond(13) { pc = 0x456f01; continue; }
            pc = 0x456efb; continue;
            }
            0x456efb => {
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            pc = 0x456f1f; continue;
            }
            0x456f01 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x28fe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x28fc),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffu32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            pc = 0x456f1f; continue;
            }
            0x456f1f => {
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x456f2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x456f2e; continue;
            }
            0x456f2e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x456bdf; continue; }
            pc = 0x456f43; continue;
            }
            0x456f43 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x456f54; continue; }
            pc = 0x456f4b; continue;
            }
            0x456f4b => {
            c.push(m,c.get(0,32));
            c.push(m, 0x456f51);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x456f54; continue;
            }
            0x456f54 => {
            c.set(0,8,0x1u32);
            pc = 0x456f56; continue;
            }
            0x456f56 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xe8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xe4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_add_battle_marker(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x457c90;
    loop {
        match pc {
            0x457c90 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x457cca);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x457cdf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(3,32,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x457cf4);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x457d0d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x457d24);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x457d3b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            { let a = c.r[3].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[5].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x29dc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x457d8c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x457d96);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x457da0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x457dd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x457de3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_remove_battle_marker(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x457df0;
    loop {
        match pc {
            0x457df0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x457e1d; continue; }
            pc = 0x457e07; continue;
            }
            0x457e07 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x457e11; continue;
            }
            0x457e11 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x457e1d; continue; }
            pc = 0x457e15; continue;
            }
            0x457e15 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x457e11; continue; }
            pc = 0x457e1d; continue;
            }
            0x457e1d => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x458042; continue; }
            pc = 0x457e25; continue;
            }
            0x457e25 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            c.set(7,32,c.r[3].wrapping_mul(4));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x19u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(0,32); let r = c.add(a,0x32u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffe7));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x32u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x457e83);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x457e8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x457ff1; continue; }
            pc = 0x457e9e; continue;
            }
            0x457e9e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            c.set(2,32,c.r[0].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x457ec9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(1,32));
            c.push(m, 0x457ede);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(0,32));
            c.push(m, 0x457ef5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x457f07);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x457f19);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x457f2b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_mul(4));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[5]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x457f55);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x457f7b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x457fa3);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29dc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x29dc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x457fd0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x457fda);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x457fe4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x457ff1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b4),32));
            { let a = c.r[6].wrapping_add(0x29dc); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45800a; continue; }
            pc = 0x458001; continue;
            }
            0x458001 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x458007);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45800a; continue;
            }
            0x45800a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28b8),32));
            { let a = c.r[6].wrapping_add(0x28b4); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x458023; continue; }
            pc = 0x45801a; continue;
            }
            0x45801a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x458020);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x458023; continue;
            }
            0x458023 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28bc),32));
            { let a = c.r[6].wrapping_add(0x28b8); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45803c; continue; }
            pc = 0x458033; continue;
            }
            0x458033 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x458039);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45803c; continue;
            }
            0x45803c => {
            { let a = c.r[6].wrapping_add(0x28bc); wm(m,a,32,c.get(5,32)); }
            pc = 0x458042; continue;
            }
            0x458042 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_resize_town_flag_arrays(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45bbd0;
    loop {
        match pc {
            0x45bbd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,8,rm(m,0x6de4b0u32,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x28c4),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(6,32));
            c.push(m, 0x45bbfd);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.r[6].wrapping_add(0xffffffff));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45bc0e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5]).wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x28c4); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45bc2d; continue; }
            pc = 0x45bc24; continue;
            }
            0x45bc24 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45bc2a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45bc2d; continue;
            }
            0x45bc2d => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x28c8),32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45bc3d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45bc4b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5]).wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x28c8); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45bc6a; continue; }
            pc = 0x45bc61; continue;
            }
            0x45bc61 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45bc67);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45bc6a; continue;
            }
            0x45bc6a => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x28cc),32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x45bc7a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45bc88);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5]).wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[7].wrapping_add(0x28cc); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x45bca3; continue; }
            pc = 0x45bc9a; continue;
            }
            0x45bc9a => {
            c.push(m,c.get(3,32));
            c.push(m, 0x45bca0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45bca3; continue;
            }
            0x45bca3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_init_town_flag_arrays(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45bcb0;
    loop {
        match pc {
            0x45bcb0 => {
            c.push(m,c.get(6,32));
            c.push(m,0x28u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x45bcba);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x28u32);
            { let a = c.r[6].wrapping_add(0x28c4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45bcc7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x28u32);
            { let a = c.r[6].wrapping_add(0x28c8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45bcd4);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x28cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x6df2f0u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x548230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_get_class_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45c030;
    loop {
        match pc {
            0x45c030 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x697e3cu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x45c04b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn nav_map_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45c060;
    loop {
        match pc {
            0x45c060 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x45c068);
            { let args = []; c.r[0] = host.call(m, 0x4493f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x45c078; continue; }
            pc = 0x45c06f; continue;
            }
            0x45c06f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x45c075);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45c078; continue;
            }
            0x45c078 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ships_get_ship_ptr(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46cc00;
    loop {
        match pc {
            0x46cc00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn show_ship_in_game_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x476a30;
    loop {
        match pc {
            0x476a30 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x476a41);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x476ae8; continue; }
            pc = 0x476a49; continue;
            }
            0x476a49 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x677937u32,8));
            { let a = rm(m,c.r[1].wrapping_add(0xc324),32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x476ae8; continue; }
            pc = 0x476a62; continue;
            }
            0x476a62 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0x14u32,16); }
            if c.cond(5) { pc = 0x476cb2; continue; }
            pc = 0x476a74; continue;
            }
            0x476a74 => {
            c.set(6,32,rm(m,0x6e2d50u32,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x476cb2; continue; }
            pc = 0x476a82; continue;
            }
            0x476a82 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x476a8d);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x476aa2; continue; }
            pc = 0x476a92; continue;
            }
            0x476a92 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x476a82; continue; }
            pc = 0x476a99; continue;
            }
            0x476a99 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x476aa2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x476ab0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x476aca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x476adf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x476ae8 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xeu32,16); }
            if c.cond(5) { pc = 0x476b99; continue; }
            pc = 0x476b00; continue;
            }
            0x476b00 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x476b0a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x476b17); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x476b33);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x809),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x808));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,8,rm(m,c.r[0],8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x476b66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m, 0x476b73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x476b83); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x476b90); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x476b99 => {
            c.push(m, 0x476b9e);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x476c53; continue; }
            pc = 0x476ba6; continue;
            }
            0x476ba6 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x476bb1);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x476bc3; continue; }
            pc = 0x476bb5; continue;
            }
            0x476bb5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xdu32,16); }
            if c.cond(7) { pc = 0x476c53; continue; }
            pc = 0x476bc3; continue;
            }
            0x476bc3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x476bdf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(7,32,rm(m,0x6e2d50u32,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x476cb2; continue; }
            pc = 0x476bed; continue;
            }
            0x476bed => {
            c.set(3,16,rm(m,c.r[7].wrapping_add(0x38),16));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x476bf8);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x476c0d; continue; }
            pc = 0x476bfd; continue;
            }
            0x476bfd => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x476bed; continue; }
            pc = 0x476c04; continue;
            }
            0x476c04 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x476c0d => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x476c1b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x476c35);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x476c4a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x476c53 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x476c63); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x476c70); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x476c80); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x476c8d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x22),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x476ca2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x476cb2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x476cb2; continue;
            }
            0x476cb2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sort_ship_indices_by_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x477cf0;
    loop {
        match pc {
            0x477cf0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x477d8e; continue; }
            pc = 0x477d00; continue;
            }
            0x477d00 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(0x1));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(7,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)));
            pc = 0x477d12; continue;
            }
            0x477d12 => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x477d7b; continue; }
            pc = 0x477d16; continue;
            }
            0x477d16 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            pc = 0x477d20; continue;
            }
            0x477d20 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x160));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m, 0x477d52);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(5,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x477d6e; continue; }
            pc = 0x477d60; continue;
            }
            0x477d60 => {
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(2)),16));
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.r[7]; wm(m,a,16,c.get(1,16)); }
            { let a = c.r[3].wrapping_add(c.r[6].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            pc = 0x477d6e; continue;
            }
            0x477d6e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x477d20; continue; }
            pc = 0x477d77; continue;
            }
            0x477d77 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x477d7b; continue;
            }
            0x477d7b => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,c.r[6].wrapping_add(0xffffffff));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x477d12; continue; }
            pc = 0x477d8a; continue;
            }
            0x477d8a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x477d8e; continue;
            }
            0x477d8e => {
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_get_displayed_hull_percent(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x495340;
    loop {
        match pc {
            0x495340 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4953b7; continue; }
            pc = 0x495365; continue;
            }
            0x495365 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x49536b);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,0x66e0f0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x64u32,32); }
            if c.cond(14) { pc = 0x4953b1; continue; }
            pc = 0x4953a8; continue;
            }
            0x4953a8 => {
            c.set(6,32,0x64u32);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4953b1 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x4953b7; continue; }
            pc = 0x4953b5; continue;
            }
            0x4953b5 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4953b7; continue;
            }
            0x4953b7 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_get_displayed_sailors(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4953c0;
    loop {
        match pc {
            0x4953c0 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4953db; continue; }
            pc = 0x4953d3; continue;
            }
            0x4953d3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4953db => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4953e3);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0f4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x40),16));
            c.set(6,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x495431; continue; }
            pc = 0x495427; continue;
            }
            0x495427 => {
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x495431 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x495438);
            { let args = []; c.r[0] = host.call(m, 0x5185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x495445; continue; }
            pc = 0x49543c; continue;
            }
            0x49543c => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x495443);
            { let args = []; c.r[0] = host.call(m, 0x5185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            pc = 0x495445; continue;
            }
            0x495445 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_get_displayed_artillery(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x495450;
    loop {
        match pc {
            0x495450 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x49546b; continue; }
            pc = 0x495463; continue;
            }
            0x495463 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x120),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49546b => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x495473);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0f8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x120),32));
            c.set(6,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            if c.cond(9) { pc = 0x4954b7; continue; }
            pc = 0x4954b5; continue;
            }
            0x4954b5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4954b7; continue;
            }
            0x4954b7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_get_displayed_cargo_weight(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4954c0;
    loop {
        match pc {
            0x4954c0 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4954db; continue; }
            pc = 0x4954d3; continue;
            }
            0x4954d3 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x118),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4954db => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4954e3);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0ecu32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x118),32));
            c.set(6,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            if c.cond(9) { pc = 0x495541; continue; }
            pc = 0x49553d; continue;
            }
            0x49553d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x495541 => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x49554a; continue; }
            pc = 0x495548; continue;
            }
            0x495548 => {
            c.set(0,32,c.get(7,32));
            pc = 0x49554a; continue;
            }
            0x49554a => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_clear_unused(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4b10;
    loop {
        match pc {
            0x4d4b10 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6]; wm(m,a,16,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,0x0u32); }
            if c.cond(4) { pc = 0x4d4b35; continue; }
            pc = 0x4d4b25; continue;
            }
            0x4d4b25 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4d4b2b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x4d4b35; continue;
            }
            0x4d4b35 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4b40;
    loop {
        match pc {
            0x4d4b40 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x4));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4b60;
    loop {
        match pc {
            0x4d4b60 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d4b72);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[7].wrapping_add(0x2));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d4b81);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[3],16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d4ba0);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[3],16));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4bc0;
    loop {
        match pc {
            0x4d4bc0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4d4bd2);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(7,32,c.r[3].wrapping_add(0x2));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d4be1);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d4bf7; continue; }
            pc = 0x4d4bee; continue;
            }
            0x4d4bee => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4d4bf4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4d4bf7; continue;
            }
            0x4d4bf7 => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4d4c4c; continue; }
            pc = 0x4d4bff; continue;
            }
            0x4d4bff => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4d4c16);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[7],16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d4c32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d4c4c => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4c60;
    loop {
        match pc {
            0x4d4c60 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6]; wm(m,a,16,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,0x0u32); }
            if c.cond(4) { pc = 0x4d4c85; continue; }
            pc = 0x4d4c75; continue;
            }
            0x4d4c75 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4d4c7b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x4d4c85; continue;
            }
            0x4d4c85 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_alloc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4c90;
    loop {
        match pc {
            0x4d4c90 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = rm(m,c.r[6],16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x4d4d42; continue; }
            pc = 0x4d4ca4; continue;
            }
            0x4d4ca4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x96u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4d4cc0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4d4cd3; continue; }
            pc = 0x4d4cc9; continue;
            }
            0x4d4cc9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4d4cd3 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4d4cff; continue; }
            pc = 0x4d4cdc; continue;
            }
            0x4d4cdc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4d4cff; continue; }
            pc = 0x4d4ce3; continue;
            }
            0x4d4ce3 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d4cfc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4d4cff; continue;
            }
            0x4d4cff => {
            c.set(2,32,0x96u32);
            c.set(3,8,0xffu32);
            pc = 0x4d4d06; continue;
            }
            0x4d4d06 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,0x0u32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            if c.cond(5) { pc = 0x4d4d06; continue; }
            pc = 0x4d4d33; continue;
            }
            0x4d4d33 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d4d3c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            pc = 0x4d4d42; continue;
            }
            0x4d4d42 => {
            c.set(1,16,rm(m,c.r[6],16));
            c.set(5,32,0x673a78u32);
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[0],16));
            c.set(3,32,c.r[0].wrapping_add(0x1c));
            { let a = c.r[6]; wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,8,0x0u32); }
            c.set(7,32,c.r[0].wrapping_add(0x7c));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4d4d79; continue;
            }
            0x4d4d79 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c2b),8));
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.fld(c.ld(m,c.r[5].wrapping_add(0xfffffffc),4));
            { let a = c.get(5,32); let r = c.sub(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d4d96; continue; }
            pc = 0x4d4d8e; continue;
            }
            0x4d4d8e => {
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            pc = 0x4d4d9c; continue;
            }
            0x4d4d96 => {
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            pc = 0x4d4d9c; continue;
            }
            0x4d4d9c => {
            c.push(m, 0x4d4da1);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x698550),8));
            c.set(1,32,c.r[6].wrapping_add(0x18));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[3]; wm(m,a,8,c.get(0,8)); }
            if c.cond(15) { pc = 0x4d4d79; continue; }
            pc = 0x4d4db9; continue;
            }
            0x4d4db9 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x10),16));
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

pub fn route_stop_pool_remove_stop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4e80;
    loop {
        match pc {
            0x4d4e80 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4d4ebb; continue; }
            pc = 0x4d4e9f; continue;
            }
            0x4d4e9f => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d4ee0; continue; }
            pc = 0x4d4ea3; continue;
            }
            0x4d4ea3 => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4d4e9f; continue; }
            pc = 0x4d4ebb; continue;
            }
            0x4d4ebb => {
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[1]),16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x4d4ef7; continue; }
            pc = 0x4d4ed0; continue;
            }
            0x4d4ed0 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            pc = 0x4d4efc; continue;
            }
            0x4d4ee0 => {
            c.push(m,0xffffu32);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4d4ef0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x509030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d4ef7 => {
            c.push(m,0xffffu32);
            pc = 0x4d4efc; continue;
            }
            0x4d4efc => {
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4d4f07);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x509030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,c.get(6,32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[2]),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d4f31; continue; }
            pc = 0x4d4f1a; continue;
            }
            0x4d4f1a => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d4f1a; continue; }
            pc = 0x4d4f31; continue;
            }
            0x4d4f31 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[2]),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.r[3].wrapping_add(c.r[2]); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7]; wm(m,a,16,c.get(6,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_remove_stop_range(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4f60;
    loop {
        match pc {
            0x4d4f60 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4d4fd8; continue; }
            pc = 0x4d4f70; continue;
            }
            0x4d4f70 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d4f7c; continue;
            }
            0x4d4f7c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4d517f; continue; }
            pc = 0x4d4f8a; continue;
            }
            0x4d4f8a => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(5,32,c.get(0,32));
            c.set(3,8,0xfbu32);
            pc = 0x4d4f91; continue;
            }
            0x4d4f91 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x3),8); let r = c.and(a,c.get(3,8),8); { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x3); wm(m,a,8,r); } }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[0]),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4d4fbe; continue; }
            pc = 0x4d4fb4; continue;
            }
            0x4d4fb4 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(2) { pc = 0x4d4fbe; continue; }
            pc = 0x4d4fba; continue;
            }
            0x4d4fba => {
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d4f91; continue; }
            pc = 0x4d4fbe; continue;
            }
            0x4d4fbe => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3));
            { let a = rm(m,c.r[0],8); let r = c.or(a,0x4u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4d4fd8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4d5059; continue; }
            pc = 0x4d4fe2; continue;
            }
            0x4d4fe2 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(3,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)),16));
            { let a = c.get(5,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d5059; continue; }
            pc = 0x4d4ff8; continue;
            }
            0x4d4ff8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d5004; continue;
            }
            0x4d5004 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(6,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x3),8));
            { let a = c.get(2,8); let r = c.or(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x2),16));
            pc = 0x4d5021; continue;
            }
            0x4d5021 => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[3]),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x3),8));
            { let a = c.get(2,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4d5046; continue; }
            pc = 0x4d503f; continue;
            }
            0x4d503f => {
            { let a = c.get(2,8); let r = c.sub(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            pc = 0x4d5046; continue;
            }
            0x4d5046 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4d517f; continue; }
            pc = 0x4d504e; continue;
            }
            0x4d504e => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d5021; continue; }
            pc = 0x4d5052; continue;
            }
            0x4d5052 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4d5059 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(7,16,rm(m,c.r[0],16));
            c.set(0,32,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)),16));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d50ac; continue; }
            pc = 0x4d507f; continue;
            }
            0x4d507f => {
            c.set(0,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d5090; continue;
            }
            0x4d5090 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d5098; continue;
            }
            0x4d5098 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)),16));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d507f; continue; }
            pc = 0x4d50ac; continue;
            }
            0x4d50ac => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d50dc; continue; }
            pc = 0x4d50b0; continue;
            }
            0x4d50b0 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4d517f; continue; }
            pc = 0x4d50b8; continue;
            }
            0x4d50b8 => {
            c.set(6,16,rm(m,c.r[1],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[0]; wm(m,a,16,0xffffu32); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4d50dc => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(7,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x3),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x4d5115; continue; }
            pc = 0x4d50f6; continue;
            }
            0x4d50f6 => {
            { let a = c.get(2,8); let r = c.sub(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[3]),16));
            c.set(5,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x3));
            { let a = rm(m,c.r[2],8); let r = c.or(a,0x4u32,8); { let a = c.r[2]; wm(m,a,8,r); } }
            pc = 0x4d5115; continue;
            }
            0x4d5115 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(5,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(3,16,rm(m,c.r[7].wrapping_add(c.r[2]),16));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[2].wrapping_add(c.r[5]); wm(m,a,16,c.get(3,16)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(c.r[5]),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d515e; continue; }
            pc = 0x4d513d; continue;
            }
            0x4d513d => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4d5186; continue; }
            pc = 0x4d5145; continue;
            }
            0x4d5145 => {
            c.set(5,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d513d; continue; }
            pc = 0x4d515a; continue;
            }
            0x4d515a => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4d515e; continue;
            }
            0x4d515e => {
            c.set(2,16,rm(m,c.r[1],16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[7].wrapping_add(c.r[3]); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,16,c.get(6,16)); }
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d517f; continue; }
            pc = 0x4d5175; continue;
            }
            0x4d5175 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[5]),16));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            pc = 0x4d517f; continue;
            }
            0x4d517f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4d5186 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(c.r[0]),16));
            { let a = c.r[7].wrapping_add(c.r[3]); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,16,c.get(6,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d51ab; continue;
            }
            0x4d51ab => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x3),8));
            c.set(0,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x3));
            { let a = c.get(3,8); let r = c.or(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[2]),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d517f; continue; }
            pc = 0x4d51ce; continue;
            }
            0x4d51ce => {
            c.set(3,8,0xfbu32);
            pc = 0x4d51d0; continue;
            }
            0x4d51d0 => {
            c.set(7,32,c.get(5,32));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4d517f; continue; }
            pc = 0x4d51d7; continue;
            }
            0x4d51d7 => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3),8); let r = c.and(a,c.get(3,8),8); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3); wm(m,a,8,r); } }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2]),16));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4d51d0; continue; }
            pc = 0x4d51f6; continue;
            }
            0x4d51f6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_execute_trade_orders(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5200;
    loop {
        match pc {
            0x4d5200 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662978u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x132),16));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4d5245);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4d5839; continue; }
            pc = 0x4d5261; continue;
            }
            0x4d5261 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x39),8));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2),8));
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4d5293; continue; }
            pc = 0x4d526b; continue;
            }
            0x4d526b => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d5272);
            { let args = []; c.r[0] = host.call(m, 0x51a3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.set(0,16,rm(m,c.r[3],8));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4d528e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x4d5839; continue;
            }
            0x4d5293 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4d52a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3],8));
            c.set(6,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4d52d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x608),32));
            c.set(2,32,0x1u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x76c),32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sbb(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(4,8); c.and(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x4d531a; continue; }
            pc = 0x4d52f6; continue;
            }
            0x4d52f6 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d5310; continue; }
            pc = 0x4d52fa; continue;
            }
            0x4d52fa => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4d5310; continue; }
            pc = 0x4d5306; continue;
            }
            0x4d5306 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            pc = 0x4d531e; continue;
            }
            0x4d5310 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            pc = 0x4d531e; continue;
            }
            0x4d531a => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            pc = 0x4d531e; continue;
            }
            0x4d531e => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(2) { pc = 0x4d5337; continue; }
            pc = 0x4d532b; continue;
            }
            0x4d532b => {
            { let a = c.r[3].wrapping_add(0x136); wm(m,a,8,0x0u32); }
            pc = 0x4d5839; continue;
            }
            0x4d5337 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa),8));
            c.set(0,32,0x2fa0be83u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,8,0x32u32);
            { let a = c.get(1,8); let r = c.sub(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d5376; continue;
            }
            0x4d5376 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4),8));
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); c.sub(a,0x18u32,32); }
            if c.cond(3) { pc = 0x4d5574; continue; }
            pc = 0x4d5387; continue;
            }
            0x4d5387 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d54e7; continue; }
            pc = 0x4d5393; continue;
            }
            0x4d5393 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d5574; continue; }
            pc = 0x4d539b; continue;
            }
            0x4d539b => {
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d53ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52ea80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4d5574; continue; }
            pc = 0x4d53c1; continue;
            }
            0x4d53c1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4d53d1; continue; }
            pc = 0x4d53c9; continue;
            }
            0x4d53c9 => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(1,32));
            pc = 0x4d53d9; continue;
            }
            0x4d53d1 => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x1u32); }
            pc = 0x4d53d9; continue;
            }
            0x4d53d9 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4d53e7; continue; }
            pc = 0x4d53e1; continue;
            }
            0x4d53e1 => {
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d53e7; continue;
            }
            0x4d53e7 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4d546c; continue; }
            pc = 0x4d540d; continue;
            }
            0x4d540d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d541f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x124),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x124); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d5447);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x4u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d5468);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4d546c; continue;
            }
            0x4d546c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d5570; continue; }
            pc = 0x4d5478; continue;
            }
            0x4d5478 => {
            { let a = rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4d5570; continue; }
            pc = 0x4d5482; continue;
            }
            0x4d5482 => {
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4d5494);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1c),32),32); }
            if c.cond(14) { pc = 0x4d5570; continue; }
            pc = 0x4d549e; continue;
            }
            0x4d549e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,0x1u32);
            c.set(7,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x124),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(0x124); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d54ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d5570; continue;
            }
            0x4d54e7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4d5574; continue; }
            pc = 0x4d54f3; continue;
            }
            0x4d54f3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d5574; continue; }
            pc = 0x4d54f7; continue;
            }
            0x4d54f7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4d5574; continue; }
            pc = 0x4d54ff; continue;
            }
            0x4d54ff => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4d550b; continue; }
            pc = 0x4d5509; continue;
            }
            0x4d5509 => {
            c.set(2,32,c.get(1,32));
            pc = 0x4d550b; continue;
            }
            0x4d550b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d5531; continue; }
            pc = 0x4d5515; continue;
            }
            0x4d5515 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x4d554e; continue;
            }
            0x4d5531 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x4d554e; continue;
            }
            0x4d554e => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,0x4u32);
            { let a = c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d555d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d5570);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ff6f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4d5570; continue;
            }
            0x4d5570 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4d5574; continue;
            }
            0x4d5574 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4d5376; continue; }
            pc = 0x4d5586; continue;
            }
            0x4d5586 => {
            { let a = rm(m,c.r[5].wrapping_add(0x3),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4d55fe; continue; }
            pc = 0x4d558c; continue;
            }
            0x4d558c => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x4d55fe; continue; }
            pc = 0x4d5599; continue;
            }
            0x4d5599 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x15e),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x7u32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x39),8));
            { let a = c.r[4].wrapping_add(0x39); wm(m,a,8,c.get(1,8)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),8));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x124),32));
            { let a = c.r[3].wrapping_add(0x124); wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.r[4].wrapping_add(0x3e); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4d55fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4d55fe; continue;
            }
            0x4d55fe => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d5605);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4d5615; continue; }
            pc = 0x4d560d; continue;
            }
            0x4d560d => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x0u32); }
            pc = 0x4d5615; continue;
            }
            0x4d5615 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[3],8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d5624);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d562e; continue;
            }
            0x4d562e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4),8));
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,0x18u32,32); }
            if c.cond(3) { pc = 0x4d580d; continue; }
            pc = 0x4d563f; continue;
            }
            0x4d563f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4d585f; continue; }
            pc = 0x4d564b; continue;
            }
            0x4d564b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4d580d; continue; }
            pc = 0x4d5657; continue;
            }
            0x4d5657 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4d5678);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ec50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d5684; continue; }
            pc = 0x4d5682; continue;
            }
            0x4d5682 => {
            c.set(6,32,c.get(0,32));
            pc = 0x4d5684; continue;
            }
            0x4d5684 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d568e; continue; }
            pc = 0x4d568c; continue;
            }
            0x4d568c => {
            c.set(6,32,c.get(0,32));
            pc = 0x4d568e; continue;
            }
            0x4d568e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4d580d; continue; }
            pc = 0x4d5696; continue;
            }
            0x4d5696 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.imul(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4d56c8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); c.sub(a,0x1000000u32,32); }
            if c.cond(12) { pc = 0x4d56ec; continue; }
            pc = 0x4d56d7; continue;
            }
            0x4d56d7 => {
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(5,32,r); }
            pc = 0x4d56ff; continue;
            }
            0x4d56ec => {
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            pc = 0x4d56ff; continue;
            }
            0x4d56ff => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x4d57ad; continue; }
            pc = 0x4d570d; continue;
            }
            0x4d570d => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d5809; continue; }
            pc = 0x4d5715; continue;
            }
            0x4d5715 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d5743; continue; }
            pc = 0x4d5727; continue;
            }
            0x4d5727 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4d5760; continue;
            }
            0x4d5743 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4d5760; continue;
            }
            0x4d5760 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de3d8u32);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d5776);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); c.sub(a,0x1000000u32,32); }
            if c.cond(12) { pc = 0x4d579a; continue; }
            pc = 0x4d5785; continue;
            }
            0x4d5785 => {
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(5,32,r); }
            pc = 0x4d57ad; continue;
            }
            0x4d579a => {
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            pc = 0x4d57ad; continue;
            }
            0x4d57ad => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4d5809; continue; }
            pc = 0x4d57b1; continue;
            }
            0x4d57b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x4d57c9; continue; }
            pc = 0x4d57c1; continue;
            }
            0x4d57c1 => {
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x4d57c9; continue;
            }
            0x4d57c9 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d57d3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x517fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x124),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(0x124); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d5809);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4d5809; continue;
            }
            0x4d5809 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x4d580d; continue;
            }
            0x4d580d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4d562e; continue; }
            pc = 0x4d581f; continue;
            }
            0x4d581f => {
            { let a = rm(m,c.r[5].wrapping_add(0x3),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d582c; continue; }
            pc = 0x4d5825; continue;
            }
            0x4d5825 => {
            { let a = rm(m,c.r[3].wrapping_add(0x136),8); let r = c.and(a,0xfcu32,8); { let a = c.r[3].wrapping_add(0x136); wm(m,a,8,r); } }
            pc = 0x4d582c; continue;
            }
            0x4d582c => {
            { let a = rm(m,c.r[3].wrapping_add(0x3c),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4d5839; continue; }
            pc = 0x4d5832; continue;
            }
            0x4d5832 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d5839);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d5839; continue;
            }
            0x4d5839 => {
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d584a);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4d585f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4d580d; continue; }
            pc = 0x4d5867; continue;
            }
            0x4d5867 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d580d; continue; }
            pc = 0x4d586b; continue;
            }
            0x4d586b => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4d580d; continue; }
            pc = 0x4d5873; continue;
            }
            0x4d5873 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4d5879);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x500ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d587f; continue; }
            pc = 0x4d587d; continue;
            }
            0x4d587d => {
            c.set(6,32,c.get(0,32));
            pc = 0x4d587f; continue;
            }
            0x4d587f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d5889; continue; }
            pc = 0x4d5887; continue;
            }
            0x4d5887 => {
            c.set(6,32,c.get(0,32));
            pc = 0x4d5889; continue;
            }
            0x4d5889 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d58af; continue; }
            pc = 0x4d5893; continue;
            }
            0x4d5893 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4d58cc; continue;
            }
            0x4d58af => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4d58cc; continue;
            }
            0x4d58cc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x3b8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d58eb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5180c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[4].wrapping_add(0x54),32); let r = c.sub(a,c.get(6,32),32); { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,r); } }
            pc = 0x4d580d; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_plan_ship_to_next_useful_stop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5900;
    loop {
        match pc {
            0x4d5900 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x2),16));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(7,32,0xfffffu32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xfffffffu32); }
            if c.cond(3) { pc = 0x4d5990; continue; }
            pc = 0x4d5933; continue;
            }
            0x4d5933 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4d5952);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d5ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d597f; continue; }
            pc = 0x4d5956; continue;
            }
            0x4d5956 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(7,32,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4d597f; continue;
            }
            0x4d597f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d5933; continue; }
            pc = 0x4d5990; continue;
            }
            0x4d5990 => {
            { let a = c.get(7,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(2) { pc = 0x4d5a80; continue; }
            pc = 0x4d599c; continue;
            }
            0x4d599c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d59ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(7,32,c.get(1,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); c.and(a,0x4000600u32,32); }
            if c.cond(5) { pc = 0x4d5a02; continue; }
            pc = 0x4d59ce; continue;
            }
            0x4d59ce => {
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4d5a07; continue; }
            pc = 0x4d59d5; continue;
            }
            0x4d59d5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,8,0x2u32);
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(3) { pc = 0x4d5a02; continue; }
            pc = 0x4d59de; continue;
            }
            0x4d59de => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4d5a07; continue; }
            pc = 0x4d59eb; continue;
            }
            0x4d59eb => {
            c.set(3,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x4d5a07; continue; }
            pc = 0x4d5a02; continue;
            }
            0x4d5a02 => {
            c.set(7,32,0xffu32);
            pc = 0x4d5a07; continue;
            }
            0x4d5a07 => {
            c.set(5,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(2) { pc = 0x4d5a80; continue; }
            pc = 0x4d5a11; continue;
            }
            0x4d5a11 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4d5a80; continue; }
            pc = 0x4d5a17; continue;
            }
            0x4d5a17 => {
            { let a = c.get(2,32); let r = c.add(a,0x2c8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d5a21; continue;
            }
            0x4d5a21 => {
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); let r = c.and(a,0x4000041u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(5) { pc = 0x4d5a6d; continue; }
            pc = 0x4d5a2e; continue;
            }
            0x4d5a2e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0x6ddbb0),32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0x6ddbb4),32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x4d5a67; continue; }
            pc = 0x4d5a61; continue;
            }
            0x4d5a61 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d5a6d; continue; }
            pc = 0x4d5a67; continue;
            }
            0x4d5a67 => {
            c.set(7,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d5a6d; continue;
            }
            0x4d5a6d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x4d5a21; continue; }
            pc = 0x4d5a80; continue;
            }
            0x4d5a80 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4d5a97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5172a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_has_tradable_goods(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5ab0;
    loop {
        match pc {
            0x4d5ab0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x132),16));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5],8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,0x6de4a0u32);
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            c.push(m, 0x4d5b0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x608),32));
            c.set(2,32,0x1u32);
            c.set(1,8,c.get(3,8));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.and(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sbb(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.and(a,0x4000e00u32,32); }
            if c.cond(5) { pc = 0x4d5c3d; continue; }
            pc = 0x4d5b2c; continue;
            }
            0x4d5b2c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.and(a,0x8u32,8); }
            if c.cond(5) { pc = 0x4d5c3d; continue; }
            pc = 0x4d5b3a; continue;
            }
            0x4d5b3a => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xe),8));
            c.set(1,8,0x2u32);
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x4d5b57; continue; }
            pc = 0x4d5b43; continue;
            }
            0x4d5b43 => {
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4d5b88; continue; }
            pc = 0x4d5b4d; continue;
            }
            0x4d5b4d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d5b57 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4d5b88; continue; }
            pc = 0x4d5b64; continue;
            }
            0x4d5b64 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x4d5b88; continue; }
            pc = 0x4d5b7a; continue;
            }
            0x4d5b7a => {
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4d5c3d; continue; }
            pc = 0x4d5b88; continue;
            }
            0x4d5b88 => {
            c.set(0,16,rm(m,c.r[5],8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.push(m,0x0u32);
            c.set(2,8,c.get(3,8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d5ba0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d5bcc; continue; }
            pc = 0x4d5ba8; continue;
            }
            0x4d5ba8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x54));
            c.set(1,32,c.r[3].wrapping_add(0x7c));
            pc = 0x4d5bb0; continue;
            }
            0x4d5bb0 => {
            { let a = rm(m,c.r[1].wrapping_add(0xffffffa0),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x4d5bc0; continue; }
            pc = 0x4d5bb6; continue;
            }
            0x4d5bb6 => {
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x4d5bc0; continue; }
            pc = 0x4d5bbb; continue;
            }
            0x4d5bbb => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x4d5c15; continue; }
            pc = 0x4d5bc0; continue;
            }
            0x4d5bc0 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4d5bb0; continue; }
            pc = 0x4d5bcc; continue;
            }
            0x4d5bcc => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d5c02; continue; }
            pc = 0x4d5bd0; continue;
            }
            0x4d5bd0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x54));
            c.set(0,32,c.r[3].wrapping_add(0x7c));
            pc = 0x4d5bd8; continue;
            }
            0x4d5bd8 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xffffffa0),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4d5bf6; continue; }
            pc = 0x4d5bdf; continue;
            }
            0x4d5bdf => {
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4d5bef; continue; }
            pc = 0x4d5be5; continue;
            }
            0x4d5be5 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x4d5bf6; continue;
            }
            0x4d5bef => {
            if c.cond(13) { pc = 0x4d5bf6; continue; }
            pc = 0x4d5bf1; continue;
            }
            0x4d5bf1 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x4d5c15; continue; }
            pc = 0x4d5bf6; continue;
            }
            0x4d5bf6 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4d5bd8; continue; }
            pc = 0x4d5c02; continue;
            }
            0x4d5c02 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4d5c09);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d5c3d; continue; }
            pc = 0x4d5c0d; continue;
            }
            0x4d5c0d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d5c1f; continue; }
            pc = 0x4d5c15; continue;
            }
            0x4d5c15 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d5c1f => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d5c3d; continue; }
            pc = 0x4d5c23; continue;
            }
            0x4d5c23 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0x7c));
            pc = 0x4d5c28; continue;
            }
            0x4d5c28 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xffffffa0),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4d5c34; continue; }
            pc = 0x4d5c2f; continue;
            }
            0x4d5c2f => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x4d5c15; continue; }
            pc = 0x4d5c34; continue;
            }
            0x4d5c34 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4d5c28; continue; }
            pc = 0x4d5c3d; continue;
            }
            0x4d5c3d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_stop_pool_rebuild_free_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d60b0;
    loop {
        match pc {
            0x4d60b0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d60c1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d61c5; continue; }
            pc = 0x4d60ce; continue;
            }
            0x4d60ce => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4d60dd);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6],16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4d6111; continue; }
            pc = 0x4d60ef; continue;
            }
            0x4d60ef => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,8,0x1u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4d60ef; continue; }
            pc = 0x4d6111; continue;
            }
            0x4d6111 => {
            { let a = rm(m,0x6dd894u32,16); c.sub(a,0x0u32,16); }
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x4d618d; continue; }
            pc = 0x4d6124; continue;
            }
            0x4d6124 => {
            c.push(m,c.get(5,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4d6127; continue;
            }
            0x4d6127 => {
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x132),16));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            c.set(0,32,c.get(1,32));
            if c.cond(13) { pc = 0x4d616d; continue; }
            pc = 0x4d6142; continue;
            }
            0x4d6142 => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,8,0x1u32); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4d6163; continue; }
            pc = 0x4d615e; continue;
            }
            0x4d615e => {
            c.set(0,32,0xffffu32);
            pc = 0x4d6163; continue;
            }
            0x4d6163 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4d6142; continue; }
            pc = 0x4d616d; continue;
            }
            0x4d616d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x180u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4d6127; continue; }
            pc = 0x4d618c; continue;
            }
            0x4d618c => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4d618d; continue;
            }
            0x4d618d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2),16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4d61bb; continue; }
            pc = 0x4d6195; continue;
            }
            0x4d6195 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x4d6197; continue;
            }
            0x4d6197 => {
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x4d61aa; continue; }
            pc = 0x4d619d; continue;
            }
            0x4d619d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(3,16,rm(m,c.r[6],16));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6]; wm(m,a,16,c.get(0,16)); }
            pc = 0x4d61aa; continue;
            }
            0x4d61aa => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(1,32); let r = c.add(a,0xdcu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4d6197; continue; }
            pc = 0x4d61bb; continue;
            }
            0x4d61bb => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4d61c1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4d61c5; continue;
            }
            0x4d61c5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sort_ship_indices_by_repair_days(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fb870;
    loop {
        match pc {
            0x5fb870 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5fb968; continue; }
            pc = 0x5fb887; continue;
            }
            0x5fb887 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x5fb896; continue;
            }
            0x5fb896 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,c.r[2].wrapping_add(0x1));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x5fb94d; continue; }
            pc = 0x5fb8a9; continue;
            }
            0x5fb8a9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            pc = 0x5fb8b3; continue;
            }
            0x5fb8b3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,0x0u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5fb90c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52aad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5fb91a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52aad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(3,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x5fb93c; continue; }
            pc = 0x5fb929; continue;
            }
            0x5fb929 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fb939);
            { let args = []; c.r[0] = host.call(m, 0x477be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5fb93c; continue;
            }
            0x5fb93c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5fb8b3; continue; }
            pc = 0x5fb949; continue;
            }
            0x5fb949 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x5fb94d; continue;
            }
            0x5fb94d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x5fb896; continue; }
            pc = 0x5fb964; continue;
            }
            0x5fb964 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5fb968; continue;
            }
            0x5fb968 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_get_motion_serialized_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x626300;
    loop {
        match pc {
            0x626300 => {
            c.set(0,32,0x1du32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_serialize_motion(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x626310;
    loop {
        match pc {
            0x626310 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x626321);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x626330);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x6));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x62633f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x62634e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0xa));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x62635d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xc));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x62636c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0xd));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x62637c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x10));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x626389);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x14));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x626398);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x18));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6263a7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x20));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6263b6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x22u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6263c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_deserialize_motion(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6263d0;
    loop {
        match pc {
            0x6263d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x6263e1);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6263f0);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0x6));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x6263ff);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(2,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62640e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0xa));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62641d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.r[7].wrapping_add(0xc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x62642c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0xd));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62643c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(0,32,c.r[7].wrapping_add(0x10));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x626449);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(1,32,c.r[7].wrapping_add(0x14));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x626458);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(2,32,c.r[7].wrapping_add(0x18));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x626467);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x20));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x626476);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.add(a,0x22u32,32); c.set(7,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x626485);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_set_target_point(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6270c7;
    loop {
        match pc {
            0x6270c7 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x8),16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            c.set(7,32,sx(c.get(2,16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x6),16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x6270ed; continue;
            }
            0x6270ed => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x6270fb; continue; }
            pc = 0x6270f4; continue;
            }
            0x6270f4 => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x627101; continue; }
            pc = 0x6270fb; continue;
            }
            0x6270fb => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            pc = 0x6270ed; continue;
            }
            0x627101 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x62711d; continue; }
            pc = 0x627105; continue;
            }
            0x627105 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            c.set(0,16,c.get(0,8));
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x62711d => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x627172; continue; }
            pc = 0x627129; continue;
            }
            0x627129 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x62715c; continue; }
            pc = 0x627135; continue;
            }
            0x627135 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x67b348),8));
            c.set(2,32,c.get(0,32));
            c.set(0,32,0x40u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            pc = 0x6271aa; continue;
            }
            0x62715c => {
            c.set(2,32,0x200u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x67b348),8));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            pc = 0x6271aa; continue;
            }
            0x627172 => {
            { let a = c.get(6,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x62719b; continue; }
            pc = 0x62717a; continue;
            }
            0x62717a => {
            c.set(0,32,0x40000000u32);
            c.push(m,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(2,32,c.get(0,32));
            c.set(0,8,0x40u32);
            { let a = c.get(2,32); let r = c.add(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x67b348),8));
            { let a = c.get(0,8); let r = c.sub(a,c.get(3,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x6271aa; continue;
            }
            0x62719b => {
            { let a = c.get(6,32); let r = c.add(a,0x200u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x67b348),8));
            pc = 0x6271aa; continue;
            }
            0x6271aa => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x6271c0; continue; }
            pc = 0x6271ae; continue;
            }
            0x6271ae => {
            c.set(2,8,0x80u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(2,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6271c0 => {
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,8,c.get(0,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,16,c.get(2,16)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "convoy_choice_window_ctor" => convoy_choice_window_ctor(m, host, regs),
        "convoy_choice_window_hide" => convoy_choice_window_hide(m, host, regs),
        "convoy_choice_window_release_resources" => convoy_choice_window_release_resources(m, host, regs),
        "convoy_choice_window_clear_options" => convoy_choice_window_clear_options(m, host, regs),
        "convoy_choice_window_build_option_list" => convoy_choice_window_build_option_list(m, host, regs),
        "convoy_choice_window_open" => convoy_choice_window_open(m, host, regs),
        "convoy_choice_window_layout" => convoy_choice_window_layout(m, host, regs),
        "convoy_choice_window_update" => convoy_choice_window_update(m, host, regs),
        "convoy_choice_window_draw" => convoy_choice_window_draw(m, host, regs),
        "convoy_choice_window_slot54_noop" => convoy_choice_window_slot54_noop(m, host, regs),
        "convoy_choice_window_update_hover_row" => convoy_choice_window_update_hover_row(m, host, regs),
        "convoy_choice_window_refresh_marked_row" => convoy_choice_window_refresh_marked_row(m, host, regs),
        "convoy_choice_window_deleting_dtor" => convoy_choice_window_deleting_dtor(m, host, regs),
        "select_next_player_ship_in_port" => select_next_player_ship_in_port(m, host, regs),
        "select_next_player_ship_at_sea" => select_next_player_ship_at_sea(m, host, regs),
        "select_next_player_convoy_at_sea" => select_next_player_convoy_at_sea(m, host, regs),
        "select_next_player_convoy_in_port" => select_next_player_convoy_in_port(m, host, regs),
        "route_planner_ctor" => route_planner_ctor(m, host, regs),
        "route_planner_ctor_clear_town_count" => route_planner_ctor_clear_town_count(m, host, regs),
        "route_planner_free_nav_data" => route_planner_free_nav_data(m, host, regs),
        "route_planner_free_town_route_tables" => route_planner_free_town_route_tables(m, host, regs),
        "route_planner_dtor" => route_planner_dtor(m, host, regs),
        "nav_map_ctor" => nav_map_ctor(m, host, regs),
        "nav_map_ctor_named" => nav_map_ctor_named(m, host, regs),
        "nav_map_dtor" => nav_map_dtor(m, host, regs),
        "nav_map_reset_state" => nav_map_reset_state(m, host, regs),
        "nav_map_reset_interaction_state" => nav_map_reset_interaction_state(m, host, regs),
        "nav_map_slot3_return_zero" => nav_map_slot3_return_zero(m, host, regs),
        "nav_map_select_ships_in_rect" => nav_map_select_ships_in_rect(m, host, regs),
        "nav_map_slot54_noop" => nav_map_slot54_noop(m, host, regs),
        "nav_map_free_cached_route" => nav_map_free_cached_route(m, host, regs),
        "nav_map_select_player_ships_in_town_at_point" => nav_map_select_player_ships_in_town_at_point(m, host, regs),
        "nav_map_handle_point_click" => nav_map_handle_point_click(m, host, regs),
        "scroll_map_hit_test_battle_marker" => scroll_map_hit_test_battle_marker(m, host, regs),
        "scroll_map_get_hint_text" => scroll_map_get_hint_text(m, host, regs),
        "scroll_map_hover_town_status_icon" => scroll_map_hover_town_status_icon(m, host, regs),
        "scroll_map_cancel_route_drag" => scroll_map_cancel_route_drag(m, host, regs),
        "scroll_map_invalidate_view" => scroll_map_invalidate_view(m, host, regs),
        "scroll_map_add_dirty_rect" => scroll_map_add_dirty_rect(m, host, regs),
        "scroll_map_clear_scroll_blit_flag" => scroll_map_clear_scroll_blit_flag(m, host, regs),
        "scroll_map_validate_selected_ships_for_departure" => scroll_map_validate_selected_ships_for_departure(m, host, regs),
        "scroll_map_add_ship_problem" => scroll_map_add_ship_problem(m, host, regs),
        "scroll_map_handle_order_click" => scroll_map_handle_order_click(m, host, regs),
        "scroll_map_add_battle_marker" => scroll_map_add_battle_marker(m, host, regs),
        "scroll_map_remove_battle_marker" => scroll_map_remove_battle_marker(m, host, regs),
        "scroll_map_resize_town_flag_arrays" => scroll_map_resize_town_flag_arrays(m, host, regs),
        "scroll_map_init_town_flag_arrays" => scroll_map_init_town_flag_arrays(m, host, regs),
        "nav_map_get_class_name" => nav_map_get_class_name(m, host, regs),
        "nav_map_scalar_deleting_dtor" => nav_map_scalar_deleting_dtor(m, host, regs),
        "ships_get_ship_ptr" => ships_get_ship_ptr(m, host, regs),
        "show_ship_in_game_view" => show_ship_in_game_view(m, host, regs),
        "sort_ship_indices_by_name" => sort_ship_indices_by_name(m, host, regs),
        "ship_get_displayed_hull_percent" => ship_get_displayed_hull_percent(m, host, regs),
        "ship_get_displayed_sailors" => ship_get_displayed_sailors(m, host, regs),
        "ship_get_displayed_artillery" => ship_get_displayed_artillery(m, host, regs),
        "ship_get_displayed_cargo_weight" => ship_get_displayed_cargo_weight(m, host, regs),
        "route_stop_pool_clear_unused" => route_stop_pool_clear_unused(m, host, regs),
        "route_stop_pool_get_save_size" => route_stop_pool_get_save_size(m, host, regs),
        "route_stop_pool_serialize" => route_stop_pool_serialize(m, host, regs),
        "route_stop_pool_deserialize" => route_stop_pool_deserialize(m, host, regs),
        "route_stop_pool_clear" => route_stop_pool_clear(m, host, regs),
        "route_stop_pool_alloc" => route_stop_pool_alloc(m, host, regs),
        "route_stop_pool_remove_stop" => route_stop_pool_remove_stop(m, host, regs),
        "route_stop_pool_remove_stop_range" => route_stop_pool_remove_stop_range(m, host, regs),
        "route_stop_pool_execute_trade_orders" => route_stop_pool_execute_trade_orders(m, host, regs),
        "route_plan_ship_to_next_useful_stop" => route_plan_ship_to_next_useful_stop(m, host, regs),
        "route_stop_has_tradable_goods" => route_stop_has_tradable_goods(m, host, regs),
        "route_stop_pool_rebuild_free_list" => route_stop_pool_rebuild_free_list(m, host, regs),
        "sort_ship_indices_by_repair_days" => sort_ship_indices_by_repair_days(m, host, regs),
        "local_map_ship_get_motion_serialized_size" => local_map_ship_get_motion_serialized_size(m, host, regs),
        "local_map_ship_serialize_motion" => local_map_ship_serialize_motion(m, host, regs),
        "local_map_ship_deserialize_motion" => local_map_ship_deserialize_motion(m, host, regs),
        "local_map_ship_set_target_point" => local_map_ship_set_target_point(m, host, regs),
        _ => return None,
    })
}
