//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn video_mode_selector_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4028d0;
    loop {
        match pc {
            0x4028d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            pc = 0x4028d8; continue;
            }
            0x4028d8 => {
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x4028ed; continue; }
            pc = 0x4028dd; continue;
            }
            0x4028dd => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(2) { pc = 0x4028d8; continue; }
            pc = 0x4028e6; continue;
            }
            0x4028e6 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4028ed);
            { let args = []; c.r[0] = host.call(m, 0x402e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4028ed; continue;
            }
            0x4028ed => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x402923);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40292f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_has_valid_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402940;
    loop {
        match pc {
            0x402940 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1cu32,32); c.set(1,32,r); }
            pc = 0x402945; continue;
            }
            0x402945 => {
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x402956; continue; }
            pc = 0x40294a; continue;
            }
            0x40294a => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(2) { pc = 0x402945; continue; }
            pc = 0x402953; continue;
            }
            0x402953 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x402956 => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_is_lowest_mode_valid(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402960;
    loop {
        match pc {
            0x402960 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402970;
    loop {
        match pc {
            0x402970 => {
            { let a = c.r[1]; wm(m,a,32,0x66a7e4u32); }
            { let a = c.get(1,32); let r = c.add(a,0x34u32,32); c.set(1,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_resolution(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402980;
    loop {
        match pc {
            0x402980 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(6,32,c.get(1,32));
            if c.cond(14) { pc = 0x402994; continue; }
            pc = 0x40298d; continue;
            }
            0x40298d => {
            c.set(0,32,0x2u32);
            pc = 0x40299a; continue;
            }
            0x402994 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x40299a; continue; }
            pc = 0x402998; continue;
            }
            0x402998 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x40299a; continue;
            }
            0x40299a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(3,32,0x1u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4029bf; continue; }
            pc = 0x4029ad; continue;
            }
            0x4029ad => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4029b7);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4029bf => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4029de; continue; }
            pc = 0x4029c5; continue;
            }
            0x4029c5 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x4029d6);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4029de => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4029f9; continue; }
            pc = 0x4029e4; continue;
            }
            0x4029e4 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4029f1);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4029f9 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1c));
            pc = 0x402a00; continue;
            }
            0x402a00 => {
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402a14; continue; }
            pc = 0x402a04; continue;
            }
            0x402a04 => {
            c.set(0,32,c.get(1,32));
            c.set(5,32,0x3u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x402a14; continue;
            }
            0x402a14 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,0x6u32,32); }
            if c.cond(12) { pc = 0x402a00; continue; }
            pc = 0x402a1d; continue;
            }
            0x402a1d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x402a26);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_color_depth(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402a30;
    loop {
        match pc {
            0x402a30 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(3,32,0x1u32);
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(14) { pc = 0x402a46; continue; }
            pc = 0x402a42; continue;
            }
            0x402a42 => {
            c.set(0,32,c.get(3,32));
            pc = 0x402a4c; continue;
            }
            0x402a46 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x402a4c; continue; }
            pc = 0x402a4a; continue;
            }
            0x402a4a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x402a4c; continue;
            }
            0x402a4c => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402a6d; continue; }
            pc = 0x402a5a; continue;
            }
            0x402a5a => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x402a64);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x402a6d => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402a95; continue; }
            pc = 0x402a7b; continue;
            }
            0x402a7b => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m, 0x402a8c);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x402a95 => {
            { let a = rm(m,c.r[1].wrapping_add(0x20),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402ab0; continue; }
            pc = 0x402a9a; continue;
            }
            0x402a9a => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x402aa7);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x402ab0 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x9));
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402ad3; continue; }
            pc = 0x402ab9; continue;
            }
            0x402ab9 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x2u32); }
            c.push(m, 0x402aca);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x402ad3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(5,32));
            pc = 0x402ad9; continue;
            }
            0x402ad9 => {
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x402aed; continue; }
            pc = 0x402add; continue;
            }
            0x402add => {
            c.set(0,32,c.get(1,32));
            c.set(5,32,0x3u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x402aed; continue;
            }
            0x402aed => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,0x6u32,32); }
            if c.cond(12) { pc = 0x402ad9; continue; }
            pc = 0x402af6; continue;
            }
            0x402af6 => {
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x402afe);
            { let args = []; c.r[0] = host.call(m, 0x402ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_update_arrow_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402ba0;
    loop {
        match pc {
            0x402ba0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[0].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,c.r[2].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(5,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2c));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(3,32,c.r[6].wrapping_add(0x2c));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x2u32); }
            c.set(2,32,0x1u32);
            pc = 0x402bf1; continue;
            }
            0x402bf1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x402c0e; continue; }
            pc = 0x402bfb; continue;
            }
            0x402bfb => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402c0e; continue; }
            pc = 0x402c00; continue;
            }
            0x402c00 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c0a; continue; }
            pc = 0x402c05; continue;
            }
            0x402c05 => {
            { let a = rm(m,c.r[5],32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x402c0e; continue; }
            pc = 0x402c0a; continue;
            }
            0x402c0a => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            pc = 0x402c0e; continue;
            }
            0x402c0e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x402c2c; continue; }
            pc = 0x402c16; continue;
            }
            0x402c16 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402c2c; continue; }
            pc = 0x402c1b; continue;
            }
            0x402c1b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c28; continue; }
            pc = 0x402c24; continue;
            }
            0x402c24 => {
            { let a = rm(m,c.r[0],32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x402c2c; continue; }
            pc = 0x402c28; continue;
            }
            0x402c28 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            pc = 0x402c2c; continue;
            }
            0x402c2c => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x402c46; continue; }
            pc = 0x402c30; continue;
            }
            0x402c30 => {
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x402c46; continue; }
            pc = 0x402c34; continue;
            }
            0x402c34 => {
            { let a = rm(m,c.r[6].wrapping_add(0xfffffffc),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c42; continue; }
            pc = 0x402c39; continue;
            }
            0x402c39 => {
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c42; continue; }
            pc = 0x402c3d; continue;
            }
            0x402c3d => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x402c46; continue; }
            pc = 0x402c42; continue;
            }
            0x402c42 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(2,8)); }
            pc = 0x402c46; continue;
            }
            0x402c46 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x402c64; continue; }
            pc = 0x402c4e; continue;
            }
            0x402c4e => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x402c64; continue; }
            pc = 0x402c52; continue;
            }
            0x402c52 => {
            { let a = rm(m,c.r[3].wrapping_add(0xfffffffc),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c60; continue; }
            pc = 0x402c57; continue;
            }
            0x402c57 => {
            { let a = rm(m,c.r[3],32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x402c60; continue; }
            pc = 0x402c5b; continue;
            }
            0x402c5b => {
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x402c64; continue; }
            pc = 0x402c60; continue;
            }
            0x402c60 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            pc = 0x402c64; continue;
            }
            0x402c64 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,0xcu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,0xcu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x402bf1; continue; }
            pc = 0x402c9a; continue;
            }
            0x402c9a => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(7,32,c.r[1].wrapping_add(0xc));
            pc = 0x402c9f; continue;
            }
            0x402c9f => {
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x402caf; continue; }
            pc = 0x402ca5; continue;
            }
            0x402ca5 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[6]).wrapping_add(0x10),8));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x402caf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402caf; continue;
            }
            0x402caf => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x402c9f; continue; }
            pc = 0x402cb8; continue;
            }
            0x402cb8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_next_resolution(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402cc0;
    loop {
        match pc {
            0x402cc0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,0x1u32);
            c.set(2,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402ce1; continue; }
            pc = 0x402cd0; continue;
            }
            0x402cd0 => {
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402ce1; continue; }
            pc = 0x402cd5; continue;
            }
            0x402cd5 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402cf9; continue; }
            pc = 0x402cdb; continue;
            }
            0x402cdb => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x28),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402cf9; continue; }
            pc = 0x402ce1; continue;
            }
            0x402ce1 => {
            c.set(2,32,c.r[0].wrapping_add(0x2));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402cff; continue; }
            pc = 0x402ce8; continue;
            }
            0x402ce8 => {
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402cff; continue; }
            pc = 0x402ced; continue;
            }
            0x402ced => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402cf9; continue; }
            pc = 0x402cf3; continue;
            }
            0x402cf3 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x28),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x402cff; continue; }
            pc = 0x402cf9; continue;
            }
            0x402cf9 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x402cff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402cff; continue;
            }
            0x402cff => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_previous_resolution(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402d10;
    loop {
        match pc {
            0x402d10 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,0x1u32);
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402d31; continue; }
            pc = 0x402d20; continue;
            }
            0x402d20 => {
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402d31; continue; }
            pc = 0x402d25; continue;
            }
            0x402d25 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402d49; continue; }
            pc = 0x402d2b; continue;
            }
            0x402d2b => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x28),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402d49; continue; }
            pc = 0x402d31; continue;
            }
            0x402d31 => {
            c.set(2,32,c.r[0].wrapping_add(0xfffffffe));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402d4f; continue; }
            pc = 0x402d38; continue;
            }
            0x402d38 => {
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x402d4f; continue; }
            pc = 0x402d3d; continue;
            }
            0x402d3d => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x402d49; continue; }
            pc = 0x402d43; continue;
            }
            0x402d43 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x28),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x402d4f; continue; }
            pc = 0x402d49; continue;
            }
            0x402d49 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x402d4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402d4f; continue;
            }
            0x402d4f => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_next_color_depth(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402d60;
    loop {
        match pc {
            0x402d60 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402d96; continue; }
            pc = 0x402d6c; continue;
            }
            0x402d6c => {
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x402d96; continue; }
            pc = 0x402d71; continue;
            }
            0x402d71 => {
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1c),32));
            c.set(6,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x402d90; continue; }
            pc = 0x402d80; continue;
            }
            0x402d80 => {
            { let a = rm(m,c.r[6].wrapping_add(0x20),32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x402d90; continue; }
            pc = 0x402d86; continue;
            }
            0x402d86 => {
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x9));
            { let a = rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x402d96; continue; }
            pc = 0x402d90; continue;
            }
            0x402d90 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x402d96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402d96; continue;
            }
            0x402d96 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_select_previous_color_depth(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402da0;
    loop {
        match pc {
            0x402da0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x402dd6; continue; }
            pc = 0x402dac; continue;
            }
            0x402dac => {
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x402dd6; continue; }
            pc = 0x402db1; continue;
            }
            0x402db1 => {
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1c),32));
            c.set(6,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x402dd0; continue; }
            pc = 0x402dc0; continue;
            }
            0x402dc0 => {
            { let a = rm(m,c.r[6].wrapping_add(0x20),32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x402dd0; continue; }
            pc = 0x402dc6; continue;
            }
            0x402dc6 => {
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x9));
            { let a = rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x402dd6; continue; }
            pc = 0x402dd0; continue;
            }
            0x402dd0 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x402dd6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402dd6; continue;
            }
            0x402dd6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_load_mode_states_from_registry(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402e00;
    loop {
        match pc {
            0x402e00 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65af4bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x824u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x692a78u32);
            c.push(m,0xf003fu32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x692980u32);
            c.push(m,0x80000001u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,0x66a00cu32,32); c.push(m, 0x402e4c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(40);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x402f86; continue; }
            pc = 0x402e54; continue;
            }
            0x402e54 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a010u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,0x692a18u32);
            { let a = c.get(5,32); let r = c.add(a,0x1cu32,32); c.set(5,32,r); }
            pc = 0x402e65; continue;
            }
            0x402e65 => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x840); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x402e84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x692a84u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x402ea2);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x454));
            c.push(m,0x692a90u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x402ec3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x440));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x400u32); }
            { let target = c.get(3,32); c.push(m, 0x402eed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x402f1b; continue; }
            pc = 0x402ef1; continue;
            }
            0x402ef1 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x402f1b; continue; }
            pc = 0x402ef7; continue;
            }
            0x402ef7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x692a6c),32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x402f09);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x402f18; continue; }
            pc = 0x402f10; continue;
            }
            0x402f10 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x402ef7; continue; }
            pc = 0x402f16; continue;
            }
            0x402f16 => {
            pc = 0x402f1b; continue;
            }
            0x402f18 => {
            { let a = c.r[5]; wm(m,a,32,c.get(6,32)); }
            pc = 0x402f1b; continue;
            }
            0x402f1b => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x83c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x402f2f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x692a30u32,32); }
            if c.cond(12) { pc = 0x402e65; continue; }
            pc = 0x402f41; continue;
            }
            0x402f41 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x692a9cu32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x400u32); }
            { let target = c.get(3,32); c.push(m, 0x402f66); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x402f86; continue; }
            pc = 0x402f6d; continue;
            }
            0x402f6d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x402f86; continue; }
            pc = 0x402f75; continue;
            }
            0x402f75 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(0x34));
            c.push(m, 0x402f86);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x402f86; continue;
            }
            0x402f86 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x828),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x830u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn video_mode_selector_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402fa0;
    loop {
        match pc {
            0x402fa0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x402fa8);
            { let args = []; c.r[0] = host.call(m, 0x402970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x402fb8; continue; }
            pc = 0x402faf; continue;
            }
            0x402faf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x402fb5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x402fb8; continue;
            }
            0x402fb8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hot_key_ctrl_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x408fd0;
    loop {
        match pc {
            0x408fd0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x408fd8);
            { let args = []; c.r[0] = host.call(m, 0x656fa3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x408fe8; continue; }
            pc = 0x408fdf; continue;
            }
            0x408fdf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x408fe5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x408fe8; continue;
            }
            0x408fe8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x409050;
    loop {
        match pc {
            0x409050 => {
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x409061);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(1,16,rm(m,0x6de4aau32,16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x409506; continue; }
            pc = 0x40907d; continue;
            }
            0x40907d => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x409088);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x38),8));
            { let a = c.r[6].wrapping_add(0xcd); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.r[6].wrapping_add(0xce); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3a),8));
            { let a = c.r[6].wrapping_add(0xcf); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3b),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4090bf; continue; }
            pc = 0x4090b3; continue;
            }
            0x4090b3 => {
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,0x0u32); }
            pc = 0x4090c9; continue;
            }
            0x4090bf => {
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,0x1u32); }
            pc = 0x4090c9; continue;
            }
            0x4090c9 => {
            c.set(0,32,rm(m,0x6cbb00u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4090f5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x40910c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x409121); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6c105cu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x40914b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,c.r[6].wrapping_add(0x728));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x409166); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x40917b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dcf20u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcf20u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40918e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4091a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4091ac); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4091b8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4091cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x41c5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4091d2);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x640));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x64u32); }
            c.push(m, 0x409200);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x409207); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffec));
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x409220); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffec));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x40923e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409247); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x409253); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffe2));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0xd80));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x409291; continue;
            }
            0x409291 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x2b8));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4092b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4092c2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xcd),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,8,u32::from(c.cond(7)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4092db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4092e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4092fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x409306); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xcd),8); c.sub(a,0x5u32,8); }
            c.set(1,8,u32::from(c.cond(2)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x40931d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409329); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,c.r[7].wrapping_add(0xfffffc60));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40934a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x409357); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409361); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x40936e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.sub(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,c.r[3].wrapping_add(0x288));
            c.push(m,c.get(5,32));
            c.push(m, 0x409389);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x409396); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0x8u32);
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xcc),8));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4093ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,c.r[7].wrapping_add(0xfffff9a8));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4093e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4093f2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4093fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x409409); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m, 0x40941e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x40942a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0x1u32);
            c.set(1,32,c.get(5,32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xcc),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x40944f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xd8u32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x409291; continue; }
            pc = 0x40948c; continue;
            }
            0x40948c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.r[6].wrapping_add(0xc98));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x40949e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4094aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4094b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4094bf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,c.r[6].wrapping_add(0x1290));
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4094d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4094e4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4094fa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x409506; continue;
            }
            0x409506 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_paint(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x409780;
    loop {
        match pc {
            0x409780 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4097bf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4099fa; continue; }
            pc = 0x4097c7; continue;
            }
            0x4097c7 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m, 0x4097d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4099fa; continue; }
            pc = 0x4097dc; continue;
            }
            0x4097dc => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0xb4),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0xb8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.push(m, 0x409809);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x409814);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x409823);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41caa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6dcf20u32);
            c.push(m, 0x40982d);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x409833);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x40983a);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x409846);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6c0f34u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[3].wrapping_add(0x1));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x40985a);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x409866);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c0f34u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x409873);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x40987d);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xbc),32));
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x40988a; continue;
            }
            0x40988a => {
            c.push(m,0x1u32);
            c.push(m, 0x409891);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xa8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4098b2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4098b8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6c0fb8),32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe2));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4098ca);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xa4),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4098f3);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4098f9);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x409900);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[3]).wrapping_add(0xcd),8));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c1058),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40991c);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.push(m, 0x40992b);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c0fc4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe2));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40993b);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.set(1,32,rm(m,0x6c0fc8u32,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x409951);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.set(2,32,rm(m,0x6c0fccu32,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x409967);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xbc),32));
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x40988a; continue; }
            pc = 0x40997c; continue;
            }
            0x40997c => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xa8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x40999a);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4099a0);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c0fd0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe2));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4099b1);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.set(7,32,rm(m,c.r[7].wrapping_add(0xa4),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4099da);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4099e0);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c0fd4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4099ed);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4099f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x4099fa; continue;
            }
            0x4099fa => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x409a10;
    loop {
        match pc {
            0x409a10 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(0x640));
            c.push(m, 0x409a23);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x409a38; continue; }
            pc = 0x409a27; continue;
            }
            0x409a27 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x409a31); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409a38 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x9e0));
            pc = 0x409a40; continue;
            }
            0x409a40 => {
            c.set(1,32,c.r[3].wrapping_add(0xfffffd48));
            c.push(m, 0x409a4b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409acd; continue; }
            pc = 0x409a4f; continue;
            }
            0x409a4f => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x409a56);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409bbd; continue; }
            pc = 0x409a5e; continue;
            }
            0x409a5e => {
            c.set(1,32,c.r[3].wrapping_add(0xfffff6f0));
            c.push(m, 0x409a69);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409cae; continue; }
            pc = 0x409a71; continue;
            }
            0x409a71 => {
            c.set(1,32,c.r[3].wrapping_add(0xfffff9a8));
            c.push(m, 0x409a7c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409d09; continue; }
            pc = 0x409a84; continue;
            }
            0x409a84 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x409a40; continue; }
            pc = 0x409a90; continue;
            }
            0x409a90 => {
            c.set(1,32,c.r[7].wrapping_add(0xc98));
            c.push(m, 0x409a9b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x409ac6; continue; }
            pc = 0x409a9f; continue;
            }
            0x409a9f => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0xc8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.set(1,32,c.r[7].wrapping_add(0x1290));
            { let a = c.r[7].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409ac6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x409ac6; continue;
            }
            0x409ac6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409acd => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x409b2f; continue; }
            pc = 0x409ad8; continue;
            }
            0x409ad8 => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x409b06; continue; }
            pc = 0x409ae7; continue;
            }
            0x409ae7 => {
            c.set(2,32,c.r[6].wrapping_mul(8));
            c.push(m,0x1u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x9e0),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x9e0));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409b06); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x409b06; continue;
            }
            0x409b06 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd),8); c.sub(a,0x5u32,8); }
            if c.cond(5) { pc = 0x409b2f; continue; }
            pc = 0x409b10; continue;
            }
            0x409b10 => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x728),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x728));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409b2f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x409b2f; continue;
            }
            0x409b2f => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(6,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x9e0));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x409b52); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x728),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x728u32,32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x409b78); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x409b7f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x409ba7; continue; }
            pc = 0x409b9f; continue;
            }
            0x409b9f => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x409ba7; continue;
            }
            0x409ba7 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x409bb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409bbd => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x409c20; continue; }
            pc = 0x409bc8; continue;
            }
            0x409bc8 => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x409bf6; continue; }
            pc = 0x409bd7; continue;
            }
            0x409bd7 => {
            c.set(2,32,c.r[6].wrapping_mul(8));
            c.push(m,0x1u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x728),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x728));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409bf6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x409bf6; continue;
            }
            0x409bf6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0xcd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409c20; continue; }
            pc = 0x409c01; continue;
            }
            0x409c01 => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x9e0),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x9e0));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x409c20); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x409c20; continue;
            }
            0x409c20 => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(6,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x9e0));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x409c43); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x728),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x728u32,32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x409c69); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x409c70); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x409c98; continue; }
            pc = 0x409c90; continue;
            }
            0x409c90 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x409c98; continue;
            }
            0x409c98 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x409ca7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409cae => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xcc),8));
            c.set(2,32,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409ce5; continue; }
            pc = 0x409cc1; continue;
            }
            0x409cc1 => {
            { let a = c.get(0,8); let r = c.or(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,0x1u32);
            { let a = c.r[7].wrapping_add(0xcc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x30));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409cde); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409ce5 => {
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,0x0u32);
            { let a = c.r[7].wrapping_add(0xcc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x30));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409d02); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409d09 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xcc),8));
            c.set(1,32,c.r[6].wrapping_add(0x3));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x409d41; continue; }
            pc = 0x409d1d; continue;
            }
            0x409d1d => {
            { let a = c.get(0,8); let r = c.or(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,0x1u32);
            { let a = c.r[7].wrapping_add(0xcc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x39));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409d3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x409d41 => {
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,0x0u32);
            { let a = c.r[7].wrapping_add(0xcc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x39));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x409d5e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x409d70;
    loop {
        match pc {
            0x409d70 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x640),32));
            c.set(1,32,c.r[5].wrapping_add(0x640));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x409d88); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0xd80));
            c.set(6,32,c.r[5].wrapping_add(0x9e0));
            c.set(3,32,0x3u32);
            pc = 0x409d99; continue;
            }
            0x409d99 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfffffd48),32));
            c.set(1,32,c.r[6].wrapping_add(0xfffffd48));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x409dab); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x409db5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfffff6f0),32));
            c.set(1,32,c.r[6].wrapping_add(0xfffff6f0));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x409dc7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfffff9a8),32));
            c.set(1,32,c.r[6].wrapping_add(0xfffff9a8));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x409dd9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x288));
            c.push(m, 0x409de4);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x409deb);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x409d99; continue; }
            pc = 0x409dfa; continue;
            }
            0x409dfa => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc98),32));
            c.set(1,32,c.r[5].wrapping_add(0xc98));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x409e0c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0x1290));
            c.push(m, 0x409e17);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,32,c.get(5,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x41c3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_set_clip_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x409e30;
    loop {
        match pc {
            0x409e30 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x94),8));
            c.set(2,32,rm(m,0x6cbaecu32,32));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6cbb08u32,32));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x30),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x409e7a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bbaa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_save_to_cfg(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40a250;
    loop {
        match pc {
            0x40a250 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b39au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.set(3,8,0x1u32);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x40a351; continue; }
            pc = 0x40a298; continue;
            }
            0x40a298 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x40a2a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3f),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3e),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3c),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xcf),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xce),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xcd),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x692e18u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x40a2f3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,rm(m,0x66a2c0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,0x692e30u32);
            c.push(m,c.get(1,32));
            c.push(m,0x692e3cu32);
            c.push(m,0x692e48u32);
            { let target = c.get(7,32); c.push(m, 0x40a312); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xcc),8));
            c.push(m,c.get(2,32));
            c.push(m,0x692e54u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x40a32f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x40a351; continue; }
            pc = 0x40a336; continue;
            }
            0x40a336 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,0x692e58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x692e64u32);
            c.push(m,0x692e6cu32);
            { let target = c.get(7,32); c.push(m, 0x40a34c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x40a351; continue;
            }
            0x40a351 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x40a35f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x40a370);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,c.get(3,8));
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

pub fn letter_settings_panel_noop_slot54(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40a390;
    loop {
        match pc {
            0x40a390 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_settings_panel_noop_slot6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40a3a0;
    loop {
        match pc {
            0x40a3a0 => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d7b0;
    loop {
        match pc {
            0x40d7b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b596u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x40d7d2);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x40d7e5);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x40e330u32);
            c.push(m,0x40e2f0u32);
            c.push(m,0x32u32);
            c.set(0,32,c.r[6].wrapping_add(0x23c));
            c.push(m,0x44u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x40d804);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x257u32);
            c.push(m,0x31fu32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0xf84));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x66ae10u32); }
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x40d825); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0xfa4); wm(m,a,32,0xea60u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d850;
    loop {
        match pc {
            0x40d850 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b5b6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66ae10u32); }
            c.push(m,0x40e330u32);
            c.push(m,0x32u32);
            c.set(0,32,c.r[6].wrapping_add(0x23c));
            c.push(m,0x44u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.push(m, 0x40d890);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x40d8a0);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x40d8af);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
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

pub fn credits_window_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40d8c0;
    loop {
        match pc {
            0x40d8c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x40d8c8);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_reset_timeline(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e120;
    loop {
        match pc {
            0x40e120 => {
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(2,32,0x32u32);
            { let a = c.r[1].wrapping_add(0xf98); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[1].wrapping_add(0x27c));
            pc = 0x40e136; continue;
            }
            0x40e136 => {
            { let a = c.r[0].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.add(a,0x44u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x40e136; continue; }
            pc = 0x40e149; continue;
            }
            0x40e149 => {
            c.set(0,32,0x1u32);
            { let a = c.r[1].wrapping_add(0xf9c); wm(m,a,32,0x0u32); }
            { let a = c.r[1].wrapping_add(0xfa0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0xf95); wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e170;
    loop {
        match pc {
            0x40e170 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.push(m,c.get(3,32));
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x40e1a0; continue; }
            pc = 0x40e192; continue;
            }
            0x40e192 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40e1a0 => {
            c.set(0,8,rm(m,0x6dcbe0u32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbe0u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x40e1ca; continue; }
            pc = 0x40e1bc; continue;
            }
            0x40e1bc => {
            { let a = 0x6dcbe0u32; wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40e1ca => {
            c.set(0,8,rm(m,0x6dcbcdu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbcdu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x40e1f4; continue; }
            pc = 0x40e1e6; continue;
            }
            0x40e1e6 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40e1f4 => {
            { let a = rm(m,c.r[6].wrapping_add(0xf94),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x40e204; continue; }
            pc = 0x40e1fc; continue;
            }
            0x40e1fc => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x40e204 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfa0),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(1,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(0xfa0); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x40e2c6; continue; }
            pc = 0x40e21c; continue;
            }
            0x40e21c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf9c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xf98),32));
            { let a = c.r[6].wrapping_add(0xfa0); wm(m,a,32,0x2u32); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xf9c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0xf98); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfa4),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e281; continue; }
            pc = 0x40e259; continue;
            }
            0x40e259 => {
            { let a = c.r[6].wrapping_add(0xf9c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x27c));
            c.set(1,32,0x32u32);
            c.set(2,32,0x4u32);
            pc = 0x40e26f; continue;
            }
            0x40e26f => {
            { let a = c.r[0].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x44u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x40e26f; continue; }
            pc = 0x40e27a; continue;
            }
            0x40e27a => {
            { let a = c.r[6].wrapping_add(0xf95); wm(m,a,8,0x1u32); }
            pc = 0x40e281; continue;
            }
            0x40e281 => {
            { let a = rm(m,c.r[6].wrapping_add(0xf95),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x40e2a0; continue; }
            pc = 0x40e289; continue;
            }
            0x40e289 => {
            c.set(2,32,c.r[6].wrapping_add(0xf84));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x40e29a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xf95); wm(m,a,8,c.get(3,8)); }
            pc = 0x40e2a0; continue;
            }
            0x40e2a0 => {
            c.set(7,32,c.r[6].wrapping_add(0x23c));
            c.set(5,32,0x32u32);
            pc = 0x40e2ab; continue;
            }
            0x40e2ab => {
            { let a = rm(m,c.r[7].wrapping_add(0x38),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x40e2be; continue; }
            pc = 0x40e2b0; continue;
            }
            0x40e2b0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf9c),32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40e2be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40e3b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40e2be; continue;
            }
            0x40e2be => {
            { let a = c.get(7,32); let r = c.add(a,0x44u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x40e2ab; continue; }
            pc = 0x40e2c4; continue;
            }
            0x40e2c4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x40e2c6; continue;
            }
            0x40e2c6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_set_paused(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e2d0;
    loop {
        match pc {
            0x40e2d0 => {
            { let a = rm(m,c.r[4].wrapping_add(0x8),32); c.sub(a,0xfffffffeu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0xf94); wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e330;
    loop {
        match pc {
            0x40e330 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6]; wm(m,a,32,0x66af74u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e34c; continue; }
            pc = 0x40e340; continue;
            }
            0x40e340 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40e349);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40e34c; continue;
            }
            0x40e34c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40e37c; continue; }
            pc = 0x40e353; continue;
            }
            0x40e353 => {
            c.push(m,c.get(7,32));
            pc = 0x40e354; continue;
            }
            0x40e354 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,c.get(1,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x40e371; continue; }
            pc = 0x40e363; continue;
            }
            0x40e363 => {
            c.push(m, 0x40e368);
            { let args = []; c.r[0] = host.call(m, 0x40a3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x40e36e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40e371; continue;
            }
            0x40e371 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e354; continue; }
            pc = 0x40e37b; continue;
            }
            0x40e37b => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x40e37c; continue;
            }
            0x40e37c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40e3a1; continue; }
            pc = 0x40e383; continue;
            }
            0x40e383 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x40e394);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e383; continue; }
            pc = 0x40e3a1; continue;
            }
            0x40e3a1 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_update_fade(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e3b0;
    loop {
        match pc {
            0x40e3b0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x40e525; continue; }
            pc = 0x40e3c2; continue;
            }
            0x40e3c2 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x40e52c)); continue;
            }
            0x40e3c9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x40e3e4; continue; }
            pc = 0x40e3d4; continue;
            }
            0x40e3d4 => {
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,0xffu32); }
            pc = 0x40e3f9; continue;
            }
            0x40e3e4 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[6].wrapping_add(0x2c),32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            pc = 0x40e3f9; continue;
            }
            0x40e3f9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x40e421; continue; }
            pc = 0x40e403; continue;
            }
            0x40e403 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40e414);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e403; continue; }
            pc = 0x40e421; continue;
            }
            0x40e421 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e525; continue; }
            pc = 0x40e42e; continue;
            }
            0x40e42e => {
            c.set(5,32,rm(m,0x66a4d0u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x40e436; continue;
            }
            0x40e436 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x40e43e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40e452; continue; }
            pc = 0x40e442; continue;
            }
            0x40e442 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x40e452);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40e452; continue;
            }
            0x40e452 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40e436; continue; }
            pc = 0x40e45d; continue;
            }
            0x40e45d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x40e464 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x40e525; continue; }
            pc = 0x40e473; continue;
            }
            0x40e473 => {
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x2u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x40e481 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e49c; continue; }
            pc = 0x40e48c; continue;
            }
            0x40e48c => {
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            pc = 0x40e4ac; continue;
            }
            0x40e49c => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[6].wrapping_add(0x34),32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            pc = 0x40e4ac; continue;
            }
            0x40e4ac => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x40e4d4; continue; }
            pc = 0x40e4b6; continue;
            }
            0x40e4b6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x40e4c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e4b6; continue; }
            pc = 0x40e4d4; continue;
            }
            0x40e4d4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e525; continue; }
            pc = 0x40e4dd; continue;
            }
            0x40e4dd => {
            c.set(5,32,rm(m,0x66a4d0u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x40e4e5; continue;
            }
            0x40e4e5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x40e4ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40e501; continue; }
            pc = 0x40e4f1; continue;
            }
            0x40e4f1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40e501);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40e501; continue;
            }
            0x40e501 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40e4e5; continue; }
            pc = 0x40e50c; continue;
            }
            0x40e50c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x40e513 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e525; continue; }
            pc = 0x40e51e; continue;
            }
            0x40e51e => {
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            pc = 0x40e525; continue;
            }
            0x40e525 => {
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

pub fn credits_page_add_text_line(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e540;
    loop {
        match pc {
            0x40e540 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b60au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,0x1cu32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40e571);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40e582; continue; }
            pc = 0x40e578; continue;
            }
            0x40e578 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x40e584; continue;
            }
            0x40e582 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x40e584; continue;
            }
            0x40e584 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40e59c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[2].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40e5ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40e5fd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x2));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x40e61b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x40e639; continue; }
            pc = 0x40e626; continue;
            }
            0x40e626 => {
            c.set(0,32,c.get(7,32));
            c.push(m,0x0u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x40e648; continue;
            }
            0x40e639 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x40e64e; continue; }
            pc = 0x40e63e; continue;
            }
            0x40e63e => {
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            pc = 0x40e648; continue;
            }
            0x40e648 => {
            { let target = rm(m,0x66a4c8u32,32); c.push(m, 0x40e64e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x40e64e; continue;
            }
            0x40e64e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a4d0u32,32); c.push(m, 0x40e680); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            if c.cond(5) { pc = 0x40e69c; continue; }
            pc = 0x40e687; continue;
            }
            0x40e687 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x40e69a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x40e6b9; continue;
            }
            0x40e69c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x40e6b9; continue;
            }
            0x40e6b9 => {
            c.set(1,32,c.r[4].wrapping_add(0x50));
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x40e6ce);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(32); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_add_image(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e6f0;
    loop {
        match pc {
            0x40e6f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x24u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40e705);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x40e739);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40e753);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x40e771); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_invalidate_rects(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e7a0;
    loop {
        match pc {
            0x40e7a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x40e7cc; continue; }
            pc = 0x40e7ae; continue;
            }
            0x40e7ae => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40e7bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e7ae; continue; }
            pc = 0x40e7cc; continue;
            }
            0x40e7cc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40e808; continue; }
            pc = 0x40e7d5; continue;
            }
            0x40e7d5 => {
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a4d0u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x40e7df; continue;
            }
            0x40e7df => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x40e7e7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40e7fb; continue; }
            pc = 0x40e7eb; continue;
            }
            0x40e7eb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x40e7fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40e7fb; continue;
            }
            0x40e7fb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40e7df; continue; }
            pc = 0x40e806; continue;
            }
            0x40e806 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x40e808; continue;
            }
            0x40e808 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e850;
    loop {
        match pc {
            0x40e850 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40e937; continue; }
            pc = 0x40e85e; continue;
            }
            0x40e85e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x40),8));
            { let a = c.get(0,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x40e871);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x40e8c0; continue; }
            pc = 0x40e87e; continue;
            }
            0x40e87e => {
            c.push(m,c.get(7,32));
            pc = 0x40e87f; continue;
            }
            0x40e87f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40e88b);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x40e8af);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e87f; continue; }
            pc = 0x40e8bf; continue;
            }
            0x40e8bf => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x40e8c0; continue;
            }
            0x40e8c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x40e937; continue; }
            pc = 0x40e8ca; continue;
            }
            0x40e8ca => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x40e8e8);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x40e8ee);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40e8fa);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(0,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x40e914);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40e927);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40e8ca; continue; }
            pc = 0x40e937; continue;
            }
            0x40e937 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_window_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40ebb0;
    loop {
        match pc {
            0x40ebb0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x40ebb8);
            { let args = []; c.r[0] = host.call(m, 0x40d850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40ebc8; continue; }
            pc = 0x40ebbf; continue;
            }
            0x40ebbf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x40ebc5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40ebc8; continue;
            }
            0x40ebc8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn credits_page_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40ebd0;
    loop {
        match pc {
            0x40ebd0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x40ebd8);
            { let args = []; c.r[0] = host.call(m, 0x40e330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x40ebe8; continue; }
            pc = 0x40ebdf; continue;
            }
            0x40ebdf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x40ebe5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40ebe8; continue;
            }
            0x40ebe8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4215d0;
    loop {
        match pc {
            0x4215d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c6b6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4215f2);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x421605);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x421615);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,0x257u32);
            c.push(m,0x31fu32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0x330));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x340); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x66c580u32); }
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x421641); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x348); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421660;
    loop {
        match pc {
            0x421660 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c6f2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66c580u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x348),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4216a0; continue; }
            pc = 0x421697; continue;
            }
            0x421697 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x42169d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4216a0; continue;
            }
            0x4216a0 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x348); wm(m,a,32,0x0u32); }
            c.push(m, 0x4216b1);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x164));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4216be);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            c.set(7,32,c.r[6].wrapping_add(0x23c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4216d2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x340));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x2u32); }
            c.push(m, 0x4216e2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x4216ee);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x4216fa);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x421709);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_release_ranking(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421720;
    loop {
        match pc {
            0x421720 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x348),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x421736; continue; }
            pc = 0x42172d; continue;
            }
            0x42172d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x421733);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x421736; continue;
            }
            0x421736 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x348); wm(m,a,32,0x0u32); }
            c.push(m, 0x421747);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            c.push(m, 0x421752);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let v = c.pop(m); c.set(6,32,v); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_show_result(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421f30;
    loop {
        match pc {
            0x421f30 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x347); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x345); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x34c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x421f5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4220d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1df),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x421f79; continue; }
            pc = 0x421f6e; continue;
            }
            0x421f6e => {
            c.push(m, 0x421f73);
            { let args = []; c.r[0] = host.call(m, 0x4425d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            pc = 0x421f79; continue;
            }
            0x421f79 => {
            c.push(m, 0x421f7e);
            { let args = []; c.r[0] = host.call(m, 0x442d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            { let a = c.get(6,32); let r = c.add(a,0x340u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(6,32));
            c.push(m, 0x421f99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x421fa4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4426c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421fb0;
    loop {
        match pc {
            0x421fb0 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.push(m,c.get(3,32));
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x421fdf; continue; }
            pc = 0x421fd2; continue;
            }
            0x421fd2 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,0x1u32); }
            pc = 0x421fdf; continue;
            }
            0x421fdf => {
            c.set(0,8,rm(m,0x6dcbe0u32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbe0u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x422008; continue; }
            pc = 0x421ffb; continue;
            }
            0x421ffb => {
            { let a = 0x6dcbe0u32; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,0x1u32); }
            pc = 0x422008; continue;
            }
            0x422008 => {
            c.set(0,8,rm(m,0x6dcbcdu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbcdu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x422031; continue; }
            pc = 0x422024; continue;
            }
            0x422024 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,0x1u32); }
            pc = 0x422031; continue;
            }
            0x422031 => {
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            c.push(m, 0x42203c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x422047; continue; }
            pc = 0x422040; continue;
            }
            0x422040 => {
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,0x1u32); }
            pc = 0x422047; continue;
            }
            0x422047 => {
            { let a = rm(m,c.r[6].wrapping_add(0x344),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x422068; continue; }
            pc = 0x42204f; continue;
            }
            0x42204f => {
            { let a = c.r[6].wrapping_add(0x344); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x422060);
            { let args = []; c.r[0] = host.call(m, 0x442d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x422068 => {
            { let a = rm(m,c.r[6].wrapping_add(0x345),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x422087; continue; }
            pc = 0x422070; continue;
            }
            0x422070 => {
            c.set(0,32,c.r[6].wrapping_add(0x330));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x422081);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x345); wm(m,a,8,c.get(3,8)); }
            pc = 0x422087; continue;
            }
            0x422087 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_on_child_result(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x422090;
    loop {
        match pc {
            0x422090 => {
            { let a = rm(m,c.r[4].wrapping_add(0x8),32); c.sub(a,0xfffffffeu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x344); wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_get_top_ranked_merchant(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4220b0;
    loop {
        match pc {
            0x4220b0 => {
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4220ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4220d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x348),32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,16,rm(m,c.r[0],8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_build_ranking(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4220d0;
    loop {
        match pc {
            0x4220d0 => {
            c.set(0,8,rm(m,0x6de4aau32,8));
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x14u32,8); c.set(0,8,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x346); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4220fb; continue; }
            pc = 0x4220f2; continue;
            }
            0x4220f2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4220f8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4220fb; continue;
            }
            0x4220fb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x348); wm(m,a,32,c.get(6,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x346),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x422115);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.r[5].wrapping_add(0x348); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            if c.cond(6) { pc = 0x4223a0; continue; }
            pc = 0x422138; continue;
            }
            0x422138 => {
            c.set(2,32,c.r[3].wrapping_add(0x14));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x422146);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.get(3,8); let r = c.add(a,0x14u32,8); c.set(3,8,r); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.r[6].wrapping_add(c.r[1]); wm(m,a,8,c.get(3,8)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x42217c; continue; }
            pc = 0x422169; continue;
            }
            0x422169 => {
            { let a = c.get(0,32); let r = c.and(a,0x8000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8000u32,32); }
            if c.cond(4) { pc = 0x42217c; continue; }
            pc = 0x422175; continue;
            }
            0x422175 => {
            c.set(0,32,0x1u32);
            pc = 0x42217e; continue;
            }
            0x42217c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42217e; continue;
            }
            0x42217e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(c.r[2]).wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x46c),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x478),16));
            { let a = c.r[6].wrapping_add(c.r[2]).wrapping_add(0x10); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x422221; continue; }
            pc = 0x4221c8; continue;
            }
            0x4221c8 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4221d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2d2),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2d4),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[1].wrapping_add(c.r[2]));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x448),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6de4a8u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4221c8; continue; }
            pc = 0x422221; continue;
            }
            0x422221 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x12); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(c.r[2]).wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42224e);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,0x7u32,32); }
            if c.cond(14) { pc = 0x42225d; continue; }
            pc = 0x42225a; continue;
            }
            0x42225a => {
            { let a = c.get(3,32); let r = c.sub(a,0x7u32,32); c.set(3,32,r); }
            pc = 0x42225d; continue;
            }
            0x42225d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(1,32,c.get(7,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x422276);
            { let args = []; c.r[0] = host.call(m, 0x4f73d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.r[6].wrapping_add(c.r[2]).wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x34c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x422299; continue; }
            pc = 0x42228d; continue;
            }
            0x42228d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x8),32));
            pc = 0x42229b; continue;
            }
            0x422299 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x42229b; continue;
            }
            0x42229b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xc),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x422307; continue; }
            pc = 0x4222c7; continue;
            }
            0x4222c7 => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,0xae147ae1u32);
            c.set(7,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x4));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4222f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x422301);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x422349; continue;
            }
            0x422307 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(7,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x42233d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x422345);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x422349; continue;
            }
            0x422349 => {
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x422363; continue; }
            pc = 0x42235d; continue;
            }
            0x42235d => {
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            pc = 0x422363; continue;
            }
            0x422363 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x1c),8));
            c.set(0,32,c.r[6].wrapping_add(c.r[2]));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x42237a; continue; }
            pc = 0x422374; continue;
            }
            0x422374 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,r); } }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            pc = 0x42237a; continue;
            }
            0x42237a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x24u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x422138; continue; }
            pc = 0x42239c; continue;
            }
            0x42239c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x4223a0; continue;
            }
            0x4223a0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4224bb; continue; }
            pc = 0x4223ae; continue;
            }
            0x4223ae => {
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x42240d; continue; }
            pc = 0x4223b3; continue;
            }
            0x4223b3 => {
            c.set(1,32,0x8u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4223be; continue;
            }
            0x4223be => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            c.set(3,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4223f4; continue; }
            pc = 0x4223d2; continue;
            }
            0x4223d2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x348),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            pc = 0x4223db; continue;
            }
            0x4223db => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x19),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4223ec; continue; }
            pc = 0x4223e2; continue;
            }
            0x4223e2 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4223ec; continue; }
            pc = 0x4223e8; continue;
            }
            0x4223e8 => {
            c.set(6,32,c.get(2,32));
            c.set(7,32,c.get(0,32));
            pc = 0x4223ec; continue;
            }
            0x4223ec => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x24u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4223db; continue; }
            pc = 0x4223f4; continue;
            }
            0x4223f4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1d); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4223be; continue; }
            pc = 0x42240d; continue;
            }
            0x42240d => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            pc = 0x42241b; continue;
            }
            0x422417 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x42241b; continue;
            }
            0x42241b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x422461; continue; }
            pc = 0x422434; continue;
            }
            0x422434 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(2,32,c.r[3].wrapping_add(c.r[2]).wrapping_add(0x4));
            pc = 0x42243e; continue;
            }
            0x42243e => {
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x19),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x422455; continue; }
            pc = 0x422445; continue;
            }
            0x422445 => {
            c.set(1,32,rm(m,c.r[2],32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x422455; continue; }
            pc = 0x42244f; continue;
            }
            0x42244f => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.get(0,32));
            pc = 0x422455; continue;
            }
            0x422455 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x24u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x42243e; continue; }
            pc = 0x422461; continue;
            }
            0x422461 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(8)));
            c.set(7,32,c.r[4].wrapping_add(0x24));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,0x9u32);
            c.set(6,32,c.get(2,32));
            c.rep_movsd(m);
            c.set(6,32,c.r[3].wrapping_add(c.r[0]));
            c.set(1,32,0x9u32);
            c.set(7,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.rep_movsd(m);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,0x9u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x24u32,32); c.set(3,32,r); }
            c.set(6,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,0x120u32,32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.rep_movsd(m);
            if c.cond(12) { pc = 0x422417; continue; }
            pc = 0x4224b1; continue;
            }
            0x4224b1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4224bb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x422566; continue; }
            pc = 0x4224cb; continue;
            }
            0x4224cb => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            pc = 0x4224cf; continue;
            }
            0x4224cf => {
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.get(3,32));
            if c.cond(13) { pc = 0x42250e; continue; }
            pc = 0x4224dc; continue;
            }
            0x4224dc => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(6,32,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x4));
            c.set(7,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            pc = 0x4224f0; continue;
            }
            0x4224f0 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4224fe; continue; }
            pc = 0x4224fa; continue;
            }
            0x4224fa => {
            c.set(7,32,c.get(6,32));
            c.set(2,32,c.get(1,32));
            pc = 0x4224fe; continue;
            }
            0x4224fe => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x24u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            if c.cond(12) { pc = 0x4224f0; continue; }
            pc = 0x42250e; continue;
            }
            0x42250e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,0x9u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.get(2,32));
            c.rep_movsd(m);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(7,32,c.get(2,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[1]));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,0x9u32);
            c.rep_movsd(m);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x348),32));
            c.set(1,32,0x9u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x24u32,32); c.set(0,32,r); }
            c.set(6,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.rep_movsd(m);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x346),8));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4224cf; continue; }
            pc = 0x422566; continue;
            }
            0x422566 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_end_screen_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x422590;
    loop {
        match pc {
            0x422590 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x422598);
            { let args = []; c.r[0] = host.call(m, 0x421660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4225a8; continue; }
            pc = 0x42259f; continue;
            }
            0x42259f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4225a5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4225a8; continue;
            }
            0x4225a8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42eab0;
    loop {
        match pc {
            0x42eab0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d396u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x42ead2);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x42eae5);
            { let args = []; c.r[0] = host.call(m, 0x4028a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x19c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x42eaf5);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x288); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66ccf8u32); }
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x2b5); wm(m,a,8,0x0u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42eb30;
    loop {
        match pc {
            0x42eb30 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d3d2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66ccf8u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x3u32); }
            c.push(m, 0x42eb60);
            { let args = []; c.r[0] = host.call(m, 0x42ebb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x288));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x42eb70);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x19c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x42eb80);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x42eb90);
            { let args = []; c.r[0] = host.call(m, 0x402970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42eb9f);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
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

pub fn main_menu_destroy_screens(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42ebb0;
    loop {
        match pc {
            0x42ebb0 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x42ebb6);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ebc7; continue; }
            pc = 0x42ebc2; continue;
            }
            0x42ebc2 => {
            c.push(m, 0x42ebc7);
            { let args = []; c.r[0] = host.call(m, 0x40d8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ebc7; continue;
            }
            0x42ebc7 => {
            c.set(1,32,rm(m,0x6d9594u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ebd6; continue; }
            pc = 0x42ebd1; continue;
            }
            0x42ebd1 => {
            c.push(m, 0x42ebd6);
            { let args = []; c.r[0] = host.call(m, 0x4ab360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ebd6; continue;
            }
            0x42ebd6 => {
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ebe5; continue; }
            pc = 0x42ebe0; continue;
            }
            0x42ebe0 => {
            c.push(m, 0x42ebe5);
            { let args = []; c.r[0] = host.call(m, 0x42fdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ebe5; continue;
            }
            0x42ebe5 => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ebf5; continue; }
            pc = 0x42ebef; continue;
            }
            0x42ebef => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x42ebf5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ebf5; continue;
            }
            0x42ebf5 => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = 0x6d1c7cu32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec0b; continue; }
            pc = 0x42ec05; continue;
            }
            0x42ec05 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x42ec0b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec0b; continue;
            }
            0x42ec0b => {
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            { let a = 0x6cbb94u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec21; continue; }
            pc = 0x42ec1b; continue;
            }
            0x42ec1b => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x42ec21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec21; continue;
            }
            0x42ec21 => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = 0x6cbdf0u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec37; continue; }
            pc = 0x42ec31; continue;
            }
            0x42ec31 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x42ec37); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec37; continue;
            }
            0x42ec37 => {
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            { let a = 0x6cbb7cu32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec4d; continue; }
            pc = 0x42ec47; continue;
            }
            0x42ec47 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x42ec4d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec4d; continue;
            }
            0x42ec4d => {
            c.set(1,32,rm(m,0x6d9594u32,32));
            { let a = 0x6cbaa4u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec63; continue; }
            pc = 0x42ec5d; continue;
            }
            0x42ec5d => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x42ec63); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec63; continue;
            }
            0x42ec63 => {
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            { let a = 0x6d9594u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec79; continue; }
            pc = 0x42ec73; continue;
            }
            0x42ec73 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x42ec79); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec79; continue;
            }
            0x42ec79 => {
            c.set(1,32,rm(m,0x6cbb30u32,32));
            { let a = 0x6cbb6cu32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42ec8f; continue; }
            pc = 0x42ec89; continue;
            }
            0x42ec89 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x42ec8f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42ec8f; continue;
            }
            0x42ec8f => {
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = 0x6cbb30u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x42eca5; continue; }
            pc = 0x42ec9f; continue;
            }
            0x42ec9f => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x42eca5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42eca5; continue;
            }
            0x42eca5 => {
            { let a = 0x6d95a0u32; wm(m,a,32,c.get(6,32)); }
            { let a = 0x6cc37cu32; wm(m,a,32,c.get(6,32)); }
            { let a = 0x6cc384u32; wm(m,a,32,c.get(6,32)); }
            { let a = 0x6cc380u32; wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42f1f0;
    loop {
        match pc {
            0x42f1f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x42f1fe); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[6].wrapping_add(0x288));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x2b4); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,0xffu32); }
            c.push(m, 0x42f231);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42f23c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4426c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x278),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x42f24b);
            { let args = []; c.r[0] = host.call(m, 0x4bbaf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x274),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x42f261);
            { let args = []; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x27c); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_process_selection(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42f4f0;
    loop {
        match pc {
            0x42f4f0 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x42f519; continue; }
            pc = 0x42f512; continue;
            }
            0x42f512 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            pc = 0x42f519; continue;
            }
            0x42f519 => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x294));
            pc = 0x42f527; continue;
            }
            0x42f527 => {
            { let a = rm(m,c.r[0],32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x42f536; continue; }
            pc = 0x42f52b; continue;
            }
            0x42f52b => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x42f527; continue; }
            pc = 0x42f534; continue;
            }
            0x42f534 => {
            pc = 0x42f564; continue;
            }
            0x42f536 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0xbau32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1abu32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f564);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42f564; continue;
            }
            0x42f564 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b4),8));
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x42f57d; continue; }
            pc = 0x42f571; continue;
            }
            0x42f571 => {
            { let a = rm(m,c.r[6].wrapping_add(0x284),32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x42f745; continue; }
            pc = 0x42f57d; continue;
            }
            0x42f57d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x284),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x42f593; continue; }
            pc = 0x42f587; continue;
            }
            0x42f587 => {
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(1,32)); }
            pc = 0x42f593; continue;
            }
            0x42f593 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x42f734; continue; }
            pc = 0x42f5a2; continue;
            }
            0x42f5a2 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x42f960)); continue;
            }
            0x42f5a9 => {
            c.set(2,32,rm(m,0x66e0b8u32,32));
            c.set(0,32,rm(m,0x6d9594u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f5c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f5cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f5d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x42f5e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4ab420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f5f2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f600); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x42f61b; continue; }
            pc = 0x42f605; continue;
            }
            0x42f605 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(0,32)); }
            pc = 0x42f611; continue;
            }
            0x42f611 => {
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x42f61b => {
            c.set(1,32,rm(m,0x66e094u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f62d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6d9594u32,32));
            c.push(m,c.get(2,32));
            pc = 0x42f72a; continue;
            }
            0x42f639 => {
            c.set(0,32,rm(m,0x66e0bcu32,32));
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f650);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f65d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f668);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.push(m, 0x42f673);
            { let args = []; c.r[0] = host.call(m, 0x42fea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f680); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f68e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x42f605; continue; }
            pc = 0x42f697; continue;
            }
            0x42f697 => {
            c.set(1,32,rm(m,0x66e094u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f6a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6cbb6cu32,32));
            c.push(m,c.get(2,32));
            pc = 0x42f72a; continue;
            }
            0x42f6b2 => {
            c.set(0,32,rm(m,0x66e09cu32,32));
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f6c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f6d6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f6e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x3u32);
            c.push(m, 0x42f6ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x497ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f6fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f709); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x42f611; continue; }
            pc = 0x42f712; continue;
            }
            0x42f712 => {
            c.set(0,32,rm(m,0x66e094u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x42f723);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            pc = 0x42f729; continue;
            }
            0x42f729 => {
            c.push(m,c.get(1,32));
            pc = 0x42f72a; continue;
            }
            0x42f72a => {
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f734);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42f734; continue;
            }
            0x42f734 => {
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,0xffu32); }
            { let a = c.r[6].wrapping_add(0x2b4); wm(m,a,8,0x0u32); }
            pc = 0x42f745; continue;
            }
            0x42f745 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x280),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0xfffffffd));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x42f75b => {
            c.set(2,32,rm(m,0x66e0a4u32,32));
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f772);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f77f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f78a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,0x0u32);
            c.push(m,0x8u32);
            c.push(m,0x0u32);
            c.push(m, 0x42f79b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4381e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f7a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f7b6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x42f611; continue; }
            pc = 0x42f7bf; continue;
            }
            0x42f7bf => {
            c.set(0,32,rm(m,0x66e094u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x42f7d0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            pc = 0x42f729; continue;
            }
            0x42f7db => {
            c.set(2,32,rm(m,0x66e0a8u32,32));
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f7f2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f7ff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f80a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x42f81b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x431cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f828); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f836); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x42f855; continue; }
            pc = 0x42f83b; continue;
            }
            0x42f83b => {
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42f84b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x42f855 => {
            c.set(1,32,rm(m,0x66e094u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f867);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(2,32));
            pc = 0x42f72a; continue;
            }
            0x42f873 => {
            c.set(0,32,rm(m,0x66e098u32,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f88a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f897); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f8a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x42f8ad);
            { let args = []; c.r[0] = host.call(m, 0x45e3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f8ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f8c8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x66e094u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x42f8d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            pc = 0x42f729; continue;
            }
            0x42f8e4 => {
            c.set(2,32,rm(m,0x66e0b4u32,32));
            c.set(0,32,rm(m,0x6cbaa4u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f8fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42f908); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42f913);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.push(m, 0x42f91e);
            { let args = []; c.r[0] = host.call(m, 0x40e120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42f92b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x42f939); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x66e094u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x42f94a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            pc = 0x42f729; continue;
            }
            0x42f955 => {
            c.set(0,32,0xfffffffbu32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_release_texture_surface(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42f990;
    loop {
        match pc {
            0x42f990 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x274),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x42f99f);
            { let args = []; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42f9b0;
    loop {
        match pc {
            0x42f9b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,0x1abu32);
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[0],32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,0xbau32,32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x42facc; continue; }
            pc = 0x42f9e4; continue;
            }
            0x42f9e4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x42fa02; continue; }
            pc = 0x42f9ef; continue;
            }
            0x42f9ef => {
            c.set(3,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(3,32); let r = c.add(a,0x1f4u32,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x294); wm(m,a,32,c.get(3,32)); }
            pc = 0x42fa02; continue;
            }
            0x42fa02 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x27c),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[0].wrapping_add(0x4),32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xbau32); }
            c.push(m, 0x42fa46);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2b5),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x42fa9a; continue; }
            pc = 0x42fa6f; continue;
            }
            0x42fa6f => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x40u32);
            c.push(m,0xffffffffu32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1f40u32);
            c.push(m, 0x42fa87);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.r[6].wrapping_add(0x2b5); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x290); wm(m,a,32,c.get(0,32)); }
            pc = 0x42fa9a; continue;
            }
            0x42fa9a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x290),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,0xfffffffeu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x42fb1c; continue; }
            pc = 0x42fabd; continue;
            }
            0x42fabd => {
            { let a = c.r[6].wrapping_add(0x2b4); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x2b5); wm(m,a,8,c.get(3,8)); }
            pc = 0x42fb1c; continue;
            }
            0x42facc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x42fb1c; continue; }
            pc = 0x42fad9; continue;
            }
            0x42fad9 => {
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x294); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,0xffu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xbau32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x42fb1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42fb1c; continue;
            }
            0x42fb1c => {
            { let a = c.get(5,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x42fb34; continue; }
            pc = 0x42fb21; continue;
            }
            0x42fb21 => {
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,0xffu32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x42fb34 => {
            { let a = c.get(5,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x42fb3f; continue; }
            pc = 0x42fb39; continue;
            }
            0x42fb39 => {
            { let a = c.r[6].wrapping_add(0x2b5); wm(m,a,8,c.get(3,8)); }
            pc = 0x42fb3f; continue;
            }
            0x42fb3f => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42fb70;
    loop {
        match pc {
            0x42fb70 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x42fb78);
            { let args = []; c.r[0] = host.call(m, 0x42eb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x42fb88; continue; }
            pc = 0x42fb7f; continue;
            }
            0x42fb7f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x42fb85);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x42fb88; continue;
            }
            0x42fb88 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_select_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42fbf0;
    loop {
        match pc {
            0x42fbf0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d4ccu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x42fc12);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x42fc25);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x42fc35);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5f0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x42fc45);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            c.push(m, 0x42fc55);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x4u32); }
            c.push(m, 0x42fc65);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb8c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x42fc75);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xc80));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x42fc85);
            { let args = []; c.r[0] = host.call(m, 0x4209e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0xcd0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66ce60u32); }
            { let a = c.r[6].wrapping_add(0xcd8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xccc); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xca8); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_select_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42fcd0;
    loop {
        match pc {
            0x42fcd0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d54eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66ce60u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd8),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x8u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x42fd2e; continue; }
            pc = 0x42fd05; continue;
            }
            0x42fd05 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x430f10u32);
            c.push(m,c.get(1,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42fd1a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x42fd20);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xcd8); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x42fd2e; continue;
            }
            0x42fd2e => {
            c.set(1,32,c.r[6].wrapping_add(0xcd0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x7u32); }
            c.push(m, 0x42fd3e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xc80));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x42fd4e);
            { let args = []; c.r[0] = host.call(m, 0x420a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb8c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x42fd5e);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x4u32); }
            c.push(m, 0x42fd6e);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            c.push(m, 0x42fd7e);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5f0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x42fd8e);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x42fd9e);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x42fdae);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42fdbd);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
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

pub fn campaign_select_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42fdd0;
    loop {
        match pc {
            0x42fdd0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x42fdd8);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            c.push(m, 0x42fde3);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x42fdf5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa98),32));
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x42fe07); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xb8c));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x42fe19); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x42fe4c; continue; }
            pc = 0x42fe23; continue;
            }
            0x42fe23 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x430f10u32);
            c.push(m,c.get(1,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42fe38);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x42fe3e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xcd8); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x42fe4c; continue;
            }
            0x42fe4c => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_select_screen_setup_lists(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42fea0;
    loop {
        match pc {
            0x42fea0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,0x164u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x42feb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x240u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x42fecd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x5f0));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x42feda);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcac),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcf0),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xcbc),32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcec),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x5f8));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42ff13);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcf4),32));
            c.set(5,32,rm(m,0x66a4a4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = c.get(5,32); c.push(m, 0x42ff33); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcac),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xca4),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42ff51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xca4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcbc),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x42ff6d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xca4),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x42ff7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            c.push(m, 0x42ff86);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcc0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd00),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xcb0),32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcfc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x244));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x42ffbf);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd04),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x42ffd9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcb0),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42ffef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42fffb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x240); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xcc8); wm(m,a,32,0xffffffffu32); }
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

pub fn campaign_select_screen_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x430a00;
    loop {
        match pc {
            0x430a00 => {
            { let a = rm(m,0x6cbb68u32,8); c.and(a,0x1u32,8); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            if c.cond(5) { pc = 0x430a22; continue; }
            pc = 0x430a0e; continue;
            }
            0x430a0e => {
            c.set(2,8,rm(m,0x6cbb68u32,8));
            { let a = 0x6cbb64u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,8); let r = c.or(a,0x1u32,8); c.set(2,8,r); }
            { let a = 0x6cbb68u32; wm(m,a,8,c.get(2,8)); }
            pc = 0x430a22; continue;
            }
            0x430a22 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x430a54; continue; }
            pc = 0x430a2b; continue;
            }
            0x430a2b => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6cbb64u32,32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(6,32); c.sub(a,0x1f4u32,32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(3) { pc = 0x430a4d; continue; }
            pc = 0x430a41; continue;
            }
            0x430a41 => {
            { let a = c.r[1].wrapping_add(0xccc); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            pc = 0x430a4d; continue;
            }
            0x430a4d => {
            { let a = 0x6cbb64u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x430a5b; continue;
            }
            0x430a54 => {
            { let a = c.r[1].wrapping_add(0xccc); wm(m,a,8,0x0u32); }
            pc = 0x430a5b; continue;
            }
            0x430a5b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0xc9c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1].wrapping_add(0xcdc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0xce0); wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_select_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x430ef0;
    loop {
        match pc {
            0x430ef0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x430ef8);
            { let args = []; c.r[0] = host.call(m, 0x42fcd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x430f08; continue; }
            pc = 0x430eff; continue;
            }
            0x430eff => {
            c.push(m,c.get(6,32));
            c.push(m, 0x430f05);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x430f08; continue;
            }
            0x430f08 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_entry_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x430f10;
    loop {
        match pc {
            0x430f10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d5b8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x430f3d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x430f4c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
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

pub fn load_save_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4313b0;
    loop {
        match pc {
            0x4313b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d6bbu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4313d3);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4313e4);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x240));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x4313f4);
            { let args = []; c.r[0] = host.call(m, 0x4d1fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x340));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.push(m, 0x431404);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x360));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            c.push(m, 0x431414);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x438));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            c.push(m, 0x431424);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x464b40u32);
            c.push(m,0x464ad0u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x510));
            c.push(m,0xf4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x5u32); }
            c.push(m, 0x431446);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x431456);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7u32); }
            c.push(m, 0x431466);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x8u32); }
            c.push(m, 0x431476);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xac8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x9u32); }
            c.push(m, 0x431486);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xba0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xau32); }
            c.push(m, 0x431496);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xd20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xbu32); }
            c.push(m, 0x4314a6);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x10d8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xcu32); }
            c.push(m, 0x4314b6);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4b0440u32);
            c.push(m,0x4b03e0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x10f8));
            c.push(m,0x20u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0xdu32); }
            c.push(m, 0x4314d5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[6]; wm(m,a,32,0x66d020u32); }
            { let a = c.r[6].wrapping_add(0x11cc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x11b0); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x11b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x11cd); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(6,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
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

pub fn load_save_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x431520;
    loop {
        match pc {
            0x431520 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d7a8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66d020u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xeu32); }
            { let a = c.r[6].wrapping_add(0x11cc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x431582; continue; }
            pc = 0x431565; continue;
            }
            0x431565 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x431579);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43157f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x431582; continue;
            }
            0x431582 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4315af; continue; }
            pc = 0x431592; continue;
            }
            0x431592 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4315a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4315ac);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4315af; continue;
            }
            0x4315af => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4315bc);
            { let args = []; c.r[0] = host.call(m, 0x4316e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4b0440u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x10f8));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0xdu32); }
            c.push(m, 0x4315d6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x10d8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0xcu32); }
            c.push(m, 0x4315e6);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xd20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0xbu32); }
            c.push(m, 0x4315f6);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xba0));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0xau32); }
            c.push(m, 0x431606);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xac8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x9u32); }
            c.push(m, 0x431616);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x8u32); }
            c.push(m, 0x431626);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x7u32); }
            c.push(m, 0x431636);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x6u32); }
            c.push(m, 0x431646);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x464b40u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x510));
            c.push(m,0xf4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x5u32); }
            c.push(m, 0x431663);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x438));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x4u32); }
            c.push(m, 0x431673);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x360));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x3u32); }
            c.push(m, 0x431683);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x340));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x2u32); }
            c.push(m, 0x431693);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x240));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x4316a3);
            { let args = []; c.r[0] = host.call(m, 0x4d20a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4316b2);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4316c1);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn load_save_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4316e0;
    loop {
        match pc {
            0x4316e0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4316ea);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x240));
            c.push(m, 0x4316f5);
            { let args = []; c.r[0] = host.call(m, 0x4d2100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x340));
            c.push(m, 0x431700);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x360));
            c.push(m, 0x43170b);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x438));
            c.push(m, 0x431716);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x510));
            c.set(3,32,0x3u32);
            pc = 0x431721; continue;
            }
            0x431721 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43172b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x431721; continue; }
            pc = 0x431734; continue;
            }
            0x431734 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7ec),32));
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x431746); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x431758); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x9d4));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43176a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xac8));
            c.push(m, 0x431775);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xba0));
            c.push(m, 0x431780);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xd20));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x431cc0;
    loop {
        match pc {
            0x431cc0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[6].wrapping_add(0x164));
            c.push(m,c.get(0,32));
            c.push(m, 0x431cd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x240));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x431ce0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(5,32,c.r[6].wrapping_add(0x510));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x11c0); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x431cfd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x6f8));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x431d10); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x431d2c; continue; }
            pc = 0x431d16; continue;
            }
            0x431d16 => {
            { let a = c.r[6].wrapping_add(0x11bc); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x11c8); wm(m,a,32,0xffffffffu32); }
            pc = 0x431d3c; continue;
            }
            0x431d2c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0x11bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x11c8); wm(m,a,32,c.get(1,32)); }
            pc = 0x431d3c; continue;
            }
            0x431d3c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x11b8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            if c.cond(5) { pc = 0x431d96; continue; }
            pc = 0x431d53; continue;
            }
            0x431d53 => {
            c.push(m, 0x431d58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x181u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x431d68); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x431d74); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x5au32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let target = rm(m,c.r[0].wrapping_add(0x78),32); c.push(m, 0x431d8c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x431d94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x431e04; continue;
            }
            0x431d96 => {
            c.push(m, 0x431d9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1d1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x431dab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x431db7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x5au32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let target = rm(m,c.r[0].wrapping_add(0x78),32); c.push(m, 0x431dcf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x431dd7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x431e04; continue; }
            pc = 0x431ddd; continue;
            }
            0x431ddd => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x431dfe; continue; }
            pc = 0x431ded; continue;
            }
            0x431ded => {
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x431e04; continue; }
            pc = 0x431df2; continue;
            }
            0x431df2 => {
            { let a = c.r[6].wrapping_add(0x11c0); wm(m,a,32,0x2u32); }
            pc = 0x431e04; continue;
            }
            0x431dfe => {
            { let a = c.r[6].wrapping_add(0x11c0); wm(m,a,32,c.get(3,32)); }
            pc = 0x431e04; continue;
            }
            0x431e04 => {
            c.set(0,8,0x1u32);
            c.set(1,32,0x514u32);
            { let a = c.r[6].wrapping_add(0x116a); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x116c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x1160); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,0xau32);
            { let a = c.r[6].wrapping_add(0x117c); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x1164); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1168); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1169); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x116b); wm(m,a,8,0xcu32); }
            { let a = c.r[6].wrapping_add(0x116d); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x116e); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x116f); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1170); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1171); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1172); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1173); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1174); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1175); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1176); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1177); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1178); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1179); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x117a); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x117b); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x117e); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x117f); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1180); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1181); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1182); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1183); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1184); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1185); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1186); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x431ed6);
            { let args = []; c.r[0] = host.call(m, 0x431ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_prepare_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x431ee0;
    loop {
        match pc {
            0x431ee0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x11cc); wm(m,a,8,0x0u32); }
            c.push(m, 0x431ef4);
            { let args = []; c.r[0] = host.call(m, 0x4343e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11e0),32));
            c.set(7,32,c.r[6].wrapping_add(0xd28));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x431f0b);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11e8),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(1,32,c.r[6].wrapping_add(0xd20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x431f24);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x431fdf; continue; }
            pc = 0x431f32; continue;
            }
            0x431f32 => {
            c.set(0,32,rm(m,0x69b138u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            c.push(m,c.get(0,32));
            c.push(m, 0x431f43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xba0),32));
            c.set(1,32,c.r[6].wrapping_add(0xba0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x431f57); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11e4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11ec),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x431f7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x431f99);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11d8),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x431fb5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x510));
            pc = 0x431fbd; continue;
            }
            0x431fbd => {
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x431fce; continue; }
            pc = 0x431fc2; continue;
            }
            0x431fc2 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x431fce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x431fce; continue;
            }
            0x431fce => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x431fbd; continue; }
            pc = 0x431fda; continue;
            }
            0x431fda => {
            pc = 0x4320ac; continue;
            }
            0x431fdf => {
            c.set(1,32,rm(m,0x69b13cu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            c.push(m, 0x431ff2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xba0),32));
            c.set(5,32,c.r[6].wrapping_add(0xba0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x432008); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11f4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11fc),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x43202c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43204a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11dc),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x432066); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x510));
            pc = 0x43206e; continue;
            }
            0x43206e => {
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x43207f; continue; }
            pc = 0x432073; continue;
            }
            0x432073 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43207f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43207f; continue;
            }
            0x43207f => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x43206e; continue; }
            pc = 0x43208b; continue;
            }
            0x43208b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4320a3; continue; }
            pc = 0x432095; continue;
            }
            0x432095 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4320a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4320a3; continue;
            }
            0x4320a3 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4320ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4320ac; continue;
            }
            0x4320ac => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            c.set(1,32,c.r[6].wrapping_add(0xd20));
            c.push(m,c.get(2,32));
            c.push(m, 0x4320be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x11bc),32); c.sub(a,0x3u32,32); }
            if c.cond(4) { pc = 0x4320ec; continue; }
            pc = 0x4320c7; continue;
            }
            0x4320c7 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x510));
            pc = 0x4320cf; continue;
            }
            0x4320cf => {
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x4320e0; continue; }
            pc = 0x4320d4; continue;
            }
            0x4320d4 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4320e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4320e0; continue;
            }
            0x4320e0 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4320cf; continue; }
            pc = 0x4320ec; continue;
            }
            0x4320ec => {
            { let a = c.r[6].wrapping_add(0x11c4); wm(m,a,32,0xffffffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_on_key_down(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x432f50;
    loop {
        match pc {
            0x432f50 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d82eu32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x432f80);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b6b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xba0),32));
            c.set(7,32,c.r[6].wrapping_add(0xba0));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x432f94); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x433010; continue; }
            pc = 0x432f98; continue;
            }
            0x432f98 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11cf),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x433010; continue; }
            pc = 0x432fa2; continue;
            }
            0x432fa2 => {
            { let a = c.get(3,32); c.sub(a,0xdu32,32); }
            if c.cond(5) { pc = 0x433010; continue; }
            pc = 0x432fa7; continue;
            }
            0x432fa7 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x432fae);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x432fb8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x432fe0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x432fef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x11cc); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x432fff; continue; }
            pc = 0x432ff9; continue;
            }
            0x432ff9 => {
            { let a = rm(m,c.r[6].wrapping_add(0x11ac),32); let r = c.dec(a,32); { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,r); } }
            pc = 0x432fff; continue;
            }
            0x432fff => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x433010);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x433010; continue;
            }
            0x433010 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433030;
    loop {
        match pc {
            0x433030 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.sub(a,0x74u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x433048);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b6b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,0x6cbb78u32,8));
            c.set(0,8,0x1u32);
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x433068; continue; }
            pc = 0x433054; continue;
            }
            0x433054 => {
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = 0x6cbb78u32; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = 0x6cbb74u32; wm(m,a,32,c.get(1,32)); }
            pc = 0x433068; continue;
            }
            0x433068 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11e0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11e8),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b8),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x4330b1; continue; }
            pc = 0x433094; continue;
            }
            0x433094 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11d8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11e4),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x4330ce; continue; }
            pc = 0x4330a8; continue;
            }
            0x4330a8 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11ec),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x4330db; continue;
            }
            0x4330b1 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11dc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11f4),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x4330ce; continue; }
            pc = 0x4330c5; continue;
            }
            0x4330c5 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11fc),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x4330db; continue;
            }
            0x4330ce => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[2]).wrapping_add(0xffffffff));
            pc = 0x4330db; continue;
            }
            0x4330db => {
            c.set(5,32,rm(m,c.r[3],32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            c.set(2,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(13)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x433350; continue; }
            pc = 0x433116; continue;
            }
            0x433116 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10c4),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x433195; continue; }
            pc = 0x433137; continue;
            }
            0x433137 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43315b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x11b0); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x433195);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x433195; continue;
            }
            0x433195 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(5,32,rm(m,0x6dccf0u32,32));
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,0xfffffffeu32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4334f8; continue; }
            pc = 0x4331c0; continue;
            }
            0x4331c0 => {
            { let a = c.get(5,32); let r = c.sub(a,rm(m,0x6cbb74u32,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1f4u32,32); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x11b4),32));
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x43322b; continue; }
            pc = 0x4331e1; continue;
            }
            0x4331e1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11b8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x433207; continue; }
            pc = 0x4331eb; continue;
            }
            0x4331eb => {
            { let a = c.r[6].wrapping_add(0x11c4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x6cbb74u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x74u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x433207 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43320f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x11cc); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x6cbb74u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x74u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x43322b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x38),32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x433253);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11d0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x433283);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0x1200));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x433294);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.r[6].wrapping_add(0x11b4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(13)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4332b8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11b4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(7,32,c.r[6].wrapping_add(0xba0));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4332d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4332dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11bc),32));
            c.set(1,32,0x3u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43330f; continue; }
            pc = 0x4332ec; continue;
            }
            0x4332ec => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4332fc; continue; }
            pc = 0x4332f6; continue;
            }
            0x4332f6 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            pc = 0x433314; continue;
            }
            0x4332fc => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.and(a,0x6u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            pc = 0x433314; continue;
            }
            0x43330f => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x433314; continue;
            }
            0x433314 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11b4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(0,32,c.r[6].wrapping_add(0x1160));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43333a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x547be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x6cbb74u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x74u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x433350 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x433375; continue; }
            pc = 0x43335d; continue;
            }
            0x43335d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x433375);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x433375; continue;
            }
            0x433375 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.r[6].wrapping_add(0x11b0); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x4334f8; continue; }
            pc = 0x43338b; continue;
            }
            0x43338b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd28),32));
            c.set(7,32,c.r[6].wrapping_add(0xd28));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x4333a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4333ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,c.r[6].wrapping_add(0x7ec));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x4333e4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x4333f2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4333fd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x43340e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9d4),32));
            c.set(7,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x43342c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x43343a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x433445); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x433456); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,0x66a4f4u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let target = c.get(5,32); c.push(m, 0x43347e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x433490); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x4334a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4334f8; continue; }
            pc = 0x4334c0; continue;
            }
            0x4334c0 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x11b4); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4334d6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x11e0));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4334e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0x1200u32,32); c.set(6,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4334f8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4334f8; continue;
            }
            0x4334f8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x74u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_delete_savegame_cleanup(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433d5b;
    loop {
        match pc {
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
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433d8d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_draw_savegame_details(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433f30;
    loop {
        match pc {
            0x433f30 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d914u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x433f61); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x11d4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x433f90);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x433f96);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x433f9d);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x433fa4);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x11b8),32));
            c.set(3,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x433fc4; continue; }
            pc = 0x433fb6; continue;
            }
            0x433fb6 => {
            c.set(2,32,rm(m,0x69af20u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x82u32);
            pc = 0x433fcc; continue;
            }
            0x433fc4 => {
            c.set(0,32,rm(m,0x69af20u32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            pc = 0x433fcc; continue;
            }
            0x433fcc => {
            c.push(m,0xd4u32);
            c.push(m, 0x433fd6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x433fe3);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x433fe9);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x11b8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(5,32,0x6bu32);
            if c.cond(5) { pc = 0x434000; continue; }
            pc = 0x433ffb; continue;
            }
            0x433ffb => {
            c.set(5,32,0xbbu32);
            pc = 0x434000; continue;
            }
            0x434000 => {
            c.push(m,0x2u32);
            c.push(m, 0x434007);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69a90cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x434019);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,0x69a914u32,32));
            c.push(m,c.get(2,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x434033);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69a91cu32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x434048);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aef1cu32,32));
            c.set(5,32,c.r[5].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x6));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x43405e);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69a934u32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x434074);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69a6f4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x43408c);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69a6e4u32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x4340a2);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x699964u32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x4340b8);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69a908u32,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xd4u32);
            c.push(m, 0x4340cd);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6b20f8u32,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0xd4u32);
            c.push(m, 0x4340e3);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x11b8),32));
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(5,32,0x6bu32);
            if c.cond(5) { pc = 0x4340fa; continue; }
            pc = 0x4340f5; continue;
            }
            0x4340f5 => {
            c.set(5,32,0xbbu32);
            pc = 0x4340fa; continue;
            }
            0x4340fa => {
            c.push(m,c.get(3,32));
            c.push(m, 0x434100);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1160),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x697438u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x434116);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,0xdeu32);
            c.push(m, 0x434126);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x116e),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69a990),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0xdeu32);
            c.push(m, 0x434145);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x116f),8));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            if c.cond(5) { pc = 0x43415d; continue; }
            pc = 0x434156; continue;
            }
            0x434156 => {
            c.set(0,32,rm(m,0x6b2d3cu32,32));
            pc = 0x434169; continue;
            }
            0x43415d => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69aa10),32));
            pc = 0x434169; continue;
            }
            0x434169 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xdeu32);
            c.push(m, 0x434175);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x116f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(5,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            if c.cond(6) { pc = 0x4341bc; continue; }
            pc = 0x434186; continue;
            }
            0x434186 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1168),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x434199);
            { let args = []; c.r[0] = host.call(m, 0x499ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xdeu32);
            c.push(m, 0x4341ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x4341bc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4341bc; continue;
            }
            0x4341bc => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1179),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a928),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x4341db);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1164),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69a938),32));
            c.push(m,c.get(1,32));
            c.push(m,0x69743cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4341fc);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x43420c);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x69a7e4u32);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1169),8));
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x434230);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1177),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a758),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x43424f);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x117c),16));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x117b),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x117a),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,0x697440u32);
            c.push(m,c.get(0,32));
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            c.push(m, 0x43427e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x434291);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x1187));
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,0xdeu32);
            c.push(m, 0x4342a7);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x119b),8));
            c.set(6,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x3));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,0xdeu32);
            c.push(m, 0x4342db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4342ef);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_init_list_metrics(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434310;
    loop {
        match pc {
            0x434310 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4]);
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x11d4); wm(m,a,32,0x2u32); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x434334); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,0x16cu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x11d0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x11e4); wm(m,a,32,0x8au32); }
            { let a = c.r[6].wrapping_add(0x11f4); wm(m,a,32,0xb2u32); }
            { let a = c.r[6].wrapping_add(0x1200); wm(m,a,32,0x1eu32); }
            { let a = c.r[6].wrapping_add(0x1204); wm(m,a,32,0x26u32); }
            { let a = c.r[6].wrapping_add(0x1208); wm(m,a,32,0x178u32); }
            { let a = c.r[6].wrapping_add(0x120c); wm(m,a,32,0x1bau32); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x144u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x11d8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x8au32,32); c.set(1,32,r); }
            c.set(2,32,0x1acu32);
            { let a = c.r[6].wrapping_add(0x11ec); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x2d0u32);
            { let a = c.r[6].wrapping_add(0x11e0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x11e8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x11f0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x11f8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x11dc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xb2u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x11fc); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_scan_savegame_files(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4343e0;
    loop {
        match pc {
            0x4343e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d964u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x9d4));
            { let a = c.r[6].wrapping_add(0x11c4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x11b0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x11b4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x434428); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x434431);
            { let args = []; c.r[0] = host.call(m, 0x638f07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11bc),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x434473; continue; }
            pc = 0x43444f; continue;
            }
            0x43444f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x434460; continue; }
            pc = 0x434459; continue;
            }
            0x434459 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x3u32); }
            pc = 0x434478; continue;
            }
            0x434460 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x6u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            pc = 0x434478; continue;
            }
            0x434473 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x0u32); }
            pc = 0x434478; continue;
            }
            0x434478 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43449d; continue; }
            pc = 0x434485; continue;
            }
            0x434485 => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x434496; continue; }
            pc = 0x43448a; continue;
            }
            0x43448a => {
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4344ab; continue; }
            pc = 0x43448f; continue;
            }
            0x43448f => {
            c.push(m,0x697460u32);
            pc = 0x4344a2; continue;
            }
            0x434496 => {
            c.push(m,0x69744cu32);
            pc = 0x4344a2; continue;
            }
            0x43449d => {
            c.push(m,0x697474u32);
            pc = 0x4344a2; continue;
            }
            0x4344a2 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x4344ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4344ab; continue;
            }
            0x4344ab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x4344bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x638fca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4344fb; continue; }
            pc = 0x4344c0; continue;
            }
            0x4344c0 => {
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x4344c9);
            { let args = []; c.r[0] = host.call(m, 0x6391ea, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x4344d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.sub(a,0xcu32,32); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(3,8,u32::from(c.cond(15)));
            c.push(m, 0x4344ed);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4344f7; continue; }
            pc = 0x4344f1; continue;
            }
            0x4344f1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x11ac),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,r); } }
            pc = 0x4344f7; continue;
            }
            0x4344f7 => {
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4344c0; continue; }
            pc = 0x4344fb; continue;
            }
            0x4344fb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x434522; continue; }
            pc = 0x434505; continue;
            }
            0x434505 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x434519);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43451f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x434522; continue;
            }
            0x434522 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x43454f; continue; }
            pc = 0x434532; continue;
            }
            0x434532 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x434546);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43454c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43454f; continue;
            }
            0x43454f => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x434569);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x434593; continue; }
            pc = 0x434579; continue;
            }
            0x434579 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x434591);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x434595; continue;
            }
            0x434593 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x434595; continue;
            }
            0x434595 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x4345b4);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x4345de; continue; }
            pc = 0x4345c4; continue;
            }
            0x4345c4 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4345dc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4345e0; continue;
            }
            0x4345de => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4345e0; continue;
            }
            0x4345e0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,c.r[0].wrapping_mul(4));
            c.push(m,c.get(2,32));
            c.push(m, 0x4345fe);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x434614);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x638fca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4347ff; continue; }
            pc = 0x434634; continue;
            }
            0x434634 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            pc = 0x434638; continue;
            }
            0x434638 => {
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x434641);
            { let args = []; c.r[0] = host.call(m, 0x6391ea, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x434653);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(2,32); c.sub(a,0xcu32,32); }
            c.set(3,8,u32::from(c.cond(15)));
            c.push(m, 0x434667);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4347df; continue; }
            pc = 0x43466f; continue;
            }
            0x43466f => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m, 0x43467d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x4u32); }
            c.push(m, 0x434690);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x43469e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m, 0x4346ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x63915b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[7].wrapping_add(c.r[5]); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x697488u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4346c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e5b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x5u32); }
            c.push(m, 0x4346d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4346e7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let a = rm(m,c.r[2].wrapping_add(0xfffffff8),32); c.sub(a,0xcu32,32); }
            if c.cond(14) { pc = 0x434702; continue; }
            pc = 0x4346f9; continue;
            }
            0x4346f9 => {
            c.push(m,0x0u32);
            c.push(m,0xcu32);
            c.push(m, 0x434702);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x434702; continue;
            }
            0x434702 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4347cf; continue; }
            pc = 0x43470c; continue;
            }
            0x43470c => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[5]),32));
            pc = 0x43470f; continue;
            }
            0x43470f => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)),32),32); }
            if c.cond(15) { pc = 0x434723; continue; }
            pc = 0x434715; continue;
            }
            0x434715 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x43470f; continue; }
            pc = 0x43471e; continue;
            }
            0x43471e => {
            pc = 0x4347cf; continue;
            }
            0x434723 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x43475e; continue; }
            pc = 0x43472d; continue;
            }
            0x43472d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x43473f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x434751);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            { let a = c.r[5].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x43472d; continue; }
            pc = 0x43475e; continue;
            }
            0x43475e => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x43476c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x6u32); }
            c.push(m, 0x434782);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x434790);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,0x69749cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4347a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e5b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x7u32); }
            c.push(m, 0x4347b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4347c4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(c.r[5]); wm(m,a,32,c.get(0,32)); }
            pc = 0x4347cf; continue;
            }
            0x4347cf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            pc = 0x4347df; continue;
            }
            0x4347df => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x434638; continue; }
            pc = 0x4347ff; continue;
            }
            0x4347ff => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x43480e; continue; }
            pc = 0x434805; continue;
            }
            0x434805 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x43480b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43480e; continue;
            }
            0x43480e => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            c.push(m, 0x43481c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43482d);
            { let args = []; c.r[0] = host.call(m, 0x638f44, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_get_savegame_prefix(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4348c0;
    loop {
        match pc {
            0x4348c0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x11bc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4]; wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x4348f5; continue; }
            pc = 0x4348d2; continue;
            }
            0x4348d2 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4348e3; continue; }
            pc = 0x4348dc; continue;
            }
            0x4348dc => {
            { let a = c.r[4]; wm(m,a,8,0x3u32); }
            pc = 0x4348f9; continue;
            }
            0x4348e3 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x6u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4]; wm(m,a,8,c.get(0,8)); }
            pc = 0x4348f9; continue;
            }
            0x4348f5 => {
            { let a = c.r[4]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4348f9; continue;
            }
            0x4348f9 => {
            c.set(2,32,rm(m,c.r[4],32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x43490f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x434920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_savegame_prefix_for_type(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434920;
    loop {
        match pc {
            0x434920 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x43497c; continue; }
            pc = 0x434933; continue;
            }
            0x434933 => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x434965; continue; }
            pc = 0x434938; continue;
            }
            0x434938 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x434954; continue; }
            pc = 0x434943; continue;
            }
            0x434943 => {
            c.push(m,0x6974d4u32);
            c.push(m, 0x43494d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x434954 => {
            c.push(m,0x6974bcu32);
            c.push(m, 0x43495e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x434965 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,0x6974b0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x434975);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x43497c => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,0x6974c8u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43498c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434a80;
    loop {
        match pc {
            0x434a80 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x434a88);
            { let args = []; c.r[0] = host.call(m, 0x431520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x434a98; continue; }
            pc = 0x434a8f; continue;
            }
            0x434a8f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x434a95);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x434a98; continue;
            }
            0x434a98 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434b40;
    loop {
        match pc {
            0x434b40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d9ffu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x434b63);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x434b74);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x434b84);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x330));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.push(m, 0x434b94);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x464b40u32);
            c.push(m,0x464ad0u32);
            c.push(m,0x2u32);
            c.set(0,32,c.r[6].wrapping_add(0x424));
            c.push(m,0xf4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            c.push(m, 0x434bb6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4b0440u32);
            c.push(m,0x4b03e0u32);
            c.push(m,0x2u32);
            c.set(1,32,c.r[6].wrapping_add(0x610));
            c.push(m,0x20u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x4u32); }
            c.push(m, 0x434bd5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x650));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x434be5);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xab4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x434bf5);
            { let args = []; c.r[0] = host.call(m, 0x4209e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0xad0); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x8u32); }
            { let a = c.r[6]; wm(m,a,32,0x66d1d8u32); }
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x434c25);
            { let args = []; c.r[0] = host.call(m, 0x4362e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434c40;
    loop {
        match pc {
            0x434c40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65da98u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66d1d8u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x8u32); }
            c.push(m, 0x434c70);
            { let args = []; c.r[0] = host.call(m, 0x4362e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xad0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x7u32); }
            c.push(m, 0x434c80);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xab4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x434c90);
            { let args = []; c.r[0] = host.call(m, 0x420a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x650));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x434ca0);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4b0440u32);
            c.push(m,0x2u32);
            c.set(0,32,c.r[6].wrapping_add(0x610));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x4u32); }
            c.push(m, 0x434cba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x464b40u32);
            c.push(m,0x2u32);
            c.set(1,32,c.r[6].wrapping_add(0x424));
            c.push(m,0xf4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3u32); }
            c.push(m, 0x434cd7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x330));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x434ce7);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x434cf7);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x434d07);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x434d16);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
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

pub fn choose_map_screen_prepare_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x435130;
    loop {
        match pc {
            0x435130 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x43513d);
            { let args = []; c.r[0] = host.call(m, 0x435da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0x650));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43514a);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69b138u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            c.push(m,c.get(0,32));
            c.push(m, 0x43515b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa84),32));
            c.set(7,32,c.r[6].wrapping_add(0x658));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x435172);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa88),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa90),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x4351a4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4351c3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa80),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4351e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4351ef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xad0));
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0xa70); wm(m,a,32,0x1u32); }
            c.push(m, 0x43520e);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xad4); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x435510;
    loop {
        match pc {
            0x435510 => {
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            c.push(m,c.get(7,32));
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4355b4; continue; }
            pc = 0x435535; continue;
            }
            0x435535 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x435562; continue; }
            pc = 0x435545; continue;
            }
            0x435545 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435559);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43555f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435562; continue;
            }
            0x435562 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43558f; continue; }
            pc = 0x435572; continue;
            }
            0x435572 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435586);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43558c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43558f; continue;
            }
            0x43558f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4358f8; continue; }
            pc = 0x4355a3; continue;
            }
            0x4355a3 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            pc = 0x4358e7; continue;
            }
            0x4355b4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4355f4; continue; }
            pc = 0x4355bf; continue;
            }
            0x4355bf => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4355f4; continue; }
            pc = 0x4355c3; continue;
            }
            0x4355c3 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xa68),32),32); }
            if c.cond(4) { pc = 0x4355f4; continue; }
            pc = 0x4355cb; continue;
            }
            0x4355cb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            c.push(m, 0x4355e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4355e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4368b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            { let a = c.r[6].wrapping_add(0xa68); wm(m,a,32,c.get(0,32)); }
            pc = 0x4355f4; continue;
            }
            0x4355f4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            c.set(5,32,c.r[6].wrapping_add(0x23c));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(1,32,c.get(5,32));
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43560c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x650));
            c.push(m, 0x435617);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x424));
            { let a = c.r[6].wrapping_add(0xa74); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43562a);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x435638; continue; }
            pc = 0x43562e; continue;
            }
            0x43562e => {
            { let a = c.r[6].wrapping_add(0xa70); wm(m,a,32,0x1u32); }
            pc = 0x435638; continue;
            }
            0x435638 => {
            c.set(1,32,c.r[6].wrapping_add(0x518));
            c.push(m, 0x435643);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x435651; continue; }
            pc = 0x435647; continue;
            }
            0x435647 => {
            { let a = c.r[6].wrapping_add(0xa70); wm(m,a,32,0x0u32); }
            pc = 0x435651; continue;
            }
            0x435651 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa70),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x435667); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa70),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x518),32));
            c.set(1,32,c.r[6].wrapping_add(0x518));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x435685); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x330));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x435692);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x435729; continue; }
            pc = 0x43569a; continue;
            }
            0x43569a => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x4356a3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x658));
            c.push(m, 0x4356ae);
            { let args = []; c.r[0] = host.call(m, 0x461480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4356d7; continue; }
            pc = 0x4356ba; continue;
            }
            0x4356ba => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4356ce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4356d4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4356d7; continue;
            }
            0x4356d7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x435704; continue; }
            pc = 0x4356e7; continue;
            }
            0x4356e7 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4356fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435701);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435704; continue;
            }
            0x435704 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4358f8; continue; }
            pc = 0x435718; continue;
            }
            0x435718 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            pc = 0x4358e7; continue;
            }
            0x435729 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x435730);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43574a; continue; }
            pc = 0x435734; continue;
            }
            0x435734 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x43573e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,c.get(0,32)); }
            pc = 0x43574a; continue;
            }
            0x43574a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa6c),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x435866; continue; }
            pc = 0x435759; continue;
            }
            0x435759 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x43576c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x435775);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43579d; continue; }
            pc = 0x435781; continue;
            }
            0x435781 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(0x470));
            c.push(m, 0x43579b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4357b3; continue;
            }
            0x43579d => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x6cbb8bu32);
            { let a = c.get(1,32); let r = c.add(a,0x470u32,32); c.set(1,32,r); }
            c.push(m, 0x4357b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4357b3; continue;
            }
            0x4357b3 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xa70),8));
            { let a = c.r[0].wrapping_add(0x474); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4357eb; continue; }
            pc = 0x4357ce; continue;
            }
            0x4357ce => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4357e2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4357e8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4357eb; continue;
            }
            0x4357eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x435818; continue; }
            pc = 0x4357fb; continue;
            }
            0x4357fb => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43580f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435815);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435818; continue;
            }
            0x435818 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x435845; continue; }
            pc = 0x435828; continue;
            }
            0x435828 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43583c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435842);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435845; continue;
            }
            0x435845 => {
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa5c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x435866 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43586d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43590f; continue; }
            pc = 0x435875; continue;
            }
            0x435875 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43589e; continue; }
            pc = 0x435881; continue;
            }
            0x435881 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435895);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43589b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43589e; continue;
            }
            0x43589e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4358cb; continue; }
            pc = 0x4358ae; continue;
            }
            0x4358ae => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4358c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4358c8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4358cb; continue;
            }
            0x4358cb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4358f8; continue; }
            pc = 0x4358db; continue;
            }
            0x4358db => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            pc = 0x4358e7; continue;
            }
            0x4358e7 => {
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4358ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4358f5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4358f8; continue;
            }
            0x4358f8 => {
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa5c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,0xfffffffeu32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43590f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xa74),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43592a; continue; }
            pc = 0x435919; continue;
            }
            0x435919 => {
            { let a = c.get(6,32); let r = c.add(a,0xa84u32,32); c.set(6,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x43592a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43592a; continue;
            }
            0x43592a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x435950;
    loop {
        match pc {
            0x435950 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.sub(a,0x58u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x435968);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b6b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,0x6cbb88u32,8));
            c.set(0,8,0x1u32);
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x435988; continue; }
            pc = 0x435974; continue;
            }
            0x435974 => {
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = 0x6cbb88u32; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = 0x6cbb84u32; wm(m,a,32,c.get(1,32)); }
            pc = 0x435988; continue;
            }
            0x435988 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa84),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa80),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa88),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            if c.cond(14) { pc = 0x4359c8; continue; }
            pc = 0x4359bf; continue;
            }
            0x4359bf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa90),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x4359d5; continue;
            }
            0x4359c8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[5]).wrapping_add(0xffffffff));
            pc = 0x4359d5; continue;
            }
            0x4359d5 => {
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x28),32),32); }
            c.set(1,8,u32::from(c.cond(13)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x435bb3; continue; }
            pc = 0x435a0e; continue;
            }
            0x435a0e => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x9f4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa60),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x435a8d; continue; }
            pc = 0x435a2b; continue;
            }
            0x435a2b => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x435a4b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xa60); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x435a89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x435a8d; continue;
            }
            0x435a8d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa60),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0xfffffffeu32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x435cd8; continue; }
            pc = 0x435aaf; continue;
            }
            0x435aaf => {
            c.set(1,32,rm(m,0x6dccf0u32,32));
            c.set(3,32,rm(m,0x6cbb84u32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            { let a = c.get(1,32); c.sub(a,0x1f4u32,32); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x435af4; continue; }
            pc = 0x435ad8; continue;
            }
            0x435ad8 => {
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x6cbb84u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x435af4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x435b1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x435b4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xa94));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x435b5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0xaa4));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x435b6c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x518));
            c.set(1,32,c.r[6].wrapping_add(0x424));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x435b85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa60),32));
            { let a = c.r[6].wrapping_add(0xa68); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa64); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x6cbb84u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x435bb3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa60),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x435bd8; continue; }
            pc = 0x435bc0; continue;
            }
            0x435bc0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x435bd8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x435bd8; continue;
            }
            0x435bd8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.r[6].wrapping_add(0xa60); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x435cd8; continue; }
            pc = 0x435beb; continue;
            }
            0x435beb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x658),32));
            c.set(7,32,c.r[6].wrapping_add(0x658));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x435c01); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x435c0d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x435c41); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x435c4f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x435c5a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x435c6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.set(7,32,rm(m,0x66a4f4u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x435c89); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x435cd8; continue; }
            pc = 0x435c8d; continue;
            }
            0x435c8d => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x435c9b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x435cd8; continue; }
            pc = 0x435c9f; continue;
            }
            0x435c9f => {
            c.set(0,32,c.r[6].wrapping_add(0xa84));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xa64); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x435cb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xa94));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x435cc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xaa4u32,32); c.set(6,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x435cd8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x435cd8; continue;
            }
            0x435cd8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_scan_map_files(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x435da0;
    loop {
        match pc {
            0x435da0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65db2du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[6].wrapping_add(0xa5c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa64); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x435de4);
            { let args = []; c.r[0] = host.call(m, 0x638f07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6975b8u32);
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x6cbdf8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x435dfc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,0x1u32); }
            c.push(m, 0x435e10);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x638fca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x435e2e; continue; }
            pc = 0x435e14; continue;
            }
            0x435e14 => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x435e1d);
            { let args = []; c.r[0] = host.call(m, 0x6391ea, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0xa5c); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x435e14; continue; }
            pc = 0x435e2e; continue;
            }
            0x435e2e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x435e55; continue; }
            pc = 0x435e38; continue;
            }
            0x435e38 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435e4c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435e52);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435e55; continue;
            }
            0x435e55 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x435e82; continue; }
            pc = 0x435e65; continue;
            }
            0x435e65 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435e79);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435e7f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435e82; continue;
            }
            0x435e82 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x435eaf; continue; }
            pc = 0x435e92; continue;
            }
            0x435e92 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x435ea6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x435eac);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x435eaf; continue;
            }
            0x435eaf => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x435ec9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x435ef3; continue; }
            pc = 0x435ed9; continue;
            }
            0x435ed9 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x435ef1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x435ef5; continue;
            }
            0x435ef3 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x435ef5; continue;
            }
            0x435ef5 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x435f14);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x435f3e; continue; }
            pc = 0x435f24; continue;
            }
            0x435f24 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x435f3c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x435f40; continue;
            }
            0x435f3e => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x435f40; continue;
            }
            0x435f40 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x435f5f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x4u32); }
            if c.cond(4) { pc = 0x435f89; continue; }
            pc = 0x435f6f; continue;
            }
            0x435f6f => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x435f87);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x435f8b; continue;
            }
            0x435f89 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x435f8b; continue;
            }
            0x435f8b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6aef28u32,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x435fa7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6aef28u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.push(m, 0x435fb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x435fc8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x638fca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4362a0; continue; }
            pc = 0x435fec; continue;
            }
            0x435fec => {
            c.set(7,32,0x4u32);
            pc = 0x435ff1; continue;
            }
            0x435ff1 => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x435ffa);
            { let args = []; c.r[0] = host.call(m, 0x6391ea, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x43600c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x5u32); }
            c.push(m, 0x43601f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            c.push(m, 0x43602d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x43603b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x639351, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x6u32); }
            c.push(m, 0x43604e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            c.push(m, 0x43605c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.push(m, 0x43606a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6392b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x7u32); }
            c.push(m, 0x43607d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            c.push(m, 0x43608b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa84),32));
            c.set(1,32,0x6dce78u32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4360ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4360fa; continue; }
            pc = 0x4360b1; continue;
            }
            0x4360b1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4360c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x4360d4);
            { let args = []; c.r[0] = host.call(m, 0x64e1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa84),32));
            c.set(1,32,0x6dce78u32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4360f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x4360b1; continue; }
            pc = 0x4360fa; continue;
            }
            0x4360fa => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x43610c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x8u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x436123);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x9u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43613a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0xcu32); }
            c.set(3,32,0x1u32);
            if c.cond(14) { pc = 0x43622e; continue; }
            pc = 0x43615a; continue;
            }
            0x43615a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x43616c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(0,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            c.push(m, 0x43617f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x436188);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x436191);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4361a0);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4361b2; continue; }
            pc = 0x4361a7; continue;
            }
            0x4361a7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43615a; continue; }
            pc = 0x4361b0; continue;
            }
            0x4361b0 => {
            pc = 0x43622e; continue;
            }
            0x4361b2 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4361f5; continue; }
            pc = 0x4361ba; continue;
            }
            0x4361ba => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x4361cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x4361de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x4361f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x4361ba; continue; }
            pc = 0x4361f5; continue;
            }
            0x4361f5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m, 0x43620a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m, 0x43621c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m, 0x43622e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43622e; continue;
            }
            0x43622e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0xbu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x436248);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0xau32); }
            c.push(m, 0x436256);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x9u32); }
            c.push(m, 0x436264);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x8u32); }
            c.push(m, 0x436272);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x1u32); }
            c.push(m, 0x436280);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa5c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x435ff1; continue; }
            pc = 0x4362a0; continue;
            }
            0x4362a0 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0x0u32); }
            c.push(m, 0x4362ae);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4362bf);
            { let args = []; c.r[0] = host.call(m, 0x638f44, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4362e0;
    loop {
        match pc {
            0x4362e0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4362eb);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0xa5c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa60); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x436323; continue; }
            pc = 0x436306; continue;
            }
            0x436306 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43631a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x436320);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x436323; continue;
            }
            0x436323 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa08),32));
            { let a = c.r[6].wrapping_add(0xa04); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x436350; continue; }
            pc = 0x436333; continue;
            }
            0x436333 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x436347);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43634d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x436350; continue;
            }
            0x436350 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0c),32));
            { let a = c.r[6].wrapping_add(0xa08); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43637d; continue; }
            pc = 0x436360; continue;
            }
            0x436360 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x436374);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x43637a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43637d; continue;
            }
            0x43637d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x330),32));
            c.set(1,32,c.r[6].wrapping_add(0x330));
            { let a = c.r[6].wrapping_add(0xa0c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa6c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xa70); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0xa74); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xa78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa7c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa80); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa75); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xa64); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xa68); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4363cf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4363e1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x424),32));
            c.set(1,32,c.r[6].wrapping_add(0x424));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4363f3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x518),32));
            c.set(1,32,c.r[6].wrapping_add(0x518));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x436405); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x650));
            c.push(m, 0x436410);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            c.set(1,32,c.r[6].wrapping_add(0xaa4));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x43641f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            c.set(2,32,c.r[6].wrapping_add(0xa84));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43642e);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            { let a = c.get(6,32); let r = c.add(a,0xa94u32,32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x43643d);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_draw_map_info(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x436450;
    loop {
        match pc {
            0x436450 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65db77u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x436478);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x43647e);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dce78u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dce78u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x436494); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa7c),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4364c3);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4364c9);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x4364d0);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x4364d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcfc8u32);
            c.push(m, 0x4364e4);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4364ea);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dcfc8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dcfc8u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4364ff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m, 0x436506);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x424),32));
            c.set(7,32,c.r[6].wrapping_add(0x424));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x436522); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x436529); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6aef30u32,32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x3cu32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x436549);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x436556);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x43655c);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dce78u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dce78u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x436572); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m, 0x436579);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6aef3cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bu32);
            c.push(m,0x3cu32);
            c.push(m, 0x436588);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,0x6aef40u32,32));
            c.push(m,c.get(1,32));
            c.set(5,32,c.r[7].wrapping_add(c.r[7]).wrapping_add(0x70));
            c.push(m,c.get(5,32));
            c.push(m,0x3cu32);
            c.push(m, 0x43659f);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6aef44u32,32));
            c.set(0,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x5));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3cu32);
            c.push(m, 0x4365b2);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,0x1u32);
            if c.cond(5) { pc = 0x4366be; continue; }
            pc = 0x4365c5; continue;
            }
            0x4365c5 => {
            c.push(m, 0x4365ca);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aef28u32,32));
            c.set(2,32,c.r[7].wrapping_add(0x6b));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x50u32);
            c.push(m, 0x4365dc);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x18u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x6975c0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xad4); wm(m,a,32,0x18u32); }
            c.push(m, 0x436605);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[5]));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0xffffffffu32);
            c.push(m,0x5u32);
            c.push(m,0x118u32);
            c.push(m,c.get(2,32));
            c.push(m,0x50u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x436628);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xab4u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x436635);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6975c8u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(5,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x5));
            c.push(m, 0x436647);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6aef34u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(3,8,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x43665f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x3u32); }
            c.push(m, 0x43666e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x43667b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x1u32); }
            c.push(m, 0x436689);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_mul(4));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(0,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m,0x118u32);
            c.push(m,c.get(7,32));
            c.push(m,0x50u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4366ae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4366b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            pc = 0x43687f; continue;
            }
            0x4366be => {
            c.push(m, 0x4366c3);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4367a6; continue; }
            pc = 0x4366d4; continue;
            }
            0x4366d4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa64),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x4366e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa9c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa94),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x4u32); }
            c.push(m, 0x436709);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xffffffc4u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x436785; continue; }
            pc = 0x436710; continue;
            }
            0x436710 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x20u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x436724);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x43672d);
            { let args = []; c.r[0] = host.call(m, 0x64e1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa9c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa94),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x43674a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xffffffc4u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x436710; continue; }
            pc = 0x436751; continue;
            }
            0x436751 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x2eu32);
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[3].wrapping_add(0xffffffff));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x436767);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[3].wrapping_add(0xfffffffe));
            c.push(m,0x2eu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x436776);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,32); let r = c.add(a,0xfffffffdu32,32); c.set(3,32,r); }
            c.push(m,0x2eu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x436785);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x436785; continue;
            }
            0x436785 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[7].wrapping_add(0x6b));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x50u32);
            c.push(m, 0x436795);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4367a6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4367a6; continue;
            }
            0x4367a6 => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xad4),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x6975d0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x5u32); }
            c.push(m, 0x4367cb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[5]));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0xffffffffu32);
            c.push(m,0x5u32);
            c.push(m,0x118u32);
            c.push(m,c.get(2,32));
            c.push(m,0x50u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4367ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0xab4));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4367fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6975d8u32);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.set(5,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x5));
            c.push(m, 0x43680d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xad0u32,32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x6u32); }
            c.push(m, 0x436824);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x7u32); }
            c.push(m, 0x436833);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x6u32); }
            c.push(m, 0x436841);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x5u32); }
            c.push(m, 0x43684f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_mul(4));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m,0x118u32);
            c.push(m,c.get(7,32));
            c.push(m,0x50u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x436874);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43687b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            pc = 0x43687f; continue;
            }
            0x43687f => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x436889);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43689a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_load_map_info(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4368b0;
    loop {
        match pc {
            0x4368b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65db99u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4368e1);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.push(m, 0x4368ef);
            { let args = []; c.r[0] = host.call(m, 0x653b0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(5,32));
            c.push(m,0x8000u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x2u32); }
            c.push(m, 0x436908);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x436911);
            { let args = []; c.r[0] = host.call(m, 0x653f17, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x436919);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x436929);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d3a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x436941; continue; }
            pc = 0x43692d; continue;
            }
            0x43692d => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.and(a,0x3fffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43693a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x436945; continue;
            }
            0x436941 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x436945; continue;
            }
            0x436945 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4369cf; continue; }
            pc = 0x43694d; continue;
            }
            0x43694d => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x43695c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54c480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0xad4); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[6],16); c.sub(a,0xaffeu32,16); }
            if c.cond(5) { pc = 0x4369a1; continue; }
            pc = 0x43696e; continue;
            }
            0x43696e => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x436984);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x43698f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4369a1; continue; }
            pc = 0x436998; continue;
            }
            0x436998 => {
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x436998; continue; }
            pc = 0x4369a1; continue;
            }
            0x4369a1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_add(0xad0));
            c.push(m, 0x4369b1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4369be; continue; }
            pc = 0x4369b5; continue;
            }
            0x4369b5 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4369bb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4369be; continue;
            }
            0x4369be => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4369cf; continue; }
            pc = 0x4369c6; continue;
            }
            0x4369c6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4369cc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4369cf; continue;
            }
            0x4369cf => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4369d5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4369e1);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.push(m, 0x4369ef);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.push(m, 0x4369fd);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x436a0e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choose_map_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x436a50;
    loop {
        match pc {
            0x436a50 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x436a58);
            { let args = []; c.r[0] = host.call(m, 0x434c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x436a68; continue; }
            pc = 0x436a5f; continue;
            }
            0x436a5f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x436a65);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x436a68; continue;
            }
            0x436a68 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x436ad0;
    loop {
        match pc {
            0x436ad0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65de80u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x436af3);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            c.set(0,32,c.r[6].wrapping_add(0x164));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x436b13);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x184));
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x436b32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x1a4));
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x2u32); }
            c.push(m, 0x436b51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x21c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            c.push(m, 0x436b61);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2f4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            c.push(m, 0x436b71);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3cc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x436b81);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x436b91);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x57c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7u32); }
            c.push(m, 0x436ba1);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x464b40u32);
            c.push(m,0x464ad0u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x654));
            c.push(m,0xf4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x8u32); }
            c.push(m, 0x436bc3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x464b40u32);
            c.push(m,0x464ad0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x930));
            c.push(m,0xf4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x9u32); }
            c.push(m, 0x436be5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4b0440u32);
            c.push(m,0x4b03e0u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0xc10));
            c.push(m,0x20u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0xau32); }
            c.push(m, 0x436c04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0xc70));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0xbu32); }
            c.push(m, 0x436c26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xcu32); }
            c.push(m, 0x436c36);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xdu32); }
            c.push(m, 0x436c46);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xeu32); }
            c.push(m, 0x436c56);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xfu32); }
            c.push(m, 0x436c66);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x10u32); }
            c.push(m, 0x436c76);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x11u32); }
            c.push(m, 0x436c86);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x14b0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x12u32); }
            c.push(m, 0x436c96);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1588));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x13u32); }
            c.push(m, 0x436ca6);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15a8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x14u32); }
            c.push(m, 0x436cb6);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x15u32); }
            c.push(m, 0x436cc6);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1748));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x16u32); }
            c.push(m, 0x436cd6);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1768));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x17u32); }
            c.push(m, 0x436ce6);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x18u32); }
            c.push(m, 0x436cf6);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x19u32); }
            c.push(m, 0x436d06);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1au32); }
            c.push(m, 0x436d16);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x1c14));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1bu32); }
            c.push(m, 0x436d38);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4c6a30u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1cu32); }
            c.push(m,0x4c6910u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x2354));
            c.push(m,0xe8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x436d5a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.push(m,0x2u32);
            c.set(0,32,c.r[6].wrapping_add(0x2a94));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1du32); }
            c.push(m, 0x436d7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1eu32); }
            c.push(m, 0x436d8c);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2d5c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1fu32); }
            c.push(m, 0x436d9c);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3110));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x20u32); }
            c.push(m, 0x436dac);
            { let args = []; c.r[0] = host.call(m, 0x4ce8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4228));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x21u32); }
            c.push(m, 0x436dbc);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x4440));
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x22u32); }
            c.push(m, 0x436ddb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0xau32);
            c.set(2,32,c.r[6].wrapping_add(0x4460));
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x23u32); }
            c.push(m, 0x436dfa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x24u32); }
            c.push(m, 0x436e0a);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x48a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x25u32); }
            c.push(m, 0x436e1a);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x4cc0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x4cd4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x27u32); }
            c.push(m, 0x436e35);
            { let args = []; c.r[0] = host.call(m, 0x42dce0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5678));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x28u32); }
            c.push(m, 0x436e45);
            { let args = []; c.r[0] = host.call(m, 0x422610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x29u32); }
            { let a = c.r[6]; wm(m,a,32,0x66d390u32); }
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x448d); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x448c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x20c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x20d); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4cbc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4220); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2d58); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x5674); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x436eba);
            { let args = []; c.r[0] = host.call(m, 0x43f870, c.r[1], &args); }
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

pub fn multiplayer_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x436ed0;
    loop {
        match pc {
            0x436ed0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e15au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66d390u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x29u32); }
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x4218); wm(m,a,8,0x0u32); }
            if c.cond(14) { pc = 0x436f44; continue; }
            pc = 0x436f16; continue;
            }
            0x436f16 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[6].wrapping_add(0x4248));
            pc = 0x436f1e; continue;
            }
            0x436f1e => {
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x436f34; continue; }
            pc = 0x436f24; continue;
            }
            0x436f24 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x436f2b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x436f31);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x436f34; continue;
            }
            0x436f34 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x436f1e; continue; }
            pc = 0x436f42; continue;
            }
            0x436f42 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x436f44; continue;
            }
            0x436f44 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,0x0u32); }
            c.push(m, 0x436f55);
            { let args = []; c.r[0] = host.call(m, 0x43f870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5678));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x28u32); }
            c.push(m, 0x436f65);
            { let args = []; c.r[0] = host.call(m, 0x422720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4cd4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x27u32); }
            c.push(m, 0x436f75);
            { let args = []; c.r[0] = host.call(m, 0x42dda0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4cc0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x26u32); }
            c.push(m, 0x436f85);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x48a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x25u32); }
            c.push(m, 0x436f95);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x24u32); }
            c.push(m, 0x436fa5);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x64f253u32);
            c.push(m,0xau32);
            c.set(0,32,c.r[6].wrapping_add(0x4460));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x23u32); }
            c.push(m, 0x436fbf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x64f253u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x4440));
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x22u32); }
            c.push(m, 0x436fd9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x4228));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x21u32); }
            c.push(m, 0x436fe9);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3110));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x20u32); }
            c.push(m, 0x436ff9);
            { let args = []; c.r[0] = host.call(m, 0x4cea10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2d5c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1fu32); }
            c.push(m, 0x437009);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1eu32); }
            c.push(m, 0x437019);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x2u32);
            c.set(2,32,c.r[6].wrapping_add(0x2a94));
            c.push(m,0xe8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1du32); }
            c.push(m, 0x437036);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4c6a30u32);
            c.push(m,0x8u32);
            c.set(0,32,c.r[6].wrapping_add(0x2354));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1cu32); }
            c.push(m, 0x437053);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4c6a30u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x1c14));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1bu32); }
            c.push(m, 0x437070);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1au32); }
            c.push(m, 0x437080);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x19u32); }
            c.push(m, 0x437090);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x18u32); }
            c.push(m, 0x4370a0);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1768));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x17u32); }
            c.push(m, 0x4370b0);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1748));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x16u32); }
            c.push(m, 0x4370c0);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x15u32); }
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.push(m, 0x4370d0);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15a8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x14u32); }
            c.push(m, 0x4370e0);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1588));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x13u32); }
            c.push(m, 0x4370f0);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x14b0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x12u32); }
            c.push(m, 0x437100);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x11u32); }
            c.push(m, 0x437110);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x10u32); }
            c.push(m, 0x437120);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xfu32); }
            c.push(m, 0x437130);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xeu32); }
            c.push(m, 0x437140);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xdu32); }
            c.push(m, 0x437150);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0xcu32); }
            c.push(m, 0x437160);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4ace40u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0xc70));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xbu32); }
            c.push(m, 0x43717d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4b0440u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0xc10));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xau32); }
            c.push(m, 0x437197);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x464b40u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x930));
            c.push(m,0xf4u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x9u32); }
            c.push(m, 0x4371b4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x464b40u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0x654));
            c.push(m,0xf4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x8u32); }
            c.push(m, 0x4371d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x57c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7u32); }
            c.push(m, 0x4371e1);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6u32); }
            c.push(m, 0x4371f1);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3cc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x437201);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2f4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            c.push(m, 0x437211);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x21c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            c.push(m, 0x437221);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x64f253u32);
            c.push(m,0x8u32);
            c.set(0,32,c.r[6].wrapping_add(0x1a4));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.push(m, 0x43723b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m,0x64f253u32);
            c.push(m,0x8u32);
            c.set(1,32,c.r[6].wrapping_add(0x184));
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x437255);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x64f253u32);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x164));
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x43726f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43727e);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_prepare(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4381e0;
    loop {
        match pc {
            0x4381e0 => {
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x164u32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4381f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x240u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43820d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x4220); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x421c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x5674); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x4ca0); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x930));
            c.set(5,32,0x3u32);
            pc = 0x43823b; continue;
            }
            0x43823b => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x438247); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x43823b; continue; }
            pc = 0x438250; continue;
            }
            0x438250 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0x5670); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,8,0x1u32);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4cb4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4cb8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c08); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x4ccc); wm(m,a,32,0x8u32); }
            { let a = c.r[6].wrapping_add(0x4cd2); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x54d0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x5668); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x4382b1; continue; }
            pc = 0x43829e; continue;
            }
            0x43829e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4382b1; continue; }
            pc = 0x4382a8; continue;
            }
            0x4382a8 => {
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,0x0u32); }
            pc = 0x4382b7; continue;
            }
            0x4382b1 => {
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,c.get(1,8)); }
            pc = 0x4382b7; continue;
            }
            0x4382b7 => {
            { let a = rm(m,c.r[6].wrapping_add(0x443c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4382ec; continue; }
            pc = 0x4382bf; continue;
            }
            0x4382bf => {
            c.set(3,32,c.r[6].wrapping_add(0x4248));
            pc = 0x4382c5; continue;
            }
            0x4382c5 => {
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4382db; continue; }
            pc = 0x4382cb; continue;
            }
            0x4382cb => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4382d2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4382d8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4382db; continue;
            }
            0x4382db => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4382c5; continue; }
            pc = 0x4382e9; continue;
            }
            0x4382e9 => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x4382ec; continue;
            }
            0x4382ec => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4382fd);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x4460));
            c.set(5,32,0xau32);
            pc = 0x438308; continue;
            }
            0x438308 => {
            c.push(m,0x6cbb98u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x438314);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x438308; continue; }
            pc = 0x43831a; continue;
            }
            0x43831a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x566c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0x4488); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43834d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x438680;
    loop {
        match pc {
            0x438680 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x2d5c));
            c.push(m, 0x438694);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4386a9; continue; }
            pc = 0x438698; continue;
            }
            0x438698 => {
            c.set(0,32,c.r[6].wrapping_add(0x4884));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4386a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4386a9; continue;
            }
            0x4386a9 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4386b3);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4224),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438ac3; continue; }
            pc = 0x4386c1; continue;
            }
            0x4386c1 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20d),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a6a; continue; }
            pc = 0x4386cf; continue;
            }
            0x4386cf => {
            c.set(1,32,c.r[6].wrapping_add(0x57c));
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4386e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4386ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a6a; continue; }
            pc = 0x4386fb; continue;
            }
            0x4386fb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43870f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x438724); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x438734); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x438747); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43875a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4387b1; continue; }
            pc = 0x438764; continue;
            }
            0x438764 => {
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4387b1; continue; }
            pc = 0x438775; continue;
            }
            0x438775 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x438780);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            { let a = c.get(0,8); let r = c.and(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4387a3; continue; }
            pc = 0x438789; continue;
            }
            0x438789 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,c.get(0,32)); }
            pc = 0x4387a3; continue;
            }
            0x4387a3 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x438775; continue; }
            pc = 0x4387b1; continue;
            }
            0x4387b1 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4387ba);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xc3u32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x85u32); }
            c.push(m, 0x4387d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,0x66a2bcu32,32));
            c.push(m,0x12cu32);
            { let target = c.get(5,32); c.push(m, 0x4387e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4387f0);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x438815);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x4cd4));
            pc = 0x43881b; continue;
            }
            0x43881b => {
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x438824);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42e080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43882e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            if c.cond(5) { pc = 0x4389ab; continue; }
            pc = 0x43883b; continue;
            }
            0x43883b => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x2u32); }
            c.push(m, 0x438850);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xa8u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43887c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43887e; continue;
            }
            0x43887e => {
            c.push(m,0xau32);
            { let target = c.get(5,32); c.push(m, 0x438882); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43888c);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1d4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4388a4; continue; }
            pc = 0x43889e; continue;
            }
            0x43889e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x64u32,32); }
            if c.cond(12) { pc = 0x43887e; continue; }
            pc = 0x4388a4; continue;
            }
            0x4388a4 => {
            c.set(1,32,rm(m,0x6df2f0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x438943; continue; }
            pc = 0x4388b2; continue;
            }
            0x4388b2 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1d4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43881b; continue; }
            pc = 0x4388c5; continue;
            }
            0x4388c5 => {
            c.set(2,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1d4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x438a6a; continue; }
            pc = 0x4388d4; continue;
            }
            0x4388d4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4388e5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.push(m,0x1u32);
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x1fc); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x438907); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x438911);
            { let args = []; c.r[0] = host.call(m, 0x547fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a6a; continue; }
            pc = 0x43891f; continue;
            }
            0x43891f => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697768u32);
            c.push(m, 0x438937);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43893e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x438a6a; continue;
            }
            0x438943 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4224),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438956; continue; }
            pc = 0x43894d; continue;
            }
            0x43894d => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438956);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438956; continue;
            }
            0x438956 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x0u32); }
            c.push(m, 0x438964);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43898d; continue; }
            pc = 0x43896e; continue;
            }
            0x43896e => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697910u32);
            c.push(m, 0x438986);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43898d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x43898d; continue;
            }
            0x43898d => {
            c.push(m,0x1cu32);
            c.set(1,32,c.get(6,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4389a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4389ab => {
            c.push(m, 0x4389b0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xa5u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4389d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x12cu32);
            { let target = c.get(5,32); c.push(m, 0x4389d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4389e2);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a0b; continue; }
            pc = 0x4389ec; continue;
            }
            0x4389ec => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69794cu32);
            c.push(m, 0x438a04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438a0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x438a0b; continue;
            }
            0x438a0b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4224),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a1e; continue; }
            pc = 0x438a15; continue;
            }
            0x438a15 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438a1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438a1e; continue;
            }
            0x438a1e => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x0u32); }
            c.push(m, 0x438a2c);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438a55; continue; }
            pc = 0x438a36; continue;
            }
            0x438a36 => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697910u32);
            c.push(m, 0x438a4e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438a55);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x438a55; continue;
            }
            0x438a55 => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x438a6a => {
            c.set(0,32,rm(m,0x6df2f0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x438b0d; continue; }
            pc = 0x438a77; continue;
            }
            0x438a77 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x0u32); }
            c.push(m, 0x438a85);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438aae; continue; }
            pc = 0x438a8f; continue;
            }
            0x438a8f => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697910u32);
            c.push(m, 0x438aa7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438aae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x438aae; continue;
            }
            0x438aae => {
            c.push(m,0x1cu32);
            c.set(1,32,c.get(6,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x438ac1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438b0d; continue;
            }
            0x438ac3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca8),32));
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(7) { pc = 0x438b02; continue; }
            pc = 0x438ace; continue;
            }
            0x438ace => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x438b28)); continue;
            }
            0x438ad5 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438adc);
            { let args = []; c.r[0] = host.call(m, 0x43a2a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438b06; continue;
            }
            0x438ade => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438ae5);
            { let args = []; c.r[0] = host.call(m, 0x43b7b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438b06; continue;
            }
            0x438ae7 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438aee);
            { let args = []; c.r[0] = host.call(m, 0x43c090, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438b06; continue;
            }
            0x438af0 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438af7);
            { let args = []; c.r[0] = host.call(m, 0x43c5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438b06; continue;
            }
            0x438af9 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438b00);
            { let args = []; c.r[0] = host.call(m, 0x43d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438b06; continue;
            }
            0x438b02 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x438b06; continue;
            }
            0x438b06 => {
            { let a = c.get(0,32); c.sub(a,0x1a0au32,32); }
            if c.cond(5) { pc = 0x438b1e; continue; }
            pc = 0x438b0d; continue;
            }
            0x438b0d => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4cd0),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0xfffffffd));
            pc = 0x438b1e; continue;
            }
            0x438b1e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_on_key_down(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x438b40;
    loop {
        match pc {
            0x438b40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e1a6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(7,32,c.r[6].wrapping_add(0x1a88));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc4),32); c.push(m, 0x438b72); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x15c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.set(3,8,c.get(0,8));
            { let target = rm(m,c.r[2].wrapping_add(0xc4),32); c.push(m, 0x438b86); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438ba0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b6b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x438d8a; continue; }
            pc = 0x438ba8; continue;
            }
            0x438ba8 => {
            { let a = c.get(5,32); c.sub(a,0xdu32,32); }
            if c.cond(5) { pc = 0x438cd6; continue; }
            pc = 0x438bb1; continue;
            }
            0x438bb1 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x438bb8);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x438bc2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x438cc0; continue; }
            pc = 0x438bd9; continue;
            }
            0x438bd9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4460),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x438be6);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x438c1a; continue; }
            pc = 0x438bed; continue;
            }
            0x438bed => {
            c.set(1,32,c.r[6].wrapping_add(0x4484));
            c.set(3,32,0x9u32);
            pc = 0x438bf8; continue;
            }
            0x438bf8 => {
            c.set(5,32,c.r[1].wrapping_add(0xfffffffc));
            c.push(m,c.get(5,32));
            c.push(m, 0x438c01);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            if c.cond(5) { pc = 0x438bf8; continue; }
            pc = 0x438c06; continue;
            }
            0x438c06 => {
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.set(1,32,c.r[6].wrapping_add(0x4460));
            c.push(m,c.get(2,32));
            c.push(m, 0x438c16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x438c1a; continue;
            }
            0x438c1a => {
            c.push(m,0xffu32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x4488); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x438c30);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x438c3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438c50);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43ee80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.push(m, 0x438c60);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x548160, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x438c6e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6cbb99u32);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x438c7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x2u32); }
            c.push(m, 0x438c8d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x438c9b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4850),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x438cc0; continue; }
            pc = 0x438cab; continue;
            }
            0x438cab => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x4498));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4848),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x438cc0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438cc0; continue;
            }
            0x438cc0 => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x438cd1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x438d82; continue;
            }
            0x438cd6 => {
            { let a = c.get(5,32); c.sub(a,0x26u32,32); }
            if c.cond(5) { pc = 0x438d23; continue; }
            pc = 0x438cdb; continue;
            }
            0x438cdb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4488),32));
            { let a = c.get(0,32); c.sub(a,0x9u32,32); }
            if c.cond(13) { pc = 0x438d82; continue; }
            pc = 0x438cea; continue;
            }
            0x438cea => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4464),32));
            c.push(m,0x6cbb9au32);
            c.push(m,c.get(0,32));
            c.push(m, 0x438cfc);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x438d82; continue; }
            pc = 0x438d03; continue;
            }
            0x438d03 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4488),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x4488); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4460));
            c.push(m,c.get(0,32));
            c.push(m, 0x438d21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438d82; continue;
            }
            0x438d23 => {
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x438d82; continue; }
            pc = 0x438d28; continue;
            }
            0x438d28 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4488),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x438d4a; continue; }
            pc = 0x438d32; continue;
            }
            0x438d32 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4488); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4460));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x438d48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438d82; continue;
            }
            0x438d4a => {
            c.push(m,0x6cbb9bu32);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x438d58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x3u32); }
            c.push(m, 0x438d6c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x438d7c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x4488); wm(m,a,32,c.get(3,32)); }
            pc = 0x438d82; continue;
            }
            0x438d82 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438d8a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438d8a; continue;
            }
            0x438d8a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438e49; continue; }
            pc = 0x438d96; continue;
            }
            0x438d96 => {
            { let a = c.get(5,32); c.sub(a,0xdu32,32); }
            if c.cond(5) { pc = 0x438e49; continue; }
            pc = 0x438d9f; continue;
            }
            0x438d9f => {
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.push(m, 0x438daa);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x438db4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x697960u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x438de9);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(4) { pc = 0x438df5; continue; }
            pc = 0x438df1; continue;
            }
            0x438df1 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x438e2b; continue;
            }
            0x438df5 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x438e2b; continue;
            }
            0x438e2b => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x438e34);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x5668); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x438e89; continue; }
            pc = 0x438e3e; continue;
            }
            0x438e3e => {
            c.push(m,0x1bu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438e47);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438e89; continue;
            }
            0x438e49 => {
            { let a = c.get(5,32); c.sub(a,0x9u32,32); }
            if c.cond(5) { pc = 0x438e89; continue; }
            pc = 0x438e4e; continue;
            }
            0x438e4e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1788),32));
            c.set(3,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc4),32); c.push(m, 0x438e62); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x438e81; continue; }
            pc = 0x438e66; continue;
            }
            0x438e66 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1908),32));
            c.set(7,32,c.r[6].wrapping_add(0x1908));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc4),32); c.push(m, 0x438e7a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x438e81; continue; }
            pc = 0x438e7e; continue;
            }
            0x438e7e => {
            c.push(m,c.get(3,32));
            pc = 0x438e82; continue;
            }
            0x438e81 => {
            c.push(m,c.get(7,32));
            pc = 0x438e82; continue;
            }
            0x438e82 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438e89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x438e89; continue;
            }
            0x438e89 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_draw_game_description(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4398b0;
    loop {
        match pc {
            0x4398b0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e1ecu32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4398e1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4ca8),32));
            c.set(6,32,0xc2u32);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4398f6; continue; }
            pc = 0x4398f1; continue;
            }
            0x4398f1 => {
            c.set(6,32,0xceu32);
            pc = 0x4398f6; continue;
            }
            0x4398f6 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x6dcfc8u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.push(m, 0x439911);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x439917);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x43991e);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x439925);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69af20u32,32));
            c.push(m,c.get(1,32));
            c.push(m,0x32u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x439934);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x439941);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x439947);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x43994e);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69a90cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x5au32);
            c.push(m,c.get(6,32));
            c.push(m, 0x43995d);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,0x69a914u32,32));
            c.push(m,c.get(0,32));
            c.set(3,32,c.r[7].wrapping_add(0x5d));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439971);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69a91cu32,32));
            c.set(5,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439983);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6aef1cu32,32));
            c.set(5,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x6));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439995);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69a934u32,32));
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4399a6);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69a6f4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4399bb);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69a6e4u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x3));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4399cd);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.add(a,0xau32,32); c.set(6,32,r); }
            c.push(m, 0x4399d7);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.push(m,0x697770u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4399ee);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.push(m,0x5au32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4399fb);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a990),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439a0f);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            c.set(3,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            if c.cond(5) { pc = 0x439a24; continue; }
            pc = 0x439a1d; continue;
            }
            0x439a1d => {
            c.set(0,32,rm(m,0x6b2d3cu32,32));
            pc = 0x439a30; continue;
            }
            0x439a24 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69aa10),32));
            pc = 0x439a30; continue;
            }
            0x439a30 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439a38);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(3,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            if c.cond(6) { pc = 0x439a76; continue; }
            pc = 0x439a46; continue;
            }
            0x439a46 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x439a56);
            { let args = []; c.r[0] = host.call(m, 0x499ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439a65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            c.push(m, 0x439a76);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x439a76; continue;
            }
            0x439a76 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x19),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69a928),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439a8e);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a938),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x439aa9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439ab5);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x69a7e4u32);
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x9),8));
            c.set(3,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439ad2);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x3));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x17),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a758),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x439aea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x439afe);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_draw_lobby_labels(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x439b20;
    loop {
        match pc {
            0x439b20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e206u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x439b51); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x439b6c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4398b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m, 0x439b73);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x1788),32));
            c.set(6,32,c.r[7].wrapping_add(0x1788));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439b8f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,c.r[1].wrapping_add(0xffffffba));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x439ba0);
            { let args = []; c.r[0] = host.call(m, 0x54c390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x439c1f; continue; }
            pc = 0x439ba4; continue;
            }
            0x439ba4 => {
            c.set(2,32,c.get(0,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x18u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x697774u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x439bd7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69af24u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x439bf0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f510, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x1u32); }
            c.push(m, 0x439bff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x439c0c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0xb9u32);
            c.push(m, 0x439c1c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x439c1f; continue;
            }
            0x439c1f => {
            c.push(m,0x2u32);
            c.push(m, 0x439c26);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439c38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69af38u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffdu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x439c51);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x1908),32));
            c.set(1,32,c.r[7].wrapping_add(0x1908));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439c6b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69af44u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffdu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x439c84);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2a94),32));
            c.set(6,32,c.r[7].wrapping_add(0x2a94));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x439c98); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,0x6dce78u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x439cad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x439cb4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439cd0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[7].wrapping_add(0x2b7c));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439ce7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x69af40u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x439d02);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x439d09);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69b358),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[5]));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x439d3d);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2c64),32));
            c.set(1,32,c.r[7].wrapping_add(0x2c64));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x439d57); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.push(m, 0x439d5e);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x69a908u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x439d79);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x439d80);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x439d94);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_invalidate_all(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x439db0;
    loop {
        match pc {
            0x439db0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x30),32));
            c.set(1,32,c.r[4]);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x439dda);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_set_lobby_controls_visible(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x439de0;
    loop {
        match pc {
            0x439de0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1788),32));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439dfb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1908),32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439e0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439e21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439e34); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439e47); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439e5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x439ed0; continue; }
            pc = 0x439e5e; continue;
            }
            0x439e5e => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x2354));
            c.set(3,32,0x8u32);
            pc = 0x439e6a; continue;
            }
            0x439e6a => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffff8c0),32));
            c.set(1,32,c.r[7].wrapping_add(0xfffff8c0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439e7e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439e8a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x439e6a; continue; }
            pc = 0x439e93; continue;
            }
            0x439e93 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439ea7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439ebb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439ecf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x439ed0; continue;
            }
            0x439ed0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x15c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x439ef0;
    loop {
        match pc {
            0x439ef0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x439f08);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b6b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,0x6cbb90u32,8));
            c.set(1,32,0x1u32);
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            if c.cond(5) { pc = 0x439f2e; continue; }
            pc = 0x439f1b; continue;
            }
            0x439f1b => {
            c.set(2,8,rm(m,0x6cbb90u32,8));
            { let a = 0x6cbb8cu32; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,8); let r = c.or(a,c.get(1,8),8); c.set(2,8,r); }
            { let a = 0x6cbb90u32; wm(m,a,8,c.get(2,8)); }
            pc = 0x439f2e; continue;
            }
            0x439f2e => {
            { let a = c.get(5,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x439f57; continue; }
            pc = 0x439f33; continue;
            }
            0x439f33 => {
            c.set(7,32,rm(m,0x6cbb8cu32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1f4u32,32); }
            if c.cond(3) { pc = 0x439f50; continue; }
            pc = 0x439f45; continue;
            }
            0x439f45 => {
            { let a = c.r[6].wrapping_add(0x4218); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            pc = 0x439f50; continue;
            }
            0x439f50 => {
            { let a = 0x6cbb8cu32; wm(m,a,32,c.get(0,32)); }
            pc = 0x439f5e; continue;
            }
            0x439f57 => {
            { let a = c.r[6].wrapping_add(0x4218); wm(m,a,8,0x0u32); }
            pc = 0x439f5e; continue;
            }
            0x439f5e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4ca8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x5u32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x439ff2; continue; }
            pc = 0x439f7d; continue;
            }
            0x439f7d => {
            { let a = c.get(5,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x43a226; continue; }
            pc = 0x439f86; continue;
            }
            0x439f86 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            { let a = c.get(6,32); let r = c.add(a,0x11d4u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x439f9a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x439fa9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x439fbd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x439fcc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4f4u32,32); c.push(m, 0x439fe8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x439ff2 => {
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43a226; continue; }
            pc = 0x439ffa; continue;
            }
            0x439ffa => {
            c.set(0,32,rm(m,0x6df2f4u32,32));
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            if c.cond(12) { pc = 0x43a009; continue; }
            pc = 0x43a004; continue;
            }
            0x43a004 => {
            c.set(0,32,0xcu32);
            pc = 0x43a009; continue;
            }
            0x43a009 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4884),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4854),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4888),32));
            c.set(5,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x488c),32));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43a1da; continue; }
            pc = 0x43a062; continue;
            }
            0x43a062 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb4),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43a094);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4888),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4854),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3100),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x421c),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x4cb4); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x43a0c7; continue; }
            pc = 0x43a0c0; continue;
            }
            0x43a0c0 => {
            { let a = c.r[6].wrapping_add(0x4218); wm(m,a,8,0x0u32); }
            pc = 0x43a0c7; continue;
            }
            0x43a0c7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb4),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43a0ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x2c),32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x43a226; continue; }
            pc = 0x43a0f8; continue;
            }
            0x43a0f8 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4854),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb8),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x4888),32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.set(0,32,c.r[6].wrapping_add(0x4884));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43a12a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2d58),32));
            { let a = c.r[6].wrapping_add(0x421c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4cb8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x68));
            c.set(1,32,c.r[6].wrapping_add(0x4cc0));
            c.push(m,c.get(0,32));
            c.push(m, 0x43a158);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4cb8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(7,32,c.r[6].wrapping_add(0x10e0));
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            c.set(0,8,u32::from(c.cond(15)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43a176); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4854),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb8),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4888),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43a1a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d58),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x62),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x60),16),16); }
            c.set(1,32,c.get(7,32));
            c.set(0,8,u32::from(c.cond(2)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43a1d0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x43a1da => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb4),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x4218); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x421c); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x43a226; continue; }
            pc = 0x43a1f4; continue;
            }
            0x43a1f4 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43a220);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4cb4); wm(m,a,32,c.get(3,32)); }
            pc = 0x43a226; continue;
            }
            0x43a226 => {
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

pub fn multiplayer_screen_update_host_lobby(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c5f0;
    loop {
        match pc {
            0x43c5f0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e2d2u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m, 0x43c617);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c621);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43c628);
            { let args = []; c.r[0] = host.call(m, 0x43e420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x566c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x566c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0xc8u32,32); }
            if c.cond(14) { pc = 0x43c666; continue; }
            pc = 0x43c640; continue;
            }
            0x43c640 => {
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x43c649);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0xcfu32); }
            c.push(m, 0x43c660);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x566c); wm(m,a,32,c.get(5,32)); }
            pc = 0x43c666; continue;
            }
            0x43c666 => {
            c.set(1,32,c.r[6].wrapping_add(0x2354));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x43c672; continue;
            }
            0x43c672 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[2].wrapping_add(0xfffff8c0));
            c.push(m, 0x43c681);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43c712; continue; }
            pc = 0x43c689; continue;
            }
            0x43c689 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x43c68d; continue;
            }
            0x43c68d => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x204),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x43c69c; continue; }
            pc = 0x43c698; continue;
            }
            0x43c698 => {
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43c6a6; continue; }
            pc = 0x43c69c; continue;
            }
            0x43c69c => {
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x43c6a6; continue;
            }
            0x43c6a6 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43c68d; continue; }
            pc = 0x43c6ac; continue;
            }
            0x43c6ac => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69785cu32);
            c.push(m, 0x43c6c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43c6c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x43c6d0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xa4u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m, 0x43c6f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c6fd);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c708);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54beb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c712);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43c712; continue;
            }
            0x43c712 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m, 0x43c71b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43c7b7; continue; }
            pc = 0x43c723; continue;
            }
            0x43c723 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x43c727; continue;
            }
            0x43c727 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x204),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x43c740; continue; }
            pc = 0x43c732; continue;
            }
            0x43c732 => {
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x43c740; continue; }
            pc = 0x43c736; continue;
            }
            0x43c736 => {
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x43c740; continue;
            }
            0x43c740 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43c727; continue; }
            pc = 0x43c746; continue;
            }
            0x43c746 => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697860u32);
            c.push(m, 0x43c75a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43c761);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x43c76a);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0xa4u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            c.push(m, 0x43c78d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c797);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xc8u32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x43c7a2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c7ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54bdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c7b7);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43c7b7; continue;
            }
            0x43c7b7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x43c672; continue; }
            pc = 0x43c7cf; continue;
            }
            0x43c7cf => {
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            c.push(m, 0x43c7da);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43c7fe; continue; }
            pc = 0x43c7de; continue;
            }
            0x43c7de => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x43cb9f; continue; }
            pc = 0x43c7fe; continue;
            }
            0x43c7fe => {
            c.set(7,32,0x1u32);
            c.set(3,32,0x2u32);
            pc = 0x43c808; continue;
            }
            0x43c808 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x43c86f; continue; }
            pc = 0x43c811; continue;
            }
            0x43c811 => {
            c.set(0,32,rm(m,0x6af21cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43c824);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6af218u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x0u32); }
            c.push(m, 0x43c842);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43c858);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x43c866); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x43cdb0; continue; }
            pc = 0x43c86f; continue;
            }
            0x43c86f => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43c808; continue; }
            pc = 0x43c875; continue;
            }
            0x43c875 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c891; continue; }
            pc = 0x43c880; continue;
            }
            0x43c880 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c88a; continue; }
            pc = 0x43c883; continue;
            }
            0x43c883 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43c895; continue; }
            pc = 0x43c886; continue;
            }
            0x43c886 => {
            c.set(7,32,c.get(3,32));
            pc = 0x43c899; continue;
            }
            0x43c88a => {
            c.set(7,32,0x1u32);
            pc = 0x43c899; continue;
            }
            0x43c891 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43c899; continue;
            }
            0x43c895 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x43c899; continue;
            }
            0x43c899 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43cb91; continue; }
            pc = 0x43c8a7; continue;
            }
            0x43c8a7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(2,32,c.r[6].wrapping_add(0x1a4));
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x1fc); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[0].wrapping_add(0x168));
            c.push(m, 0x43c8cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[6].wrapping_add(0x184));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(0x2e8));
            c.push(m, 0x43c8e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1cc),8));
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x46c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1c4),8));
            { let a = c.r[0].wrapping_add(0x468); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m, 0x43c91b);
            { let args = []; c.r[0] = host.call(m, 0x49a2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43c926);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43c92d);
            { let args = []; c.r[0] = host.call(m, 0x43fd20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43c937);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xc8u32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x43c942); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43c94e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x497ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(7,32,c.r[6].wrapping_add(0x4c6c));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.add(a,0x1b80u32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x43c969);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43c97a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,0x66e0a4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43c98d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x43cb91; continue; }
            pc = 0x43c996; continue;
            }
            0x43c996 => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.add(a,0x1b80u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43c9ab);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.r[2].wrapping_add(0x168));
            c.push(m, 0x43c9c5);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43c9cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(5,32,c.r[6].wrapping_add(0x1908));
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x43c9e3);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43c9eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x46c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43ca12); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43ca2d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x43ca67);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1fc); wm(m,a,8,0x1u32); }
            c.push(m, 0x43ca7c);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ca89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,0x1u32); }
            c.push(m, 0x43ca98);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43caa5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43cabb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43f690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(3,32)); }
            c.push(m,0x8u32);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43cad7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x43cafd; continue; }
            pc = 0x43cade; continue;
            }
            0x43cade => {
            c.push(m,c.get(0,32));
            c.push(m, 0x43cae4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43caeb);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43caf8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43cdb0; continue;
            }
            0x43cafd => {
            c.push(m, 0x43cb02);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697864u32);
            c.push(m, 0x43cb23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb2a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x43cb40);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x440440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb47);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69786cu32);
            c.push(m, 0x43cb5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697874u32);
            c.push(m, 0x43cb7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb82);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43cb8f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43cba2; continue;
            }
            0x43cb91 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb98);
            { let args = []; c.r[0] = host.call(m, 0x43fd20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cb9f);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43cb9f; continue;
            }
            0x43cb9f => {
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            pc = 0x43cba2; continue;
            }
            0x43cba2 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43cba9);
            { let args = []; c.r[0] = host.call(m, 0x43edb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            c.push(m, 0x43cbb4);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43cc3e; continue; }
            pc = 0x43cbbc; continue;
            }
            0x43cbbc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43cbca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x214),8));
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x1c4); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1fc); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43cc34; continue; }
            pc = 0x43cbf5; continue;
            }
            0x43cbf5 => {
            c.set(2,8,0x1u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43cbf9; continue;
            }
            0x43cbf9 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x43cc13; continue; }
            pc = 0x43cc04; continue;
            }
            0x43cc04 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1fc),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x43cc13; continue; }
            pc = 0x43cc0f; continue;
            }
            0x43cc0f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x43cc18; continue;
            }
            0x43cc13 => {
            c.set(1,32,0x1u32);
            pc = 0x43cc18; continue;
            }
            0x43cc18 => {
            { let a = c.get(2,8); let r = c.and(a,c.get(1,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43cbf9; continue; }
            pc = 0x43cc20; continue;
            }
            0x43cc20 => {
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43cc34); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43cc34; continue;
            }
            0x43cc34 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43cc3e);
            { let args = []; c.r[0] = host.call(m, 0x547fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43cc3e; continue;
            }
            0x43cc3e => {
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m, 0x43cc49);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43ccab; continue; }
            pc = 0x43cc4d; continue;
            }
            0x43cc4d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(7,32,c.r[6].wrapping_add(0x4cd4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43cc61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42e080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43cc6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43ccab; continue; }
            pc = 0x43cc70; continue;
            }
            0x43cc70 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x43cc7f);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xa8u32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x44));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43ccab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43ccab; continue;
            }
            0x43ccab => {
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m, 0x43ccb6);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43cd09; continue; }
            pc = 0x43ccba; continue;
            }
            0x43ccba => {
            c.push(m,0x1u32);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43ccc6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49c0d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x5b9du32);
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43ccda);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43cce4);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43ccfa; continue; }
            pc = 0x43cce9; continue;
            }
            0x43cce9 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(1,32)); }
            pc = 0x43ccfa; continue;
            }
            0x43ccfa => {
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43cd09);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43cd09; continue;
            }
            0x43cd09 => {
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m, 0x43cd14);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43cdb0; continue; }
            pc = 0x43cd1c; continue;
            }
            0x43cd1c => {
            c.set(2,32,rm(m,0x6cbb7cu32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.add(a,0x164u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x43cd30);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,0x240u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43cd42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x43cd4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43cda6; continue; }
            pc = 0x43cd59; continue;
            }
            0x43cd59 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x43cd62);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xc6u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43cd79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43cda4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43cdb0; continue;
            }
            0x43cda6 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43cdb0);
            { let args = []; c.r[0] = host.call(m, 0x547f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43cdb0; continue;
            }
            0x43cdb0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x1a0au32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x68u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "video_mode_selector_init" => video_mode_selector_init(m, host, regs),
        "video_mode_selector_has_valid_mode" => video_mode_selector_has_valid_mode(m, host, regs),
        "video_mode_selector_is_lowest_mode_valid" => video_mode_selector_is_lowest_mode_valid(m, host, regs),
        "video_mode_selector_dtor" => video_mode_selector_dtor(m, host, regs),
        "video_mode_selector_select_resolution" => video_mode_selector_select_resolution(m, host, regs),
        "video_mode_selector_select_color_depth" => video_mode_selector_select_color_depth(m, host, regs),
        "video_mode_selector_update_arrow_buttons" => video_mode_selector_update_arrow_buttons(m, host, regs),
        "video_mode_selector_select_next_resolution" => video_mode_selector_select_next_resolution(m, host, regs),
        "video_mode_selector_select_previous_resolution" => video_mode_selector_select_previous_resolution(m, host, regs),
        "video_mode_selector_select_next_color_depth" => video_mode_selector_select_next_color_depth(m, host, regs),
        "video_mode_selector_select_previous_color_depth" => video_mode_selector_select_previous_color_depth(m, host, regs),
        "video_mode_selector_load_mode_states_from_registry" => video_mode_selector_load_mode_states_from_registry(m, host, regs),
        "video_mode_selector_scalar_deleting_dtor" => video_mode_selector_scalar_deleting_dtor(m, host, regs),
        "hot_key_ctrl_scalar_deleting_dtor" => hot_key_ctrl_scalar_deleting_dtor(m, host, regs),
        "letter_settings_panel_show" => letter_settings_panel_show(m, host, regs),
        "letter_settings_panel_paint" => letter_settings_panel_paint(m, host, regs),
        "letter_settings_panel_update" => letter_settings_panel_update(m, host, regs),
        "letter_settings_panel_dtor" => letter_settings_panel_dtor(m, host, regs),
        "letter_settings_panel_set_clip_rect" => letter_settings_panel_set_clip_rect(m, host, regs),
        "letter_settings_panel_save_to_cfg" => letter_settings_panel_save_to_cfg(m, host, regs),
        "letter_settings_panel_noop_slot54" => letter_settings_panel_noop_slot54(m, host, regs),
        "letter_settings_panel_noop_slot6" => letter_settings_panel_noop_slot6(m, host, regs),
        "credits_window_ctor" => credits_window_ctor(m, host, regs),
        "credits_window_dtor" => credits_window_dtor(m, host, regs),
        "credits_window_release_resources" => credits_window_release_resources(m, host, regs),
        "credits_window_reset_timeline" => credits_window_reset_timeline(m, host, regs),
        "credits_window_update" => credits_window_update(m, host, regs),
        "credits_window_set_paused" => credits_window_set_paused(m, host, regs),
        "credits_page_dtor" => credits_page_dtor(m, host, regs),
        "credits_page_update_fade" => credits_page_update_fade(m, host, regs),
        "credits_page_add_text_line" => credits_page_add_text_line(m, host, regs),
        "credits_page_add_image" => credits_page_add_image(m, host, regs),
        "credits_page_invalidate_rects" => credits_page_invalidate_rects(m, host, regs),
        "credits_page_draw" => credits_page_draw(m, host, regs),
        "credits_window_deleting_dtor" => credits_window_deleting_dtor(m, host, regs),
        "credits_page_deleting_dtor" => credits_page_deleting_dtor(m, host, regs),
        "game_end_screen_ctor" => game_end_screen_ctor(m, host, regs),
        "game_end_screen_dtor" => game_end_screen_dtor(m, host, regs),
        "game_end_screen_release_ranking" => game_end_screen_release_ranking(m, host, regs),
        "game_end_screen_show_result" => game_end_screen_show_result(m, host, regs),
        "game_end_screen_update" => game_end_screen_update(m, host, regs),
        "game_end_screen_on_child_result" => game_end_screen_on_child_result(m, host, regs),
        "game_end_screen_get_top_ranked_merchant" => game_end_screen_get_top_ranked_merchant(m, host, regs),
        "game_end_screen_build_ranking" => game_end_screen_build_ranking(m, host, regs),
        "game_end_screen_deleting_dtor" => game_end_screen_deleting_dtor(m, host, regs),
        "main_menu_ctor" => main_menu_ctor(m, host, regs),
        "main_menu_dtor" => main_menu_dtor(m, host, regs),
        "main_menu_destroy_screens" => main_menu_destroy_screens(m, host, regs),
        "main_menu_reset" => main_menu_reset(m, host, regs),
        "main_menu_process_selection" => main_menu_process_selection(m, host, regs),
        "main_menu_release_texture_surface" => main_menu_release_texture_surface(m, host, regs),
        "main_menu_on_mouse_event" => main_menu_on_mouse_event(m, host, regs),
        "main_menu_scalar_deleting_dtor" => main_menu_scalar_deleting_dtor(m, host, regs),
        "campaign_select_screen_ctor" => campaign_select_screen_ctor(m, host, regs),
        "campaign_select_screen_dtor" => campaign_select_screen_dtor(m, host, regs),
        "campaign_select_screen_release_resources" => campaign_select_screen_release_resources(m, host, regs),
        "campaign_select_screen_setup_lists" => campaign_select_screen_setup_lists(m, host, regs),
        "campaign_select_screen_on_mouse_event" => campaign_select_screen_on_mouse_event(m, host, regs),
        "campaign_select_screen_scalar_deleting_dtor" => campaign_select_screen_scalar_deleting_dtor(m, host, regs),
        "campaign_entry_dtor" => campaign_entry_dtor(m, host, regs),
        "load_save_screen_ctor" => load_save_screen_ctor(m, host, regs),
        "load_save_screen_dtor" => load_save_screen_dtor(m, host, regs),
        "load_save_screen_release_resources" => load_save_screen_release_resources(m, host, regs),
        "load_save_screen_open" => load_save_screen_open(m, host, regs),
        "load_save_screen_prepare_list" => load_save_screen_prepare_list(m, host, regs),
        "load_save_screen_on_key_down" => load_save_screen_on_key_down(m, host, regs),
        "load_save_screen_on_mouse_event" => load_save_screen_on_mouse_event(m, host, regs),
        "load_save_screen_delete_savegame_cleanup" => load_save_screen_delete_savegame_cleanup(m, host, regs),
        "load_save_screen_draw_savegame_details" => load_save_screen_draw_savegame_details(m, host, regs),
        "load_save_screen_init_list_metrics" => load_save_screen_init_list_metrics(m, host, regs),
        "load_save_screen_scan_savegame_files" => load_save_screen_scan_savegame_files(m, host, regs),
        "load_save_screen_get_savegame_prefix" => load_save_screen_get_savegame_prefix(m, host, regs),
        "get_savegame_prefix_for_type" => get_savegame_prefix_for_type(m, host, regs),
        "load_save_screen_scalar_deleting_dtor" => load_save_screen_scalar_deleting_dtor(m, host, regs),
        "choose_map_screen_ctor" => choose_map_screen_ctor(m, host, regs),
        "choose_map_screen_dtor" => choose_map_screen_dtor(m, host, regs),
        "choose_map_screen_prepare_list" => choose_map_screen_prepare_list(m, host, regs),
        "choose_map_screen_process_input" => choose_map_screen_process_input(m, host, regs),
        "choose_map_screen_on_mouse_event" => choose_map_screen_on_mouse_event(m, host, regs),
        "choose_map_screen_scan_map_files" => choose_map_screen_scan_map_files(m, host, regs),
        "choose_map_screen_release_resources" => choose_map_screen_release_resources(m, host, regs),
        "choose_map_screen_draw_map_info" => choose_map_screen_draw_map_info(m, host, regs),
        "choose_map_screen_load_map_info" => choose_map_screen_load_map_info(m, host, regs),
        "choose_map_screen_scalar_deleting_dtor" => choose_map_screen_scalar_deleting_dtor(m, host, regs),
        "multiplayer_screen_ctor" => multiplayer_screen_ctor(m, host, regs),
        "multiplayer_screen_dtor" => multiplayer_screen_dtor(m, host, regs),
        "multiplayer_screen_prepare" => multiplayer_screen_prepare(m, host, regs),
        "multiplayer_screen_process_input" => multiplayer_screen_process_input(m, host, regs),
        "multiplayer_screen_on_key_down" => multiplayer_screen_on_key_down(m, host, regs),
        "multiplayer_screen_draw_game_description" => multiplayer_screen_draw_game_description(m, host, regs),
        "multiplayer_screen_draw_lobby_labels" => multiplayer_screen_draw_lobby_labels(m, host, regs),
        "multiplayer_screen_invalidate_all" => multiplayer_screen_invalidate_all(m, host, regs),
        "multiplayer_screen_set_lobby_controls_visible" => multiplayer_screen_set_lobby_controls_visible(m, host, regs),
        "multiplayer_screen_on_mouse_event" => multiplayer_screen_on_mouse_event(m, host, regs),
        "multiplayer_screen_update_host_lobby" => multiplayer_screen_update_host_lobby(m, host, regs),
        _ => return None,
    })
}
