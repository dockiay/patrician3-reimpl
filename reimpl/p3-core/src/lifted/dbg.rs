//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn debug_info_window_factory(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4103d0;
    loop {
        match pc {
            0x4103d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b70au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0xdcu32);
            c.push(m, 0x4103f2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x41045c; continue; }
            pc = 0x410405; continue;
            }
            0x410405 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41040c);
            { let args = []; c.r[0] = host.call(m, 0x6571ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xffffu32);
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,0x67c9a4u32); }
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0xc2); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xc6); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66bb58u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x41045c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_get_runtime_class(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x410470;
    loop {
        match pc {
            0x410470 => {
            c.set(0,32,0x66b3d0u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x410480;
    loop {
        match pc {
            0x410480 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x410488);
            { let args = []; c.r[0] = host.call(m, 0x6571ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,0x67c9a4u32); }
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0xffffu32);
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xc6); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66bb58u32); }
            { let a = c.r[6].wrapping_add(0xc2); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4104d0;
    loop {
        match pc {
            0x4104d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b724u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66bb58u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x410512; continue; }
            pc = 0x410507; continue;
            }
            0x410507 => {
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4c4u32,32); c.push(m, 0x410512); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410512; continue;
            }
            0x410512 => {
            c.set(1,32,c.r[6].wrapping_add(0xd4));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1]; wm(m,a,32,0x67c9a4u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x41052c);
            { let args = []; c.r[0] = host.call(m, 0x65366f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41053b);
            { let args = []; c.r[0] = host.call(m, 0x6572ac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_get_message_map(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x410550;
    loop {
        match pc {
            0x410550 => {
            c.set(0,32,0x66bb50u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_on_paint(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4106c0;
    loop {
        match pc {
            0x4106c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b736u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x4106e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x655d09, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0xc6),16); c.sub(a,0x4u32,16); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            if c.cond(3) { pc = 0x410757; continue; }
            pc = 0x4106f7; continue;
            }
            0x4106f7 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc2),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41071b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x410b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc2),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x41073a; continue; }
            pc = 0x41072b; continue;
            }
            0x41072b => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410738);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x411310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410757; continue;
            }
            0x41073a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x410757; continue; }
            pc = 0x41074a; continue;
            }
            0x41074a => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410757);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4110d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410757; continue;
            }
            0x410757 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc6),16));
            { let a = c.get(0,32); c.sub(a,0x1cu32,32); }
            if c.cond(7) { pc = 0x410a95; continue; }
            pc = 0x410769; continue;
            }
            0x410769 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x410ac0)); continue;
            }
            0x410770 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x41078d; continue;
            }
            0x41078d => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41079a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4115d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x41079f => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x4107bc; continue;
            }
            0x4107bc => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4107d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x412140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x410a9e; continue;
            }
            0x4107d6 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x4107f3; continue;
            }
            0x4107f3 => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410808);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x412aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x410a9e; continue;
            }
            0x41080d => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x41082a; continue;
            }
            0x41082a => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41083f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x412fa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x410a9e; continue;
            }
            0x410844 => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410850);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x415700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410855 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410861);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x416040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410866 => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x410872);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x416310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410877 => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410883);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x414e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410888 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x4108a5; continue;
            }
            0x4108a5 => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4108b2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x411940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x4108b7 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4108c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x417bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4108c8 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc2),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4108ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x410b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410902);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x417940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x410907 => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410915);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x416600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x41091a => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410928);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x416600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x41092d => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410941);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x418120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x410946 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41095c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x417790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x410961 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc8),16));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410977);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x414210, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x41097c => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410990);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4173e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x410a9e; continue;
            }
            0x410995 => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4109a1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4184a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109a6 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4109b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109b7 => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4109c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x418990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109c8 => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4109d4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x419280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109d9 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4109e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x419630, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109ea => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4109f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4198f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x4109fb => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410a07);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x419eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410a0c => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410a2d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x410b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x410a9e; continue; }
            pc = 0x410a46; continue;
            }
            0x410a46 => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410a5b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4127f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x410a9e; continue;
            }
            0x410a5d => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410a69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x413390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410a6b => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x410a77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x413560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410a79 => {
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x410a85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x413730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410a87 => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x410a93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x413be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x410a9e; continue;
            }
            0x410a95 => {
            { let a = c.r[6].wrapping_add(0xc6); wm(m,a,16,0xau32); }
            pc = 0x410a9e; continue;
            }
            0x410a9e => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x410aaf);
            { let args = []; c.r[0] = host.call(m, 0x655d7b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_ware_table(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4115d0;
    loop {
        match pc {
            0x4115d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b7f6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x694c2cu32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x411627);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411644); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411651);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694c44u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.set(5,32,0x46u32);
            c.push(m, 0x411664);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41167c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411689);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694c4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x411697);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xb4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4116b2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4116bf);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694c58u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x4116cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4116e8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4116f5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694c64u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x411703);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x154u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41171e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x41172b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694c70u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x411739);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411754); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411761);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            pc = 0x41176e; continue;
            }
            0x41176a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x41176e; continue;
            }
            0x41176e => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let a = c.get(5,32); let r = c.add(a,0x12u32,32); c.set(5,32,r); }
            c.push(m, 0x411782);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41179a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4117a8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4117be; continue; }
            pc = 0x4117b2; continue;
            }
            0x4117b2 => {
            c.set(3,32,0xc8u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x46u32); }
            pc = 0x4117c8; continue;
            }
            0x4117be => {
            c.set(3,32,0x7d0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x4cu32); }
            pc = 0x4117c8; continue;
            }
            0x4117c8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4117dd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694c80u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4117ed);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411806); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[7],32),32); }
            if c.cond(14) { pc = 0x41181a; continue; }
            pc = 0x41180a; continue;
            }
            0x41180a => {
            c.push(m,0x694c88u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x411818);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x411842; continue;
            }
            0x41181a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x41182f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694c8cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41183f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x411842; continue;
            }
            0x411842 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x154u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411858); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x30),8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x694c94u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x411877);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41188d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x30c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.push(m,0x694c9cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4118b0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xb4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4118c9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.push(m,0x694ca4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4118e7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411900); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x41176a; continue; }
            pc = 0x411915; continue;
            }
            0x411915 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x411926);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_ware_table_extended(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x411940;
    loop {
        match pc {
            0x411940 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b888u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(7,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4a2u32,16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x694cacu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4119d3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4119ee); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x411a75; continue; }
            pc = 0x4119f7; continue;
            }
            0x4119f7 => {
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c8),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2e4),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2e0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2dc),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2d8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a4u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x7u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x9a4),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,0x694cd0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411a5e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411a75); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x411a75; continue;
            }
            0x411a75 => {
            c.push(m,0x694d18u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411a83);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411a9c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411aa9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d30u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.set(7,32,0x46u32);
            c.push(m, 0x411abc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411ad4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411ae1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d38u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411aef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xb4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411b0a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411b17);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d40u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411b25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411b40); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411b4d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411b5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x154u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411b76); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411b83);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d58u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411b91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411bac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411bb9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d64u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411bc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411be2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411bef);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d70u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411bfd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x244u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411c18); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411c25);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d78u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411c33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x294u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411c4e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411c5b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d80u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411c69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2e4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411c84); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411c91);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d88u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411c9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x334u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411cba); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411cc7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d90u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411cd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xcu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411cf0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411cfd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694d98u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x411d0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xdu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x3d4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411d26); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411d33);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x4f0));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x411d48; continue;
            }
            0x411d48 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x411d60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xeu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411d78); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x411d85);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x411db4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694da0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411dc4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411ddd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x411df8; continue; }
            pc = 0x411de8; continue;
            }
            0x411de8 => {
            c.push(m,0x694da8u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x411df6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x411e20; continue;
            }
            0x411df8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x411e0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694dacu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411e1d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x411e20; continue;
            }
            0x411e20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411e36); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694db4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x411e49);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411e5f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x30c),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694db8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x411e75);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xb4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411e8e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x60),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694dbcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x411ea1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411eba); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc0),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694dc0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x411ed0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x154u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411ee9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x411efd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x694dc4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411f12);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x244u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411f2b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x294u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x411f3b; continue;
            }
            0x411f3b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.push(m,0x694dc8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x411f51);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411f6a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,0x50u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3d4u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x411f3b; continue; }
            pc = 0x411f87; continue;
            }
            0x411f87 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x694dccu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411f9d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x3d4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411fb6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x411d48; continue; }
            pc = 0x411fd3; continue;
            }
            0x411fd3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x672cb4u32); }
            { let a = c.get(2,32); let r = c.add(a,0x124u32,32); c.set(2,32,r); }
            c.set(5,32,0x6c0c64u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x411fee; continue;
            }
            0x411fee => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.push(m, 0x411ffe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xfu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412016); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x412023);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.push(m,0x694dd0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x412039);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41204f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,16,rm(m,c.r[2],16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694dd4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x412071);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41208a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x6c0c7cu32,32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x411fee; continue; }
            pc = 0x4120af; continue;
            }
            0x4120af => {
            c.push(m,0x694ddcu32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.push(m, 0x4120c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x10u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4120d8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4120e5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x2bc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x694de8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4120ff);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412118); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x412129);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_construction_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4127f0;
    loop {
        match pc {
            0x4127f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b92fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.set(5,32,0x46u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x694e78u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.push(m, 0x412857);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412874); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x412882);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x762),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x412a69; continue; }
            pc = 0x4128a1; continue;
            }
            0x4128a1 => {
            pc = 0x4128a7; continue;
            }
            0x4128a3 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4128a7; continue;
            }
            0x4128a7 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4128af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x694e8cu32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.set(3,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.push(m, 0x4128c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4128dd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x4128eb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x412901);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x1eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412919); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x412927);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.push(m,c.get(0,32));
            if c.cond(3) { pc = 0x412984; continue; }
            pc = 0x41293e; continue;
            }
            0x41293e => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x412948);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x41294f);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x412959);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412974); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x412982);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4129ac; continue;
            }
            0x412984 => {
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,0x694e90u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x412993);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4129ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4129ac; continue;
            }
            0x4129ac => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            c.push(m,c.get(0,32));
            c.push(m,0x694ea0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4129c1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x15eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4129da); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7],8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.push(m,0x694eb0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4129f4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412a0d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(2) { pc = 0x412a51; continue; }
            pc = 0x412a14; continue;
            }
            0x412a14 => {
            { let a = c.get(0,8); c.sub(a,0x1eu32,8); }
            if c.cond(3) { pc = 0x412a51; continue; }
            pc = 0x412a18; continue;
            }
            0x412a18 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x3u32);
            { let a = c.get(0,32); let r = c.sub(a,0x15u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,0x694ebcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x412a38);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412a51); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x412a51; continue;
            }
            0x412a51 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x12u32,32); c.set(5,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4128a3; continue; }
            pc = 0x412a69; continue;
            }
            0x412a69 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            c.push(m, 0x412a77);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x412a88);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_merchant_office_buildings(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x412aa0;
    loop {
        match pc {
            0x412aa0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b971u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x694ec8u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,0x0u32); }
            c.push(m, 0x412ad9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412af6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x412b04);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x412b3c; continue; }
            pc = 0x412b19; continue;
            }
            0x412b19 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x412b3c; continue; }
            pc = 0x412b26; continue;
            }
            0x412b26 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x412b34);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x412b95; continue; }
            pc = 0x412b3c; continue;
            }
            0x412b3c => {
            c.push(m,0x694ee4u32);
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m, 0x412b4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x46u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412b63); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x412b71);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x412b82);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x412b95 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,0x694f04u32);
            c.set(5,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x412bba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x46u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412bd3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x0u32); }
            c.push(m, 0x412be1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694f08u32);
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m, 0x412bef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x46u32);
            c.push(m,0x1eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412c08); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x0u32); }
            c.push(m, 0x412c16);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,0x58u32);
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x2cc),16));
            c.set(1,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x412d0a; continue; }
            pc = 0x412c39; continue;
            }
            0x412c39 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x412c44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            c.set(3,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m, 0x412c53);
            { let args = []; c.r[0] = host.call(m, 0x4d4a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x694f10u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x412c68);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412c7e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x6),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m, 0x412c94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412cac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x0u32); }
            c.push(m, 0x412cba);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc),16));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,0x694f14u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x412cda);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412cf3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.set(0,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x412c39; continue; }
            pc = 0x412d06; continue;
            }
            0x412d06 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x412d0a; continue;
            }
            0x412d0a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2d0),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x412d83; continue; }
            pc = 0x412d42; continue;
            }
            0x412d42 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x412d4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(0xffffffeb));
            c.set(0,32,0x55555556u32);
            { let a = rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x24),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x24); wm(m,a,16,r); } }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1c),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1c); wm(m,a,16,r); } }
            c.set(2,32,c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1c));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x412d42; continue; }
            pc = 0x412d83; continue;
            }
            0x412d83 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.r[0].wrapping_add(0x2de));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            pc = 0x412d95; continue;
            }
            0x412d95 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x672a18),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x3u32); }
            { let a = c.get(2,16); let r = c.imul(a,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x1c),16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(c.r[1]).wrapping_add(0x1c); wm(m,a,16,c.get(2,16)); }
            pc = 0x412daf; continue;
            }
            0x412daf => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x24),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x412e61; continue; }
            pc = 0x412dbd; continue;
            }
            0x412dbd => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694f20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x412dd2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412de8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6a581c),32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m,c.get(0,32));
            c.push(m, 0x412df9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412e11); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,0x0u32); }
            c.push(m, 0x412e1f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x1c),16));
            c.set(2,16,rm(m,c.r[5],16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x694f24u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x412e41);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412e5a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            pc = 0x412e61; continue;
            }
            0x412e61 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x412daf; continue; }
            pc = 0x412e71; continue;
            }
            0x412e71 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,0x6u32,32); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x412d95; continue; }
            pc = 0x412e84; continue;
            }
            0x412e84 => {
            c.push(m,0x2cu32);
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x412e92);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x2ce),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x412ef1; continue; }
            pc = 0x412eb6; continue;
            }
            0x412eb6 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x412ebe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x412edb; continue; }
            pc = 0x412ecb; continue;
            }
            0x412ecb => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x38),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x38); wm(m,a,16,r); } }
            c.set(0,32,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x38));
            pc = 0x412ee4; continue;
            }
            0x412edb => {
            { let a = c.get(0,8); c.sub(a,0x1eu32,8); }
            if c.cond(5) { pc = 0x412ee4; continue; }
            pc = 0x412edf; continue;
            }
            0x412edf => {
            { let a = rm(m,c.r[4].wrapping_add(0x62),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(0x62); wm(m,a,16,r); } }
            pc = 0x412ee4; continue;
            }
            0x412ee4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x412eb6; continue; }
            pc = 0x412ef1; continue;
            }
            0x412ef1 => {
            { let a = c.get(7,32); let r = c.add(a,0x19u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x412ef6; continue;
            }
            0x412ef6 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x38),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x412f36; continue; }
            pc = 0x412f00; continue;
            }
            0x412f00 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(0,32));
            c.push(m,0x694f34u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x412f1d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412f33); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            pc = 0x412f36; continue;
            }
            0x412f36 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x15u32,32); }
            if c.cond(12) { pc = 0x412ef6; continue; }
            pc = 0x412f3c; continue;
            }
            0x412f3c => {
            { let a = rm(m,c.r[4].wrapping_add(0x62),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x412f71; continue; }
            pc = 0x412f44; continue;
            }
            0x412f44 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x38),16));
            c.push(m,c.get(0,32));
            c.push(m,0x694f40u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x412f5b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x412f71); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x412f71; continue;
            }
            0x412f71 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x412f82);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_distance_matrix(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x413390;
    loop {
        match pc {
            0x413390 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b9ddu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,c.get(4,8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x694ff4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x413402);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41341d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(3,32,0x64u32);
            if c.cond(14) { pc = 0x41347c; continue; }
            pc = 0x413426; continue;
            }
            0x413426 => {
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,8,rm(m,c.r[0],8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x413439);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64cb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x1),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x41344d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x413461);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413474); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x32u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x413426; continue; }
            pc = 0x41347c; continue;
            }
            0x41347c => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x30u32); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x413536; continue; }
            pc = 0x413492; continue;
            }
            0x413492 => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x4134a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4134bf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            c.push(m, 0x4134cd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x64u32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4134d4; continue;
            }
            0x4134d4 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4134e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x531b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695010u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4134f5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41350e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,0x32u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4134d4; continue; }
            pc = 0x41351e; continue;
            }
            0x41351e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x413492; continue; }
            pc = 0x413536; continue;
            }
            0x413536 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x413547);
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

pub fn debug_page_player_letter_counts(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x413560;
    loop {
        match pc {
            0x413560 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b9fdu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x114u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0x10cu32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x134); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m, 0x4135a2);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4135b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd736u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4135f1; continue; }
            pc = 0x4135cb; continue;
            }
            0x4135cb => {
            c.set(6,32,rm(m,0x6dd730u32,32));
            pc = 0x4135d1; continue;
            }
            0x4135d1 => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x4),8));
            { let a = rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x14),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x14); wm(m,a,16,r); } }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4135d1; continue; }
            pc = 0x4135f1; continue;
            }
            0x4135f1 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695014u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x413601);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x13c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x5u32);
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41361f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695028u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41362f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x5u32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413649); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,0x19u32);
            pc = 0x41364e; continue;
            }
            0x41364e => {
            { let a = rm(m,c.r[4].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x14),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4136f0; continue; }
            pc = 0x41365a; continue;
            }
            0x41365a => {
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695040u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41366a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413680); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6947f8),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x413691);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x128); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x32u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4136ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x128); wm(m,a,8,0x0u32); }
            c.push(m, 0x4136bd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x14),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695044u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4136d4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4136ed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            pc = 0x4136f0; continue;
            }
            0x4136f0 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x86u32,32); }
            if c.cond(2) { pc = 0x41364e; continue; }
            pc = 0x4136fd; continue;
            }
            0x4136fd => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x128); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x413711);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x120),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x120u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_low_productivity_buildings(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x413730;
    loop {
        match pc {
            0x413730 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ba54u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x43cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6df2f0u32,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xcu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x448); wm(m,a,32,0x0u32); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,0x14u32);
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x413bb2; continue; }
            pc = 0x413786; continue;
            }
            0x413786 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x458),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x41379b; continue;
            }
            0x41379b => {
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(7,32); let r = c.sub(a,0x9f8u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(3,32,c.r[7].wrapping_add(c.r[0]));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x784),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4139e2; continue; }
            pc = 0x4137d3; continue;
            }
            0x4137d3 => {
            pc = 0x4137d9; continue;
            }
            0x4137d5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x4137d9; continue;
            }
            0x4137d9 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4137e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4139b7; continue; }
            pc = 0x41380f; continue;
            }
            0x41380f => {
            pc = 0x413815; continue;
            }
            0x413811 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x413815; continue;
            }
            0x413815 => {
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x41381d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x41399a; continue; }
            pc = 0x413834; continue;
            }
            0x413834 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x848),16); c.sub(a,0x190u32,16); }
            if c.cond(3) { pc = 0x41399a; continue; }
            pc = 0x41384c; continue;
            }
            0x41384c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x413861);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x5u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41387c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x41388d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x4138a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x50u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4138be); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x4138cf);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[3].wrapping_add(0x888));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.set(7,32,0x4u32);
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4138e7; continue;
            }
            0x4138e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = rm(m,c.r[2],16); c.sub(a,0x3e8u32,16); }
            if c.cond(2) { pc = 0x413949; continue; }
            pc = 0x4138f2; continue;
            }
            0x4138f2 => {
            { let a = c.get(7,32); c.sub(a,0x5u32,32); }
            if c.cond(4) { pc = 0x413949; continue; }
            pc = 0x4138f7; continue;
            }
            0x4138f7 => {
            c.set(0,32,c.get(7,32));
            c.set(3,32,0xau32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,8); let r = c.add(a,0x30u32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0x30u32,8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x3au32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            pc = 0x413949; continue;
            }
            0x413949 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x15u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x4138e7; continue; }
            pc = 0x41395a; continue;
            }
            0x41395a => {
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x413968);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413986); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x413997);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x11u32,32); c.set(5,32,r); }
            pc = 0x41399a; continue;
            }
            0x41399a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x413811; continue; }
            pc = 0x4139af; continue;
            }
            0x4139af => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4139b7; continue;
            }
            0x4139b7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4139c6; continue; }
            pc = 0x4139bf; continue;
            }
            0x4139bf => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4139c6);
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4139c6; continue;
            }
            0x4139c6 => {
            c.set(1,32,rm(m,0x6de4a8u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4137d5; continue; }
            pc = 0x4139de; continue;
            }
            0x4139de => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4139e2; continue;
            }
            0x4139e2 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x762),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x413ba7; continue; }
            pc = 0x4139ff; continue;
            }
            0x4139ff => {
            pc = 0x413a05; continue;
            }
            0x413a01 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x413a05; continue;
            }
            0x413a05 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x413a0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x413b8a; continue; }
            pc = 0x413a24; continue;
            }
            0x413a24 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x848),16); c.sub(a,0x190u32,16); }
            if c.cond(3) { pc = 0x413b8a; continue; }
            pc = 0x413a3c; continue;
            }
            0x413a3c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m, 0x413a51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x5u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413a6c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x413a7d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x413a93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x50u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413aae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x413abf);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[3].wrapping_add(0x888));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.set(7,32,0x4u32);
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x413ad7; continue;
            }
            0x413ad7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = rm(m,c.r[2],16); c.sub(a,0x3e8u32,16); }
            if c.cond(2) { pc = 0x413b39; continue; }
            pc = 0x413ae2; continue;
            }
            0x413ae2 => {
            { let a = c.get(7,32); c.sub(a,0x5u32,32); }
            if c.cond(4) { pc = 0x413b39; continue; }
            pc = 0x413ae7; continue;
            }
            0x413ae7 => {
            c.set(0,32,c.get(7,32));
            c.set(3,32,0xau32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,8); let r = c.add(a,0x30u32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0x30u32,8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x3au32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x20u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            pc = 0x413b39; continue;
            }
            0x413b39 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x15u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x413ad7; continue; }
            pc = 0x413b4a; continue;
            }
            0x413b4a => {
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x413b58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413b76); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,0x0u32); }
            c.push(m, 0x413b87);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x11u32,32); c.set(5,32,r); }
            pc = 0x413b8a; continue;
            }
            0x413b8a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x413a01; continue; }
            pc = 0x413b9f; continue;
            }
            0x413b9f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x413ba7; continue;
            }
            0x413ba7 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x41379b; continue; }
            pc = 0x413baf; continue;
            }
            0x413baf => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x413bb2; continue;
            }
            0x413bb2 => {
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x448); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x413bc6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x440),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x448u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_bathhouse_notables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x413be0;
    loop {
        match pc {
            0x413be0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ba86u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,0x46u32);
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x695048u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x413c4c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413c67); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,0x6e54c8u32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x413e9d; continue; }
            pc = 0x413c73; continue;
            }
            0x413c73 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x413e9d; continue; }
            pc = 0x413c84; continue;
            }
            0x413c84 => {
            c.set(0,32,0x6dda00u32);
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,0xd4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xd4u32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x413ca5; continue;
            }
            0x413ca1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x413ca5; continue;
            }
            0x413ca5 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m, 0x413cc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413cdf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x413ced);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,0xfffff928u32);
            { let a = c.get(7,32); let r = c.add(a,0x6d8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(5,32,0x64u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x413d0d; continue;
            }
            0x413d09 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x413d0d; continue;
            }
            0x413d0d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xfffffffc),8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x695058u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x413d30);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413d45); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.add(a,0x96u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x413d09; continue; }
            pc = 0x413d57; continue;
            }
            0x413d57 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x76c),32));
            { let a = c.get(4,8); c.and(a,0x20u32,8); }
            if c.cond(5) { pc = 0x413da5; continue; }
            pc = 0x413d6d; continue;
            }
            0x413d6d => {
            c.push(m,0x695070u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x413d7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413d92); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x413da0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x413e69; continue;
            }
            0x413da5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x6e54c8u32,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2]),32),32); }
            if c.cond(6) { pc = 0x413def; continue; }
            pc = 0x413db9; continue;
            }
            0x413db9 => {
            c.push(m,0x695088u32);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x413dc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413dde); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x413dec);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x32u32,32); c.set(5,32,r); }
            pc = 0x413def; continue;
            }
            0x413def => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x413df1; continue;
            }
            0x413df1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xac),8));
            c.set(0,32,0x2u32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x413e63; continue; }
            pc = 0x413e11; continue;
            }
            0x413e11 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x6d8),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x413e31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x413e3b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x413e52); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x413e60);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x64u32,32); c.set(5,32,r); }
            pc = 0x413e63; continue;
            }
            0x413e63 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x413df1; continue; }
            pc = 0x413e69; continue;
            }
            0x413e69 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            if c.cond(7) { pc = 0x413ca1; continue; }
            pc = 0x413e9c; continue;
            }
            0x413e9c => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x413e9d; continue;
            }
            0x413e9d => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x413eae);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_ship_route_and_trade_info(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x414210;
    loop {
        match pc {
            0x414210 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bb40u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0x69508cu32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x41426c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414287); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(7,32));
            c.push(m,0x695098u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41429b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4142b2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4142e5; continue; }
            pc = 0x4142c0; continue;
            }
            0x4142c0 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4142d1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4142e5 => {
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.set(1,32,0x6de4a0u32);
            if c.cond(3) { pc = 0x414311; continue; }
            pc = 0x414309; continue;
            }
            0x414309 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x414312; continue;
            }
            0x414311 => {
            c.push(m,c.get(5,32));
            pc = 0x414312; continue;
            }
            0x414312 => {
            c.push(m, 0x414317);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[7].wrapping_add(0x160));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x414329);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x30u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414342); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414350);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x19),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x414364);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x41436c);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6950a8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41437c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x30u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414396); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x6950b8u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4143a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x44u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4143bd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4143cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.push(m,c.get(0,32));
            c.push(m,0x6950c4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4143e0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x44u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4143fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414444; continue; }
            pc = 0x414409; continue;
            }
            0x414409 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x41441a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x44u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414436); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414444);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x414444; continue;
            }
            0x414444 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6950c8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41445a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x44u32);
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414474); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6950d8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414488);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x44u32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4144a2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x6950e4u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4144b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4144c9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4144d7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x6950f0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4144ec);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414506); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414550; continue; }
            pc = 0x414515; continue;
            }
            0x414515 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x414526);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414542); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414550);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x414550; continue;
            }
            0x414550 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6950f4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414566);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414580); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695104u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414594);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4145ae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x695110u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4145bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x68u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4145d5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4145e3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            c.push(m,c.get(0,32));
            c.push(m,0x69511cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4145f8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x68u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414612); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x41465c; continue; }
            pc = 0x414621; continue;
            }
            0x414621 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x414632);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x68u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41464e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x41465c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x41465c; continue;
            }
            0x41465c => {
            c.push(m,0x695120u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x41466a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x7au32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414683); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414691);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.push(m,c.get(0,32));
            c.push(m,0x69512cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4146a9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x7au32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4146c3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414710; continue; }
            pc = 0x4146d5; continue;
            }
            0x4146d5 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x4146e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x7au32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414702); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414710);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x414710; continue;
            }
            0x414710 => {
            c.push(m,0x695130u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x41471e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x8cu32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41473a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414748);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13a),8));
            c.push(m,c.get(0,32));
            c.push(m,0x695138u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414760);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x8cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41477d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13a),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4147cd; continue; }
            pc = 0x41478f; continue;
            }
            0x41478f => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x4147a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x8cu32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4147bf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4147cd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4147cd; continue;
            }
            0x4147cd => {
            c.push(m,0x69513cu32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4147db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xcu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4147fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414808);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414855; continue; }
            pc = 0x414817; continue;
            }
            0x414817 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x414828);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xdu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414847); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414855);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x414855; continue;
            }
            0x414855 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4148a2; continue; }
            pc = 0x414864; continue;
            }
            0x414864 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x414875);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xeu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x168u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414894); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4148a2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4148a2; continue;
            }
            0x4148a2 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4148fc; continue; }
            pc = 0x4148b9; continue;
            }
            0x4148b9 => {
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[7].wrapping_add(0x38),8),8); }
            if c.cond(4) { pc = 0x4148fc; continue; }
            pc = 0x4148be; continue;
            }
            0x4148be => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m, 0x4148cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xfu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4148ee); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4148fc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4148fc; continue;
            }
            0x4148fc => {
            c.push(m,0x695144u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x41490a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x10u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x208u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414929); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414937);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69514cu32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x414945);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x11u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414964); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414972);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695154u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x414980);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x12u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x2d0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41499f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4149ad);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69515cu32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4149bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x13u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4149da); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4149e8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695168u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4149f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x14u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xa7u32);
            c.push(m,0x370u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414a15); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414a23);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x54));
            c.set(3,32,0xb9u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xffffffb0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x414a42; continue;
            }
            0x414a42 => {
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x414a53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x15u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414a6b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x414a79);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x414aac; continue; }
            pc = 0x414a83; continue;
            }
            0x414a83 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695178u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414a93);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414aac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x414aac; continue;
            }
            0x414aac => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414b54; continue; }
            pc = 0x414abf; continue;
            }
            0x414abf => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x4f4),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x414b01; continue; }
            pc = 0x414aec; continue;
            }
            0x414aec => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695180u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414afc);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x414b13; continue;
            }
            0x414b01 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695188u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414b10);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x414b13; continue;
            }
            0x414b13 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x2d0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414b29); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de3d8u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x414b3a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fstp64(m,c.r[4]);
            c.push(m,0x69518cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414b4f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x414b62; continue;
            }
            0x414b54 => {
            c.push(m,0x695192u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x414b62);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414b62; continue;
            }
            0x414b62 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414b78); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414c8a; continue; }
            pc = 0x414b8b; continue;
            }
            0x414b8b => {
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414b9f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ee90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x414bbf; continue; }
            pc = 0x414ba4; continue;
            }
            0x414ba4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x38),8));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x414bbd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ee90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x414bbf; continue;
            }
            0x414bbf => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695194u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414bcf);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x208u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414be8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x4f4),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x414c37; continue; }
            pc = 0x414c22; continue;
            }
            0x414c22 => {
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x69519cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x414c32);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x414c49; continue;
            }
            0x414c37 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6951a4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414c46);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x414c49; continue;
            }
            0x414c49 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414c5f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de3d8u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x414c70);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6951a8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414c85);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x414c98; continue;
            }
            0x414c8a => {
            c.push(m,0x6951aeu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x414c98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414c98; continue;
            }
            0x414c98 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x168u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414cae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x414dfe; continue; }
            pc = 0x414cbf; continue;
            }
            0x414cbf => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x414dda; continue; }
            pc = 0x414cd2; continue;
            }
            0x414cd2 => {
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414ce6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ee90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x414d09; continue; }
            pc = 0x414ceb; continue;
            }
            0x414ceb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x37),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x414d07);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ee90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x414d09; continue;
            }
            0x414d09 => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6951b0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414d19);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414d32); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x4f4),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x414d84; continue; }
            pc = 0x414d6f; continue;
            }
            0x414d6f => {
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6951b8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x414d7f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x414d96; continue;
            }
            0x414d84 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6951c0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414d93);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x414d96; continue;
            }
            0x414d96 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x370u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414dac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de3d8u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x13b),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x414dc0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6951c4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x414dd5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x414de8; continue;
            }
            0x414dda => {
            c.push(m,0x6951cau32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x414de8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414de8; continue;
            }
            0x414de8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414dfe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x414dfe; continue;
            }
            0x414dfe => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.add(a,0x12u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x414a42; continue; }
            pc = 0x414e21; continue;
            }
            0x414e21 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x414e32);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_trade_detail_table(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x416600;
    loop {
        match pc {
            0x416600 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bd5du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x60u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6de4a0u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            c.set(1,8,c.get(4,8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x6954bcu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41668c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x98),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4166aa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x4166c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4166e6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4166f7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6954ccu32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x416708);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416724); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416735);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6954d4u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x416746);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x82u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416765); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416776);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x84),8));
            c.set(7,32,0xbeu32);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4167b5; continue; }
            pc = 0x416786; continue;
            }
            0x416786 => {
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6954dcu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x416796);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4167ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x416786; continue; }
            pc = 0x4167b5; continue;
            }
            0x4167b5 => {
            c.push(m,0x6954e4u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x4167c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4167e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4167f2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6954e8u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m, 0x416806);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416821); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416832);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6954f0u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416846);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416861); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416872);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6954f8u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416886);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4168a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4168b2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695500u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x4168c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4168e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4168f2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695508u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x28u32,32); c.set(7,32,r); }
            c.push(m, 0x416906);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416921); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416932);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695510u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416946);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416961); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416972);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695514u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m, 0x416986);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4169a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4169b2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69551cu32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m, 0x4169c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xcu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4169e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4169f2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x416a7d; continue; }
            pc = 0x4169fd; continue;
            }
            0x4169fd => {
            c.push(m,0x695524u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x416a0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xdu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416a29); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416a3a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69552cu32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416a4e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xeu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416a69); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416a7a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            pc = 0x416a7d; continue;
            }
            0x416a7d => {
            c.push(m,0x695534u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x416a8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0xfu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416aa9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416aba);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,0x2bu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x4u32); }
            c.set(1,32,c.r[0].wrapping_add(0x886));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x416adb; continue;
            }
            0x416ad7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x416adb; continue;
            }
            0x416adb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xfffffffe),16));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x416afa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x10u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416b12); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x416b20);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c2c),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x416b49; continue; }
            pc = 0x416b3a; continue;
            }
            0x416b3a => {
            c.set(1,8,0x46u32);
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,0xc8u32); }
            pc = 0x416b56; continue;
            }
            0x416b49 => {
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,0x7d0u32); }
            c.set(1,8,0x4cu32);
            pc = 0x416b56; continue;
            }
            0x416b56 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,32,c.get(0,32));
            c.set(3,32,c.get(2,32));
            c.set(0,32,sx(c.get(1,8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x416b80);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x69553cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x416b91);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x82u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416baa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x84),8));
            { let a = c.get(0,32); let r = c.add(a,0x4fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,0xbeu32);
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x416c24; continue; }
            pc = 0x416bce; continue;
            }
            0x416bce => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            pc = 0x416bd6; continue;
            }
            0x416bd6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x84),32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x695544u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x416bfa);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416c0f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            if c.cond(12) { pc = 0x416bd6; continue; }
            pc = 0x416c24; continue;
            }
            0x416c24 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x416c5d; continue; }
            pc = 0x416c3b; continue;
            }
            0x416c3b => {
            if c.cond(12) { pc = 0x416c41; continue; }
            pc = 0x416c3d; continue;
            }
            0x416c3d => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x416c5d; continue; }
            pc = 0x416c41; continue;
            }
            0x416c41 => {
            c.push(m,0x0u32);
            c.push(m,0x3e8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x416c4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            pc = 0x416d76; continue;
            }
            0x416c5d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x416cb3; continue; }
            pc = 0x416c75; continue;
            }
            0x416c75 => {
            if c.cond(12) { pc = 0x416c7b; continue; }
            pc = 0x416c77; continue;
            }
            0x416c77 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x416cb3; continue; }
            pc = 0x416c7b; continue;
            }
            0x416c7b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(3,32); let r = c.sbb(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.adc(a,0x0u32,32); c.set(3,32,r); }
            c.push(m,0x3e8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x416c9b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            pc = 0x416d79; continue;
            }
            0x416cb3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x416d06; continue; }
            pc = 0x416ccb; continue;
            }
            0x416ccb => {
            if c.cond(12) { pc = 0x416cd1; continue; }
            pc = 0x416ccd; continue;
            }
            0x416ccd => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x416d06; continue; }
            pc = 0x416cd1; continue;
            }
            0x416cd1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(3,32); let r = c.sbb(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.adc(a,0x0u32,32); c.set(3,32,r); }
            c.push(m,0x3e8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x416cf1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            pc = 0x416d79; continue;
            }
            0x416d06 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x416d59; continue; }
            pc = 0x416d1e; continue;
            }
            0x416d1e => {
            if c.cond(12) { pc = 0x416d24; continue; }
            pc = 0x416d20; continue;
            }
            0x416d20 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x416d59; continue; }
            pc = 0x416d24; continue;
            }
            0x416d24 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(3,32); let r = c.sbb(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.adc(a,0x0u32,32); c.set(3,32,r); }
            c.push(m,0x3e8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x416d44);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            pc = 0x416d79; continue;
            }
            0x416d59 => {
            { let a = c.get(1,32); let r = c.add(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(3,32); let r = c.adc(a,0x0u32,32); c.set(3,32,r); }
            c.push(m,0x3e8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x416d6d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            pc = 0x416d76; continue;
            }
            0x416d76 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x416d79; continue;
            }
            0x416d79 => {
            c.push(m, 0x416d7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a6e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x69554cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416d8f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416da4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc4),32));
            c.push(m,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x84),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695550u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416dd3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416de8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x64),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x84),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695558u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416e10);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416e25); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x310),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x84),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695560u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416e50);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416e65); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.add(a,0x5au32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x490),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x84),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695568u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416e90);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416ea5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(3,32));
            c.push(m,0x695570u32);
            c.push(m,c.get(0,32));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416ebc);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416edf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x84),8));
            { let a = c.get(7,32); let r = c.add(a,0xb4u32,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x416f38; continue; }
            pc = 0x416ef0; continue;
            }
            0x416ef0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[0].wrapping_add(0xfffffffa));
            c.fstp32(m,c.r[4].wrapping_add(0x88));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x88),4));
            { let v = c.ld(m,0x66bc68u32,4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695578u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x416f20);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416f35); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x64u32,32); c.set(7,32,r); }
            pc = 0x416f38; continue;
            }
            0x416f38 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695580u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416f52);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416f67); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,c.r[5].wrapping_add(0xfffffffe));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m, 0x416f7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x416f85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.add(a,0x16u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x15u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x416ad7; continue; }
            pc = 0x416fa5; continue;
            }
            0x416fa5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x695588u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x416fb9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416fd2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,c.r[5].wrapping_add(0xfffffffe));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x416fe4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x416fed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x17u32);
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x7fu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x416ffd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,0x7fu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417007);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x84),8));
            c.set(7,32,0xbbu32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41703a; continue; }
            pc = 0x417017; continue;
            }
            0x417017 => {
            c.set(3,32,0x4u32);
            pc = 0x41701c; continue;
            }
            0x41701c => {
            c.push(m,0x17u32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41702b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417034);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x41701c; continue; }
            pc = 0x41703a; continue;
            }
            0x41703a => {
            c.push(m,0x17u32);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417049);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417052);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417064);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41706d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41707f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417088);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41709a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170be);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x28u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170d0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170eb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4170f4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417106);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41710f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417121);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41712a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x84),8));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41716e; continue; }
            pc = 0x417138; continue;
            }
            0x417138 => {
            c.push(m,0x17u32);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417147);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417150);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            c.push(m,0x17u32);
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417162);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41716b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            pc = 0x41716e; continue;
            }
            0x41716e => {
            c.push(m,0x17u32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41717d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417186);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x695590u32);
            c.set(1,32,c.r[4].wrapping_add(0x88));
            { let a = c.get(5,32); let r = c.add(a,0x28u32,32); c.set(5,32,r); }
            c.push(m, 0x41719a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x11u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4171b8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4171c9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695598u32);
            c.set(1,32,c.r[4].wrapping_add(0x88));
            c.push(m, 0x4171da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x12u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x12));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x26cu32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4171fb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x41720c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6955a0u32);
            c.set(1,32,c.r[4].wrapping_add(0x88));
            c.push(m, 0x41721d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x13u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x24));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x26cu32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41723e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x41724f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6955a4u32);
            c.set(1,32,c.r[4].wrapping_add(0x88));
            c.push(m, 0x417260);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x14u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x36));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x26cu32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417281); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x417292);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,0x4u32); }
            c.set(3,32,c.r[0].wrapping_add(0x308));
            c.set(7,32,c.r[0].wrapping_add(0x2d8));
            pc = 0x4172b5; continue;
            }
            0x4172b5 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6955acu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4172c7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2bcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4172ec); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0xfffffff8),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x44));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x44));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x44),4));
            { let v = c.ld(m,0x66bc58u32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6955b0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41731b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417334); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[3],16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6955b8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x417347);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417360); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x80),32));
            { let a = c.get(5,32); let r = c.add(a,0x12u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4172b5; continue; }
            pc = 0x41737e; continue;
            }
            0x41737e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x6955c0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x417392);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2bcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4173ab); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4173bc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x6cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_tile_map(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4173e0;
    loop {
        match pc {
            0x4173e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bd77u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x4176f6; continue; }
            pc = 0x41741a; continue;
            }
            0x41741a => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x417671; continue; }
            pc = 0x41745f; continue;
            }
            0x41745f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x417664; continue; }
            pc = 0x417469; continue;
            }
            0x417469 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x8u32); }
            pc = 0x417471; continue;
            }
            0x417471 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(5) { pc = 0x4174a0; continue; }
            pc = 0x417496; continue;
            }
            0x417496 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x2au32); }
            pc = 0x4175d2; continue;
            }
            0x4174a0 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x3fu32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(5) { pc = 0x4174be; continue; }
            pc = 0x4174aa; continue;
            }
            0x4174aa => {
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(2,8)); }
            pc = 0x4175d2; continue;
            }
            0x4174be => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x20u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(5) { pc = 0x4174fa; continue; }
            pc = 0x4174c8; continue;
            }
            0x4174c8 => {
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4174d6; continue; }
            pc = 0x4174cc; continue;
            }
            0x4174cc => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x74u32); }
            pc = 0x4175d2; continue;
            }
            0x4174d6 => {
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4174e4; continue; }
            pc = 0x4174da; continue;
            }
            0x4174da => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x38u32); }
            pc = 0x4175d2; continue;
            }
            0x4174e4 => {
            { let a = c.get(0,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4174f1; continue; }
            pc = 0x4174e8; continue;
            }
            0x4174e8 => {
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0x34u32,8); c.set(0,8,r); }
            pc = 0x4175ce; continue;
            }
            0x4174f1 => {
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0x30u32,8); c.set(0,8,r); }
            pc = 0x4175ce; continue;
            }
            0x4174fa => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1fu32,32); }
            if c.cond(7) { pc = 0x4175bf; continue; }
            pc = 0x417508; continue;
            }
            0x417508 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x417708)); continue;
            }
            0x41750f => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x6bu32); }
            pc = 0x4175c4; continue;
            }
            0x417519 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x70u32); }
            pc = 0x4175c4; continue;
            }
            0x417523 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x6eu32); }
            pc = 0x4175c4; continue;
            }
            0x41752d => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x61u32); }
            pc = 0x4175c4; continue;
            }
            0x417537 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x73u32); }
            pc = 0x4175c4; continue;
            }
            0x417541 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x6du32); }
            pc = 0x4175c4; continue;
            }
            0x417548 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x23u32); }
            pc = 0x4175c4; continue;
            }
            0x41754f => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x6cu32); }
            pc = 0x4175c4; continue;
            }
            0x417556 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x72u32); }
            pc = 0x4175c4; continue;
            }
            0x41755d => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x71u32); }
            pc = 0x4175c4; continue;
            }
            0x417564 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x69u32); }
            pc = 0x4175c4; continue;
            }
            0x41756b => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x68u32); }
            pc = 0x4175c4; continue;
            }
            0x417572 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x7eu32); }
            pc = 0x4175c4; continue;
            }
            0x417579 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x2au32); }
            pc = 0x4175c4; continue;
            }
            0x417580 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x66u32); }
            pc = 0x4175c4; continue;
            }
            0x417587 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x26u32); }
            pc = 0x4175c4; continue;
            }
            0x41758e => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x25u32); }
            pc = 0x4175c4; continue;
            }
            0x417595 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x79u32); }
            pc = 0x4175c4; continue;
            }
            0x41759c => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x75u32); }
            pc = 0x4175c4; continue;
            }
            0x4175a3 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x64u32); }
            pc = 0x4175c4; continue;
            }
            0x4175aa => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x63u32); }
            pc = 0x4175c4; continue;
            }
            0x4175b1 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x62u32); }
            pc = 0x4175c4; continue;
            }
            0x4175b8 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x67u32); }
            pc = 0x4175c4; continue;
            }
            0x4175bf => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x3fu32); }
            pc = 0x4175c4; continue;
            }
            0x4175c4 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x4175d2; continue; }
            pc = 0x4175c8; continue;
            }
            0x4175c8 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            { let a = c.get(0,8); let r = c.add(a,0xe0u32,8); c.set(0,8,r); }
            pc = 0x4175ce; continue;
            }
            0x4175ce => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(0,8)); }
            pc = 0x4175d2; continue;
            }
            0x4175d2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4175e2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ca70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xfffffffeu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfffffff0u32,32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x1u32); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x5c),32); c.push(m, 0x417632); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x0u32); }
            c.push(m, 0x417640);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x417471; continue; }
            pc = 0x417660; continue;
            }
            0x417660 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x417664; continue;
            }
            0x417664 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x41745f; continue; }
            pc = 0x417671; continue;
            }
            0x417671 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x6de4e0),8));
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x6955c4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x417697);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4176ae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x6955dcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4176cb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0xau32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4176e2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4176f3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4176f6; continue;
            }
            0x4176f6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_ware_stock_production(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x417790;
    loop {
        match pc {
            0x417790 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bd99u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4177e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4177ff); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x41780d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x6955ecu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x417822);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417839); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,0x28u32);
            c.set(3,32,0x6c0bb0u32);
            { let a = c.get(5,32); let r = c.add(a,0xc4u32,32); c.set(5,32,r); }
            pc = 0x417849; continue;
            }
            0x417849 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x417855);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41786d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x41787b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff40),32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x6955f0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x417891);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4178a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x6955f4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4178ba);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xfau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4178d3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x24c),32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x6955f8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4178e9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417902); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,0x6c0c10u32,32); }
            if c.cond(12) { pc = 0x417849; continue; }
            pc = 0x417917; continue;
            }
            0x417917 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x417928);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_price_threshold_chart(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x418120;
    loop {
        match pc {
            0x418120 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65be07u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x418150; continue; }
            pc = 0x41814e; continue;
            }
            0x41814e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x418150; continue;
            }
            0x418150 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m, 0x41817b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41819b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x60));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4181ac);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x22u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(5,32)); }
            c.set(3,32,0x20u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x6c0bb0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4fcu32,32); c.set(7,32,r); }
            pc = 0x4181cf; continue;
            }
            0x4181cf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m, 0x4181df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4181fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41820b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41821a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418227);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x4183f9; continue; }
            pc = 0x418235; continue;
            }
            0x418235 => {
            c.fild32(m,c.r[4].wrapping_add(0x60));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffff4),32));
            c.fstp32(m,c.r[4].wrapping_add(0x60));
            c.fld(c.ld(m,0x66bc80u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('/',v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x60));
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('*',v); }
            c.push(m, 0x418270);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.get(5,32); let r = c.add(a,0x64u32,32); c.set(5,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418283);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[3].wrapping_add(0x14));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x41828f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffff8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('*',v); }
            c.push(m, 0x4182b4);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.get(5,32); let r = c.add(a,0x64u32,32); c.set(5,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4182c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[3].wrapping_add(0x14));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4182d3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffffc),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('*',v); }
            c.push(m, 0x4182f8);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.get(5,32); let r = c.add(a,0x64u32,32); c.set(5,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41830b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[3].wrapping_add(0x14));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x418317);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('*',v); }
            c.push(m, 0x41833f);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,0x69567cu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(5,32); let r = c.add(a,0x64u32,32); c.set(5,32,r); }
            c.push(m, 0x418352);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418370); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x418381);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xfffffffc),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xfffffff8),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x60),4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x60));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x60),4));
            { let v = c.ld(m,0x66bc7cu32,4); c.fop('*',v); }
            c.push(m, 0x4183b1);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x64u32);
            c.push(m,0x69567eu32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m, 0x4183c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4183e4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4183f5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            pc = 0x4183f9; continue;
            }
            0x4183f9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6c0c10u32,32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4181cf; continue; }
            pc = 0x418427; continue;
            }
            0x418427 => {
            { let a = c.get(5,32); let r = c.sub(a,0x2u32,32); c.set(5,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.set(7,32,c.r[5].wrapping_add(0xfffffffe));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x41843c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418449);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x22u32);
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,0x64u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418459);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418463);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x22u32);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x384u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418476);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x418483);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_ware_town_grid_header(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4184a0;
    loop {
        match pc {
            0x4184a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65be29u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x695680u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4184de);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2du32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4184f9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(5,32,0x64u32);
            c.set(7,32,0x6c0bb0u32);
            pc = 0x418503; continue;
            }
            0x418503 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x41850f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418527); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x418534);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x1eu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6c0c10u32,32); }
            if c.cond(12) { pc = 0x418503; continue; }
            pc = 0x418542; continue;
            }
            0x418542 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(5,32,0x46u32);
            if c.cond(6) { pc = 0x418595; continue; }
            pc = 0x418552; continue;
            }
            0x418552 => {
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x418563);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41857b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x418588);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(5,32); let r = c.add(a,0x12u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x418552; continue; }
            pc = 0x418595; continue;
            }
            0x418595 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4185a6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_mission_event_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4185c0;
    loop {
        match pc {
            0x4185c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65be63u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dd744u32,32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x695684u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x41860c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418629); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x418637);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69568cu32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x418645);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x2du32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41865e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x41866c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695690u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x41867a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x5au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418693); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4186a1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695698u32);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4186af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x82u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4186cb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x4186d9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd748u32,32));
            c.set(7,32,0x1eu32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x41883f; continue; }
            pc = 0x4186f0; continue;
            }
            0x4186f0 => {
            c.push(m,c.get(5,32));
            pc = 0x4186f7; continue;
            }
            0x4186f3 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4186f7; continue;
            }
            0x4186f7 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x418702);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,0x6719f361u32);
            c.set(1,32,rm(m,c.r[5],32));
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(3,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(3,32));
            c.push(m,0x6956a0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418729);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41873f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(3,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6956a4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x418763);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2du32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418779); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6956a8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418791);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x5au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4187a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x6956acu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4187bb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x82u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4187d4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x6),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x693990),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x4187eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418806); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            c.push(m, 0x418814);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dd748u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0x11u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x4186f3; continue; }
            pc = 0x41883e; continue;
            }
            0x41883e => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x41883f; continue;
            }
            0x41883f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,0x6956b0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x418853);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41886d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,0x6dd746u32,16));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x6956c0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418892);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1bu32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4188ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6dd748u32,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6956d4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4188c6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4188e0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,0x6dd746u32,16));
            c.set(2,32,rm(m,0x6dd73cu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,0x3du32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd748u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x41896b; continue; }
            pc = 0x41891b; continue;
            }
            0x41891b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6956e8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41892b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418944); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6dd73cu32,32));
            c.set(2,32,rm(m,0x6dd748u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x11u32,32); c.set(3,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x41891b; continue; }
            pc = 0x41896b; continue;
            }
            0x41896b => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41897c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_ship_details_and_pirate_nests(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x418990;
    loop {
        match pc {
            0x418990 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65be75u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc2),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x418c08; continue; }
            pc = 0x4189d5; continue;
            }
            0x4189d5 => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6956f4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4189ff);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418a15); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x1c),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x695704u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418a2e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418a47); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x20),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x695710u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418a60);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x140u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418a79); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m,0x69571cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418a8f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418aa8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695728u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418ac1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x208u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418ada); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x120),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695734u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418af0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            c.push(m,0xa0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418b0a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x154),16));
            c.push(m,c.get(0,32));
            c.push(m,0x69573cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418b23);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            c.push(m,0x10eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418b3d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x3f),8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695744u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418b51);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418b6b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x69574cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x418b79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x418b7b; continue;
            }
            0x418b7b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[3]).wrapping_add(0x13c),8));
            { let a = c.get(0,32); c.sub(a,0xffu32,32); }
            if c.cond(15) { pc = 0x418bd8; continue; }
            pc = 0x418b8b; continue;
            }
            0x418b8b => {
            if c.cond(4) { pc = 0x418bd1; continue; }
            pc = 0x418b8d; continue;
            }
            0x418b8d => {
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x418bd8; continue; }
            pc = 0x418b92; continue;
            }
            0x418b92 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x41923c)); continue;
            }
            0x418b99 => {
            c.push(m,0x695756u32);
            pc = 0x418bdd; continue;
            }
            0x418ba0 => {
            c.push(m,0x695758u32);
            pc = 0x418bdd; continue;
            }
            0x418ba7 => {
            c.push(m,0x69575au32);
            pc = 0x418bdd; continue;
            }
            0x418bae => {
            c.push(m,0x69575cu32);
            pc = 0x418bdd; continue;
            }
            0x418bb5 => {
            c.push(m,0x69575eu32);
            pc = 0x418bdd; continue;
            }
            0x418bbc => {
            c.push(m,0x695760u32);
            pc = 0x418bdd; continue;
            }
            0x418bc3 => {
            c.push(m,0x695762u32);
            pc = 0x418bdd; continue;
            }
            0x418bca => {
            c.push(m,0x695764u32);
            pc = 0x418bdd; continue;
            }
            0x418bd1 => {
            c.push(m,0x695766u32);
            pc = 0x418bdd; continue;
            }
            0x418bd8 => {
            c.push(m,0x695768u32);
            pc = 0x418bdd; continue;
            }
            0x418bdd => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x418be6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x418b7b; continue; }
            pc = 0x418bec; continue;
            }
            0x418bec => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            c.push(m,0x1e0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418c03); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,0x22u32);
            pc = 0x418c08; continue;
            }
            0x418c08 => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x418c0c; continue;
            }
            0x418c0c => {
            c.set(7,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(3,32); let r = c.add(a,0x5u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4191de; continue; }
            pc = 0x418c1e; continue;
            }
            0x418c1e => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x16),16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb4),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb0),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x69576cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418c6e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418c84); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x418c8d);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x418ca0; continue; }
            pc = 0x418c92; continue;
            }
            0x418c92 => {
            c.push(m,0x695798u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x418ca0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x418ca0; continue;
            }
            0x418ca0 => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x418cfe; continue; }
            pc = 0x418ca7; continue;
            }
            0x418ca7 => {
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x418cfa; continue; }
            pc = 0x418cae; continue;
            }
            0x418cae => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x418cbc; continue; }
            pc = 0x418cb5; continue;
            }
            0x418cb5 => {
            c.push(m,0x6957a0u32);
            pc = 0x418cd7; continue;
            }
            0x418cbc => {
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x418cc7; continue; }
            pc = 0x418cc0; continue;
            }
            0x418cc0 => {
            c.push(m,0x6957a4u32);
            pc = 0x418cd7; continue;
            }
            0x418cc7 => {
            { let a = c.get(0,8); c.sub(a,0x1bu32,8); }
            if c.cond(5) { pc = 0x418cd2; continue; }
            pc = 0x418ccb; continue;
            }
            0x418ccb => {
            c.push(m,0x6957a8u32);
            pc = 0x418cd7; continue;
            }
            0x418cd2 => {
            c.push(m,0x6957acu32);
            pc = 0x418cd7; continue;
            }
            0x418cd7 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x418ce0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x418cfa; continue; }
            pc = 0x418cf0; continue;
            }
            0x418cf0 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(5,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x418cae; continue; }
            pc = 0x418cfa; continue;
            }
            0x418cfa => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x418cfe; continue;
            }
            0x418cfe => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418d14); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.add(a,0x11u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x419207; continue; }
            pc = 0x418d37; continue;
            }
            0x418d37 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x160));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x6957b0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x418d5c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418d72); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1c),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6957b8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418d8d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xa0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418da6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x20),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6957c4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418dc1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x10eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418dda); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6957d0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418df0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x17cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418e09); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6957d8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418e22);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1ccu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418e3b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x6957e0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x418e52);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1feu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418e6b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6957ecu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418e81);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418e9a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6957f4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418eb0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x2b2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418ec9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6957fcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418edf);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x2f8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418ef8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x418eff);
            { let args = []; c.r[0] = host.call(m, 0x5185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695804u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418f16);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x33eu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418f2f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x118),32));
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x695810u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418f56);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x398u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418f6f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x11c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x118),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x695818u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x418fa3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x3cau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x418fbc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x419025; continue; }
            pc = 0x418fc9; continue;
            }
            0x418fc9 => {
            c.set(5,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x3),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x418feb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x418ffc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x695820u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41900c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x3fcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419025); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419025; continue;
            }
            0x419025 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x120),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695828u32);
            c.push(m,c.get(1,32));
            { let a = c.get(3,32); let r = c.add(a,0x11u32,32); c.set(3,32,r); }
            c.push(m, 0x41903e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xa0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419057); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x154),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695830u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419070);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xf0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419089); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15d),8));
            c.push(m,c.get(0,32));
            c.push(m,0x695838u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4190a1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x140u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4190ba); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x159),8));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x695840u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4190de);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4190f7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x3f),8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695848u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41910b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1e0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419124); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x695850u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x419132);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x419134; continue;
            }
            0x419134 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x13c),8));
            { let a = c.get(0,32); c.sub(a,0xffu32,32); }
            if c.cond(15) { pc = 0x419191; continue; }
            pc = 0x419144; continue;
            }
            0x419144 => {
            if c.cond(4) { pc = 0x41918a; continue; }
            pc = 0x419146; continue;
            }
            0x419146 => {
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x419191; continue; }
            pc = 0x41914b; continue;
            }
            0x41914b => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x41925c)); continue;
            }
            0x419152 => {
            c.push(m,0x69585au32);
            pc = 0x419196; continue;
            }
            0x419159 => {
            c.push(m,0x69585cu32);
            pc = 0x419196; continue;
            }
            0x419160 => {
            c.push(m,0x69585eu32);
            pc = 0x419196; continue;
            }
            0x419167 => {
            c.push(m,0x695860u32);
            pc = 0x419196; continue;
            }
            0x41916e => {
            c.push(m,0x695862u32);
            pc = 0x419196; continue;
            }
            0x419175 => {
            c.push(m,0x695864u32);
            pc = 0x419196; continue;
            }
            0x41917c => {
            c.push(m,0x695866u32);
            pc = 0x419196; continue;
            }
            0x419183 => {
            c.push(m,0x695868u32);
            pc = 0x419196; continue;
            }
            0x41918a => {
            c.push(m,0x69586au32);
            pc = 0x419196; continue;
            }
            0x419191 => {
            c.push(m,0x69586cu32);
            pc = 0x419196; continue;
            }
            0x419196 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x41919f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x419134; continue; }
            pc = 0x4191a5; continue;
            }
            0x4191a5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x230u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4191bb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x11u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x418d37; continue; }
            pc = 0x4191d8; continue;
            }
            0x4191d8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x419207; continue;
            }
            0x4191de => {
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695870u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4191ee);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419204); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x419207; continue;
            }
            0x419207 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x418c0c; continue; }
            pc = 0x419215; continue;
            }
            0x419215 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x419226);
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

pub fn debug_page_player_reputation_by_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x419280;
    loop {
        match pc {
            0x419280 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65becfu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x4192dd; continue; }
            pc = 0x4192ba; continue;
            }
            0x4192ba => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4192cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4192dd => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4192eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.push(m,0x695880u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[5].wrapping_add(0x11c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x419309);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419326); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x419334);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695888u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x419342);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41935b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x419369);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695890u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x419377);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419390); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x41939e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69589cu32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x4193ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0xfau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4193c8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x4193d6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6958a8u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x4193e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419400); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x41940e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6958b0u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x41941c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,0x226u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419438); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x419446);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(7,32,0x32u32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x419533; continue; }
            pc = 0x419458; continue;
            }
            0x419458 => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m, 0x419469);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419481); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x41948f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x64u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x3u32); }
            pc = 0x41949c; continue;
            }
            0x41949c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[0],4));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x20));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6958c0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4194c6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4194db); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,0x96u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x41949c; continue; }
            pc = 0x4194ec; continue;
            }
            0x4194ec => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4194f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f9580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6958c8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419509);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41951e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.add(a,0x11u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x419458; continue; }
            pc = 0x41952f; continue;
            }
            0x41952f => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x419533; continue;
            }
            0x419533 => {
            c.push(m,0x6958ccu32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x419541);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419559); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x419567);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x41956e);
            { let args = []; c.r[0] = host.call(m, 0x4f73d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6958e0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419581);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x226u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41959a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x6958e4u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.get(7,32); let r = c.add(a,0x11u32,32); c.set(7,32,r); }
            c.push(m, 0x4195ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4195c3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            c.push(m, 0x4195d1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6958f8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4195e5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xfau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4195fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41960f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_convoy_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x419630;
    loop {
        match pc {
            0x419630 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bee9u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6dd88cu32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6958fcu32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x41969a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4196b5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,0x1eu32);
            c.set(0,16,rm(m,0x6dd896u32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4198c9; continue; }
            pc = 0x4196ca; continue;
            }
            0x4196ca => {
            c.push(m,c.get(5,32));
            pc = 0x4196cb; continue;
            }
            0x4196cb => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x69590cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4196f0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419706); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x419730; continue; }
            pc = 0x419713; continue;
            }
            0x419713 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            pc = 0x419735; continue;
            }
            0x419730 => {
            c.set(1,32,0x695928u32);
            pc = 0x419735; continue;
            }
            0x419735 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x695938u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41974b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419764); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x1c),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x41977d; continue; }
            pc = 0x419774; continue;
            }
            0x419774 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            pc = 0x419782; continue;
            }
            0x41977d => {
            c.set(0,32,0x695948u32);
            pc = 0x419782; continue;
            }
            0x419782 => {
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x12),16));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x69594cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419799);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1c2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4197b2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.and(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            c.set(0,32,0x69595cu32);
            if c.cond(4) { pc = 0x4197c5; continue; }
            pc = 0x4197c0; continue;
            }
            0x4197c0 => {
            c.set(0,32,0x695964u32);
            pc = 0x4197c5; continue;
            }
            0x4197c5 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4197cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4197ea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4197f8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x16),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695970u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41980e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419827); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(3,32); let r = c.add(a,0x10u32,32); c.set(3,32,r); }
            c.push(m, 0x419833);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x41989c; continue; }
            pc = 0x419847; continue;
            }
            0x419847 => {
            c.push(m,0x5bu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x419852);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x41986f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x69597cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x41987d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x419847; continue; }
            pc = 0x41989c; continue;
            }
            0x41989c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x32u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4198af); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(0,16,rm(m,0x6dd896u32,16));
            { let a = c.get(3,32); let r = c.add(a,0x10u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4196cb; continue; }
            pc = 0x4198c8; continue;
            }
            0x4198c8 => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4198c9; continue;
            }
            0x4198c9 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4198da);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn debug_page_town_info_history(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4198f0;
    loop {
        match pc {
            0x4198f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bf5bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc0),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x419e9e; continue; }
            pc = 0x419926; continue;
            }
            0x419926 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(3,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4a2u32,16));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            c.set(2,8,c.get(4,8));
            c.push(m,c.get(1,32));
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xfu32,32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            c.push(m,0x695980u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x419999);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4199b4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x4199ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0xdcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4199ea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x4199f8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695990u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419a06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419a1f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x419a2d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x20000u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0x1fu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x20000u32,32); }
            if c.cond(5) { pc = 0x419ac7; continue; }
            pc = 0x419a49; continue;
            }
            0x419a49 => {
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            if c.cond(13) { pc = 0x419a8e; continue; }
            pc = 0x419a52; continue;
            }
            0x419a52 => {
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x419a5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419a7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419a89);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419b51; continue;
            }
            0x419a8e => {
            c.push(m,0x69599cu32);
            c.push(m, 0x419a98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419ab4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419ac2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419b51; continue;
            }
            0x419ac7 => {
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x419b02; continue; }
            pc = 0x419acc; continue;
            }
            0x419acc => {
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x6959acu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419ae6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419b00); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419b3a; continue;
            }
            0x419b02 => {
            c.push(m,0x6959b8u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419b10);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419b2c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419b3a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419b3a; continue;
            }
            0x419b3a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419b51); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419b51; continue;
            }
            0x419b51 => {
            c.push(m,0x6959bcu32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419b5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419b78); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419b86);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de588u32);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc0),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x419b9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x535300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x20000u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x20000u32,32); }
            if c.cond(5) { pc = 0x419c34; continue; }
            pc = 0x419bb2; continue;
            }
            0x419bb2 => {
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x419bf7; continue; }
            pc = 0x419bb7; continue;
            }
            0x419bb7 => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419bc8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419be4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419bf2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419cc7; continue;
            }
            0x419bf7 => {
            c.push(m,0x6959c8u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419c05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419c21); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419c2f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419cc7; continue;
            }
            0x419c34 => {
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x419c78; continue; }
            pc = 0x419c39; continue;
            }
            0x419c39 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c8),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6959d8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x419c5c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419c76); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419cb0; continue;
            }
            0x419c78 => {
            c.push(m,0x6959e4u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x419c86);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419ca2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419cb0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419cb0; continue;
            }
            0x419cb0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419cc7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419cc7; continue;
            }
            0x419cc7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de588u32);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc0),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x419cdb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5353c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6959e8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x419ceb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            c.push(m,0x1c2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419d05); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            pc = 0x419d20; continue;
            }
            0x419d18 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x419d20; continue;
            }
            0x419d20 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc0),16));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de588u32);
            c.push(m, 0x419d3b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5353a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            c.set(5,32,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(5,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            c.push(m,0x6959f8u32);
            c.push(m,c.get(0,32));
            { let a = c.get(5,32); let r = c.and(a,0x1fu32,32); c.set(5,32,r); }
            c.push(m, 0x419d61);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419d77); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.and(a,0x20000u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x20000u32,32); }
            if c.cond(5) { pc = 0x419dee; continue; }
            pc = 0x419d85; continue;
            }
            0x419d85 => {
            { let a = c.get(5,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x419dbf; continue; }
            pc = 0x419d8a; continue;
            }
            0x419d8a => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x419d9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419db6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            pc = 0x419e62; continue;
            }
            0x419dbf => {
            c.push(m,0x695a08u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x419dcd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419de8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            pc = 0x419e62; continue;
            }
            0x419dee => {
            { let a = c.get(5,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x419e35; continue; }
            pc = 0x419df3; continue;
            }
            0x419df3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x2c8),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(2,32));
            c.push(m,0x695a18u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x419e1a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419e33); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x419e6c; continue;
            }
            0x419e35 => {
            c.push(m,0x695a24u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x419e43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0xcu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419e5e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            pc = 0x419e62; continue;
            }
            0x419e62 => {
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x419e6c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x419e6c; continue;
            }
            0x419e6c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,0x13u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x419d18; continue; }
            pc = 0x419e8a; continue;
            }
            0x419e8a => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x419e9b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x419e9e; continue;
            }
            0x419e9e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_treasury_ledger(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x419eb0;
    loop {
        match pc {
            0x419eb0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65bf8du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x695a28u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x695a38u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x695a44u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x695a54u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x695a64u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x695a74u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x695a7cu32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x695a88u32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x695a94u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x695aa4u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x695ab0u32); }
            if c.cond(3) { pc = 0x41a1e8; continue; }
            pc = 0x419f3a; continue;
            }
            0x419f3a => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x695abcu32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x419fa2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419fbd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x5c));
            c.push(m, 0x419fd7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0xdcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x419ff3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x41a000);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695accu32);
            c.set(1,32,c.r[4].wrapping_add(0x5c));
            c.push(m, 0x41a00e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0x32u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a027); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x41a034);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x700),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695ad4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41a04a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x19u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a064); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,0x4bu32);
            { let a = c.get(5,32); let r = c.add(a,0x730u32,32); c.set(5,32,r); }
            pc = 0x41a06f; continue;
            }
            0x41a06f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1c),32));
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.push(m, 0x41a07d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x32u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a095); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            c.push(m, 0x41a0a3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x695ad8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41a0b6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a0cf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695adcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41a0e2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a0fb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x19u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            if c.cond(13) { pc = 0x41a11f; continue; }
            pc = 0x41a103; continue;
            }
            0x41a103 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x41a139; continue;
            }
            0x41a11f => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x41a139; continue;
            }
            0x41a139 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,0xbu32,32); }
            if c.cond(12) { pc = 0x41a06f; continue; }
            pc = 0x41a146; continue;
            }
            0x41a146 => {
            c.push(m,0x695ae0u32);
            c.set(1,32,c.r[4].wrapping_add(0x5c));
            c.push(m, 0x41a154);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x32u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a16c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            c.push(m, 0x41a17a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695ae8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41a18e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a1a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x695aecu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41a1bb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x104u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41a1d4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41a1e5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x41a1e8; continue;
            }
            0x41a1e8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_cmd_send_test_letter_type_0x45(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41b0e0;
    loop {
        match pc {
            0x41b0e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c13bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x41b105);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x41b112);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.and(a,0x7fu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x63u32,32); }
            if c.cond(14) { pc = 0x41b11f; continue; }
            pc = 0x41b11c; continue;
            }
            0x41b11c => {
            { let a = c.get(6,32); let r = c.sub(a,0x64u32,32); c.set(6,32,r); }
            pc = 0x41b11f; continue;
            }
            0x41b11f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc0),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x45u32); }
            { let a = c.r[4].wrapping_add(0xd); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,0x0u32); }
            c.push(m, 0x41b13a);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[6].wrapping_mul(8).wrapping_add(0x66b0b0),16));
            c.set(2,16,rm(m,c.r[6].wrapping_mul(8).wrapping_add(0x66b0b4),16));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x41b172);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41b183);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_cmd_send_test_letter_type_0x21(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41b420;
    loop {
        match pc {
            0x41b420 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c14du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x41b442);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x21u32); }
            c.push(m, 0x41b45a);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(6,32); c.div32(b); }
            c.set(6,32,rm(m,0x6ddb70u32,32));
            { let a = c.r[4].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x41b46d);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(6,32,rm(m,0x6ddb74u32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x41b486);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x41b499);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x6u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            { let a = c.get(2,32); let r = c.add(a,0x3u32,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.add(a,0xeu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x41b4d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41b4e9);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_window_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41bfe0;
    loop {
        match pc {
            0x41bfe0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x41bfe8);
            { let args = []; c.r[0] = host.call(m, 0x4104d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x41bff8; continue; }
            pc = 0x41bfef; continue;
            }
            0x41bfef => {
            c.push(m,c.get(6,32));
            c.push(m, 0x41bff5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x41bff8; continue;
            }
            0x41bff8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_helper_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41c000;
    loop {
        match pc {
            0x41c000 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x41c008);
            { let args = []; c.r[0] = host.call(m, 0x41c020, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x41c018; continue; }
            pc = 0x41c00f; continue;
            }
            0x41c00f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x41c015);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x41c018; continue;
            }
            0x41c018 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_info_helper_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41c020;
    loop {
        match pc {
            0x41c020 => {
            { let a = c.r[1]; wm(m,a,32,0x67c9a4u32); }
            { let args = []; c.r[0] = host.call(m, 0x65366f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_box_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c7f50;
    loop {
        match pc {
            0x4c7f50 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4c7f59);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4c7f6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x180));
            c.push(m, 0x4c7f76);
            { let args = []; c.r[0] = host.call(m, 0x4afe40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x62c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c7fbd; continue; }
            pc = 0x4c7f82; continue;
            }
            0x4c7f82 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x630),32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4c7fad; continue; }
            pc = 0x4c7f8f; continue;
            }
            0x4c7f8f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x62c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c7fa2; continue; }
            pc = 0x4c7f9c; continue;
            }
            0x4c7f9c => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x4c7fa2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c7fa2; continue;
            }
            0x4c7fa2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x630),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4c7f8f; continue; }
            pc = 0x4c7fad; continue;
            }
            0x4c7fad => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x62c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c7fb9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4c7fbd; continue;
            }
            0x4c7fbd => {
            { let a = c.r[6].wrapping_add(0x62c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x630); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x634); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x638); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x63c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x628); wm(m,a,32,0xff80f300u32); }
            { let a = c.r[6].wrapping_add(0x17c); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_box_clear_dirty_and_forward_to_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c8b70;
    loop {
        match pc {
            0x4c8b70 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4c8b78);
            { let args = []; c.r[0] = host.call(m, 0x4b2410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(1,32,c.r[6].wrapping_add(0x180));
            { let v = c.pop(m); c.set(6,32,v); }
            { let target = rm(m,c.r[0].wrapping_add(0xf8),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_box_add_dirty_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c90d0;
    loop {
        match pc {
            0x4c90d0 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c90de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x180),32));
            { let a = c.get(6,32); let r = c.add(a,0x180u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4c90f2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4c9101; continue; }
            pc = 0x4c90f6; continue;
            }
            0x4c90f6 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c90fd);
            { let args = []; c.r[0] = host.call(m, 0x4b2220, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c910c; continue; }
            pc = 0x4c9101; continue;
            }
            0x4c9101 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xa0),32); c.push(m, 0x4c910c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c910c; continue;
            }
            0x4c910c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_box_get_total_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c9160;
    loop {
        match pc {
            0x4c9160 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x544),32));
            c.set(1,32,c.r[3].wrapping_add(0x544));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4c917c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x180),32));
            c.set(7,32,c.r[3].wrapping_add(0x180));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x4c9190); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c91a2; continue; }
            pc = 0x4c9194; continue;
            }
            0x4c9194 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4c91a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c91b3; continue;
            }
            0x4c91a2 => {
            c.push(m,0xcu32);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4c91b0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4c91b3; continue;
            }
            0x4c91b3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4c91bf; continue; }
            pc = 0x4c91bd; continue;
            }
            0x4c91bd => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c91bf; continue;
            }
            0x4c91bf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4c91d9; continue; }
            pc = 0x4c91d6; continue;
            }
            0x4c91d6 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c91d9; continue;
            }
            0x4c91d9 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x94),32));
            c.set(1,32,c.r[3].wrapping_add(0x94));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c91ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4c91f9; continue; }
            pc = 0x4c91f7; continue;
            }
            0x4c91f7 => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c91f9; continue;
            }
            0x4c91f9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4c9207; continue; }
            pc = 0x4c9204; continue;
            }
            0x4c9204 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c9207; continue;
            }
            0x4c9207 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4c9215; continue; }
            pc = 0x4c9212; continue;
            }
            0x4c9212 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c9215; continue;
            }
            0x4c9215 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(14) { pc = 0x4c921b; continue; }
            pc = 0x4c9219; continue;
            }
            0x4c9219 => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c921b; continue;
            }
            0x4c921b => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(14) { pc = 0x4c9223; continue; }
            pc = 0x4c9220; continue;
            }
            0x4c9220 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c9223; continue;
            }
            0x4c9223 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(14) { pc = 0x4c922b; continue; }
            pc = 0x4c9228; continue;
            }
            0x4c9228 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c922b; continue;
            }
            0x4c922b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_box_set_size_from_triple(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c9240;
    loop {
        match pc {
            0x4c9240 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x4c925a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(1,32,c.r[6].wrapping_add(0x180));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4c926a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn edit_box_load_from_ini(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c9a70;
    loop {
        match pc {
            0x4c9a70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6623e6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,0x7fffffffu32);
            { let a = c.r[6].wrapping_add(0x168); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x16c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x170); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x179); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x140); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x144); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x148); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x14c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x150); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x154); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x670f24u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x158); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15b),8));
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x178); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x17b); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x160); wm(m,a,32,0x3e8u32); }
            c.push(m, 0x4c9b0d);
            { let args = []; c.r[0] = host.call(m, 0x4d2100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4c9b23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c9b2a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d21f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6c0040u32,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x1u32); }
            c.push(m, 0x4c9b4e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4c9b7b; continue; }
            pc = 0x4c9b59; continue;
            }
            0x4c9b59 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c9b6a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c9b7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4c9b7b; continue;
            }
            0x4c9b7b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,0x670f24u32,32));
            c.set(0,32,rm(m,0x6bfa98u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c9b9c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            { let a = c.r[6].wrapping_add(0x158); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let n = 0x18u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4c9be2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4c9bf5; continue; }
            pc = 0x4c9bea; continue;
            }
            0x4c9bea => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4c9bf1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4c9c21; continue; }
            pc = 0x4c9bf5; continue;
            }
            0x4c9bf5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c9c1d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x4c9c21; continue;
            }
            0x4c9c21 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x14c); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0x150); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4c9c3b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x44),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4c9d46; continue; }
            pc = 0x4c9c45; continue;
            }
            0x4c9c45 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,0x6bfab4u32,32));
            c.set(2,32,rm(m,0x6bfaa8u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c9c71);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c9d46; continue; }
            pc = 0x4c9c79; continue;
            }
            0x4c9c79 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6bfaccu32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4c9c92);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4c9d46; continue; }
            pc = 0x4c9c9e; continue;
            }
            0x4c9c9e => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4c9cb0; continue; }
            pc = 0x4c9cab; continue;
            }
            0x4c9cab => {
            c.set(5,32,0x1u32);
            pc = 0x4c9cb0; continue;
            }
            0x4c9cb0 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4c9ce1; continue; }
            pc = 0x4c9cb5; continue;
            }
            0x4c9cb5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c9cdd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x4c9ce1; continue;
            }
            0x4c9ce1 => {
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(13) { pc = 0x4c9cf1; continue; }
            pc = 0x4c9ce9; continue;
            }
            0x4c9ce9 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            pc = 0x4c9cf1; continue;
            }
            0x4c9cf1 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4c9d00; continue; }
            pc = 0x4c9cf5; continue;
            }
            0x4c9cf5 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x4c9cfc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4c9d2c; continue; }
            pc = 0x4c9d00; continue;
            }
            0x4c9d00 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c9d28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x4c9d2c; continue;
            }
            0x4c9d2c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x14c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x150); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4c9d46); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c9d46; continue;
            }
            0x4c9d46 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4c9d52); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4c9d5d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0x178); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x174); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4c9d7d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4c9d8e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x44),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn edit_box_layout_cursor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ca700;
    loop {
        match pc {
            0x4ca700 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6624d6u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4ca720);
            { let args = []; c.r[0] = host.call(m, 0x4d2fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x14),32); c.push(m, 0x4ca727); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4caa8a; continue; }
            pc = 0x4ca72f; continue;
            }
            0x4ca72f => {
            { let a = rm(m,c.r[6].wrapping_add(0x168),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4caa8a; continue; }
            pc = 0x4ca73c; continue;
            }
            0x4ca73c => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            c.push(m, 0x4ca76c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,c.r[6].wrapping_add(0x94));
            c.set(7,32,c.r[6].wrapping_add(0x140));
            c.set(0,32,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4ca807; continue; }
            pc = 0x4ca7a5; continue;
            }
            0x4ca7a5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xfffffff8),32),32); }
            if c.cond(13) { pc = 0x4ca807; continue; }
            pc = 0x4ca7ae; continue;
            }
            0x4ca7ae => {
            c.push(m,0x1u32);
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4ca7bf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64d0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x3u32); }
            c.push(m, 0x4ca7ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m, 0x4ca7dc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4ca7ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ca807; continue;
            }
            0x4ca807 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ca872; continue; }
            pc = 0x4ca811; continue;
            }
            0x4ca811 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0xfffffff8),32),32); }
            if c.cond(13) { pc = 0x4ca872; continue; }
            pc = 0x4ca81a; continue;
            }
            0x4ca81a => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4ca82c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64d0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x4u32); }
            c.push(m, 0x4ca83b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m, 0x4ca849);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4ca86c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            pc = 0x4ca872; continue;
            }
            0x4ca872 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4ca887);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x5u32); }
            c.push(m, 0x4ca896);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m, 0x4ca8a4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4ca8c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4ca8ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4ca920); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x150),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x144),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x144); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4ca946);
            { let args = []; c.r[0] = host.call(m, 0x4d2e70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4caa01; continue; }
            pc = 0x4ca94d; continue;
            }
            0x4ca94d => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4ca9ef; continue; }
            pc = 0x4ca954; continue;
            }
            0x4ca954 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4caa4a; continue; }
            pc = 0x4ca95b; continue;
            }
            0x4ca95b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.set(0,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ca975); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(14) { pc = 0x4ca9c7; continue; }
            pc = 0x4ca98e; continue;
            }
            0x4ca98e => {
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ca991); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(5,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[3]; wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ca9b1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4caa4a; continue;
            }
            0x4ca9c7 => {
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ca9ca); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4caa4a; continue; }
            pc = 0x4ca9ce; continue;
            }
            0x4ca9ce => {
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4caa4a; continue; }
            pc = 0x4ca9d4; continue;
            }
            0x4ca9d4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14c),32));
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4caa4a; continue;
            }
            0x4ca9ef => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x4caa4a; continue; }
            pc = 0x4ca9f3; continue;
            }
            0x4ca9f3 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4caa4a; continue;
            }
            0x4caa01 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14c),32));
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4caa19); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4caa35; continue; }
            pc = 0x4caa27; continue;
            }
            0x4caa27 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4caa4a; continue; }
            pc = 0x4caa35; continue;
            }
            0x4caa35 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4caa3c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4caa4a; continue;
            }
            0x4caa4a => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4caa58);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4caa66);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4caa77);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4caa8a => {
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let a = c.get(6,32); let r = c.add(a,0x140u32,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn viper_slider_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cf7a0;
    loop {
        match pc {
            0x4cf7a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4cf7a9);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m, 0x4cf7b4);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x16c));
            c.push(m, 0x4cf7bf);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x244),32));
            c.set(1,32,c.r[6].wrapping_add(0x244));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4cf7d1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x330));
            c.push(m, 0x4cf7dc);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0xcu32);
            c.set(1,32,c.r[6].wrapping_add(0x35c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x350); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x354); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0x358); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4cf803);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.set(2,32,c.r[6].wrapping_add(0x368));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4cf812);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xbf800000u32);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x37c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x378); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x374); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xff000000u32);
            { let a = c.r[6].wrapping_add(0x388); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x384); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x38c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x38d); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x390); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x394); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x395); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x398); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x39c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x3a8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x3ac); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x3a0); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn viper_slider_set_value(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d0bd0;
    loop {
        match pc {
            0x4d0bd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x350),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4d0bfb; continue; }
            pc = 0x4d0be7; continue;
            }
            0x4d0be7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x354),32));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4d0bfd; continue; }
            pc = 0x4d0bf1; continue;
            }
            0x4d0bf1 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4d0bfb; continue; }
            pc = 0x4d0bf5; continue;
            }
            0x4d0bf5 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4d0bfd; continue; }
            pc = 0x4d0bfb; continue;
            }
            0x4d0bfb => {
            c.set(7,32,c.get(0,32));
            pc = 0x4d0bfd; continue;
            }
            0x4d0bfd => {
            c.set(0,32,c.r[6].wrapping_add(0x374));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x374),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x244));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4d0c26); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x350),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4d0c3e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[3].wrapping_add(c.r[3]));
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x354),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('*',v); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('+',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4d0c99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0cb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "debug_info_window_factory" => debug_info_window_factory(m, host, regs),
        "debug_info_window_get_runtime_class" => debug_info_window_get_runtime_class(m, host, regs),
        "debug_info_window_ctor" => debug_info_window_ctor(m, host, regs),
        "debug_info_window_dtor" => debug_info_window_dtor(m, host, regs),
        "debug_info_window_get_message_map" => debug_info_window_get_message_map(m, host, regs),
        "debug_info_window_on_paint" => debug_info_window_on_paint(m, host, regs),
        "debug_page_town_ware_table" => debug_page_town_ware_table(m, host, regs),
        "debug_page_town_ware_table_extended" => debug_page_town_ware_table_extended(m, host, regs),
        "debug_page_town_construction_list" => debug_page_town_construction_list(m, host, regs),
        "debug_page_merchant_office_buildings" => debug_page_merchant_office_buildings(m, host, regs),
        "debug_page_town_distance_matrix" => debug_page_town_distance_matrix(m, host, regs),
        "debug_page_player_letter_counts" => debug_page_player_letter_counts(m, host, regs),
        "debug_page_low_productivity_buildings" => debug_page_low_productivity_buildings(m, host, regs),
        "debug_page_town_bathhouse_notables" => debug_page_town_bathhouse_notables(m, host, regs),
        "debug_page_ship_route_and_trade_info" => debug_page_ship_route_and_trade_info(m, host, regs),
        "debug_page_town_trade_detail_table" => debug_page_town_trade_detail_table(m, host, regs),
        "debug_page_town_tile_map" => debug_page_town_tile_map(m, host, regs),
        "debug_page_town_ware_stock_production" => debug_page_town_ware_stock_production(m, host, regs),
        "debug_page_town_price_threshold_chart" => debug_page_town_price_threshold_chart(m, host, regs),
        "debug_page_ware_town_grid_header" => debug_page_ware_town_grid_header(m, host, regs),
        "debug_page_mission_event_list" => debug_page_mission_event_list(m, host, regs),
        "debug_page_ship_details_and_pirate_nests" => debug_page_ship_details_and_pirate_nests(m, host, regs),
        "debug_page_player_reputation_by_town" => debug_page_player_reputation_by_town(m, host, regs),
        "debug_page_convoy_list" => debug_page_convoy_list(m, host, regs),
        "debug_page_town_info_history" => debug_page_town_info_history(m, host, regs),
        "debug_page_town_treasury_ledger" => debug_page_town_treasury_ledger(m, host, regs),
        "debug_cmd_send_test_letter_type_0x45" => debug_cmd_send_test_letter_type_0x45(m, host, regs),
        "debug_cmd_send_test_letter_type_0x21" => debug_cmd_send_test_letter_type_0x21(m, host, regs),
        "debug_info_window_scalar_deleting_dtor" => debug_info_window_scalar_deleting_dtor(m, host, regs),
        "debug_info_helper_scalar_deleting_dtor" => debug_info_helper_scalar_deleting_dtor(m, host, regs),
        "debug_info_helper_dtor" => debug_info_helper_dtor(m, host, regs),
        "combo_box_reset" => combo_box_reset(m, host, regs),
        "combo_box_clear_dirty_and_forward_to_list" => combo_box_clear_dirty_and_forward_to_list(m, host, regs),
        "combo_box_add_dirty_rect" => combo_box_add_dirty_rect(m, host, regs),
        "combo_box_get_total_size" => combo_box_get_total_size(m, host, regs),
        "combo_box_set_size_from_triple" => combo_box_set_size_from_triple(m, host, regs),
        "edit_box_load_from_ini" => edit_box_load_from_ini(m, host, regs),
        "edit_box_layout_cursor" => edit_box_layout_cursor(m, host, regs),
        "viper_slider_reset" => viper_slider_reset(m, host, regs),
        "viper_slider_set_value" => viper_slider_set_value(m, host, regs),
        _ => return None,
    })
}
