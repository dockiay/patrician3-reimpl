//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn multiplayer_screen_update_hotseat_setup_tail_start(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e040;
    loop {
        match pc {
            0x43e040 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc1u32); }
            c.push(m, 0x43e05c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e066);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let args = []; c.r[0] = host.call(m, 0x43e148, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_invalidate_slot_row(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e1f0;
    loop {
        match pc {
            0x43e1f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4]);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x15eu32); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x320u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_mul(4).wrapping_add(0x64));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e231);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_add_chat_line(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e240;
    loop {
        match pc {
            0x43e240 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.r[6].wrapping_add(0x448d); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,0x64u32,32); }
            if c.cond(5) { pc = 0x43e2de; continue; }
            pc = 0x43e25a; continue;
            }
            0x43e25a => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4248),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x4248));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43e27b; continue; }
            pc = 0x43e26b; continue;
            }
            0x43e26b => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43e272);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x43e278);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43e27b; continue;
            }
            0x43e27b => {
            c.set(0,32,c.r[6].wrapping_add(0x424c));
            c.push(m,0x18cu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43e28d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x43d9));
            c.push(m,0x63u32);
            c.set(2,32,c.r[6].wrapping_add(0x43d8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43e2a2);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.push(m, 0x43e2a9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x43e2bb; continue; }
            pc = 0x43e2b1; continue;
            }
            0x43e2b1 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x43e2bd; continue;
            }
            0x43e2bb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43e2bd; continue;
            }
            0x43e2bd => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x43d4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[6].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x4440));
            c.push(m,c.get(2,32));
            c.push(m, 0x43e2d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x443b); wm(m,a,8,c.get(3,8)); }
            pc = 0x43e344; continue;
            }
            0x43e2de => {
            c.push(m,0x4u32);
            c.push(m, 0x43e2e5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43e2f6; continue; }
            pc = 0x43e2ec; continue;
            }
            0x43e2ec => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x43e2f8; continue;
            }
            0x43e2f6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43e2f8; continue;
            }
            0x43e2f8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4248); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x4440));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4248),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43e323);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x43d8); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x448c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,c.get(0,32)); }
            pc = 0x43e344; continue;
            }
            0x43e344 => {
            { let a = c.get(6,32); let r = c.add(a,0x4864u32,32); c.set(6,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x43e355);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_start_button_enabled(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e3c0;
    loop {
        match pc {
            0x43e3c0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43e411; continue; }
            pc = 0x43e3ce; continue;
            }
            0x43e3ce => {
            c.set(1,8,0x1u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43e3d2; continue;
            }
            0x43e3d2 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x43e3ec; continue; }
            pc = 0x43e3dd; continue;
            }
            0x43e3dd => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1fc),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x43e3ec; continue; }
            pc = 0x43e3e8; continue;
            }
            0x43e3e8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x43e3f1; continue;
            }
            0x43e3ec => {
            c.set(2,32,0x1u32);
            pc = 0x43e3f1; continue;
            }
            0x43e3f1 => {
            { let a = c.get(1,8); let r = c.and(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43e3d2; continue; }
            pc = 0x43e3f9; continue;
            }
            0x43e3f9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43e411); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43e411; continue;
            }
            0x43e411 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_chat_scroll(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e420;
    loop {
        match pc {
            0x43e420 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x448d),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e4bf; continue; }
            pc = 0x43e431; continue;
            }
            0x43e431 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x448c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e486; continue; }
            pc = 0x43e43b; continue;
            }
            0x43e43b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            c.push(m,c.get(0,32));
            c.push(m, 0x43e44d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4850),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4834),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43e47d; continue; }
            pc = 0x43e468; continue;
            }
            0x43e468 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4848),32));
            c.set(1,32,c.r[6].wrapping_add(0x4498));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x43e47d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43e47d; continue;
            }
            0x43e47d => {
            { let a = c.r[6].wrapping_add(0x448c); wm(m,a,8,0x0u32); }
            pc = 0x43e4b8; continue;
            }
            0x43e486 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4850),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4834),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x43e4b8; continue; }
            pc = 0x43e49e; continue;
            }
            0x43e49e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x43e4b8; continue; }
            pc = 0x43e4a2; continue;
            }
            0x43e4a2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4848),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x4498));
            c.push(m, 0x43e4b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43e4b8; continue;
            }
            0x43e4b8 => {
            { let a = c.r[6].wrapping_add(0x448d); wm(m,a,8,0x0u32); }
            pc = 0x43e4bf; continue;
            }
            0x43e4bf => {
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            c.push(m, 0x43e4ca);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x4844); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x43e4e5; continue; }
            pc = 0x43e4d4; continue;
            }
            0x43e4d4 => {
            { let a = c.get(6,32); let r = c.add(a,0x4864u32,32); c.set(6,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x43e4e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43e4e5; continue;
            }
            0x43e4e5 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_abort_session(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e4f0;
    loop {
        match pc {
            0x43e4f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4224),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e505; continue; }
            pc = 0x43e4fe; continue;
            }
            0x43e4fe => {
            c.push(m,0x1u32);
            c.push(m, 0x43e505);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43e505; continue;
            }
            0x43e505 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x0u32); }
            c.push(m, 0x43e513);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e53c; continue; }
            pc = 0x43e51d; continue;
            }
            0x43e51d => {
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697910u32);
            c.push(m, 0x43e535);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43e53c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x43e53c; continue;
            }
            0x43e53c => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_invalidate_sex_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e760;
    loop {
        match pc {
            0x43e760 => {
            c.set(0,32,c.r[1].wrapping_add(0x2b7c));
            { let a = c.get(1,32); let r = c.add(a,0x2a94u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x43e779);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_save_local_profile(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e870;
    loop {
        match pc {
            0x43e870 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1a4));
            { let a = c.get(1,32); let r = c.add(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43e88b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,c.r[6].wrapping_add(0x184));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x43e8a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1cc),8));
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.r[2].wrapping_add(0x46c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c4),8));
            { let a = c.r[1].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m, 0x43e8da);
            { let args = []; c.r[0] = host.call(m, 0x49a2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_store_slot_profile(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ea60;
    loop {
        match pc {
            0x43ea60 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e35du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x43ea99);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x43eaa3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x43ead7; continue; }
            pc = 0x43eab2; continue;
            }
            0x43eab2 => {
            c.set(1,8,rm(m,c.r[0],8));
            c.set(5,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43eacb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64cb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697918u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43ead7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43ead7; continue;
            }
            0x43ead7 => {
            c.set(5,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43eae4);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x164));
            c.push(m, 0x43eaf1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x184));
            c.push(m,c.get(0,32));
            c.push(m, 0x43eb02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43eb09);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4));
            c.push(m, 0x43eb16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(5,32,c.r[6].wrapping_add(0x1a88));
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43eb30);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43eb3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43eb4a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43ee80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x43eb56);
            { let args = []; c.r[0] = host.call(m, 0x440750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x484c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x43eb75);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x486c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4864),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x179); wm(m,a,8,0x1u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            { let a = c.r[5].wrapping_add(0x170); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x43eba0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x214),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x43ebbe);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_init_list_geometry(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ebe0;
    loop {
        match pc {
            0x43ebe0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4]);
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let a = c.r[6].wrapping_add(0x484c); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x43ec03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,0xa0u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x4848); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x4874); wm(m,a,32,0x16fu32); }
            { let a = c.r[6].wrapping_add(0x4878); wm(m,a,32,0x14au32); }
            { let a = c.r[6].wrapping_add(0x487c); wm(m,a,32,0x304u32); }
            { let a = c.r[6].wrapping_add(0x4864); wm(m,a,32,0x173u32); }
            { let a = c.r[6].wrapping_add(0x4868); wm(m,a,32,0x14eu32); }
            { let a = c.r[6].wrapping_add(0x486c); wm(m,a,32,0x2ecu32); }
            { let a = c.r[6].wrapping_add(0x4858); wm(m,a,32,0x2u32); }
            { let a = c.r[6].wrapping_add(0x4850); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x152u32,32); c.set(0,32,r); }
            c.set(1,32,0x6dce78u32);
            { let a = c.r[6].wrapping_add(0x4880); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xfffffffdu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4870); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x43ec8b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x485c); wm(m,a,32,0xdu32); }
            { let a = c.r[6].wrapping_add(0x4854); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4884); wm(m,a,32,0x195u32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x3c));
            { let a = c.r[6].wrapping_add(0x4888); wm(m,a,32,0x104u32); }
            { let a = c.r[6].wrapping_add(0x488c); wm(m,a,32,0x2c1u32); }
            { let a = c.r[6].wrapping_add(0x4c60); wm(m,a,32,0x2u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x14));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[6].wrapping_add(0x4890); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dce78u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x43ece8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,0xfau32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x4c5c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x4894); wm(m,a,32,0x91u32); }
            { let a = c.r[6].wrapping_add(0x4898); wm(m,a,32,0xb4u32); }
            { let a = c.r[6].wrapping_add(0x489c); wm(m,a,32,0x27bu32); }
            { let a = c.r[6].wrapping_add(0x4c64); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xb4u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x48a0); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_pick_home_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ed40;
    loop {
        match pc {
            0x43ed40 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m, 0x43ed4e);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43eda1; continue; }
            pc = 0x43ed52; continue;
            }
            0x43ed52 => {
            c.push(m,0x1u32);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43ed5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49c0d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x5b9du32);
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43ed72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43ed7c);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43ed92; continue; }
            pc = 0x43ed81; continue;
            }
            0x43ed81 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(1,32)); }
            pc = 0x43ed92; continue;
            }
            0x43ed92 => {
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43eda1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43eda1; continue;
            }
            0x43eda1 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_sex_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43edb0;
    loop {
        match pc {
            0x43edb0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[3].wrapping_add(0x2b7c));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43edc5);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[3].wrapping_add(0x2a94));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x43edf9; continue; }
            pc = 0x43edd1; continue;
            }
            0x43edd1 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43edd8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43ede1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[3].wrapping_add(0x4cd1); wm(m,a,8,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x43edf7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43ee2a; continue;
            }
            0x43edf9 => {
            c.push(m, 0x43edfe);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43ee6a; continue; }
            pc = 0x43ee02; continue;
            }
            0x43ee02 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43ee0b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43ee14); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[3].wrapping_add(0x4cd1); wm(m,a,8,0x1u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x43ee2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43ee2a; continue;
            }
            0x43ee2a => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x43ee39); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[0].wrapping_add(0xffffffec));
            { let a = c.get(0,32); let r = c.add(a,0x1eu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43ee6a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43ee6a; continue;
            }
            0x43ee6a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_compose_chat_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ee80;
    loop {
        match pc {
            0x43ee80 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e396u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m,0x69791cu32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.push(m, 0x43eeb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x164),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x164));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffdu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x43eee1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x3u32); }
            c.push(m, 0x43eef0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.push(m, 0x43eefe);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x697920u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x43ef0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x43ef1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ef2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x43ef40);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x43ef4e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_release_lobby_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ef70;
    loop {
        match pc {
            0x43ef70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(3,32,0x8u32);
            c.set(7,32,c.r[6].wrapping_add(0x2354));
            pc = 0x43ef82; continue;
            }
            0x43ef82 => {
            c.set(0,32,c.r[7].wrapping_add(0xfffff8c0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43ef90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ef98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x43ef82; continue; }
            pc = 0x43efa1; continue;
            }
            0x43efa1 => {
            c.set(7,32,c.r[6].wrapping_add(0x2a94));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43efaf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x2b7c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x43efbd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x2c64));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x43efcb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xef8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43efd9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x10e0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43efe7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x11d4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43eff5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x12c8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f003);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f011);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1908));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f01f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1a88));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f02d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xfec));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f03b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x15c8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f049);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43f134; continue; }
            pc = 0x43f055; continue;
            }
            0x43f055 => {
            c.set(0,32,c.r[6].wrapping_add(0x2354));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x8u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x43f067; continue;
            }
            0x43f067 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffff8c0u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f079);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43f085);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x43f067; continue; }
            pc = 0x43f09e; continue;
            }
            0x43f09e => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f0a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f0ae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f0b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xef8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f0c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x10e0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f0d2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x11d4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f0e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x12c8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f0ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f0fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1908));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f10a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1a88));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f118);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xfec));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f126);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x15c8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f134);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43f134; continue;
            }
            0x43f134 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_assign_merchant_to_slot(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f2b0;
    loop {
        match pc {
            0x43f2b0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e3a8u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54d4),32));
            { let a = c.get(5,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x43f44d; continue; }
            pc = 0x43f2e4; continue;
            }
            0x43f2e4 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            c.push(m, 0x43f2fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.push(m,c.get(5,32));
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            c.set(3,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x184));
            c.push(m, 0x43f317);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43f325);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43f330);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f343);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[3],32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4),32));
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[1],8));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x164));
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,0x697928u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x43f365);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x43f373);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43f413; continue; }
            pc = 0x43f390; continue;
            }
            0x43f390 => {
            c.set(5,32,c.r[6].wrapping_add(0x1a88));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43f39d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43f3aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f3b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43ee80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.push(m, 0x43f3c6);
            { let args = []; c.r[0] = host.call(m, 0x440750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x484c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x43f3e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x486c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4864),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x179); wm(m,a,8,0x1u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0x170); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x43f413);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43f413; continue;
            }
            0x43f413 => {
            c.set(2,32,c.r[7].wrapping_mul(8));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x15eu32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x320u32); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f44d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43f44d; continue;
            }
            0x43f44d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_load_slot_into_form(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f470;
    loop {
        match pc {
            0x43f470 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f4a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x184));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43f4b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_load_selected_merchant_into_form(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f4d0;
    loop {
        match pc {
            0x43f4d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e3c2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f4fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.push(m,c.get(2,32));
            c.set(0,32,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43f520);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f530);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.push(m, 0x43f548);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43f559);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x43f56a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f57a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43f592);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43f5a3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x43f5b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_is_merchant_choice_free(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f5e0;
    loop {
        match pc {
            0x43f5e0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4cb0),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            c.push(m,c.get(7,32));
            c.set(0,8,0x1u32);
            { let a = c.r[1].wrapping_add(0x4c68); wm(m,a,32,0xffffffffu32); }
            if c.cond(14) { pc = 0x43f626; continue; }
            pc = 0x43f5fb; continue;
            }
            0x43f5fb => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x5664),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x43f626; continue; }
            pc = 0x43f607; continue;
            }
            0x43f607 => {
            c.set(6,32,c.r[1].wrapping_add(0x54d4));
            pc = 0x43f60d; continue;
            }
            0x43f60d => {
            c.set(5,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1d4),32),32); }
            if c.cond(5) { pc = 0x43f61e; continue; }
            pc = 0x43f618; continue;
            }
            0x43f618 => {
            { let a = c.r[1].wrapping_add(0x4c68); wm(m,a,32,c.get(2,32)); }
            pc = 0x43f61e; continue;
            }
            0x43f61e => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x43f60d; continue; }
            pc = 0x43f626; continue;
            }
            0x43f626 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[1].wrapping_add(0x1d4));
            c.set(6,32,0x8u32);
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54d4),32));
            pc = 0x43f63c; continue;
            }
            0x43f63c => {
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x43f64a; continue; }
            pc = 0x43f640; continue;
            }
            0x43f640 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4c68),32),32); }
            if c.cond(4) { pc = 0x43f64a; continue; }
            pc = 0x43f648; continue;
            }
            0x43f648 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x43f64a; continue;
            }
            0x43f64a => {
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x43f63c; continue; }
            pc = 0x43f650; continue;
            }
            0x43f650 => {
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

pub fn multiplayer_screen_is_merchant_index_unused(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f660;
    loop {
        match pc {
            0x43f660 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x1d4u32,32); c.set(1,32,r); }
            c.set(2,32,0x8u32);
            pc = 0x43f672; continue;
            }
            0x43f672 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x43f678; continue; }
            pc = 0x43f676; continue;
            }
            0x43f676 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x43f678; continue;
            }
            0x43f678 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x43f672; continue; }
            pc = 0x43f67e; continue;
            }
            0x43f67e => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn compose_initial_and_family_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f690;
    loop {
        match pc {
            0x43f690 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e413u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x43f6eb; continue; }
            pc = 0x43f6d5; continue;
            }
            0x43f6d5 => {
            c.set(0,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x43f6e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64cb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43f6f9; continue;
            }
            0x43f6eb => {
            c.push(m,0x6cbba6u32);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x43f6f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43f6f9; continue;
            }
            0x43f6f9 => {
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x43f708);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,0x697930u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x43f71c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x5u32); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m, 0x43f731);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x6u32); }
            c.push(m, 0x43f740);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x5u32); }
            c.push(m, 0x43f74e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x4u32); }
            c.push(m, 0x43f75c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43f76c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3u32); }
            c.push(m, 0x43f782);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x2u32); }
            c.push(m, 0x43f790);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x43f79e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.push(m, 0x43f7ac);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_find_player_list_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f7c0;
    loop {
        match pc {
            0x43f7c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x5664),32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x43f7ea; continue; }
            pc = 0x43f7d0; continue;
            }
            0x43f7d0 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x54d4u32,32); c.set(1,32,r); }
            pc = 0x43f7db; continue;
            }
            0x43f7db => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x43f7e1; continue; }
            pc = 0x43f7df; continue;
            }
            0x43f7df => {
            c.set(0,32,c.get(2,32));
            pc = 0x43f7e1; continue;
            }
            0x43f7e1 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x43f7db; continue; }
            pc = 0x43f7e9; continue;
            }
            0x43f7e9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x43f7ea; continue;
            }
            0x43f7ea => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_rebuild_player_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f7f0;
    loop {
        match pc {
            0x43f7f0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43f84f; continue; }
            pc = 0x43f7fe; continue;
            }
            0x43f7fe => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,0x0u32); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x43f84f; continue; }
            pc = 0x43f813; continue;
            }
            0x43f813 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43f81e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            { let a = c.get(0,8); let r = c.and(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x43f841; continue; }
            pc = 0x43f827; continue;
            }
            0x43f827 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,c.get(0,32)); }
            pc = 0x43f841; continue;
            }
            0x43f841 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x43f813; continue; }
            pc = 0x43f84f; continue;
            }
            0x43f84f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_get_player_list_entry(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f860;
    loop {
        match pc {
            0x43f860 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f870;
    loop {
        match pc {
            0x43f870 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x43f87c);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x190u32);
            c.set(0,32,c.r[6].wrapping_add(0x54d4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43f88f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x21c));
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,0x0u32); }
            c.push(m, 0x43f8a7);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2f4));
            c.push(m, 0x43f8b2);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3cc));
            c.push(m, 0x43f8bd);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            c.push(m, 0x43f8c8);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x57c));
            c.push(m, 0x43f8d3);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0xc70));
            c.set(5,32,c.r[6].wrapping_add(0xc10));
            c.set(7,32,c.r[6].wrapping_add(0x930));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x3u32); }
            pc = 0x43f8ed; continue;
            }
            0x43f8ed => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xfffffd24),32));
            c.set(1,32,c.r[7].wrapping_add(0xfffffd24));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43f8ff); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43f909); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43f910);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43f917);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x20u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xd8u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x43f8ed; continue; }
            pc = 0x43f931; continue;
            }
            0x43f931 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xef8),32));
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43f943); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfec),32));
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43f955); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43f967); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43f979); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43f98b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43f99d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x14b0));
            c.push(m, 0x43f9a8);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.push(m, 0x43f9b3);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.push(m, 0x43f9be);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43f9c9);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m, 0x43f9d4);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x2354));
            c.set(3,32,0x8u32);
            pc = 0x43f9df; continue;
            }
            0x43f9df => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xfffff8c0),32));
            c.set(1,32,c.r[7].wrapping_add(0xfffff8c0));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43f9f1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43f9fb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x43f9df; continue; }
            pc = 0x43fa04; continue;
            }
            0x43fa04 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43fa16); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x43fa28); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x43fa3a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2d5c));
            c.push(m, 0x43fa45);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3110));
            c.push(m, 0x43fa50);
            { let args = []; c.r[0] = host.call(m, 0x4ceb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            c.push(m, 0x43fa5b);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x48a4));
            c.push(m, 0x43fa66);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x54d0); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,0x0u32); }
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

pub fn multiplayer_screen_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43fa80;
    loop {
        match pc {
            0x43fa80 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x43fa8a);
            { let args = []; c.r[0] = host.call(m, 0x4b6e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43fb90; continue; }
            pc = 0x43fa98; continue;
            }
            0x43fa98 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4224),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43fb90; continue; }
            pc = 0x43faa6; continue;
            }
            0x43faa6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43fb15; continue; }
            pc = 0x43fab0; continue;
            }
            0x43fab0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x214),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x43fb01; continue; }
            pc = 0x43fac0; continue;
            }
            0x43fac0 => {
            c.set(3,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(7,32,c.r[6].wrapping_add(0x2c64));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43fad4);
            { let args = []; c.r[0] = host.call(m, 0x4c78a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43fadd);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43fb90; continue; }
            pc = 0x43fae8; continue;
            }
            0x43fae8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x214),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43fafd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43fb01 => {
            c.push(m,0x697934u32);
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m, 0x43fb11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43fb15 => {
            c.set(0,32,rm(m,0x6df2fcu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43fb90; continue; }
            pc = 0x43fb1e; continue;
            }
            0x43fb1e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x214),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x43fb80; continue; }
            pc = 0x43fb2a; continue;
            }
            0x43fb2a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x2c64));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x16c),8));
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1bc));
            c.push(m, 0x43fb47);
            { let args = []; c.r[0] = host.call(m, 0x4c78a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[3],32));
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43fb52);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x43fb90; continue; }
            pc = 0x43fb59; continue;
            }
            0x43fb59 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x214),32));
            c.set(0,32,rm(m,0x6df2fcu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x16c),8));
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1bc),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43fb7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43fb80 => {
            c.push(m,0x697938u32);
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m, 0x43fb90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43fb90; continue;
            }
            0x43fb90 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_disconnect_and_notify_matcher(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43fd20;
    loop {
        match pc {
            0x43fd20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e425u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x43fd47);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1u32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xa4u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43fd6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x66a2bcu32,32));
            c.push(m,0x96u32);
            { let target = c.get(7,32); c.push(m, 0x43fd78); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43fd82);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x96u32);
            { let target = c.get(7,32); c.push(m, 0x43fd89); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x43fdc0; continue; }
            pc = 0x43fd95; continue;
            }
            0x43fd95 => {
            c.set(5,32,c.r[6].wrapping_add(0x4248));
            pc = 0x43fd9b; continue;
            }
            0x43fd9b => {
            c.set(3,32,rm(m,c.r[5],32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43fdb2; continue; }
            pc = 0x43fda2; continue;
            }
            0x43fda2 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43fda9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x43fdaf);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43fdb2; continue;
            }
            0x43fdb2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43fd9b; continue; }
            pc = 0x43fdc0; continue;
            }
            0x43fdc0 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43fdd2);
            { let args = []; c.r[0] = host.call(m, 0x54a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,0x6cbba7u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43fde9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69793cu32);
            c.push(m, 0x43fe04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43fe0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697944u32);
            c.push(m, 0x43fe22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43fe29);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df330u32);
            c.push(m, 0x43fe38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43fe45);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_show_error_message_for_code(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43ff40;
    loop {
        match pc {
            0x43ff40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e457u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x43ff8a; continue; }
            pc = 0x43ff60; continue;
            }
            0x43ff60 => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(8).wrapping_add(0x6af134),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ff76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_mul(8).wrapping_add(0x6af130),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            pc = 0x440037; continue;
            }
            0x43ff8a => {
            { let a = c.get(6,32); c.sub(a,0xc8u32,32); }
            if c.cond(15) { pc = 0x44000d; continue; }
            pc = 0x43ff92; continue;
            }
            0x43ff92 => {
            if c.cond(4) { pc = 0x43ffe9; continue; }
            pc = 0x43ff94; continue;
            }
            0x43ff94 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x43ffc5; continue; }
            pc = 0x43ff98; continue;
            }
            0x43ff98 => {
            { let a = c.get(6,32); c.sub(a,0x64u32,32); }
            if c.cond(5) { pc = 0x440057; continue; }
            pc = 0x43ffa1; continue;
            }
            0x43ffa1 => {
            c.set(0,32,rm(m,0x69af80u32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ffb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af7cu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x2u32); }
            pc = 0x440037; continue;
            }
            0x43ffc5 => {
            c.set(0,32,rm(m,0x69af78u32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ffd9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af74u32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x440037; continue;
            }
            0x43ffe9 => {
            c.set(0,32,rm(m,0x69af88u32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43fffd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af84u32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            pc = 0x440037; continue;
            }
            0x44000d => {
            { let a = c.get(6,32); c.sub(a,0x1f4u32,32); }
            if c.cond(5) { pc = 0x440057; continue; }
            pc = 0x440015; continue;
            }
            0x440015 => {
            c.set(0,32,rm(m,0x69af90u32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x440029);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af8cu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x4u32); }
            pc = 0x440037; continue;
            }
            0x440037 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x440044);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x440057);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x440057; continue;
            }
            0x440057 => {
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x440065); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_enter_slot_pick_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x440080;
    loop {
        match pc {
            0x440080 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x440089);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x44009a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4400ae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4400c4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4400cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x5u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_get_slot_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4402f0;
    loop {
        match pc {
            0x4402f0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x4402f7; continue;
            }
            0x4402f7 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x204),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x440308; continue; }
            pc = 0x440302; continue;
            }
            0x440302 => {
            c.set(2,8,0x1u32);
            { let a = c.get(2,8); let n = c.get(1,8); let r = c.shl(a,n,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.or(a,c.get(2,8),8); c.set(0,8,r); }
            pc = 0x440308; continue;
            }
            0x440308 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x4402f7; continue; }
            pc = 0x44030e; continue;
            }
            0x44030e => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_send_matcher_command(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x440310;
    loop {
        match pc {
            0x440310 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e471u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x5d20),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x440364; continue; }
            pc = 0x44033a; continue;
            }
            0x44033a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x440364; continue; }
            pc = 0x440342; continue;
            }
            0x440342 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x440353);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x440364 => {
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,0x69796cu32);
            { let target = rm(m,0x66a4ecu32,32); c.push(m, 0x440372); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a4f0u32,32); c.push(m, 0x44037b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x440415; continue; }
            pc = 0x440383; continue;
            }
            0x440383 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(3) { pc = 0x4403ba; continue; }
            pc = 0x44039b; continue;
            }
            0x44039b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,0x69797cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4403b5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4403c8; continue;
            }
            0x4403ba => {
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x4403c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4403c8; continue;
            }
            0x4403c8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4403e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4403ef);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x4au32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x440407); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x440415);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x440415; continue;
            }
            0x440415 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x440426);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_build_matcher_info_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x440440;
    loop {
        match pc {
            0x440440 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e4b2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x697988u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x440472);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xf),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(7) { pc = 0x4404db; continue; }
            pc = 0x4404a1; continue;
            }
            0x4404a1 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x44067c)); continue;
            }
            0x4404a8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69aac0),32));
            c.push(m,c.get(1,32));
            pc = 0x4404d2; continue;
            }
            0x4404b7 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ac24),32));
            pc = 0x4404d1; continue;
            }
            0x4404c5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69acac),32));
            pc = 0x4404d1; continue;
            }
            0x4404d1 => {
            c.push(m,c.get(0,32));
            pc = 0x4404d2; continue;
            }
            0x4404d2 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x4404db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4404db; continue;
            }
            0x4404db => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4404f8; continue; }
            pc = 0x4404e5; continue;
            }
            0x4404e5 => {
            c.push(m,0x697a34u32);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x4404f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x440586; continue;
            }
            0x4404f8 => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x6cbba8u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x470),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44050f);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x440528; continue; }
            pc = 0x440516; continue;
            }
            0x440516 => {
            c.set(2,32,rm(m,0x6aef28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m, 0x440526);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x440586; continue;
            }
            0x440528 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x697a48u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x470),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44053e);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x440557; continue; }
            pc = 0x440545; continue;
            }
            0x440545 => {
            c.set(1,32,rm(m,0x6aef2cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x440555);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x440586; continue;
            }
            0x440557 => {
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.get(2,32); let r = c.add(a,0x470u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x44056d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(14) { pc = 0x440586; continue; }
            pc = 0x440579; continue;
            }
            0x440579 => {
            c.push(m,0x0u32);
            c.push(m,0xfu32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x440586);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x440586; continue;
            }
            0x440586 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xf),8));
            { let a = c.get(1,8); c.sub(a,0x6u32,8); }
            if c.cond(5) { pc = 0x44059b; continue; }
            pc = 0x440593; continue;
            }
            0x440593 => {
            c.set(6,32,rm(m,0x6b2d3cu32,32));
            pc = 0x4405a8; continue;
            }
            0x44059b => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69aa10),32));
            pc = 0x4405a8; continue;
            }
            0x4405a8 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4405b7; continue; }
            pc = 0x4405af; continue;
            }
            0x4405af => {
            c.set(2,32,rm(m,0x6b2accu32,32));
            pc = 0x4405c8; continue;
            }
            0x4405b7 => {
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            c.set(2,32,rm(m,0x6b2adcu32,32));
            if c.cond(4) { pc = 0x4405c8; continue; }
            pc = 0x4405c2; continue;
            }
            0x4405c2 => {
            c.set(2,32,rm(m,0x6b2aecu32,32));
            pc = 0x4405c8; continue;
            }
            0x4405c8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,0x69a7e4u32);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x17),8));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a758),32));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x9),8));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.push(m,c.get(6,32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69a990),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x440612);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x440625);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x3u32); }
            c.push(m, 0x44063b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.push(m, 0x440649);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x440657);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x440665);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x440730;
    loop {
        match pc {
            0x440730 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x440738);
            { let args = []; c.r[0] = host.call(m, 0x436ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x440748; continue; }
            pc = 0x44073f; continue;
            }
            0x44073f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x440745);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x440748; continue;
            }
            0x440748 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn options_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45cac0;
    loop {
        match pc {
            0x45cac0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65eb73u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x45cae4);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1d0));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x0u32); }
            c.push(m, 0x45caf7);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.push(m,0x14u32);
            c.set(0,32,c.r[6].wrapping_add(0x2a8));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x45cb19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x14c8));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x2u32); }
            c.push(m, 0x45cb29);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15bc));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x3u32); }
            c.push(m, 0x45cb39);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x16b0));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x4u32); }
            c.push(m, 0x45cb49);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x17a4));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x5u32); }
            c.push(m, 0x45cb59);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1898));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x6u32); }
            c.push(m, 0x45cb69);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4cf710u32);
            c.push(m,0x4cf5d0u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x1990));
            c.push(m,0x3b0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x7u32); }
            c.push(m, 0x45cb8b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4ace40u32);
            c.push(m,0x4acdd0u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0x24a0));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x8u32); }
            c.push(m, 0x45cbad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x2728));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x9u32); }
            c.push(m, 0x45cbcf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x29e0));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0xau32); }
            c.push(m, 0x45cbf1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x34u32);
            c.set(2,32,c.r[6].wrapping_add(0x198));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0xbu32); }
            { let a = c.r[6]; wm(m,a,32,0x66dce0u32); }
            c.push(m, 0x45cc0c);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x45cc18);
            { let args = []; c.r[0] = host.call(m, 0x4028a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x45cc21);
            { let args = []; c.r[0] = host.call(m, 0x402970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn options_window_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45cc40;
    loop {
        match pc {
            0x45cc40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ec60u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66dce0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xbu32); }
            c.push(m, 0x45cc70);
            { let args = []; c.r[0] = host.call(m, 0x45cd80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x29e0));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0xau32); }
            c.push(m, 0x45cc8d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x2728));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x9u32); }
            c.push(m, 0x45ccaa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0x24a0));
            c.push(m,0xd8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x8u32); }
            c.push(m, 0x45ccc7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4cf710u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x1990));
            c.push(m,0x3b0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x7u32); }
            c.push(m, 0x45cce4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x1898));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x45ccf4);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x17a4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x45cd04);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x16b0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x4u32); }
            c.push(m, 0x45cd14);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x15bc));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            c.push(m, 0x45cd24);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x14c8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x45cd34);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x14u32);
            c.set(1,32,c.r[6].wrapping_add(0x2a8));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x45cd51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x1d0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x45cd61);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x45cd70);
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

pub fn options_window_unload_children(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45cd80;
    loop {
        match pc {
            0x45cd80 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x45cd8b);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1d0));
            c.push(m, 0x45cd96);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x2a8));
            c.set(3,32,0x14u32);
            pc = 0x45cda1; continue;
            }
            0x45cda1 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x45cdab); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x45cda1; continue; }
            pc = 0x45cdb4; continue;
            }
            0x45cdb4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x14c8));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x45cdc6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x15bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x15bc));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x45cdd8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x16b0),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b0));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x45cdea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x17a4));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x45cdfc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1898),32));
            c.set(1,32,c.r[6].wrapping_add(0x1898));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x45ce0e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x29e0));
            c.set(3,32,c.r[6].wrapping_add(0x24a0));
            { let a = c.get(6,32); let r = c.add(a,0x1990u32,32); c.set(6,32,r); }
            c.set(5,32,0x3u32);
            pc = 0x45ce25; continue;
            }
            0x45ce25 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45ce2c);
            { let args = []; c.r[0] = host.call(m, 0x4cf7a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x45ce33);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffd48),32));
            c.set(1,32,c.r[7].wrapping_add(0xfffffd48));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x45ce45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x45ce4f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x3b0u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xd8u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x45ce25; continue; }
            pc = 0x45ce64; continue;
            }
            0x45ce64 => {
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

pub fn options_window_apply_and_save_settings(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45df10;
    loop {
        match pc {
            0x45df10 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65ec8cu32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45df44; continue; }
            pc = 0x45df40; continue;
            }
            0x45df40 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x45dfb3; continue; }
            pc = 0x45df44; continue;
            }
            0x45df44 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45dfb3; continue; }
            pc = 0x45df4d; continue;
            }
            0x45df4d => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45dfb3; continue; }
            pc = 0x45df5e; continue;
            }
            0x45df5e => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x45df67);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x66de7c),32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45dfb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45dfb3; continue;
            }
            0x45dfb3 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x16c),8));
            { let a = c.r[1].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x180),8));
            c.set(1,32,c.r[6].wrapping_add(0x1990));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x45dfdb);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x1d40));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x21),8));
            { let a = c.r[6].wrapping_add(0x170); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x45dfff);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x23); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x20f0));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x23),8));
            { let a = c.r[6].wrapping_add(0x174); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x45e023);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x184),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x22),8));
            { let a = c.r[6].wrapping_add(0x178); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x25); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x168),8));
            { let a = c.r[1].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x164),8));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x188),32));
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            c.set(2,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[2].wrapping_add(0x1dd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45e0ab; continue; }
            pc = 0x45e090; continue;
            }
            0x45e090 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16c),32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x66de94u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x45e0ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x599e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45e0ab; continue;
            }
            0x45e0ab => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x16c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698340u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m, 0x45e0d2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,0x66a2c0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x698344u32);
            c.push(m,c.get(0,32));
            c.push(m,0x698350u32);
            c.push(m,0x698358u32);
            { let target = c.get(7,32); c.push(m, 0x45e0f1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x170),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698360u32);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(2,32));
            c.set(3,8,u32::from(c.cond(5)));
            c.push(m, 0x45e10c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e12e; continue; }
            pc = 0x45e113; continue;
            }
            0x45e113 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698364u32);
            c.push(m,c.get(0,32));
            c.push(m,0x698370u32);
            c.push(m,0x698378u32);
            { let target = c.get(7,32); c.push(m, 0x45e129); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e12e; continue;
            }
            0x45e12e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x174),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698380u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e144);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e166; continue; }
            pc = 0x45e14b; continue;
            }
            0x45e14b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698384u32);
            c.push(m,c.get(0,32));
            c.push(m,0x698390u32);
            c.push(m,0x698398u32);
            { let target = c.get(7,32); c.push(m, 0x45e161); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e166; continue;
            }
            0x45e166 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x178),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6983a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e17c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e19e; continue; }
            pc = 0x45e183; continue;
            }
            0x45e183 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x6983a4u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6983b0u32);
            c.push(m,0x6983b8u32);
            { let target = c.get(7,32); c.push(m, 0x45e199); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e19e; continue;
            }
            0x45e19e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6983c0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e1b4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e1d6; continue; }
            pc = 0x45e1bb; continue;
            }
            0x45e1bb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x6983c4u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6983d0u32);
            c.push(m,0x6983d8u32);
            { let target = c.get(7,32); c.push(m, 0x45e1d1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e1d6; continue;
            }
            0x45e1d6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6983e0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e1ec);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e20e; continue; }
            pc = 0x45e1f3; continue;
            }
            0x45e1f3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x6983e4u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6983f0u32);
            c.push(m,0x6983f8u32);
            { let target = c.get(7,32); c.push(m, 0x45e209); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e20e; continue;
            }
            0x45e20e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x184),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698400u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e224);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e246; continue; }
            pc = 0x45e22b; continue;
            }
            0x45e22b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698404u32);
            c.push(m,c.get(0,32));
            c.push(m,0x698410u32);
            c.push(m,0x69841cu32);
            { let target = c.get(7,32); c.push(m, 0x45e241); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e246; continue;
            }
            0x45e246 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x164),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698424u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e25c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e27e; continue; }
            pc = 0x45e263; continue;
            }
            0x45e263 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698428u32);
            c.push(m,c.get(0,32));
            c.push(m,0x698434u32);
            c.push(m,0x698440u32);
            { let target = c.get(7,32); c.push(m, 0x45e279); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e27e; continue;
            }
            0x45e27e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x168),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698448u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e294);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e2b6; continue; }
            pc = 0x45e29b; continue;
            }
            0x45e29b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x69844cu32);
            c.push(m,c.get(0,32));
            c.push(m,0x698458u32);
            c.push(m,0x698464u32);
            { let target = c.get(7,32); c.push(m, 0x45e2b1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e2b6; continue;
            }
            0x45e2b6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x188),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x69846cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e2cc);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e2ee; continue; }
            pc = 0x45e2d3; continue;
            }
            0x45e2d3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698470u32);
            c.push(m,c.get(0,32));
            c.push(m,0x69847cu32);
            c.push(m,0x698488u32);
            { let target = c.get(7,32); c.push(m, 0x45e2e9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e2ee; continue;
            }
            0x45e2ee => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x698490u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e304);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e326; continue; }
            pc = 0x45e30b; continue;
            }
            0x45e30b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x698494u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6984a0u32);
            c.push(m,0x6984acu32);
            { let target = c.get(7,32); c.push(m, 0x45e321); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e326; continue;
            }
            0x45e326 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x190),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6984b4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e33c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e35e; continue; }
            pc = 0x45e343; continue;
            }
            0x45e343 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x6984b8u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6984c4u32);
            c.push(m,0x6984ccu32);
            { let target = c.get(7,32); c.push(m, 0x45e359); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e35e; continue;
            }
            0x45e35e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6984d4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x45e374);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x45e396; continue; }
            pc = 0x45e37b; continue;
            }
            0x45e37b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x6984d8u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6984e4u32);
            c.push(m,0x6984f0u32);
            { let target = c.get(7,32); c.push(m, 0x45e391); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            pc = 0x45e396; continue;
            }
            0x45e396 => {
            c.push(m,0x6984f8u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let target = c.get(7,32); c.push(m, 0x45e3a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x45e3b0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
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

pub fn options_window_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45f1d0;
    loop {
        match pc {
            0x45f1d0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65ed3du32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x78u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0xd),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            c.push(m,c.get(6,32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x45f21b; continue; }
            pc = 0x45f208; continue;
            }
            0x45f208 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45f21b; continue; }
            pc = 0x45f211; continue;
            }
            0x45f211 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45f21b);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f21b; continue;
            }
            0x45f21b => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x45f24f; continue; }
            pc = 0x45f237; continue;
            }
            0x45f237 => {
            c.set(1,32,c.get(5,32));
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            c.push(m, 0x45f245);
            { let args = []; c.r[0] = host.call(m, 0x460250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xfffffffdu32);
            pc = 0x4601c8; continue;
            }
            0x45f24f => {
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45f263; continue; }
            pc = 0x45f258; continue;
            }
            0x45f258 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x45f263);
            { let args = []; c.r[0] = host.call(m, 0x42c080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f263; continue;
            }
            0x45f263 => {
            c.set(6,32,c.r[5].wrapping_add(0x2a8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x45f271; continue;
            }
            0x45f271 => {
            c.push(m, 0x45f276);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45f28e; continue; }
            pc = 0x45f27a; continue;
            }
            0x45f27a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x45f271; continue; }
            pc = 0x45f28e; continue;
            }
            0x45f28e => {
            { let a = c.get(7,32); c.sub(a,0x13u32,32); }
            c.set(0,32,0x3u32);
            c.set(3,32,0x5u32);
            if c.cond(7) { pc = 0x45fa4f; continue; }
            pc = 0x45f2a1; continue;
            }
            0x45f2a1 => {
            pc = m.r32(c.r[7].wrapping_mul(4).wrapping_add(0x4601e4)); continue;
            }
            0x45f2a8 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.push(m, 0x45f2b3);
            { let args = []; c.r[0] = host.call(m, 0x402d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f2b3; continue;
            }
            0x45f2b3 => {
            { let a = c.r[5].wrapping_add(0x168); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0x164); wm(m,a,32,c.get(0,32)); }
            pc = 0x45f304; continue;
            }
            0x45f2ca => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.push(m, 0x45f2d5);
            { let args = []; c.r[0] = host.call(m, 0x402cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f2b3; continue;
            }
            0x45f2d7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.push(m, 0x45f2e2);
            { let args = []; c.r[0] = host.call(m, 0x402da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f2ef; continue;
            }
            0x45f2e4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.push(m, 0x45f2ef);
            { let args = []; c.r[0] = host.call(m, 0x402d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45f2ef; continue;
            }
            0x45f2ef => {
            { let a = c.r[5].wrapping_add(0x164); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1cc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[5].wrapping_add(0x168); wm(m,a,32,c.get(0,32)); }
            pc = 0x45f304; continue;
            }
            0x45f304 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.r[5].wrapping_add(0x390));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f319); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f328); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f337); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f360);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x560));
            c.set(6,32,c.r[5].wrapping_add(0x478));
            pc = 0x45f9f9; continue;
            }
            0x45f371 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x16c),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0x16c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x45f39f; continue; }
            pc = 0x45f384; continue;
            }
            0x45f384 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x648),32));
            c.set(1,32,c.r[5].wrapping_add(0x648));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x16c); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f39f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f39f; continue;
            }
            0x45f39f => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x730),32));
            c.set(6,32,c.r[5].wrapping_add(0x730));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f3b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x648));
            pc = 0x45f97b; continue;
            }
            0x45f3bd => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x16c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x16c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x45f3e3; continue; }
            pc = 0x45f3d0; continue;
            }
            0x45f3d0 => {
            c.set(1,32,c.r[5].wrapping_add(0x730));
            { let a = c.r[5].wrapping_add(0x16c); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f3e3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f3e3; continue;
            }
            0x45f3e3 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x648),32));
            c.set(6,32,c.r[5].wrapping_add(0x648));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f3f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x730));
            pc = 0x45f9f9; continue;
            }
            0x45f401 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x17c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45f424; continue; }
            pc = 0x45f40b; continue;
            }
            0x45f40b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x818),32));
            c.set(1,32,c.r[5].wrapping_add(0x818));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x17c); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f422); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f42b; continue;
            }
            0x45f424 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x17c); wm(m,a,32,c.get(0,32)); }
            pc = 0x45f42b; continue;
            }
            0x45f42b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x900),32));
            c.set(6,32,c.r[5].wrapping_add(0x900));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f43e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x818));
            pc = 0x45f97b; continue;
            }
            0x45f449 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x17c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x45f45f; continue; }
            pc = 0x45f453; continue;
            }
            0x45f453 => {
            { let a = c.r[5].wrapping_add(0x17c); wm(m,a,32,0x0u32); }
            pc = 0x45f486; continue;
            }
            0x45f45f => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[5].wrapping_add(0x17c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x45f486; continue; }
            pc = 0x45f46b; continue;
            }
            0x45f46b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x900),32));
            c.set(1,32,c.r[5].wrapping_add(0x900));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x17c); wm(m,a,32,0x4u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f486); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f486; continue;
            }
            0x45f486 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x818),32));
            c.set(6,32,c.r[5].wrapping_add(0x818));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f499); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x900));
            pc = 0x45f9f9; continue;
            }
            0x45f4a4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x9e8),32));
            c.set(6,32,c.r[5].wrapping_add(0x9e8));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x180); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f4c1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xad0),32));
            c.set(7,32,c.r[5].wrapping_add(0xad0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f4d4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f4e3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f4f2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f501); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f52a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x698529u32; wm(m,a,8,0x0u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f536 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x9e8),32));
            c.set(6,32,c.r[5].wrapping_add(0x9e8));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x180); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f553); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xad0),32));
            c.set(7,32,c.r[5].wrapping_add(0xad0));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f566); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f575); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f584); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f593); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f5bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x698529u32; wm(m,a,8,0x1u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f5c8 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xbb8),32));
            c.set(6,32,c.r[5].wrapping_add(0xbb8));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x184); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f5e5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xca0),32));
            c.set(7,32,c.r[5].wrapping_add(0xca0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f5f8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f607); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f616); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f625); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f64e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x698528u32; wm(m,a,8,0x0u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f65a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xbb8),32));
            c.set(6,32,c.r[5].wrapping_add(0xbb8));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x184); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f677); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xca0),32));
            c.set(7,32,c.r[5].wrapping_add(0xca0));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f68a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f699); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f6a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f6b7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f6e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x698528u32; wm(m,a,8,0x1u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f6ec => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xd88),32));
            c.set(6,32,c.r[5].wrapping_add(0xd88));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x188); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f709); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe70),32));
            c.set(7,32,c.r[5].wrapping_add(0xe70));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f71c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f72b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f73a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f749); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f772);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdb8u32,32));
            { let a = c.r[0].wrapping_add(0x1dd); wm(m,a,8,0x0u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f783 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xd88),32));
            c.set(6,32,c.r[5].wrapping_add(0xd88));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x188); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f7a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xe70),32));
            c.set(7,32,c.r[5].wrapping_add(0xe70));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f7b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f7c2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f7d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45f7e0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45f809);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdb8u32,32));
            { let a = c.r[0].wrapping_add(0x1dd); wm(m,a,8,0x1u32); }
            pc = 0x45fa4f; continue;
            }
            0x45f81a => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xf58),32));
            c.set(6,32,c.r[5].wrapping_add(0xf58));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x18c); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f837); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1040),32));
            c.set(7,32,c.r[5].wrapping_add(0x1040));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f84a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f859); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fa08; continue;
            }
            0x45f85e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xf58),32));
            c.set(6,32,c.r[5].wrapping_add(0xf58));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x18c); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f87b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1040),32));
            c.set(7,32,c.r[5].wrapping_add(0x1040));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f88e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f9f9; continue;
            }
            0x45f893 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x190),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x45f8c1; continue; }
            pc = 0x45f8a6; continue;
            }
            0x45f8a6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1128),32));
            c.set(1,32,c.r[5].wrapping_add(0x1128));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f8c1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f8c1; continue;
            }
            0x45f8c1 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1210),32));
            c.set(6,32,c.r[5].wrapping_add(0x1210));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f8d4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x1128));
            pc = 0x45f97b; continue;
            }
            0x45f8df => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x190),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x45f905; continue; }
            pc = 0x45f8f2; continue;
            }
            0x45f8f2 => {
            c.set(1,32,c.r[5].wrapping_add(0x1210));
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f905); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f905; continue;
            }
            0x45f905 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1128),32));
            c.set(6,32,c.r[5].wrapping_add(0x1128));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f918); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x1210));
            pc = 0x45f9f9; continue;
            }
            0x45f923 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x194),32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x194); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45f93d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x194),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x45f962; continue; }
            pc = 0x45f947; continue;
            }
            0x45f947 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12f8),32));
            c.set(1,32,c.r[5].wrapping_add(0x12f8));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x194); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f962); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f962; continue;
            }
            0x45f962 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x13e0),32));
            c.set(6,32,c.r[5].wrapping_add(0x13e0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f975); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x12f8));
            pc = 0x45f97b; continue;
            }
            0x45f97b => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45f98a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45f999); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            pc = 0x45fa23; continue;
            }
            0x45f9a7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x194),32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x194); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45f9c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0x194),32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x45f9e0; continue; }
            pc = 0x45f9c9; continue;
            }
            0x45f9c9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x13e0),32));
            c.set(1,32,c.r[5].wrapping_add(0x13e0));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x194); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x45f9e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45f9e0; continue;
            }
            0x45f9e0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x12f8),32));
            c.set(6,32,c.r[5].wrapping_add(0x12f8));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x45f9f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[5].wrapping_add(0x13e0));
            pc = 0x45f9f9; continue;
            }
            0x45f9f9 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x45fa08); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fa08; continue;
            }
            0x45fa08 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x45fa17); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            pc = 0x45fa23; continue;
            }
            0x45fa23 => {
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x45fa26); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45fa4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fa4f; continue;
            }
            0x45fa4f => {
            c.set(1,32,c.r[5].wrapping_add(0x2728));
            c.push(m, 0x45fa5a);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fa87; continue; }
            pc = 0x45fa5e; continue;
            }
            0x45fa5e => {
            c.set(6,32,c.r[5].wrapping_add(0x1990));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fa6b);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fa74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fa7b);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fa87);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fa87; continue;
            }
            0x45fa87 => {
            c.set(1,32,c.r[5].wrapping_add(0x29e0));
            c.push(m, 0x45fa92);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fabf; continue; }
            pc = 0x45fa96; continue;
            }
            0x45fa96 => {
            c.set(6,32,c.r[5].wrapping_add(0x1990));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45faa3);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45faac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fab3);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fabf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fabf; continue;
            }
            0x45fabf => {
            c.set(1,32,c.r[5].wrapping_add(0x2810));
            c.push(m, 0x45faca);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45faf7; continue; }
            pc = 0x45face; continue;
            }
            0x45face => {
            c.set(6,32,c.r[5].wrapping_add(0x1d40));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fadb);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fae4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45faeb);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45faf7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45faf7; continue;
            }
            0x45faf7 => {
            c.set(1,32,c.r[5].wrapping_add(0x2ac8));
            c.push(m, 0x45fb02);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fb2f; continue; }
            pc = 0x45fb06; continue;
            }
            0x45fb06 => {
            c.set(6,32,c.r[5].wrapping_add(0x1d40));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb13);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb23);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fb2f; continue;
            }
            0x45fb2f => {
            c.set(1,32,c.r[5].wrapping_add(0x28f8));
            c.push(m, 0x45fb3a);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fb67; continue; }
            pc = 0x45fb3e; continue;
            }
            0x45fb3e => {
            c.set(6,32,c.r[5].wrapping_add(0x20f0));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb4b);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb5b);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb67);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fb67; continue;
            }
            0x45fb67 => {
            c.set(1,32,c.r[5].wrapping_add(0x2bb0));
            c.push(m, 0x45fb72);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fb9f; continue; }
            pc = 0x45fb76; continue;
            }
            0x45fb76 => {
            c.set(6,32,c.r[5].wrapping_add(0x20f0));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb83);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb8c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fb93);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fb9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fb9f; continue;
            }
            0x45fb9f => {
            c.set(6,32,c.r[5].wrapping_add(0x14c8));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fbac);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45fc48; continue; }
            pc = 0x45fbb4; continue;
            }
            0x45fbb4 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x45fbbd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x45fbc4);
            { let args = []; c.r[0] = host.call(m, 0x45df10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0a8u32,32));
            c.set(2,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fbdc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fbe7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x45fbf8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x431cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x45fc06); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x45fc26; continue; }
            pc = 0x45fc0b; continue;
            }
            0x45fc0b => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fc1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,0xfffffffdu32);
            pc = 0x4601c8; continue;
            }
            0x45fc26 => {
            c.set(2,32,rm(m,0x66e098u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45fc38);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45fc48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45fc48; continue;
            }
            0x45fc48 => {
            c.set(6,32,c.r[5].wrapping_add(0x15bc));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45fc55);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45ff48; continue; }
            pc = 0x45fc5d; continue;
            }
            0x45fc5d => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x45fc66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x45fc6d);
            { let args = []; c.r[0] = host.call(m, 0x45df10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x66e0a8u32,32));
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fc84);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fc8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0xd),8));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(7) { pc = 0x45fcbb; continue; }
            pc = 0x45fc9e; continue;
            }
            0x45fc9e => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x460234)); continue;
            }
            0x45fca5 => {
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            pc = 0x45fcae; continue;
            }
            0x45fcaa => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            pc = 0x45fcae; continue;
            }
            0x45fcae => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x1u32);
            c.push(m, 0x45fcbb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x431cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x45fcbb; continue;
            }
            0x45fcbb => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x45fcc9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x45ff26; continue; }
            pc = 0x45fcd2; continue;
            }
            0x45fcd2 => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45fce3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4601be; continue; }
            pc = 0x45fcf0; continue;
            }
            0x45fcf0 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,8,0x1u32); }
            c.push(m, 0x45fd1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m, 0x45fd2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x45fd37);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x2u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,0xac44u32); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,0x10u32); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(3,32)); }
            c.push(m,0x6dcb70u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x45fd8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dcb74u32);
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m, 0x45fd99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dcb8cu32,32));
            c.set(2,32,rm(m,0x6dcb94u32,32));
            c.set(1,32,0x5u32);
            c.set(6,32,0x6dcb78u32);
            c.set(7,32,c.r[4].wrapping_add(0x48));
            c.rep_movsd(m);
            c.set(1,32,rm(m,0x6dcb90u32,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x164),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcb98u32,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcba0u32,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcb9cu32,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcba4u32,32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcbacu32,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcba8u32,32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcbb0u32,32));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcbb4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x168),32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x45fe93; continue; }
            pc = 0x45fe37; continue;
            }
            0x45fe37 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45fe4d; continue; }
            pc = 0x45fe3b; continue;
            }
            0x45fe3b => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x320u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x258u32); }
            pc = 0x45fe78; continue;
            }
            0x45fe4d => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x45fe64; continue; }
            pc = 0x45fe52; continue;
            }
            0x45fe52 => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x400u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x300u32); }
            pc = 0x45fe78; continue;
            }
            0x45fe64 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x45fe78; continue; }
            pc = 0x45fe68; continue;
            }
            0x45fe68 => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x500u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x400u32); }
            pc = 0x45fe78; continue;
            }
            0x45fe78 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x45fe86; continue; }
            pc = 0x45fe7c; continue;
            }
            0x45fe7c => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x10u32); }
            pc = 0x45fe93; continue;
            }
            0x45fe86 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x45fe93; continue; }
            pc = 0x45fe8b; continue;
            }
            0x45fe8b => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x20u32); }
            pc = 0x45fe93; continue;
            }
            0x45fe93 => {
            c.set(0,32,rm(m,0x6cc40cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x45fea3);
            { let args = []; c.r[0] = host.call(m, 0x4bccc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x45fee8; continue; }
            pc = 0x45feac; continue;
            }
            0x45feac => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x45feb7);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x45fecd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x45fedb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,0x3u32); }
            pc = 0x45feff; continue;
            }
            0x45fee8 => {
            { let a = c.r[5].wrapping_add(0x168); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0x164); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,0x4u32); }
            pc = 0x45feff; continue;
            }
            0x45feff => {
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x45ff08);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x45ff1c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xfffffffdu32);
            pc = 0x4601c8; continue;
            }
            0x45ff26 => {
            c.set(0,32,rm(m,0x66e098u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x45ff37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x45ff48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45ff48; continue;
            }
            0x45ff48 => {
            c.set(6,32,c.r[5].wrapping_add(0x16b0));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45ff55);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x460040; continue; }
            pc = 0x45ff5d; continue;
            }
            0x45ff5d => {
            c.set(2,32,rm(m,0x69ae50u32,32));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x45ff72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69ae4cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xa0); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x45ff8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x45ffa1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x45ffaf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x460040; continue; }
            pc = 0x45ffb8; continue;
            }
            0x45ffb8 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x45ffc1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45ffcb);
            { let args = []; c.r[0] = host.call(m, 0x549130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x45ffd2);
            { let args = []; c.r[0] = host.call(m, 0x45df10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x45ffdd);
            { let args = []; c.r[0] = host.call(m, 0x442d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x460036; continue; }
            pc = 0x45ffe6; continue;
            }
            0x45ffe6 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x460021; continue; }
            pc = 0x45fffa; continue;
            }
            0x45fffa => {
            c.push(m,0x69850cu32);
            c.push(m, 0x460004);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x46000f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698514u32);
            pc = 0x460026; continue;
            }
            0x460021 => {
            c.push(m,0x69851cu32);
            pc = 0x460026; continue;
            }
            0x460026 => {
            c.push(m, 0x46002b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x460036);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x460036; continue;
            }
            0x460036 => {
            c.set(0,32,0xfffffffbu32);
            pc = 0x4601c8; continue;
            }
            0x460040 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1990),32));
            c.set(7,32,c.r[5].wrapping_add(0x1990));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc4),32); c.push(m, 0x460054); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46006b; continue; }
            pc = 0x460058; continue;
            }
            0x460058 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x46005f);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46006b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46006b; continue;
            }
            0x46006b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1d40),32));
            c.set(6,32,c.r[5].wrapping_add(0x1d40));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc4),32); c.push(m, 0x46007f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x460096; continue; }
            pc = 0x460083; continue;
            }
            0x460083 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x46008a);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x460096);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x460096; continue;
            }
            0x460096 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x20f0),32));
            c.set(6,32,c.r[5].wrapping_add(0x20f0));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc4),32); c.push(m, 0x4600aa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4600c1; continue; }
            pc = 0x4600ae; continue;
            }
            0x4600ae => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4600b5);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4600c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4600c1; continue;
            }
            0x4600c1 => {
            c.set(6,32,c.r[5].wrapping_add(0x1898));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4600ce);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4600ec; continue; }
            pc = 0x4600d2; continue;
            }
            0x4600d2 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x4600db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4600e2);
            { let args = []; c.r[0] = host.call(m, 0x460250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xfffffffdu32);
            pc = 0x4601c8; continue;
            }
            0x4600ec => {
            c.set(6,32,c.r[5].wrapping_add(0x17a4));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4600f9);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4601c5; continue; }
            pc = 0x460101; continue;
            }
            0x460101 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x46010a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x460111);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46016b; continue; }
            pc = 0x46012f; continue;
            }
            0x46012f => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x46013a);
            { let args = []; c.r[0] = host.call(m, 0x442d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46016b; continue; }
            pc = 0x460143; continue;
            }
            0x460143 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[1].wrapping_add(0x2884); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(2,32); let r = c.add(a,0xfffffc18u32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x28c4); wm(m,a,32,c.get(2,32)); }
            pc = 0x46016b; continue;
            }
            0x46016b => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x460172);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x21),8));
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4601b7; continue; }
            pc = 0x46018c; continue;
            }
            0x46018c => {
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4601b7; continue; }
            pc = 0x460195; continue;
            }
            0x460195 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[1].wrapping_add(0x2884); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[2].wrapping_add(0x28c4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4601b7; continue;
            }
            0x4601b7 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4601be);
            { let args = []; c.r[0] = host.call(m, 0x45df10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4601be; continue;
            }
            0x4601be => {
            c.set(0,32,0xfffffffdu32);
            pc = 0x4601c8; continue;
            }
            0x4601c5 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x4601c8; continue;
            }
            0x4601c8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x84u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn options_window_restore_snapshot(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x460250;
    loop {
        match pc {
            0x460250 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x19c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x198),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a0),32));
            { let a = c.r[6].wrapping_add(0x170); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1a8),32));
            { let a = c.r[6].wrapping_add(0x16c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a4),32));
            { let a = c.r[6].wrapping_add(0x17c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1b4),32));
            { let a = c.r[6].wrapping_add(0x174); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ac),32));
            { let a = c.r[6].wrapping_add(0x178); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1cc),32));
            { let a = c.r[6].wrapping_add(0x180); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x184); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4602bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1cc),32));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x164); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4602d4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x402980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x16c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x648),32));
            { let a = c.r[6].wrapping_add(0x168); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x648));
            { let a = c.r[6].wrapping_add(0x188); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x460301); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x730),32));
            c.set(1,32,c.r[6].wrapping_add(0x730));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x46031d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x818),32));
            c.set(1,32,c.r[6].wrapping_add(0x818));
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x460339); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x900),32));
            c.set(1,32,c.r[6].wrapping_add(0x900));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x460355); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9e8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9e8));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x460370); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xad0),32));
            c.set(7,32,0x1u32);
            c.set(1,32,c.r[6].wrapping_add(0xad0));
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x460390); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x184),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbb8),32));
            c.set(1,32,c.r[6].wrapping_add(0xbb8));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4603ab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x184),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xca0),32));
            c.set(1,32,c.r[6].wrapping_add(0xca0));
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4603c6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x188),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd88),32));
            c.set(1,32,c.r[6].wrapping_add(0xd88));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4603e1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x188),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe70),32));
            c.set(1,32,c.r[6].wrapping_add(0xe70));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4603fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x170),32));
            c.set(7,32,c.r[6].wrapping_add(0x1990));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x460410);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x174),32));
            c.set(3,32,c.r[6].wrapping_add(0x1d40));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x460424);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x178),32));
            c.set(5,32,c.r[6].wrapping_add(0x20f0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x460438);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d0bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(0,32,0x1u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = 0x698529u32; wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x184),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(7,32));
            c.set(2,8,u32::from(c.cond(4)));
            { let a = 0x698528u32; wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x460466);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x460472);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x460479);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x460485);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x46048c);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x460498);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x16c),8));
            { let a = c.r[1].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x180),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4604bc);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4604cc);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[2].wrapping_add(0x23); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4604dc);
            { let args = []; c.r[0] = host.call(m, 0x4d0e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x184),8));
            { let a = c.r[0].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x180),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x168),8));
            { let a = c.r[1].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x164),8));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x188),32));
            c.set(2,32,rm(m,0x6cbdb8u32,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2].wrapping_add(0x1dd); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn options_window_set_game_speed_local(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4605e0;
    loop {
        match pc {
            0x4605e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0x1a8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x17c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x460609);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x460675; continue; }
            pc = 0x46060d; continue;
            }
            0x46060d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x818),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x900));
            { let a = c.get(6,32); let r = c.add(a,0x818u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x46062d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x46063c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x46064b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x460674);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x460675; continue;
            }
            0x460675 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn options_window_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x460710;
    loop {
        match pc {
            0x460710 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x460718);
            { let args = []; c.r[0] = host.call(m, 0x45cc40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x460728; continue; }
            pc = 0x46071f; continue;
            }
            0x46071f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x460725);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x460728; continue;
            }
            0x460728 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_main_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x463180;
    loop {
        match pc {
            0x463180 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ee38u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4631a2);
            { let args = []; c.r[0] = host.call(m, 0x4c0980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x4631b2);
            { let args = []; c.r[0] = host.call(m, 0x42eab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6]; wm(m,a,32,0x66e3c0u32); }
            { let a = c.r[6].wrapping_add(0x2fc); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x304); wm(m,a,32,0x0u32); }
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

pub fn game_main_window_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4631e0;
    loop {
        match pc {
            0x4631e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ee4au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66e3c0u32); }
            c.set(1,32,c.r[6].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x463213);
            { let args = []; c.r[0] = host.call(m, 0x42eb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x463222);
            { let args = []; c.r[0] = host.call(m, 0x4c09a0, c.r[1], &args); }
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

pub fn game_main_window_shutdown(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x463c60;
    loop {
        match pc {
            0x463c60 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            { let target = rm(m,0x66a56cu32,32); c.push(m, 0x463c6a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x463c77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,0x6dcd28u32);
            pc = 0x463c7c; continue;
            }
            0x463c7c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x463c83);
            { let args = []; c.r[0] = host.call(m, 0x4c11d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0xa8u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6dd118u32,32); }
            if c.cond(12) { pc = 0x463c7c; continue; }
            pc = 0x463c91; continue;
            }
            0x463c91 => {
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x463c9b);
            { let args = []; c.r[0] = host.call(m, 0x4b8880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x463ca5);
            { let args = []; c.r[0] = host.call(m, 0x4b39e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x463cb5; continue; }
            pc = 0x463caf; continue;
            }
            0x463caf => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x463cb5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x463cb5; continue;
            }
            0x463cb5 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6cbb40u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x463ccd; continue; }
            pc = 0x463cc4; continue;
            }
            0x463cc4 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x463cca);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x463ccd; continue;
            }
            0x463ccd => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = 0x6cc3e8u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x463ce9; continue; }
            pc = 0x463cdd; continue;
            }
            0x463cdd => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x463ce3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x463ce9; continue;
            }
            0x463ce9 => {
            c.set(6,32,rm(m,0x6cbdb8u32,32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x463d09; continue; }
            pc = 0x463cf3; continue;
            }
            0x463cf3 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x463cfa);
            { let args = []; c.r[0] = host.call(m, 0x441ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x463d00);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6cbdb8u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x463d09; continue;
            }
            0x463d09 => {
            c.push(m, 0x463d0e);
            { let args = []; c.r[0] = host.call(m, 0x4bc930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x463d13);
            { let args = []; c.r[0] = host.call(m, 0x4bc770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_main_window_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464380;
    loop {
        match pc {
            0x464380 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x464388);
            { let args = []; c.r[0] = host.call(m, 0x4631e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x464398; continue; }
            pc = 0x46438f; continue;
            }
            0x46438f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x464395);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x464398; continue;
            }
            0x464398 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn find_game_cd_drive_letter(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464800;
    loop {
        match pc {
            0x464800 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef82u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x46482c);
            { let args = []; c.r[0] = host.call(m, 0x464630, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0xffu32); }
            c.push(m, 0x46483f);
            { let args = []; c.r[0] = host.call(m, 0x653b0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x1u32); }
            if c.cond(6) { pc = 0x464935; continue; }
            pc = 0x46485e; continue;
            }
            0x46485e => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(c.r[7]),8),8)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x698adcu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x464872);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x8000u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x46488a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4648c5; continue; }
            pc = 0x46488e; continue;
            }
            0x46488e => {
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x46489e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d3a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4648a9);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x464923; continue; }
            pc = 0x4648ae; continue;
            }
            0x4648ae => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.get(0,8); c.sub(a,0x50u32,8); }
            if c.cond(5) { pc = 0x464923; continue; }
            pc = 0x4648b7; continue;
            }
            0x4648b7 => {
            { let a = c.get(4,8); c.sub(a,0x32u32,8); }
            if c.cond(5) { pc = 0x464923; continue; }
            pc = 0x4648bc; continue;
            }
            0x4648bc => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[7]),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            pc = 0x464935; continue;
            }
            0x4648c5 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[7]),8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x698ae8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4648d9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x0u32);
            c.push(m,0x8000u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4648f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x464923; continue; }
            pc = 0x4648f5; continue;
            }
            0x4648f5 => {
            c.set(0,32,c.r[4].wrapping_add(0x1a));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x464905);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d3a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x464910);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x464923; continue; }
            pc = 0x464915; continue;
            }
            0x464915 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x1a),16));
            { let a = c.get(0,8); c.sub(a,0x50u32,8); }
            if c.cond(5) { pc = 0x464923; continue; }
            pc = 0x46491e; continue;
            }
            0x46491e => {
            { let a = c.get(4,8); c.sub(a,0x32u32,8); }
            if c.cond(4) { pc = 0x46492e; continue; }
            pc = 0x464923; continue;
            }
            0x464923 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x46485e; continue; }
            pc = 0x46492c; continue;
            }
            0x46492c => {
            pc = 0x464935; continue;
            }
            0x46492e => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[7]),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            pc = 0x464935; continue;
            }
            0x464935 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x464942; continue; }
            pc = 0x464939; continue;
            }
            0x464939 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46493f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x464942; continue;
            }
            0x464942 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x464950);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x464961);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_message_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x466d70;
    loop {
        match pc {
            0x466d70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f07cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x466d93);
            { let args = []; c.r[0] = host.call(m, 0x462140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xa0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x466da4);
            { let args = []; c.r[0] = host.call(m, 0x4209e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xbc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x466db4);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.push(m, 0x466dc4);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x28c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            c.push(m, 0x466dd4);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x374); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x378); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.push(m,0x25u32);
            c.set(2,32,c.r[6].wrapping_add(0x3ec));
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x6u32); }
            c.push(m, 0x466e0a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6]; wm(m,a,32,0x66e7f0u32); }
            { let a = c.r[6].wrapping_add(0x3b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x384); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3c0); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x3ea); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3d4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3eb); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3e8); wm(m,a,8,0x3cu32); }
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

pub fn event_message_window_present_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x467090;
    loop {
        match pc {
            0x467090 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4670aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bbaa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_message_window_slot3_release_arg_return_zero(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4673e0;
    loop {
        match pc {
            0x4673e0 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4673e9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_window_open_or_defer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x469380;
    loop {
        match pc {
            0x469380 => {
            c.set(0,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(5) { pc = 0x4693a1; continue; }
            pc = 0x469391; continue;
            }
            0x469391 => {
            { let a = rm(m,0x6e59ccu32,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4693a1; continue; }
            pc = 0x46939a; continue;
            }
            0x46939a => {
            c.set(3,32,0x1u32);
            pc = 0x4693a3; continue;
            }
            0x4693a1 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4693a3; continue;
            }
            0x4693a3 => {
            c.push(m, 0x4693a8);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x38),8));
            c.set(7,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4693dd; continue; }
            pc = 0x4693cd; continue;
            }
            0x4693cd => {
            { let a = c.r[6].wrapping_add(0x3ea); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4693dd => {
            { let a = c.r[6].wrapping_add(0x3ea); wm(m,a,8,0x0u32); }
            c.set(3,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4693f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3dc); wm(m,a,16,0x11u32); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x46940b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x46941b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x64),32); c.push(m, 0x46944e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x469455);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x469461); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x3));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x3));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x469494);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x28c));
            c.set(0,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x4694ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4694c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4694c9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); let r = c.sub(a,0xfu32,32); c.set(3,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4694dd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.push(m,0x64u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,0xfu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x4694f7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x469503); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_window_close(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x469510;
    loop {
        match pc {
            0x469510 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x46951d);
            { let args = []; c.r[0] = host.call(m, 0x462320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x469529);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x469535); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x46953c);
            { let args = []; c.r[0] = host.call(m, 0x46ac50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x469547);
            { let args = []; c.r[0] = host.call(m, 0x4425d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            if c.cond(4) { pc = 0x4695a2; continue; }
            pc = 0x469555; continue;
            }
            0x469555 => {
            { let a = c.get(0,32); c.sub(a,0xbu32,32); }
            if c.cond(4) { pc = 0x4695a2; continue; }
            pc = 0x46955a; continue;
            }
            0x46955a => {
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            if c.cond(4) { pc = 0x4695a2; continue; }
            pc = 0x46955f; continue;
            }
            0x46955f => {
            { let a = c.get(0,32); c.sub(a,0xdu32,32); }
            if c.cond(4) { pc = 0x4695a2; continue; }
            pc = 0x469564; continue;
            }
            0x469564 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(2) { pc = 0x4695b7; continue; }
            pc = 0x469570; continue;
            }
            0x469570 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x469579);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4695a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4695b7; continue;
            }
            0x4695a2 => {
            c.set(1,32,rm(m,0x6cc93cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3e0),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4695b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4695b7; continue;
            }
            0x4695b7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            c.set(1,32,c.r[6].wrapping_add(0xbc));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4695c9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a4));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4695db); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            c.set(3,8,0x1u32);
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x469656; continue; }
            pc = 0x4695f7; continue;
            }
            0x4695f7 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3eb),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3de),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3de),16));
            if c.cond(5) { pc = 0x46962d; continue; }
            pc = 0x46960e; continue;
            }
            0x46960e => {
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.set(1,8,u32::from(c.cond(2)));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[0].wrapping_add(0x2639); wm(m,a,8,c.get(3,8)); }
            pc = 0x46964a; continue;
            }
            0x46962d => {
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.set(1,8,u32::from(c.cond(2)));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[0].wrapping_add(0x2637); wm(m,a,8,c.get(3,8)); }
            pc = 0x46964a; continue;
            }
            0x46964a => {
            { let a = c.r[0].wrapping_add(0x263a); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x2638); wm(m,a,8,c.get(2,8)); }
            pc = 0x469656; continue;
            }
            0x469656 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1au32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x19u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x4696d1; continue; }
            pc = 0x469670; continue;
            }
            0x469670 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3eb),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3de),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3de),16));
            if c.cond(5) { pc = 0x4696a7; continue; }
            pc = 0x469687; continue;
            }
            0x469687 => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.set(0,8,u32::from(c.cond(2)));
            { let a = c.get(1,32); c.sub(a,0x19u32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); let r = c.or(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[1].wrapping_add(0x2639); wm(m,a,8,c.get(3,8)); }
            pc = 0x4696c5; continue;
            }
            0x4696a7 => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.set(0,8,u32::from(c.cond(2)));
            { let a = c.get(1,32); c.sub(a,0x19u32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); let r = c.or(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[1].wrapping_add(0x2637); wm(m,a,8,c.get(3,8)); }
            pc = 0x4696c5; continue;
            }
            0x4696c5 => {
            { let a = c.r[1].wrapping_add(0x263a); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x2638); wm(m,a,8,c.get(2,8)); }
            pc = 0x4696d1; continue;
            }
            0x4696d1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(1,32,c.r[6].wrapping_add(0x28c));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4696e3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x3c0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_window_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46af30;
    loop {
        match pc {
            0x46af30 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x46af38);
            { let args = []; c.r[0] = host.call(m, 0x466e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x46af48; continue; }
            pc = 0x46af3f; continue;
            }
            0x46af3f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x46af45);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46af48; continue;
            }
            0x46af48 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn can_player_inspect_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x494740;
    loop {
        match pc {
            0x494740 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(3,32));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x49477a; continue; }
            pc = 0x49476e; continue;
            }
            0x49476e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x49477a => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x4947a4; continue; }
            pc = 0x49478d; continue;
            }
            0x49478d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x4947a4; continue;
            }
            0x4947a4 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x10u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4947bd; continue; }
            pc = 0x4947b5; continue;
            }
            0x4947b5 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4947bd; continue; }
            pc = 0x4947b9; continue;
            }
            0x4947b9 => {
            c.set(5,32,c.get(0,32));
            pc = 0x4947f2; continue;
            }
            0x4947bd => {
            c.set(5,16,rm(m,c.r[2].wrapping_add(0x12),16));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            c.set(2,16,rm(m,0x6dd894u32,16));
            pc = 0x4947cc; continue;
            }
            0x4947cc => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0],8));
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x4947e1; continue;
            }
            0x4947e1 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x4947cc; continue; }
            pc = 0x4947ea; continue;
            }
            0x4947ea => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4947f2; continue;
            }
            0x4947f2 => {
            { let a = c.get(0,16); c.sub(a,0x14u32,16); }
            if c.cond(4) { pc = 0x4947fe; continue; }
            pc = 0x4947f8; continue;
            }
            0x4947f8 => {
            { let a = c.get(0,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x49482f; continue; }
            pc = 0x4947fe; continue;
            }
            0x4947fe => {
            c.set(0,32,rm(m,0x6e59ccu32,32));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x49482f; continue; }
            pc = 0x494807; continue;
            }
            0x494807 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m, 0x494819);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6087f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x494829; continue; }
            pc = 0x49481d; continue;
            }
            0x49481d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x494829 => {
            c.set(6,32,rm(m,0x6dfc14u32,32));
            pc = 0x49482f; continue;
            }
            0x49482f => {
            { let a = c.get(5,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x494908; continue; }
            pc = 0x494839; continue;
            }
            0x494839 => {
            { let a = c.get(5,16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x494908; continue; }
            pc = 0x494843; continue;
            }
            0x494843 => {
            { let a = c.get(5,16); c.sub(a,0x1u32,16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x494849; continue;
            }
            0x494849 => {
            { let a = c.get(5,16); c.sub(a,0x4u32,16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x49484f; continue;
            }
            0x49484f => {
            { let a = c.get(5,16); c.sub(a,0x5u32,16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x494855; continue;
            }
            0x494855 => {
            { let a = c.get(5,16); c.sub(a,0x6u32,16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x49485b; continue;
            }
            0x49485b => {
            { let a = c.get(5,16); c.sub(a,0x2u32,16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x494861; continue;
            }
            0x494861 => {
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x494870; continue; }
            pc = 0x494866; continue;
            }
            0x494866 => {
            { let a = c.get(5,16); c.sub(a,0x3u32,16); }
            if c.cond(5) { pc = 0x494977; continue; }
            pc = 0x494870; continue;
            }
            0x494870 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x49487b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(6,16,rm(m,0x6dd894u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(6,16),16); }
            if c.cond(3) { pc = 0x4948b3; continue; }
            pc = 0x49488b; continue;
            }
            0x49488b => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            pc = 0x494894; continue;
            }
            0x494894 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x39),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x4948aa; continue;
            }
            0x4948aa => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(6,16),16); }
            if c.cond(2) { pc = 0x494894; continue; }
            pc = 0x4948b3; continue;
            }
            0x4948b3 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4948cd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4948f9; continue; }
            pc = 0x4948d1; continue;
            }
            0x4948d1 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = rm(m,c.r[1].wrapping_add(0xc324),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4948ed; continue; }
            pc = 0x4948e4; continue;
            }
            0x4948e4 => {
            c.push(m, 0x4948e9);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4948f9; continue; }
            pc = 0x4948ed; continue;
            }
            0x4948ed => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4948f9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x494908 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x494913);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(5,16,rm(m,0x6dd894u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x494977; continue; }
            pc = 0x494923; continue;
            }
            0x494923 => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x494929; continue;
            }
            0x494929 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x50),16));
            { let a = c.get(1,16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x494943; continue;
            }
            0x494943 => {
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x52),16));
            { let a = c.get(6,16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x494950; continue;
            }
            0x494950 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x49496e; continue; }
            pc = 0x494958; continue;
            }
            0x494958 => {
            c.set(2,16,rm(m,c.r[2].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x494965; continue;
            }
            0x494965 => {
            { let a = c.get(6,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x49481d; continue; }
            pc = 0x49496e; continue;
            }
            0x49496e => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x494929; continue; }
            pc = 0x494977; continue;
            }
            0x494977 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x496c90;
    loop {
        match pc {
            0x496c90 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660929u32);
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
            c.push(m, 0x496cb5);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x496cc8);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x496cd8);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x470); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x478));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            c.push(m, 0x496cf3);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x550));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            c.push(m, 0x496d03);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x628));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x5u32); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x496d15);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0x71c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x6u32); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x496d27);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x810));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x7u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x496d39);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x8u32); }
            c.push(m, 0x496d49);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x9u32); }
            c.push(m, 0x496d59);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.set(0,32,c.r[6].wrapping_add(0xaec));
            c.push(m,0x12u32);
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0xau32); }
            c.push(m, 0x496d7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[6].wrapping_add(0x1b40));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0xbu32); }
            c.push(m, 0x496d8b);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1b60));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0xcu32); }
            c.push(m, 0x496d9b);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1bc0));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0xdu32); }
            c.push(m, 0x496dab);
            { let args = []; c.r[0] = host.call(m, 0x434b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x20u32);
            c.set(1,32,c.r[6].wrapping_add(0x1b80));
            c.push(m,0x66f608u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0xeu32); }
            { let a = c.r[6]; wm(m,a,32,0x66f6c8u32); }
            c.push(m, 0x496dc9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x496dd3);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            c.push(m, 0x496dde);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            c.push(m, 0x496de9);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x478));
            c.push(m, 0x496df4);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x550));
            c.push(m, 0x496dff);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496e09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496e13); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496e1e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496e30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496e42); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0xaec));
            c.set(3,32,0x12u32);
            c.set(7,32,c.get(0,32));
            pc = 0x496e4f; continue;
            }
            0x496e4f => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496e59); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x496e4f; continue; }
            pc = 0x496e62; continue;
            }
            0x496e62 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
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

pub fn new_game_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x496e80;
    loop {
        match pc {
            0x496e80 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660a0cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66f6c8u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xeu32); }
            c.push(m, 0x496eb5);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            c.push(m, 0x496ec0);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            c.push(m, 0x496ecb);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x478));
            c.push(m, 0x496ed6);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x550));
            c.push(m, 0x496ee1);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x628),32));
            c.set(7,32,c.r[6].wrapping_add(0x628));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496ef5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x71c),32));
            c.set(3,32,c.r[6].wrapping_add(0x71c));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496f09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x810),32));
            c.set(5,32,c.r[6].wrapping_add(0x810));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496f1d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496f2f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x496f41); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xaec));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x12u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x496f59; continue;
            }
            0x496f55 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x496f59; continue;
            }
            0x496f59 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x496f61); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x496f55; continue; }
            pc = 0x496f7a; continue;
            }
            0x496f7a => {
            c.set(1,32,c.r[6].wrapping_add(0x1bc0));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xdu32); }
            c.push(m, 0x496f8a);
            { let args = []; c.r[0] = host.call(m, 0x434c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1b60));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xcu32); }
            c.push(m, 0x496f9a);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x1b40));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xbu32); }
            c.push(m, 0x496faa);
            { let args = []; c.r[0] = host.call(m, 0x4b0440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x12u32);
            c.set(0,32,c.r[6].wrapping_add(0xaec));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0xau32); }
            c.push(m, 0x496fc7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x9u32); }
            c.push(m, 0x496fd7);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x8u32); }
            c.push(m, 0x496fe7);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x7u32); }
            c.push(m, 0x496ff3);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x6u32); }
            c.push(m, 0x496fff);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x5u32); }
            c.push(m, 0x49700b);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x550));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.push(m, 0x49701b);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x478));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x3u32); }
            c.push(m, 0x49702b);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x470));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.push(m, 0x49703b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x49704b);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x49705b);
            { let args = []; c.r[0] = host.call(m, 0x4c9970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x49706a);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x497080;
    loop {
        match pc {
            0x497080 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x497089);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            c.push(m, 0x497094);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            c.push(m, 0x49709f);
            { let args = []; c.r[0] = host.call(m, 0x4c99f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x478));
            c.push(m, 0x4970aa);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x550));
            c.push(m, 0x4970b5);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x628),32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4970c7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x71c),32));
            c.set(1,32,c.r[6].wrapping_add(0x71c));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4970d9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x810),32));
            c.set(1,32,c.r[6].wrapping_add(0x810));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4970eb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4970fd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49710f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0xaecu32,32); c.set(6,32,r); }
            c.set(7,32,0x12u32);
            pc = 0x49711a; continue;
            }
            0x49711a => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x497124); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x49711a; continue; }
            pc = 0x49712d; continue;
            }
            0x49712d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_prepare(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x497ee0;
    loop {
        match pc {
            0x497ee0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660a50u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x1bb4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x1bb8); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x497f22; continue; }
            pc = 0x497f17; continue;
            }
            0x497f17 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,c.get(0,32)); }
            pc = 0x497f2c; continue;
            }
            0x497f22 => {
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,0x0u32); }
            pc = 0x497f2c; continue;
            }
            0x497f2c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x497f33);
            { let args = []; c.r[0] = host.call(m, 0x49a2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x497f41; continue; }
            pc = 0x497f38; continue;
            }
            0x497f38 => {
            c.set(1,32,rm(m,0x69b294u32,32));
            c.push(m,c.get(1,32));
            pc = 0x497f48; continue;
            }
            0x497f41 => {
            c.set(2,32,rm(m,0x69b28cu32,32));
            c.push(m,c.get(2,32));
            pc = 0x497f48; continue;
            }
            0x497f48 => {
            c.set(1,32,c.r[6].wrapping_add(0x904));
            c.push(m, 0x497f53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.push(m,0x20u32);
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x66f608u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1b80));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x497f70);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x497fc9; continue; }
            pc = 0x497f77; continue;
            }
            0x497f77 => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.push(m, 0x497f98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6baa14u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x497fa6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m, 0x497fb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x497fc7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x497fdb; continue;
            }
            0x497fc9 => {
            c.set(2,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m,c.get(2,32));
            c.push(m, 0x497fdb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x497fdb; continue;
            }
            0x497fdb => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba7ccu32);
            c.push(m, 0x497fec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x497ff6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6df2fcu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x49802e; continue; }
            pc = 0x498000; continue;
            }
            0x498000 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49800c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x498023);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            pc = 0x49802e; continue;
            }
            0x49802e => {
            c.push(m,0x6d1c81u32);
            c.set(1,32,c.r[6].wrapping_add(0x470));
            c.push(m, 0x49803e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_update_duration_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x499af0;
    loop {
        match pc {
            0x499af0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x499b66; continue; }
            pc = 0x499aff; continue;
            }
            0x499aff => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x499bbc)); continue;
            }
            0x499b06 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x499b17); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x0u32); }
            pc = 0x499b66; continue;
            }
            0x499b23 => {
            c.set(7,32,0xeu32);
            pc = 0x499b3d; continue;
            }
            0x499b2a => {
            c.set(7,32,0xdu32);
            pc = 0x499b3d; continue;
            }
            0x499b31 => {
            c.set(7,32,0x2u32);
            pc = 0x499b3d; continue;
            }
            0x499b38 => {
            c.set(7,32,0x4u32);
            pc = 0x499b3d; continue;
            }
            0x499b3d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.set(0,8,u32::from(c.cond(12)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x499b58); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x1bb0),32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x499b66; continue; }
            pc = 0x499b60; continue;
            }
            0x499b60 => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(7,32)); }
            pc = 0x499b66; continue;
            }
            0x499b66 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x15cc),32));
            c.set(7,32,c.r[6].wrapping_add(0x15cc));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x499b83); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x499b99); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x499bb7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn objective_duration_get_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x499ed0;
    loop {
        match pc {
            0x499ed0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,0x12u32,32); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            if c.cond(7) { pc = 0x49a086; continue; }
            pc = 0x499ee7; continue;
            }
            0x499ee7 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x49a09c)); continue;
            }
            0x499eee => {
            c.set(0,32,rm(m,0x69aac0u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499eff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f04 => {
            c.set(1,32,rm(m,0x69aac4u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f1b => {
            c.set(2,32,rm(m,0x69aac8u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f2d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f32 => {
            c.set(0,32,rm(m,0x69aaccu32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f48 => {
            c.set(1,32,rm(m,0x69aad0u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f5a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f5f => {
            c.set(2,32,rm(m,0x69aad4u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f76 => {
            c.set(0,32,rm(m,0x69aad8u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f87);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499f8c => {
            c.set(1,32,rm(m,0x69aadcu32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499f9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499fa3 => {
            c.set(2,32,rm(m,0x69aae0u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499fb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499fba => {
            c.set(0,32,rm(m,0x69aae4u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499fcb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499fd0 => {
            c.set(1,32,rm(m,0x69aae8u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499fe2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499fe7 => {
            c.set(2,32,rm(m,0x69aaecu32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x499ff9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499ffe => {
            c.set(0,32,rm(m,0x69aaf0u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a00f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a014 => {
            c.set(1,32,rm(m,0x69aaf4u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a026);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a02b => {
            c.set(2,32,rm(m,0x69aaf8u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a03d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a042 => {
            c.set(0,32,rm(m,0x6b2d1cu32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a053);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a058 => {
            c.set(1,32,rm(m,0x6b2d20u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a06a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a06f => {
            c.set(2,32,rm(m,0x6b2d24u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a081);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a086 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,0x6d1c83u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a096);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn objective_get_condition_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a0f0;
    loop {
        match pc {
            0x49a0f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660ae2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,0x6b2d3cu32,32));
            { let a = c.get(6,32); c.sub(a,0x6u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x49a137; continue; }
            pc = 0x49a130; continue;
            }
            0x49a130 => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x69aa10),32));
            pc = 0x49a137; continue;
            }
            0x49a137 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,0x6baa0cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x49a147);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6u32,32); }
            if c.cond(4) { pc = 0x49a154; continue; }
            pc = 0x49a14f; continue;
            }
            0x49a14f => {
            { let a = c.get(6,32); c.sub(a,0x7u32,32); }
            if c.cond(5) { pc = 0x49a172; continue; }
            pc = 0x49a154; continue;
            }
            0x49a154 => {
            c.set(2,32,rm(m,0x69aa30u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x49a164);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6baa10u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x49a172);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a172; continue;
            }
            0x49a172 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.sub(a,0x12u32,32); }
            if c.cond(7) { pc = 0x49a244; continue; }
            pc = 0x49a17f; continue;
            }
            0x49a17f => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x49a27c)); continue;
            }
            0x49a186 => {
            c.set(0,32,rm(m,0x69abecu32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a191 => {
            c.set(1,32,rm(m,0x69abf0u32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a19d => {
            c.set(2,32,rm(m,0x69abf4u32,32));
            pc = 0x49a220; continue;
            }
            0x49a1a5 => {
            c.set(0,32,rm(m,0x69abf8u32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a1ad => {
            c.set(1,32,rm(m,0x69abfcu32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a1b6 => {
            c.set(2,32,rm(m,0x69ac00u32,32));
            pc = 0x49a220; continue;
            }
            0x49a1be => {
            c.set(0,32,rm(m,0x69ac04u32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a1c6 => {
            c.set(1,32,rm(m,0x69ac08u32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a1cf => {
            c.set(2,32,rm(m,0x69ac0cu32,32));
            pc = 0x49a220; continue;
            }
            0x49a1d7 => {
            c.set(0,32,rm(m,0x69ac10u32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a1df => {
            c.set(1,32,rm(m,0x69ac14u32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a1e8 => {
            c.set(2,32,rm(m,0x69ac18u32,32));
            pc = 0x49a220; continue;
            }
            0x49a1f0 => {
            c.set(0,32,rm(m,0x69ac1cu32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a1f8 => {
            c.set(1,32,rm(m,0x69ac20u32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a201 => {
            c.set(2,32,rm(m,0x69ac24u32,32));
            pc = 0x49a220; continue;
            }
            0x49a209 => {
            c.set(0,32,rm(m,0x6b2d1cu32,32));
            c.push(m,c.get(0,32));
            pc = 0x49a221; continue;
            }
            0x49a211 => {
            c.set(1,32,rm(m,0x6b2d20u32,32));
            c.push(m,c.get(1,32));
            pc = 0x49a221; continue;
            }
            0x49a21a => {
            c.set(2,32,rm(m,0x6b2d24u32,32));
            pc = 0x49a220; continue;
            }
            0x49a220 => {
            c.push(m,c.get(2,32));
            pc = 0x49a221; continue;
            }
            0x49a221 => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x49a22a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a23a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x49a25c; continue;
            }
            0x49a244 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x6d1c84u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a254);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x49a25c; continue;
            }
            0x49a25c => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x49a26a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_update_arrow_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a2d0;
    loop {
        match pc {
            0x49a2d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x46c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xaec),32));
            c.set(1,32,c.r[6].wrapping_add(0xaec));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a2ee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x46c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(1,32,c.r[6].wrapping_add(0xbd4));
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a30a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13fc));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a325); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14e4),32));
            c.set(1,32,c.r[6].wrapping_add(0x14e4));
            { let a = c.get(2,32); c.sub(a,0x6u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a341); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x474),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x196c),32));
            c.set(1,32,c.r[6].wrapping_add(0x196c));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,8,u32::from(c.cond(7)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a35c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x474),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a54),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a54));
            { let a = c.get(2,8); c.sub(a,0x2u32,8); }
            c.set(2,8,u32::from(c.cond(2)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a378); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xe8c));
            { let a = c.get(2,32); c.sub(a,0x514u32,32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a397); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf74),32));
            c.set(1,32,c.r[6].wrapping_add(0xf74));
            { let a = c.get(2,32); c.sub(a,0x578u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a3b6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x122c),32));
            c.set(1,32,c.r[6].wrapping_add(0x122c));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a3d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1314),32));
            c.set(1,32,c.r[6].wrapping_add(0x1314));
            { let a = c.get(2,32); c.sub(a,0x4u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a3ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1bac),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x179c),32));
            c.set(1,32,c.r[6].wrapping_add(0x179c));
            { let a = c.get(2,32); c.sub(a,0x5u32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a409); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1884),32));
            c.set(1,32,c.r[6].wrapping_add(0x1884));
            { let a = rm(m,c.r[6].wrapping_add(0x1bac),32); c.sub(a,0x4u32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49a423); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x499af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_update_details_button_text(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a430;
    loop {
        match pc {
            0x49a430 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660af4u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x20u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.set(1,32,c.r[6].wrapping_add(0x1b80));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x66f608u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x49a466);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x49a4cd; continue; }
            pc = 0x49a46d; continue;
            }
            0x49a46d => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.push(m, 0x49a48e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6baa14u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x49a49c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m, 0x49a4ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x49a4bd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49a4cd => {
            c.set(2,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m,c.get(2,32));
            c.push(m, 0x49a4df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn new_game_screen_load_map_for_start_year(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a4f0;
    loop {
        match pc {
            0x49a4f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660b1du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x1ba0),32));
            c.set(1,32,rm(m,0x6ba5e8u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            if c.cond(12) { pc = 0x49a542; continue; }
            pc = 0x49a532; continue;
            }
            0x49a532 => {
            c.push(m,0x6baa1cu32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x49a540);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a56e; continue;
            }
            0x49a542 => {
            c.set(1,32,c.r[0].wrapping_add(0xfffffaec));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x15));
            c.push(m,c.get(2,32));
            c.push(m,0x6baa28u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49a56b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x49a56e; continue;
            }
            0x49a56e => {
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49a57f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x49a589);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a599);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x49a5af);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(6,32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_on_start_year_changed(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a620;
    loop {
        match pc {
            0x49a620 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x6d1c85u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x470),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a635);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49a6d5; continue; }
            pc = 0x49a644; continue;
            }
            0x49a644 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            c.set(0,32,0x66666667u32);
            { let a = c.get(1,32); let r = c.sub(a,0x514u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.r[3].wrapping_add(0xfffffaec));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x49a693; continue; }
            pc = 0x49a67e; continue;
            }
            0x49a67e => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49a68a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x49a693);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x49a693; continue;
            }
            0x49a693 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x49a6a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a6bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x810));
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x49a6d5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x49a6d5; continue;
            }
            0x49a6d5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x470),32));
            c.push(m,0x6baa34u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49a6e6);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49a790; continue; }
            pc = 0x49a6f1; continue;
            }
            0x49a6f1 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            c.set(0,32,0x66666667u32);
            c.set(1,32,c.r[7].wrapping_add(0xfffffaec));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,32,0x66666667u32);
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.r[3].wrapping_add(0xfffffaec));
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x49a749; continue; }
            pc = 0x49a72c; continue;
            }
            0x49a72c => {
            c.set(0,32,rm(m,0x6ba5e8u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(0,32,0x18u32);
            if c.cond(13) { pc = 0x49a73d; continue; }
            pc = 0x49a73a; continue;
            }
            0x49a73a => {
            c.set(0,32,c.r[1].wrapping_add(0x15));
            pc = 0x49a73d; continue;
            }
            0x49a73d => {
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a749);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x545650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a749; continue;
            }
            0x49a749 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m, 0x49a75b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a772);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x810),32));
            c.set(1,32,c.r[6].wrapping_add(0x810));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x49a78f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x49a790; continue;
            }
            0x49a790 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_refresh_map_and_town_labels(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a7a0;
    loop {
        match pc {
            0x49a7a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660b37u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a7cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[6].wrapping_add(0x810));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a7dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x71c),32));
            c.set(5,32,c.r[6].wrapping_add(0x71c));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x3c),32); c.push(m, 0x49a7f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x470),32));
            c.set(7,32,c.r[6].wrapping_add(0x470));
            c.push(m,0x6d1c86u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            c.push(m, 0x49a81c);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49a838; continue; }
            pc = 0x49a823; continue;
            }
            0x49a823 => {
            c.set(2,32,rm(m,0x6aef28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x49a833);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a8e2; continue;
            }
            0x49a838 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x6baa40u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49a845);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49a860; continue; }
            pc = 0x49a84c; continue;
            }
            0x49a84c => {
            c.set(0,32,rm(m,0x6aef2cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x49a85b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a8e2; continue;
            }
            0x49a860 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x49a86a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dce78u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x49a888);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x49a8c6; continue; }
            pc = 0x49a88c; continue;
            }
            0x49a88c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,0x20u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49a8a0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f71b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x49a8a9);
            { let args = []; c.r[0] = host.call(m, 0x64e1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x49a8c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x49a88c; continue; }
            pc = 0x49a8c6; continue;
            }
            0x49a8c6 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x49a8d4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x49a8e2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x49a8e2; continue;
            }
            0x49a8e2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x474),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x49a8fa; continue; }
            pc = 0x49a8ec; continue;
            }
            0x49a8ec => {
            c.push(m,0x6baa4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x49a8fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49a8fa; continue;
            }
            0x49a8fa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x49a906);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            c.push(m, 0x49a911);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,0x6d1c87u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49a91e);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x49a948; continue; }
            pc = 0x49a925; continue;
            }
            0x49a925 => {
            c.set(1,32,c.r[6].wrapping_add(0x168));
            c.push(m, 0x49a930);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,0x6d1c88u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49a93d);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x49a948; continue; }
            pc = 0x49a944; continue;
            }
            0x49a944 => {
            c.set(0,8,0x1u32);
            pc = 0x49a94a; continue;
            }
            0x49a948 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x49a94a; continue;
            }
            0x49a94a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49a95a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x49a96b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn new_game_screen_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49a9a0;
    loop {
        match pc {
            0x49a9a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x49a9a8);
            { let args = []; c.r[0] = host.call(m, 0x496e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x49a9b8; continue; }
            pc = 0x49a9af; continue;
            }
            0x49a9af => {
            c.push(m,c.get(6,32));
            c.push(m, 0x49a9b5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x49a9b8; continue;
            }
            0x49a9b8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_details_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49aa50;
    loop {
        match pc {
            0x49aa50 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660b9du32);
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
            c.push(m, 0x49aa75);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x49aa8a);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x4c6910u32);
            c.set(7,32,c.r[6].wrapping_add(0x23c));
            c.push(m,0x10u32);
            c.push(m,0xe8u32);
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x49aaac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(5,32,c.r[6].wrapping_add(0x10bc));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x49aabe);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x11b0));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            c.push(m, 0x49aace);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            { let a = c.r[6]; wm(m,a,32,0x66f830u32); }
            c.push(m, 0x49aae0);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x49aae7);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x10u32);
            pc = 0x49aaec; continue;
            }
            0x49aaec => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49aaf6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x49aaec; continue; }
            pc = 0x49aaff; continue;
            }
            0x49aaff => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x49ab0a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            c.set(1,32,c.r[6].wrapping_add(0x11b0));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49ab1c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
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

pub fn game_details_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49ab40;
    loop {
        match pc {
            0x49ab40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660bf4u32);
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
            { let a = c.r[6]; wm(m,a,32,0x66f830u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x4u32); }
            c.push(m, 0x49ab73);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x164));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x49ab80);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x23c));
            c.set(3,32,0x10u32);
            pc = 0x49ab8b; continue;
            }
            0x49ab8b => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49ab95); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x49ab8b; continue; }
            pc = 0x49ab9e; continue;
            }
            0x49ab9e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x10bc));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x49abb2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11b0),32));
            c.set(3,32,c.r[6].wrapping_add(0x11b0));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49abc6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            c.push(m, 0x49abd2);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.push(m, 0x49abde);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x10u32);
            c.set(0,32,c.r[6].wrapping_add(0x23c));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.push(m, 0x49abfb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x49ac07);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x49ac16);
            { let args = []; c.r[0] = host.call(m, 0x4b4a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
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

pub fn game_details_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49ac30;
    loop {
        match pc {
            0x49ac30 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m, 0x49ac3a);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x164));
            c.push(m, 0x49ac45);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[7].wrapping_add(0x23c));
            c.set(3,32,0x10u32);
            pc = 0x49ac50; continue;
            }
            0x49ac50 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x49ac5a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x49ac50; continue; }
            pc = 0x49ac63; continue;
            }
            0x49ac63 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10bc),32));
            c.set(1,32,c.r[7].wrapping_add(0x10bc));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x49ac75); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x11b0),32));
            c.set(1,32,c.r[7].wrapping_add(0x11b0));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_details_screen_load_values(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49b180;
    loop {
        match pc {
            0x49b180 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,0x1b80u32,32); c.set(0,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x12a4));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x49b19f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0x324));
            c.set(3,32,0x6u32);
            pc = 0x49b1ad; continue;
            }
            0x49b1ad => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffffff18),32));
            c.set(1,32,c.r[6].wrapping_add(0xffffff18));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(0,8,u32::from(c.cond(15)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b1c5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(1,32,c.get(6,32));
            c.set(0,8,u32::from(c.cond(12)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b1d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x1d0u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x49b1ad; continue; }
            pc = 0x49b1e1; continue;
            }
            0x49b1e1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12bc),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(13) { pc = 0x49b1f8; continue; }
            pc = 0x49b1f0; continue;
            }
            0x49b1f0 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x49b1f8; continue; }
            pc = 0x49b1f4; continue;
            }
            0x49b1f4 => {
            c.set(0,8,0x1u32);
            pc = 0x49b1fa; continue;
            }
            0x49b1f8 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x49b1fa; continue;
            }
            0x49b1fa => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xd1c),32));
            c.set(1,32,c.r[5].wrapping_add(0xd1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b20a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12bc),32));
            c.set(6,32,0x1u32);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x49b221; continue; }
            pc = 0x49b219; continue;
            }
            0x49b219 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x49b221; continue; }
            pc = 0x49b21d; continue;
            }
            0x49b21d => {
            c.set(0,8,0x1u32);
            pc = 0x49b223; continue;
            }
            0x49b221 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x49b223; continue;
            }
            0x49b223 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe04),32));
            c.set(1,32,c.r[5].wrapping_add(0xe04));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b233); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0x12c0),32); c.sub(a,0x2u32,32); }
            if c.cond(13) { pc = 0x49b244; continue; }
            pc = 0x49b23c; continue;
            }
            0x49b23c => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x49b244; continue; }
            pc = 0x49b240; continue;
            }
            0x49b240 => {
            c.set(0,8,0x1u32);
            pc = 0x49b246; continue;
            }
            0x49b244 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x49b246; continue;
            }
            0x49b246 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xeec),32));
            c.set(1,32,c.r[5].wrapping_add(0xeec));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b256); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0x12c0),32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x49b266; continue; }
            pc = 0x49b25e; continue;
            }
            0x49b25e => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x49b266; continue; }
            pc = 0x49b262; continue;
            }
            0x49b262 => {
            c.set(0,8,0x1u32);
            pc = 0x49b268; continue;
            }
            0x49b266 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x49b268; continue;
            }
            0x49b268 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfd4),32));
            c.set(1,32,c.r[5].wrapping_add(0xfd4));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b278); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5].wrapping_add(0x12c4); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_details_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49b5c0;
    loop {
        match pc {
            0x49b5c0 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.push(m,c.get(6,32));
            c.set(2,8,c.get(0,8));
            c.set(6,32,c.get(1,32));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x49b5ef; continue; }
            pc = 0x49b5e0; continue;
            }
            0x49b5e0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            c.set(0,32,0xfffffffeu32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49b5ef => {
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x23c));
            pc = 0x49b5f8; continue;
            }
            0x49b5f8 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x49b5ff);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x49b60f; continue; }
            pc = 0x49b603; continue;
            }
            0x49b603 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x10u32,32); }
            if c.cond(2) { pc = 0x49b5f8; continue; }
            pc = 0x49b60f; continue;
            }
            0x49b60f => {
            { let a = c.get(7,32); c.sub(a,0xfu32,32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(7) { pc = 0x49b682; continue; }
            pc = 0x49b615; continue;
            }
            0x49b615 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x49b8a0),8));
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x49b888)); continue;
            }
            0x49b624 => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x12a4),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x12a4));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); c.sub(a,0x2u32,32); }
            if c.cond(13) { pc = 0x49b668; continue; }
            pc = 0x49b643; continue;
            }
            0x49b643 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.r[0]; wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x23c),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x23c));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b668); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b668; continue;
            }
            0x49b668 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x324));
            pc = 0x49b67b; continue;
            }
            0x49b67b => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b682); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b682; continue;
            }
            0x49b682 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x49b6bc; continue; }
            pc = 0x49b686; continue;
            }
            0x49b686 => {
            { let a = c.get(7,32); c.sub(a,0x10u32,32); }
            if c.cond(3) { pc = 0x49b6bc; continue; }
            pc = 0x49b68b; continue;
            }
            0x49b68b => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[6]));
            c.set(1,32,c.r[0].wrapping_add(0x324));
            { let a = c.get(0,32); let r = c.add(a,0x23cu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49b6bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x49b6bc; continue;
            }
            0x49b6bc => {
            c.set(7,32,c.r[6].wrapping_add(0x11b0));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x49b6c9);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x49b842; continue; }
            pc = 0x49b6d1; continue;
            }
            0x49b6d1 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x49b6da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,0xfffffffeu32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49b6e2 => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x12a4),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x12a4));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x49b726; continue; }
            pc = 0x49b701; continue;
            }
            0x49b701 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.r[0]; wm(m,a,32,0x3u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x23c),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x23c));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b726); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b726; continue;
            }
            0x49b726 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x154));
            pc = 0x49b67b; continue;
            }
            0x49b73e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x12bc),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x12bc); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x49b769; continue; }
            pc = 0x49b752; continue;
            }
            0x49b752 => {
            c.set(1,32,c.r[6].wrapping_add(0xd1c));
            { let a = c.r[6].wrapping_add(0x12bc); wm(m,a,32,0x3u32); }
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b769); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b769; continue;
            }
            0x49b769 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe04),32));
            c.set(1,32,c.r[6].wrapping_add(0xe04));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b77a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b68b; continue;
            }
            0x49b77f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12bc),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x12bc); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(13) { pc = 0x49b7aa; continue; }
            pc = 0x49b793; continue;
            }
            0x49b793 => {
            c.set(1,32,c.r[6].wrapping_add(0xe04));
            { let a = c.r[6].wrapping_add(0x12bc); wm(m,a,32,0x1u32); }
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b7aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b7aa; continue;
            }
            0x49b7aa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd1c),32));
            c.set(1,32,c.r[6].wrapping_add(0xd1c));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b7bb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b68b; continue;
            }
            0x49b7c0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c0),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x12c0); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x49b7eb; continue; }
            pc = 0x49b7d4; continue;
            }
            0x49b7d4 => {
            c.set(1,32,c.r[6].wrapping_add(0xeec));
            { let a = c.r[6].wrapping_add(0x12c0); wm(m,a,32,0x2u32); }
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b7eb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b7eb; continue;
            }
            0x49b7eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfd4),32));
            c.set(1,32,c.r[6].wrapping_add(0xfd4));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b7fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b68b; continue;
            }
            0x49b801 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c0),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x12c0); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(13) { pc = 0x49b82c; continue; }
            pc = 0x49b815; continue;
            }
            0x49b815 => {
            c.set(1,32,c.r[6].wrapping_add(0xfd4));
            { let a = c.r[6].wrapping_add(0x12c0); wm(m,a,32,0x1u32); }
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49b82c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b82c; continue;
            }
            0x49b82c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xeec),32));
            c.set(1,32,c.r[6].wrapping_add(0xeec));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49b83d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49b68b; continue;
            }
            0x49b842 => {
            c.set(7,32,c.r[6].wrapping_add(0x10bc));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x49b84f);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x49b880; continue; }
            pc = 0x49b853; continue;
            }
            0x49b853 => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(6,32); let r = c.add(a,0x12a4u32,32); c.set(6,32,r); }
            c.push(m,0x20u32);
            { let a = c.get(0,32); let r = c.add(a,0x1b80u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49b86c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x49b878); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49b880 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_settings_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49b8d0;
    loop {
        match pc {
            0x49b8d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x49b8d8);
            { let args = []; c.r[0] = host.call(m, 0x49ab40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x49b8e8; continue; }
            pc = 0x49b8df; continue;
            }
            0x49b8df => {
            c.push(m,c.get(6,32));
            c.push(m, 0x49b8e5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x49b8e8; continue;
            }
            0x49b8e8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn mission_select_screen_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ab1a0;
    loop {
        match pc {
            0x4ab1a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6613deu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4ab1c3);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4ab1d4);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x4ab1e4);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5f0));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x2u32); }
            c.push(m, 0x4ab1f4);
            { let args = []; c.r[0] = host.call(m, 0x430f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            c.push(m, 0x4ab204);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            c.push(m, 0x4ab214);
            { let args = []; c.r[0] = host.call(m, 0x464ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb8c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x5u32); }
            c.push(m, 0x4ab224);
            { let args = []; c.r[0] = host.call(m, 0x4209e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xbe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x670018u32); }
            { let a = c.r[6].wrapping_add(0xbb8); wm(m,a,32,0x4u32); }
            { let a = c.r[6].wrapping_add(0xbb4); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0xbd8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xbf0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xbe4); wm(m,a,8,c.get(3,8)); }
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

pub fn mission_select_screen_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ab270;
    loop {
        match pc {
            0x4ab270 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661452u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x670018u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x7u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4ab2ce; continue; }
            pc = 0x4ab2a5; continue;
            }
            0x4ab2a5 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x4ac520u32);
            c.push(m,c.get(1,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab2ba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4ab2c0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xbf0); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4ab2ce; continue;
            }
            0x4ab2ce => {
            c.set(1,32,c.r[6].wrapping_add(0xbe8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x6u32); }
            c.push(m, 0x4ab2de);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb8c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x4ab2ee);
            { let args = []; c.r[0] = host.call(m, 0x420a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x4u32); }
            c.push(m, 0x4ab2fe);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            c.push(m, 0x4ab30e);
            { let args = []; c.r[0] = host.call(m, 0x464b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x5f0));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x4ab31e);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x4ab32e);
            { let args = []; c.r[0] = host.call(m, 0x430fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x4ab33e);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ab34d);
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

pub fn mission_select_screen_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ab360;
    loop {
        match pc {
            0x4ab360 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4ab368);
            { let args = []; c.r[0] = host.call(m, 0x4b4af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            c.push(m, 0x4ab373);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4ab385); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa98),32));
            c.set(1,32,c.r[6].wrapping_add(0xa98));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4ab397); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4ab3ca; continue; }
            pc = 0x4ab3a1; continue;
            }
            0x4ab3a1 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x4ac520u32);
            c.push(m,c.get(1,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab3b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4ab3bc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xbf0); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4ab3ca; continue;
            }
            0x4ab3ca => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn mission_select_screen_load_campaign(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ab420;
    loop {
        match pc {
            0x4ab420 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661477u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,0x164u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4ab44e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x240u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4ab462);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,0x6bb0f4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4ab48e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x6bb100u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6d9598u32);
            c.push(m,0x6bb11cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4ab4b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[6].wrapping_add(0xbe8));
            c.push(m, 0x4ab4c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x6bb124u32);
            c.push(m,c.get(5,32));
            c.push(m,0x6bb140u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4ab4e3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0xbec); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4ab518; continue; }
            pc = 0x4ab4f5; continue;
            }
            0x4ab4f5 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x4ac520u32);
            c.push(m,c.get(1,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab509);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4ab50f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xbf0); wm(m,a,32,c.get(5,32)); }
            pc = 0x4ab518; continue;
            }
            0x4ab518 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4ab526);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x4ab550; continue; }
            pc = 0x4ab536; continue;
            }
            0x4ab536 => {
            c.push(m,0x4ac520u32);
            c.push(m,0x430f60u32);
            c.set(7,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(3,32));
            c.push(m,0x20u32);
            c.push(m,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4ab54e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4ab552; continue;
            }
            0x4ab550 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4ab552; continue;
            }
            0x4ab552 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xbf0); wm(m,a,32,c.get(7,32)); }
            if c.cond(14) { pc = 0x4ab6ec; continue; }
            pc = 0x4ab565; continue;
            }
            0x4ab565 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4ab567; continue;
            }
            0x4ab567 => {
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6bb14cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab577);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x6bb15cu32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6d9599u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4ab5a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.push(m,0x6dbb58u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x4ab5b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6bb178u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab5c4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x6bb188u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6d959au32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4ab5ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4));
            c.push(m, 0x4ab603);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.push(m,0x6bb1a4u32);
            c.push(m,0x6bb1a8u32);
            c.set(1,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x4));
            c.push(m, 0x4ab61c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6bb1acu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ab62c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x6bb1bcu32);
            c.push(m,0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4ab64a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ab6d5; continue; }
            pc = 0x4ab652; continue;
            }
            0x4ab652 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4ab65d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[7].wrapping_add(c.r[2]).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4ab67f);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x7bu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x29u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x7bu32);
            c.push(m,0x29u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x4ab6a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xc));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            pc = 0x4ab6e0; continue;
            }
            0x4ab6d5 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbf0),32));
            { let a = c.r[7].wrapping_add(c.r[2]).wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            pc = 0x4ab6e0; continue;
            }
            0x4ab6e0 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x20u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4ab567; continue; }
            pc = 0x4ab6ec; continue;
            }
            0x4ab6ec => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbec),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbd8),32));
            { let a = c.r[6].wrapping_add(0xbb0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xbb8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbdc),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0xbd4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xbd8); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x4ab728; continue; }
            pc = 0x4ab722; continue;
            }
            0x4ab722 => {
            { let a = c.r[6].wrapping_add(0xbd8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4ab728; continue;
            }
            0x4ab728 => {
            c.set(5,32,c.r[6].wrapping_add(0x5f0));
            { let a = c.r[6].wrapping_add(0xbdc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4ab73b);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc04),32));
            c.set(7,32,c.r[6].wrapping_add(0x5f8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ab752);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc0c),32));
            c.set(3,32,rm(m,0x66a4a4u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbcc),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xbbc),32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc08),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let target = c.get(3,32); c.push(m, 0x4ab78f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbbc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbb0),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ab7ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbb0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbcc),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4ab7c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbb0),32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ab7d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            c.push(m, 0x4ab7e2);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc14),32));
            c.set(7,32,c.r[6].wrapping_add(0x244));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ab7f9);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbd0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc1c),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xbc0),32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc18),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let target = c.get(3,32); c.push(m, 0x4ab830); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbc0),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,0x64u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ab846);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4ab852); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x240); wm(m,a,8,0x0u32); }
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0xbe0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            c.push(m, 0x4ab870);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4ab87d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn mission_select_screen_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4abfd0;
    loop {
        match pc {
            0x4abfd0 => {
            { let a = rm(m,0x6d9590u32,8); c.and(a,0x1u32,8); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            if c.cond(5) { pc = 0x4abff2; continue; }
            pc = 0x4abfde; continue;
            }
            0x4abfde => {
            c.set(2,8,rm(m,0x6d9590u32,8));
            { let a = 0x6d958cu32; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,8); let r = c.or(a,0x1u32,8); c.set(2,8,r); }
            { let a = 0x6d9590u32; wm(m,a,8,c.get(2,8)); }
            pc = 0x4abff2; continue;
            }
            0x4abff2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x4ac024; continue; }
            pc = 0x4abffb; continue;
            }
            0x4abffb => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6d958cu32,32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(6,32); c.sub(a,0x1f4u32,32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(3) { pc = 0x4ac01d; continue; }
            pc = 0x4ac011; continue;
            }
            0x4ac011 => {
            { let a = c.r[1].wrapping_add(0xbe4); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            pc = 0x4ac01d; continue;
            }
            0x4ac01d => {
            { let a = 0x6d958cu32; wm(m,a,32,c.get(0,32)); }
            pc = 0x4ac02b; continue;
            }
            0x4ac024 => {
            { let a = c.r[1].wrapping_add(0xbe4); wm(m,a,8,0x0u32); }
            pc = 0x4ac02b; continue;
            }
            0x4ac02b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0xba8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1].wrapping_add(0xbf4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0xbf8); wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn mission_select_screen_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ac500;
    loop {
        match pc {
            0x4ac500 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4ac508);
            { let args = []; c.r[0] = host.call(m, 0x4ab270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4ac518; continue; }
            pc = 0x4ac50f; continue;
            }
            0x4ac50f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4ac515);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4ac518; continue;
            }
            0x4ac518 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn mission_entry_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ac520;
    loop {
        match pc {
            0x4ac520 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6614d8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x4ac54d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ac55c);
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

pub fn screen_window_insert_child(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b4ce0;
    loop {
        match pc {
            0x4b4ce0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4b4d01);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(7,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4b4d1a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4b4d47; continue; }
            pc = 0x4b4d27; continue;
            }
            0x4b4d27 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0x1));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4b4d44);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4b4d47; continue;
            }
            0x4b4d47 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0xcu32);
            c.set(1,32,c.r[7].wrapping_add(c.r[3]).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[7].wrapping_add(c.r[3]); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4b4d5f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4b4d79; continue; }
            pc = 0x4b4d70; continue;
            }
            0x4b4d70 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b4d76);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b4d79; continue;
            }
            0x4b4d79 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.push(m, 0x4b4d87);
            { let args = []; c.r[0] = host.call(m, 0x4b2230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,32,0x1u32);
            c.push(m,c.get(3,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x4b4d9b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xa8),32); c.push(m, 0x4b4dae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4b4dbf; continue; }
            pc = 0x4b4db8; continue;
            }
            0x4b4db8 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4dbf; continue;
            }
            0x4b4dbf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4b4dd0; continue; }
            pc = 0x4b4dc9; continue;
            }
            0x4b4dc9 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4dd0; continue;
            }
            0x4b4dd0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4b4de1; continue; }
            pc = 0x4b4dda; continue;
            }
            0x4b4dda => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4de1; continue;
            }
            0x4b4de1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.r[6].wrapping_add(0x13c); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4b4df8; continue; }
            pc = 0x4b4df1; continue;
            }
            0x4b4df1 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4df8; continue;
            }
            0x4b4df8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(0,32)); }
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

pub fn screen_window_set_hover_child(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b5c20;
    loop {
        match pc {
            0x4b5c20 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4b5c96; continue; }
            pc = 0x4b5c33; continue;
            }
            0x4b5c33 => {
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5d52; continue; }
            pc = 0x4b5c3c; continue;
            }
            0x4b5c3c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4b5c79; continue; }
            pc = 0x4b5c44; continue;
            }
            0x4b5c44 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xac),32); c.push(m, 0x4b5c58); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b5d52; continue; }
            pc = 0x4b5c60; continue;
            }
            0x4b5c60 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xa4),32); c.push(m, 0x4b5c73); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4b5c79 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xa8),32); c.push(m, 0x4b5c90); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4b5c96 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5cd8; continue; }
            pc = 0x4b5c9b; continue;
            }
            0x4b5c9b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b5cd8; continue; }
            pc = 0x4b5cab; continue;
            }
            0x4b5cab => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xac),32); c.push(m, 0x4b5cb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b5cd8; continue; }
            pc = 0x4b5cb9; continue;
            }
            0x4b5cb9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xa8),32); c.push(m, 0x4b5cd8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4b5cd8; continue;
            }
            0x4b5cd8 => {
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5d48; continue; }
            pc = 0x4b5cdd; continue;
            }
            0x4b5cdd => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(7,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b5d48; continue; }
            pc = 0x4b5cef; continue;
            }
            0x4b5cef => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4b5cf4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b5d48; continue; }
            pc = 0x4b5cf8; continue;
            }
            0x4b5cf8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xac),32); c.push(m, 0x4b5d09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b5d33; continue; }
            pc = 0x4b5d0d; continue;
            }
            0x4b5d0d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xa4),32); c.push(m, 0x4b5d23); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b5d33; continue; }
            pc = 0x4b5d27; continue;
            }
            0x4b5d27 => {
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4b5d33 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xac),32); c.push(m, 0x4b5d44); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b5d52; continue; }
            pc = 0x4b5d48; continue;
            }
            0x4b5d48 => {
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,0xffffffffu32); }
            pc = 0x4b5d52; continue;
            }
            0x4b5d52 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_left_button_down(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b5d60;
    loop {
        match pc {
            0x4b5d60 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x4b5d82); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x120),8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x11c); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4b5db9; continue; }
            pc = 0x4b5db2; continue;
            }
            0x4b5db2 => {
            { let a = c.r[6].wrapping_add(0x121); wm(m,a,8,0x1u32); }
            pc = 0x4b5db9; continue;
            }
            0x4b5db9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x124); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[6].wrapping_add(0xb6); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x128); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4b5e02; continue; }
            pc = 0x4b5de6; continue;
            }
            0x4b5de6 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x158),32); c.push(m, 0x4b5e02); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b5e02; continue;
            }
            0x4b5e02 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5f4b; continue; }
            pc = 0x4b5e11; continue;
            }
            0x4b5e11 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4b5e24); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b5f4b; continue; }
            pc = 0x4b5e2c; continue;
            }
            0x4b5e2c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b5e6b; continue; }
            pc = 0x4b5e37; continue;
            }
            0x4b5e37 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b5e6b; continue; }
            pc = 0x4b5e3f; continue;
            }
            0x4b5e3f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b5e6b; continue; }
            pc = 0x4b5e51; continue;
            }
            0x4b5e51 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b5e6b; continue;
            }
            0x4b5e6b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b5ebb; continue; }
            pc = 0x4b5e7c; continue;
            }
            0x4b5e7c => {
            c.set(7,32,rm(m,0x6bbabcu32,32));
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5e99; continue; }
            pc = 0x4b5e87; continue;
            }
            0x4b5e87 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b5e99; continue; }
            pc = 0x4b5e8f; continue;
            }
            0x4b5e8f => {
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b5ebe; continue; }
            pc = 0x4b5e99; continue;
            }
            0x4b5e99 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4b5ebb; continue; }
            pc = 0x4b5ea5; continue;
            }
            0x4b5ea5 => {
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b5eb3; continue; }
            pc = 0x4b5eab; continue;
            }
            0x4b5eab => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b5f40; continue; }
            pc = 0x4b5eb3; continue;
            }
            0x4b5eb3 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4b5ea5; continue; }
            pc = 0x4b5ebb; continue;
            }
            0x4b5ebb => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x4b5ebe; continue;
            }
            0x4b5ebe => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5ee4; continue; }
            pc = 0x4b5eca; continue;
            }
            0x4b5eca => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b5ee4; continue; }
            pc = 0x4b5ece; continue;
            }
            0x4b5ece => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b5ee4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b5ee4; continue;
            }
            0x4b5ee4 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x4b5f03; continue; }
            pc = 0x4b5eef; continue;
            }
            0x4b5eef => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xbc),32); c.push(m, 0x4b5f03); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b5f03; continue;
            }
            0x4b5f03 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb0),32); c.push(m, 0x4b5f1f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b5f3d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4b5f7f; continue;
            }
            0x4b5f40 => {
            { let a = 0x6bbabcu32; wm(m,a,32,c.get(7,32)); }
            pc = 0x4b5ebe; continue;
            }
            0x4b5f4b => {
            { let a = rm(m,c.r[6].wrapping_add(0xc8),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4b5f7f; continue; }
            pc = 0x4b5f54; continue;
            }
            0x4b5f54 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5f7f; continue; }
            pc = 0x4b5f5f; continue;
            }
            0x4b5f5f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b5f75); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,0xffffffffu32); }
            pc = 0x4b5f7f; continue;
            }
            0x4b5f7f => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x18),32); c.push(m, 0x4b5f8d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_left_button_up(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b5fa0;
    loop {
        match pc {
            0x4b5fa0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x4b5fc3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x118),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x11c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,0xfffffffeu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x121); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xb6); wm(m,a,8,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b600e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b6028; continue; }
            pc = 0x4b601e; continue;
            }
            0x4b601e => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b6028);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5c20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b6028; continue;
            }
            0x4b6028 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc4),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4b60b5; continue; }
            pc = 0x4b6035; continue;
            }
            0x4b6035 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b60b5; continue; }
            pc = 0x4b6040; continue;
            }
            0x4b6040 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4b6053); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b60b5; continue; }
            pc = 0x4b6057; continue;
            }
            0x4b6057 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b6095; continue; }
            pc = 0x4b6061; continue;
            }
            0x4b6061 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b6095; continue; }
            pc = 0x4b6069; continue;
            }
            0x4b6069 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b6095; continue; }
            pc = 0x4b607b; continue;
            }
            0x4b607b => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b6095; continue;
            }
            0x4b6095 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffeu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b60ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            pc = 0x4b6116; continue;
            }
            0x4b60b5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b612e; continue; }
            pc = 0x4b60c0; continue;
            }
            0x4b60c0 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b60f8; continue; }
            pc = 0x4b60c4; continue;
            }
            0x4b60c4 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b60f8; continue; }
            pc = 0x4b60cc; continue;
            }
            0x4b60cc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b60f8; continue; }
            pc = 0x4b60de; continue;
            }
            0x4b60de => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b60f8; continue;
            }
            0x4b60f8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffeu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b6110); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            pc = 0x4b6116; continue;
            }
            0x4b6116 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffeu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b612e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b612e; continue;
            }
            0x4b612e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b61cd; continue; }
            pc = 0x4b613d; continue;
            }
            0x4b613d => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b6175; continue; }
            pc = 0x4b6141; continue;
            }
            0x4b6141 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b6175; continue; }
            pc = 0x4b6149; continue;
            }
            0x4b6149 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b6175; continue; }
            pc = 0x4b615b; continue;
            }
            0x4b615b => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b6175; continue;
            }
            0x4b6175 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffeu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b618d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b61ab); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x158),32); c.push(m, 0x4b61cd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b61cd; continue;
            }
            0x4b61cd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_right_button_down(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b61e0;
    loop {
        match pc {
            0x4b61e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x4b6204); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x120),8));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x11c); wm(m,a,32,0xffffffffu32); }
            c.set(0,8,0x1u32);
            if c.cond(4) { pc = 0x4b623c; continue; }
            pc = 0x4b6236; continue;
            }
            0x4b6236 => {
            { let a = c.r[6].wrapping_add(0x121); wm(m,a,8,c.get(0,8)); }
            pc = 0x4b623c; continue;
            }
            0x4b623c => {
            { let a = c.r[6].wrapping_add(0xb5); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b626f; continue; }
            pc = 0x4b6253; continue;
            }
            0x4b6253 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x158),32); c.push(m, 0x4b626f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b626f; continue;
            }
            0x4b626f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b63b4; continue; }
            pc = 0x4b627e; continue;
            }
            0x4b627e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4b6291); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b63b4; continue; }
            pc = 0x4b6299; continue;
            }
            0x4b6299 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b62d7; continue; }
            pc = 0x4b62a3; continue;
            }
            0x4b62a3 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b62d7; continue; }
            pc = 0x4b62ab; continue;
            }
            0x4b62ab => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b62d7; continue; }
            pc = 0x4b62bd; continue;
            }
            0x4b62bd => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b62d7; continue;
            }
            0x4b62d7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b6326; continue; }
            pc = 0x4b62e7; continue;
            }
            0x4b62e7 => {
            c.set(7,32,rm(m,0x6bbabcu32,32));
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b6304; continue; }
            pc = 0x4b62f2; continue;
            }
            0x4b62f2 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b6304; continue; }
            pc = 0x4b62fa; continue;
            }
            0x4b62fa => {
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b6329; continue; }
            pc = 0x4b6304; continue;
            }
            0x4b6304 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4b6326; continue; }
            pc = 0x4b6310; continue;
            }
            0x4b6310 => {
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b631e; continue; }
            pc = 0x4b6316; continue;
            }
            0x4b6316 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b63a9; continue; }
            pc = 0x4b631e; continue;
            }
            0x4b631e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4b6310; continue; }
            pc = 0x4b6326; continue;
            }
            0x4b6326 => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x4b6329; continue;
            }
            0x4b6329 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b634e; continue; }
            pc = 0x4b6334; continue;
            }
            0x4b6334 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b634e; continue; }
            pc = 0x4b6338; continue;
            }
            0x4b6338 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b634e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b634e; continue;
            }
            0x4b634e => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x4b636d; continue; }
            pc = 0x4b6359; continue;
            }
            0x4b6359 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xbc),32); c.push(m, 0x4b636d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b636d; continue;
            }
            0x4b636d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb0),32); c.push(m, 0x4b6389); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b63a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b63e8; continue;
            }
            0x4b63a9 => {
            { let a = 0x6bbabcu32; wm(m,a,32,c.get(7,32)); }
            pc = 0x4b6329; continue;
            }
            0x4b63b4 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc8),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4b63e8; continue; }
            pc = 0x4b63bd; continue;
            }
            0x4b63bd => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b63e8; continue; }
            pc = 0x4b63c8; continue;
            }
            0x4b63c8 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b63de); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,0xffffffffu32); }
            pc = 0x4b63e8; continue;
            }
            0x4b63e8 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x18),32); c.push(m, 0x4b63f6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_right_button_up(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b6400;
    loop {
        match pc {
            0x4b6400 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x4b6423); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x120),8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x118),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x11c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4b645c; continue; }
            pc = 0x4b6455; continue;
            }
            0x4b6455 => {
            { let a = c.r[6].wrapping_add(0x121); wm(m,a,8,0x1u32); }
            pc = 0x4b645c; continue;
            }
            0x4b645c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[6].wrapping_add(0xb5); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x4b6506; continue; }
            pc = 0x4b6479; continue;
            }
            0x4b6479 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(5,32));
            if c.cond(12) { pc = 0x4b64a8; continue; }
            pc = 0x4b647e; continue;
            }
            0x4b647e => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b64a8; continue; }
            pc = 0x4b6486; continue;
            }
            0x4b6486 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b64a8; continue; }
            pc = 0x4b6498; continue;
            }
            0x4b6498 => {
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4b64a8; continue;
            }
            0x4b64a8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0xfffffffdu32);
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b64c2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,0xfffffffdu32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b64de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x158),32); c.push(m, 0x4b6500); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4b6626; continue;
            }
            0x4b6506 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b6520; continue; }
            pc = 0x4b6516; continue;
            }
            0x4b6516 => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b6520);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5c20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b6520; continue;
            }
            0x4b6520 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc4),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4b65ad; continue; }
            pc = 0x4b652d; continue;
            }
            0x4b652d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b65ad; continue; }
            pc = 0x4b6538; continue;
            }
            0x4b6538 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4b654b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b65ad; continue; }
            pc = 0x4b654f; continue;
            }
            0x4b654f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b658d; continue; }
            pc = 0x4b6559; continue;
            }
            0x4b6559 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b658d; continue; }
            pc = 0x4b6561; continue;
            }
            0x4b6561 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b658d; continue; }
            pc = 0x4b6573; continue;
            }
            0x4b6573 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b658d; continue;
            }
            0x4b658d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffdu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b65a5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            pc = 0x4b660e; continue;
            }
            0x4b65ad => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b6626; continue; }
            pc = 0x4b65b8; continue;
            }
            0x4b65b8 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b65f0; continue; }
            pc = 0x4b65bc; continue;
            }
            0x4b65bc => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b65f0; continue; }
            pc = 0x4b65c4; continue;
            }
            0x4b65c4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4b65f0; continue; }
            pc = 0x4b65d6; continue;
            }
            0x4b65d6 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4b65f0; continue;
            }
            0x4b65f0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffdu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xb4),32); c.push(m, 0x4b6608); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            pc = 0x4b660e; continue;
            }
            0x4b660e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0xfffffffdu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b6626); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4b6626; continue;
            }
            0x4b6626 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,0xfffffffdu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x4b6634); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b663f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b7370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_set_hot_child(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b7920;
    loop {
        match pc {
            0x4b7920 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(7,32,c.get(1,32));
            if c.cond(4) { pc = 0x4b7974; continue; }
            pc = 0x4b792d; continue;
            }
            0x4b792d => {
            c.set(6,32,rm(m,0x6bbabcu32,32));
            { let a = c.get(6,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b7950; continue; }
            pc = 0x4b7938; continue;
            }
            0x4b7938 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[7].wrapping_add(0xc0),32),32); }
            if c.cond(13) { pc = 0x4b7950; continue; }
            pc = 0x4b7940; continue;
            }
            0x4b7940 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4b7977; continue; }
            pc = 0x4b7950; continue;
            }
            0x4b7950 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4b7974; continue; }
            pc = 0x4b795c; continue;
            }
            0x4b795c => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x94),32));
            pc = 0x4b7962; continue;
            }
            0x4b7962 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b796c; continue; }
            pc = 0x4b7968; continue;
            }
            0x4b7968 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4b79d4; continue; }
            pc = 0x4b796c; continue;
            }
            0x4b796c => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4b7962; continue; }
            pc = 0x4b7974; continue;
            }
            0x4b7974 => {
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            pc = 0x4b7977; continue;
            }
            0x4b7977 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc4),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b79ad; continue; }
            pc = 0x4b7984; continue;
            }
            0x4b7984 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1]),32));
            if c.cond(4) { pc = 0x4b79ad; continue; }
            pc = 0x4b7997; continue;
            }
            0x4b7997 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4b79ad; continue; }
            pc = 0x4b799b; continue;
            }
            0x4b799b => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b79ad); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b79ad; continue;
            }
            0x4b79ad => {
            { let a = c.get(6,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[7].wrapping_add(0xc4); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x4b79cc; continue; }
            pc = 0x4b79b8; continue;
            }
            0x4b79b8 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0]),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xbc),32); c.push(m, 0x4b79cc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b79cc; continue;
            }
            0x4b79cc => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4b79d4 => {
            { let a = 0x6bbabcu32; wm(m,a,32,c.get(6,32)); }
            pc = 0x4b7977; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_clear_hot_child(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b79e0;
    loop {
        match pc {
            0x4b79e0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b7a16; continue; }
            pc = 0x4b79f1; continue;
            }
            0x4b79f1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2]),32));
            if c.cond(4) { pc = 0x4b7a16; continue; }
            pc = 0x4b7a04; continue;
            }
            0x4b7a04 => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc0),32); c.push(m, 0x4b7a16); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b7a16; continue;
            }
            0x4b7a16 => {
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,0xffffffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn display_audio_config_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd290;
    loop {
        match pc {
            0x4bd290 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d23u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6c7cccu32,32));
            c.set(5,32,c.r[6].wrapping_add(0x4));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4bd2d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4bd2e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bd2f1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,32,0xac44u32); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x10u32); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,0x2u32); }
            c.set(0,32,c.get(6,32));
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

pub fn leave_current_battle_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611fa0;
    loop {
        match pc {
            0x611fa0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x3fc),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x611fbb; continue; }
            pc = 0x611faa; continue;
            }
            0x611faa => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x611fbb; continue; }
            pc = 0x611fb6; continue;
            }
            0x611fb6 => {
            { let args = []; c.r[0] = host.call(m, 0x6100d9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x611fbb => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6369b0;
    loop {
        match pc {
            0x6369b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6685b2u32);
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
            c.push(m, 0x6369d5);
            { let args = []; c.r[0] = host.call(m, 0x4b15a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x6369e6);
            { let args = []; c.r[0] = host.call(m, 0x4624a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6369f8);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x198));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x636a0a);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            { let a = c.r[6]; wm(m,a,32,0x67b6a8u32); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x67b6a0u32); }
            { let a = c.r[6].wrapping_add(0x290); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x294); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2ac); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x636a3b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x636a46); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x298); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2a8); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x636a66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2a4); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[2].wrapping_add(0x144),32); c.push(m, 0x636a7a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbdb8u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0x1de); wm(m,a,8,0x1u32); }
            c.set(0,32,c.get(6,32));
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

pub fn tutorial_window_ctor_with_arg(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636aa0;
    loop {
        match pc {
            0x636aa0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6685eeu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x636aca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x636adb);
            { let args = []; c.r[0] = host.call(m, 0x4624a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x636aed);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x198));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x636aff);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            { let a = c.r[6]; wm(m,a,32,0x67b6a8u32); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x67b6a0u32); }
            { let a = c.r[6].wrapping_add(0x290); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x294); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2ac); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x636b30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x636b3b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x298); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2a8); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x636b5b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2a4); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0x144),32); c.push(m, 0x636b6f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(0x1de); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
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

pub fn tutorial_window_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636ba0;
    loop {
        match pc {
            0x636ba0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x668645u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(3,32,c.r[6].wrapping_add(0x94));
            { let a = c.r[6]; wm(m,a,32,0x67b6a8u32); }
            { let a = c.r[3]; wm(m,a,32,0x67b6a0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x636bf0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x198),32));
            c.set(5,32,c.r[6].wrapping_add(0x198));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x636c04); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x298); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x2a8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x636c26); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2a4); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0x144),32); c.push(m, 0x636c3a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x2u32); }
            { let a = c.r[1].wrapping_add(0x1de); wm(m,a,8,0x0u32); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x636c53);
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x636c5f);
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.push(m, 0x636c6b);
            { let args = []; c.r[0] = host.call(m, 0x4624d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x636c7a);
            { let args = []; c.r[0] = host.call(m, 0x4b1640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_reset_state(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636c90;
    loop {
        match pc {
            0x636c90 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(1,32,c.r[6].wrapping_add(0xb0));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x636ca5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x198),32));
            c.set(1,32,c.r[6].wrapping_add(0x198));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x636cb7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x280); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x298); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x2a8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x636cd9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x2a4); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let target = rm(m,c.r[0].wrapping_add(0x144),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_free_pages(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636cf0;
    loop {
        match pc {
            0x636cf0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x636d00);
            { let args = []; c.r[0] = host.call(m, 0x4438a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x290),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x29c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2a0); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x636d3b; continue; }
            pc = 0x636d1e; continue;
            }
            0x636d1e => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x636d32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x636d38);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x636d3b; continue;
            }
            0x636d3b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x294),32));
            { let a = c.r[6].wrapping_add(0x290); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x636d68; continue; }
            pc = 0x636d4b; continue;
            }
            0x636d4b => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x636d5f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x636d65);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x636d68; continue;
            }
            0x636d68 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            { let a = c.r[6].wrapping_add(0x294); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x636dbb; continue; }
            pc = 0x636d78; continue;
            }
            0x636d78 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x636da2; continue; }
            pc = 0x636d84; continue;
            }
            0x636d84 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x636d94);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x636d84; continue; }
            pc = 0x636da2; continue;
            }
            0x636da2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x636db5; continue; }
            pc = 0x636dac; continue;
            }
            0x636dac => {
            c.push(m,c.get(0,32));
            c.push(m, 0x636db2);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x636db5; continue;
            }
            0x636db5 => {
            { let a = c.r[6].wrapping_add(0x2ac); wm(m,a,32,c.get(3,32)); }
            pc = 0x636dbb; continue;
            }
            0x636dbb => {
            { let a = c.r[6].wrapping_add(0x288); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_read_savegame_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636dd0;
    loop {
        match pc {
            0x636dd0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x668676u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,0x6c7a00u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x2u32); }
            c.push(m, 0x636e1b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6ee124u32);
            c.push(m,0x6c7a14u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x636e46);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x636e56);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            c.push(m, 0x636e6c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x636e7a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_load_pages(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637080;
    loop {
        match pc {
            0x637080 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6686d8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(5,32));
            c.push(m,0x6c7a90u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x6370c7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c7aa4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6370e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x288); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x637125; continue; }
            pc = 0x6370f1; continue;
            }
            0x6370f1 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x6370fe);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x63710f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x637125 => {
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            { let a = c.r[6].wrapping_add(0x2a4); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.push(m, 0x637138);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x637162; continue; }
            pc = 0x637148; continue;
            }
            0x637148 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(5,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x637160);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x637164; continue;
            }
            0x637162 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x637164; continue;
            }
            0x637164 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x290); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x637182);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x6371ac; continue; }
            pc = 0x637192; continue;
            }
            0x637192 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(5,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x6371aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x6371ae; continue;
            }
            0x6371ac => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x6371ae; continue;
            }
            0x6371ae => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x294); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x6371c8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.r[6].wrapping_add(0x2ac); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6371e0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            if c.cond(14) { pc = 0x63742d; continue; }
            pc = 0x637202; continue;
            }
            0x637202 => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c7ab0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x637212);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6ee126u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x63723d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x290),32));
            c.set(5,32,c.r[7].wrapping_mul(4));
            c.push(m,0x6dbb58u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m, 0x637256);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x290),32));
            c.push(m,0x6c7abeu32);
            c.push(m,0x6c7ac0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m, 0x63726d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c7ac4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x63727d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6ee127u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x6372a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x294),32));
            c.push(m,0x6dbb58u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m, 0x6372ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c7ad4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x6372ca);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6ee128u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x6372f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x637303);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x5u32); }
            c.push(m, 0x637311);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c79fcu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x637320);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64d390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x637410; continue; }
            pc = 0x637329; continue;
            }
            0x637329 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            c.push(m,0x14u32);
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(8)); wm(m,a,16,0x1u32); }
            c.push(m, 0x63733c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            { let a = c.r[2].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4),32));
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,0x6c79fcu32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x637363);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x637372);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x63737c);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            c.push(m,0x2cu32);
            c.push(m, 0x63738e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f6ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x63739b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6373a5);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[5].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            c.push(m,0x2cu32);
            c.push(m, 0x6373b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f6ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x6373c4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x6373ce);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            c.push(m,0x2cu32);
            c.push(m, 0x6373e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f6ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x6373ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6373f7);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,16,0xffu32); }
            { let a = c.r[5].wrapping_add(0x12); wm(m,a,8,0xe7u32); }
            pc = 0x637410; continue;
            }
            0x637410 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            c.push(m, 0x63741e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x637202; continue; }
            pc = 0x63742d; continue;
            }
            0x63742d => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x63743b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x637449);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x63745a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_relayout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637470;
    loop {
        match pc {
            0x637470 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x12c),32); c.push(m, 0x63747b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x637487); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6374c0;
    loop {
        match pc {
            0x6374c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x50u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x6374d5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637856; continue; }
            pc = 0x6374dd; continue;
            }
            0x6374dd => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x50));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x637513);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x637658; continue; }
            pc = 0x63751e; continue;
            }
            0x63751e => {
            c.set(2,32,rm(m,0x670f48u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x63752a);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x63753f);
            { let args = []; c.r[0] = host.call(m, 0x4bb430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m, 0x637546);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x637550);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x280),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x284),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x637658; continue; }
            pc = 0x637589; continue;
            }
            0x637589 => {
            c.set(1,32,c.r[5].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x637599; continue;
            }
            0x637599 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x10),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x6375bd; continue; }
            pc = 0x6375ae; continue;
            }
            0x6375ae => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x6375c9; continue;
            }
            0x6375bd => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x6375cd; continue; }
            pc = 0x6375c1; continue;
            }
            0x6375c1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x6375c9; continue;
            }
            0x6375c9 => {
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            pc = 0x6375cd; continue;
            }
            0x6375cd => {
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x637631; continue; }
            pc = 0x6375d1; continue;
            }
            0x6375d1 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x6375d3; continue;
            }
            0x6375d3 => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(5,32),32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[6],8));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x6375f4);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x6375fa);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x637618);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xcu32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x6375d3; continue; }
            pc = 0x637625; continue;
            }
            0x637625 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x637631; continue;
            }
            0x637631 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1cu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x637599; continue; }
            pc = 0x637658; continue;
            }
            0x637658 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2ac),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),16); c.sub(a,0x0u32,16); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x637856; continue; }
            pc = 0x63767e; continue;
            }
            0x63767e => {
            c.set(2,32,c.r[1].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x637685; continue;
            }
            0x637685 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x637835; continue; }
            pc = 0x637698; continue;
            }
            0x637698 => {
            c.set(5,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(3,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x6376c9; continue; }
            pc = 0x6376c7; continue;
            }
            0x6376c7 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x6376c9; continue;
            }
            0x6376c9 => {
            c.set(2,32,rm(m,0x6dcce4u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x6376da; continue; }
            pc = 0x6376d3; continue;
            }
            0x6376d3 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.get(2,32));
            pc = 0x6376da; continue;
            }
            0x6376da => {
            c.set(2,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,0xfffffffeu32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(3,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x637705; continue;
            }
            0x6376fd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x637705; continue;
            }
            0x637705 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[7].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x637717);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x6c),8));
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,8); let r = c.sub(a,0x14u32,8); c.set(2,8,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(6,8,c.get(1,8));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.or(a,0xffu32,8); c.set(2,8,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x637759);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(5,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[6]));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x637785);
            { let args = []; c.r[0] = host.call(m, 0x4bd680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(2,32,c.get(3,32));
            c.set(1,32,c.r[7].wrapping_add(0x1));
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[6]));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x6377bf);
            { let args = []; c.r[0] = host.call(m, 0x4bd680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x6377f6);
            { let args = []; c.r[0] = host.call(m, 0x4bd680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[3]));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x63781b);
            { let args = []; c.r[0] = host.call(m, 0x4bd680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x6376fd; continue; }
            pc = 0x637831; continue;
            }
            0x637831 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x637835; continue;
            }
            0x637835 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1],16));
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(12) { pc = 0x637685; continue; }
            pc = 0x637856; continue;
            }
            0x637856 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_pulse_highlight(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637860;
    loop {
        match pc {
            0x637860 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x28c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x2ac),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),16); c.sub(a,0x0u32,16); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x637a63; continue; }
            pc = 0x637889; continue;
            }
            0x637889 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x637894; continue;
            }
            0x637894 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[3],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x637a3a; continue; }
            pc = 0x6378a7; continue;
            }
            0x6378a7 => {
            c.set(6,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x6),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[3].wrapping_add(0xa),16));
            c.set(5,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x6378d8; continue; }
            pc = 0x6378d6; continue;
            }
            0x6378d6 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x6378d8; continue;
            }
            0x6378d8 => {
            c.set(0,32,rm(m,0x6dcce4u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x6378e4; continue; }
            pc = 0x6378e1; continue;
            }
            0x6378e1 => {
            c.set(7,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x6378e4; continue;
            }
            0x6378e4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x6378ef); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x637902; continue; }
            pc = 0x6378f3; continue;
            }
            0x6378f3 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x6378fe); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63793e; continue; }
            pc = 0x637902; continue;
            }
            0x637902 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x63790d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x63793e; continue; }
            pc = 0x637911; continue;
            }
            0x637911 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x63791c);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63792f; continue; }
            pc = 0x637920; continue;
            }
            0x637920 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x63792d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x63793a; continue;
            }
            0x63792f => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x63793a);
            { let args = []; c.r[0] = host.call(m, 0x453ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x63793a; continue;
            }
            0x63793a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x63793e; continue;
            }
            0x63793e => {
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x637956; continue; }
            pc = 0x63794a; continue;
            }
            0x63794a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x48),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637a3a; continue; }
            pc = 0x637956; continue;
            }
            0x637956 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x6u32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x637980);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(0,32,c.r[5].wrapping_add(0x6));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x6379aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.r[6].wrapping_add(0xfffffffb));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x6379db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xfffffffbu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x637a0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x4bu32,32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x12),8));
            c.set(1,16,sx(c.get(0,8),8)&0xffffu32);
            { let a = rm(m,c.r[3].wrapping_add(0x10),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[3].wrapping_add(0x10); wm(m,a,16,r); } }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,0x82u32,16); }
            if c.cond(6) { pc = 0x637a35; continue; }
            pc = 0x637a2e; continue;
            }
            0x637a2e => {
            { let a = c.get(1,16); c.sub(a,0xffu32,16); }
            if c.cond(2) { pc = 0x637a3a; continue; }
            pc = 0x637a35; continue;
            }
            0x637a35 => {
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            pc = 0x637a3a; continue;
            }
            0x637a3a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1],16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x637894; continue; }
            pc = 0x637a5f; continue;
            }
            0x637a5f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x637a63; continue;
            }
            0x637a63 => {
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637a70;
    loop {
        match pc {
            0x637a70 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc0),32));
            c.push(m, 0x637a86);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xfffffffdu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x637ac9; continue; }
            pc = 0x637a8d; continue;
            }
            0x637a8d => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,c.r[6].wrapping_add(0xb0));
            c.push(m,c.get(0,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc0),32));
            c.push(m, 0x637aa5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xfffffffeu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x637ac9; continue; }
            pc = 0x637aac; continue;
            }
            0x637aac => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x198));
            c.push(m,c.get(2,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc0),32));
            c.push(m, 0x637ac4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x637ad3; continue; }
            pc = 0x637ac9; continue;
            }
            0x637ac9 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x12c),32); c.push(m, 0x637ad3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637ad3; continue;
            }
            0x637ad3 => {
            c.set(1,32,c.r[6].wrapping_add(0xb0));
            c.push(m, 0x637ade);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637aee; continue; }
            pc = 0x637ae2; continue;
            }
            0x637ae2 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x637aec); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637b1a; continue;
            }
            0x637aee => {
            c.set(1,32,c.r[6].wrapping_add(0x198));
            c.push(m, 0x637af9);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x637b09; continue; }
            pc = 0x637aff; continue;
            }
            0x637aff => {
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x134),32); c.push(m, 0x637b07); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637b1a; continue;
            }
            0x637b09 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x637b1a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x637b1a; continue;
            }
            0x637b1a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2a0),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x637b45; continue; }
            pc = 0x637b24; continue;
            }
            0x637b24 => {
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x29c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x637b45; continue; }
            pc = 0x637b34; continue;
            }
            0x637b34 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x138),32); c.push(m, 0x637b3e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2a0); wm(m,a,8,0x1u32); }
            pc = 0x637b45; continue;
            }
            0x637b45 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x444030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_bring_to_front(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637bc0;
    loop {
        match pc {
            0x637bc0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x637bd1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            c.push(m,c.get(7,32));
            c.push(m, 0x637be3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(3,32,c.r[6].wrapping_add(0x198));
            c.push(m,c.get(3,32));
            c.push(m, 0x637bf5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x637c01);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x637c0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x637c19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_next_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637c20;
    loop {
        match pc {
            0x637c20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x637c55);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x637c68); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x288),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x637c87; continue; }
            pc = 0x637c7b; continue;
            }
            0x637c7b => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x140),32); c.push(m, 0x637c85); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637c99; continue;
            }
            0x637c87 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x637c99; continue; }
            pc = 0x637c92; continue;
            }
            0x637c92 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(1,32)); }
            pc = 0x637c99; continue;
            }
            0x637c99 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x637ca5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x637cd4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x637ce7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_previous_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637cf0;
    loop {
        match pc {
            0x637cf0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x637d25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x637d38); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x637d4e; continue; }
            pc = 0x637d47; continue;
            }
            0x637d47 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x28c); wm(m,a,32,c.get(0,32)); }
            pc = 0x637d4e; continue;
            }
            0x637d4e => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x637d5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x637d89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x637d9c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_play_page_voice(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637df0;
    loop {
        match pc {
            0x637df0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x28c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x294),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x637e19; continue; }
            pc = 0x637e06; continue;
            }
            0x637e06 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0xffffu32);
            c.push(m, 0x637e19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x442280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x637e19; continue;
            }
            0x637e19 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "multiplayer_screen_update_hotseat_setup_tail_start" => multiplayer_screen_update_hotseat_setup_tail_start(m, host, regs),
        "multiplayer_screen_invalidate_slot_row" => multiplayer_screen_invalidate_slot_row(m, host, regs),
        "multiplayer_screen_add_chat_line" => multiplayer_screen_add_chat_line(m, host, regs),
        "multiplayer_screen_update_start_button_enabled" => multiplayer_screen_update_start_button_enabled(m, host, regs),
        "multiplayer_screen_update_chat_scroll" => multiplayer_screen_update_chat_scroll(m, host, regs),
        "multiplayer_screen_abort_session" => multiplayer_screen_abort_session(m, host, regs),
        "multiplayer_screen_invalidate_sex_buttons" => multiplayer_screen_invalidate_sex_buttons(m, host, regs),
        "multiplayer_screen_save_local_profile" => multiplayer_screen_save_local_profile(m, host, regs),
        "multiplayer_screen_store_slot_profile" => multiplayer_screen_store_slot_profile(m, host, regs),
        "multiplayer_screen_init_list_geometry" => multiplayer_screen_init_list_geometry(m, host, regs),
        "multiplayer_screen_pick_home_town" => multiplayer_screen_pick_home_town(m, host, regs),
        "multiplayer_screen_update_sex_buttons" => multiplayer_screen_update_sex_buttons(m, host, regs),
        "multiplayer_screen_compose_chat_text" => multiplayer_screen_compose_chat_text(m, host, regs),
        "multiplayer_screen_release_lobby_controls" => multiplayer_screen_release_lobby_controls(m, host, regs),
        "multiplayer_screen_assign_merchant_to_slot" => multiplayer_screen_assign_merchant_to_slot(m, host, regs),
        "multiplayer_screen_load_slot_into_form" => multiplayer_screen_load_slot_into_form(m, host, regs),
        "multiplayer_screen_load_selected_merchant_into_form" => multiplayer_screen_load_selected_merchant_into_form(m, host, regs),
        "multiplayer_screen_is_merchant_choice_free" => multiplayer_screen_is_merchant_choice_free(m, host, regs),
        "multiplayer_screen_is_merchant_index_unused" => multiplayer_screen_is_merchant_index_unused(m, host, regs),
        "compose_initial_and_family_name" => compose_initial_and_family_name(m, host, regs),
        "multiplayer_screen_find_player_list_index" => multiplayer_screen_find_player_list_index(m, host, regs),
        "multiplayer_screen_rebuild_player_list" => multiplayer_screen_rebuild_player_list(m, host, regs),
        "multiplayer_screen_get_player_list_entry" => multiplayer_screen_get_player_list_entry(m, host, regs),
        "multiplayer_screen_release_resources" => multiplayer_screen_release_resources(m, host, regs),
        "multiplayer_screen_update" => multiplayer_screen_update(m, host, regs),
        "multiplayer_screen_disconnect_and_notify_matcher" => multiplayer_screen_disconnect_and_notify_matcher(m, host, regs),
        "multiplayer_show_error_message_for_code" => multiplayer_show_error_message_for_code(m, host, regs),
        "multiplayer_screen_enter_slot_pick_mode" => multiplayer_screen_enter_slot_pick_mode(m, host, regs),
        "multiplayer_screen_get_slot_mask" => multiplayer_screen_get_slot_mask(m, host, regs),
        "multiplayer_screen_send_matcher_command" => multiplayer_screen_send_matcher_command(m, host, regs),
        "multiplayer_screen_build_matcher_info_text" => multiplayer_screen_build_matcher_info_text(m, host, regs),
        "multiplayer_screen_scalar_deleting_dtor" => multiplayer_screen_scalar_deleting_dtor(m, host, regs),
        "options_window_ctor" => options_window_ctor(m, host, regs),
        "options_window_dtor" => options_window_dtor(m, host, regs),
        "options_window_unload_children" => options_window_unload_children(m, host, regs),
        "options_window_apply_and_save_settings" => options_window_apply_and_save_settings(m, host, regs),
        "options_window_update" => options_window_update(m, host, regs),
        "options_window_restore_snapshot" => options_window_restore_snapshot(m, host, regs),
        "options_window_set_game_speed_local" => options_window_set_game_speed_local(m, host, regs),
        "options_window_scalar_deleting_dtor" => options_window_scalar_deleting_dtor(m, host, regs),
        "game_main_window_ctor" => game_main_window_ctor(m, host, regs),
        "game_main_window_dtor" => game_main_window_dtor(m, host, regs),
        "game_main_window_shutdown" => game_main_window_shutdown(m, host, regs),
        "game_main_window_scalar_deleting_dtor" => game_main_window_scalar_deleting_dtor(m, host, regs),
        "find_game_cd_drive_letter" => find_game_cd_drive_letter(m, host, regs),
        "event_message_window_ctor" => event_message_window_ctor(m, host, regs),
        "event_message_window_present_rect" => event_message_window_present_rect(m, host, regs),
        "event_message_window_slot3_release_arg_return_zero" => event_message_window_slot3_release_arg_return_zero(m, host, regs),
        "event_window_open_or_defer" => event_window_open_or_defer(m, host, regs),
        "event_window_close" => event_window_close(m, host, regs),
        "event_window_deleting_dtor" => event_window_deleting_dtor(m, host, regs),
        "can_player_inspect_ship" => can_player_inspect_ship(m, host, regs),
        "new_game_screen_ctor" => new_game_screen_ctor(m, host, regs),
        "new_game_screen_dtor" => new_game_screen_dtor(m, host, regs),
        "new_game_screen_release_resources" => new_game_screen_release_resources(m, host, regs),
        "new_game_screen_prepare" => new_game_screen_prepare(m, host, regs),
        "new_game_screen_update_duration_buttons" => new_game_screen_update_duration_buttons(m, host, regs),
        "objective_duration_get_text" => objective_duration_get_text(m, host, regs),
        "objective_get_condition_text" => objective_get_condition_text(m, host, regs),
        "new_game_screen_update_arrow_buttons" => new_game_screen_update_arrow_buttons(m, host, regs),
        "new_game_screen_update_details_button_text" => new_game_screen_update_details_button_text(m, host, regs),
        "new_game_screen_load_map_for_start_year" => new_game_screen_load_map_for_start_year(m, host, regs),
        "new_game_screen_on_start_year_changed" => new_game_screen_on_start_year_changed(m, host, regs),
        "new_game_screen_refresh_map_and_town_labels" => new_game_screen_refresh_map_and_town_labels(m, host, regs),
        "new_game_screen_deleting_dtor" => new_game_screen_deleting_dtor(m, host, regs),
        "game_details_screen_ctor" => game_details_screen_ctor(m, host, regs),
        "game_details_screen_dtor" => game_details_screen_dtor(m, host, regs),
        "game_details_screen_release_resources" => game_details_screen_release_resources(m, host, regs),
        "game_details_screen_load_values" => game_details_screen_load_values(m, host, regs),
        "game_details_screen_process_input" => game_details_screen_process_input(m, host, regs),
        "game_settings_screen_scalar_deleting_dtor" => game_settings_screen_scalar_deleting_dtor(m, host, regs),
        "mission_select_screen_ctor" => mission_select_screen_ctor(m, host, regs),
        "mission_select_screen_dtor" => mission_select_screen_dtor(m, host, regs),
        "mission_select_screen_release_resources" => mission_select_screen_release_resources(m, host, regs),
        "mission_select_screen_load_campaign" => mission_select_screen_load_campaign(m, host, regs),
        "mission_select_screen_on_mouse_event" => mission_select_screen_on_mouse_event(m, host, regs),
        "mission_select_screen_scalar_deleting_dtor" => mission_select_screen_scalar_deleting_dtor(m, host, regs),
        "mission_entry_dtor" => mission_entry_dtor(m, host, regs),
        "screen_window_insert_child" => screen_window_insert_child(m, host, regs),
        "screen_window_set_hover_child" => screen_window_set_hover_child(m, host, regs),
        "screen_window_on_left_button_down" => screen_window_on_left_button_down(m, host, regs),
        "screen_window_on_left_button_up" => screen_window_on_left_button_up(m, host, regs),
        "screen_window_on_right_button_down" => screen_window_on_right_button_down(m, host, regs),
        "screen_window_on_right_button_up" => screen_window_on_right_button_up(m, host, regs),
        "screen_window_set_hot_child" => screen_window_set_hot_child(m, host, regs),
        "screen_window_clear_hot_child" => screen_window_clear_hot_child(m, host, regs),
        "display_audio_config_ctor" => display_audio_config_ctor(m, host, regs),
        "leave_current_battle_view" => leave_current_battle_view(m, host, regs),
        "tutorial_window_ctor" => tutorial_window_ctor(m, host, regs),
        "tutorial_window_ctor_with_arg" => tutorial_window_ctor_with_arg(m, host, regs),
        "tutorial_window_dtor" => tutorial_window_dtor(m, host, regs),
        "tutorial_window_reset_state" => tutorial_window_reset_state(m, host, regs),
        "tutorial_window_free_pages" => tutorial_window_free_pages(m, host, regs),
        "tutorial_window_read_savegame_name" => tutorial_window_read_savegame_name(m, host, regs),
        "tutorial_window_load_pages" => tutorial_window_load_pages(m, host, regs),
        "tutorial_window_relayout" => tutorial_window_relayout(m, host, regs),
        "tutorial_window_draw" => tutorial_window_draw(m, host, regs),
        "tutorial_window_pulse_highlight" => tutorial_window_pulse_highlight(m, host, regs),
        "tutorial_window_update" => tutorial_window_update(m, host, regs),
        "tutorial_window_bring_to_front" => tutorial_window_bring_to_front(m, host, regs),
        "tutorial_window_next_page" => tutorial_window_next_page(m, host, regs),
        "tutorial_window_previous_page" => tutorial_window_previous_page(m, host, regs),
        "tutorial_window_play_page_voice" => tutorial_window_play_page_voice(m, host, regs),
        _ => return None,
    })
}
