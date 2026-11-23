//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn dyn_gfx_text_box_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420e60;
    loop {
        match pc {
            0x420e60 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x420e68);
            { let args = []; c.r[0] = host.call(m, 0x4b15a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66c458u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dyn_gfx_text_box_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420e80;
    loop {
        match pc {
            0x420e80 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c678u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66c458u32); }
            c.set(1,32,c.r[6].wrapping_add(0x9c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x420eb6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x420ec5);
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

pub fn dyn_gfx_text_box_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420ee0;
    loop {
        match pc {
            0x420ee0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x420ee8);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x9c));
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dyn_gfx_text_box_load_texture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420f10;
    loop {
        match pc {
            0x420f10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c692u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,0x69618cu32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x420f53);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x69619cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x420f71);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x420f7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x420f8f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x420f9d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x420fae);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dyn_gfx_text_box_set_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420fd0;
    loop {
        match pc {
            0x420fd0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x94),32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x421006; continue; }
            pc = 0x420fe2; continue;
            }
            0x420fe2 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x421006; continue; }
            pc = 0x420fe9; continue;
            }
            0x420fe9 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0xc),32));
            pc = 0x42100e; continue;
            }
            0x421006 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            pc = 0x42100e; continue;
            }
            0x42100e => {
            c.set(2,32,c.r[0].wrapping_mul(4));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x42101f; continue; }
            pc = 0x42101d; continue;
            }
            0x42101d => {
            c.set(0,32,c.get(2,32));
            pc = 0x42101f; continue;
            }
            0x42101f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dyn_gfx_text_box_set_clip_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4214a0;
    loop {
        match pc {
            0x4214a0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4214e5; continue; }
            pc = 0x4214ab; continue;
            }
            0x4214ab => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4214e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4214e5; continue;
            }
            0x4214e5 => {
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dyn_gfx_text_box_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421550;
    loop {
        match pc {
            0x421550 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x421558);
            { let args = []; c.r[0] = host.call(m, 0x420e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x421568; continue; }
            pc = 0x42155f; continue;
            }
            0x42155f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x421565);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x421568; continue;
            }
            0x421568 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn atlas_texture_get_frame(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45c080;
    loop {
        match pc {
            0x45c080 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x45c0c5; continue; }
            pc = 0x45c08b; continue;
            }
            0x45c08b => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45c0c5; continue; }
            pc = 0x45c092; continue;
            }
            0x45c092 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x45c09d; continue; }
            pc = 0x45c09b; continue;
            }
            0x45c09b => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x45c09d; continue;
            }
            0x45c09d => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x45c0c5 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn draw_texture_excluding_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4673f0;
    loop {
        match pc {
            0x4673f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4]);
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x467406);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46740f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d0u32,32); c.push(m, 0x46741d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4674ad; continue; }
            pc = 0x467425; continue;
            }
            0x467425 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x467441);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[6]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x46745e);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[0]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x46747b);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4674a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(36); return c.r;
            }
            0x4674ad => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4674ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(36); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn direct_texture_button_ctor_default(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0890;
    loop {
        match pc {
            0x4b0890 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6618b6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bb5b8u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4b08bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c69a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.r[5].wrapping_add(0xec));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.r[5]; wm(m,a,32,0x670798u32); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4b08f5);
            { let args = []; c.r[0] = host.call(m, 0x4c6aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[5].wrapping_add(0x130));
            pc = 0x4b08fb; continue;
            }
            0x4b08fb => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4b0957; continue; }
            pc = 0x4b08ff; continue;
            }
            0x4b08ff => {
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4b0957; continue; }
            pc = 0x4b0904; continue;
            }
            0x4b0904 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b091e; continue; }
            pc = 0x4b090a; continue;
            }
            0x4b090a => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b091e; continue; }
            pc = 0x4b0915; continue;
            }
            0x4b0915 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b091b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b091e; continue;
            }
            0x4b091e => {
            c.push(m,0xcu32);
            c.set(1,32,c.r[3].wrapping_add(0xffffffd0));
            c.push(m,0x0u32);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b0939);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4b0943);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,32,0x6u32); }
            pc = 0x4b0957; continue;
            }
            0x4b0957 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xcu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4b08fb; continue; }
            pc = 0x4b0963; continue;
            }
            0x4b0963 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(5,32));
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

pub fn direct_texture_button_ctor_named(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0980;
    loop {
        match pc {
            0x4b0980 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6618d6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4b09aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c69a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0x190); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.r[5].wrapping_add(0xec));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.r[5]; wm(m,a,32,0x670798u32); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4b09e4);
            { let args = []; c.r[0] = host.call(m, 0x4c6aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[5].wrapping_add(0x130));
            pc = 0x4b09ea; continue;
            }
            0x4b09ea => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4b0a46; continue; }
            pc = 0x4b09ee; continue;
            }
            0x4b09ee => {
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4b0a46; continue; }
            pc = 0x4b09f3; continue;
            }
            0x4b09f3 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b0a0d; continue; }
            pc = 0x4b09f9; continue;
            }
            0x4b09f9 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b0a0d; continue; }
            pc = 0x4b0a04; continue;
            }
            0x4b0a04 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0a0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b0a0d; continue;
            }
            0x4b0a0d => {
            c.push(m,0xcu32);
            c.set(1,32,c.r[3].wrapping_add(0xffffffd0));
            c.push(m,0x0u32);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b0a28);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4b0a32);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,32,0x6u32); }
            pc = 0x4b0a46; continue;
            }
            0x4b0a46 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xcu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4b09ea; continue; }
            pc = 0x4b0a52; continue;
            }
            0x4b0a52 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(5,32));
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

pub fn direct_texture_button_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0a70;
    loop {
        match pc {
            0x4b0a70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6618f6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[5]; wm(m,a,32,0x670798u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.push(m, 0x4b0aa4);
            { let args = []; c.r[0] = host.call(m, 0x4c6aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[5].wrapping_add(0x130));
            c.set(6,32,c.r[5].wrapping_add(0xec));
            pc = 0x4b0ab2; continue;
            }
            0x4b0ab2 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4b0b0e; continue; }
            pc = 0x4b0ab6; continue;
            }
            0x4b0ab6 => {
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4b0b0e; continue; }
            pc = 0x4b0abb; continue;
            }
            0x4b0abb => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b0ad5; continue; }
            pc = 0x4b0ac1; continue;
            }
            0x4b0ac1 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b0ad5; continue; }
            pc = 0x4b0acc; continue;
            }
            0x4b0acc => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0ad2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b0ad5; continue;
            }
            0x4b0ad5 => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[3].wrapping_add(0xffffffd0));
            c.push(m,0x0u32);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b0af0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4b0afa);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,32,0x6u32); }
            pc = 0x4b0b0e; continue;
            }
            0x4b0b0e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xcu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4b0ab2; continue; }
            pc = 0x4b0b1a; continue;
            }
            0x4b0b1a => {
            c.set(1,32,c.r[5].wrapping_add(0x190));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b0b2a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b0b39);
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
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

pub fn direct_texture_button_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0b50;
    loop {
        match pc {
            0x4b0b50 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            c.push(m, 0x4b0b5b);
            { let args = []; c.r[0] = host.call(m, 0x4c6aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,c.r[3].wrapping_add(0x130));
            c.set(6,32,c.r[3].wrapping_add(0xec));
            pc = 0x4b0b69; continue;
            }
            0x4b0b69 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4b0bc5; continue; }
            pc = 0x4b0b6d; continue;
            }
            0x4b0b6d => {
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4b0bc5; continue; }
            pc = 0x4b0b72; continue;
            }
            0x4b0b72 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b0b8c; continue; }
            pc = 0x4b0b78; continue;
            }
            0x4b0b78 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[3]).wrapping_add(0xfc),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b0b8c; continue; }
            pc = 0x4b0b83; continue;
            }
            0x4b0b83 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0b89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b0b8c; continue;
            }
            0x4b0b8c => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,0x0u32);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(c.r[3]).wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b0ba7);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x4b0bb1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,32,0x6u32); }
            pc = 0x4b0bc5; continue;
            }
            0x4b0bc5 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xcu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4b0b69; continue; }
            pc = 0x4b0bd1; continue;
            }
            0x4b0bd1 => {
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

pub fn direct_texture_button_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0c70;
    loop {
        match pc {
            0x4b0c70 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x661908u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4b0ca7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b0cae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c6d20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b0cc1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn direct_texture_button_slot3_release_string(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0ce0;
    loop {
        match pc {
            0x4b0ce0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66191au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4b0d13);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b0d1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c74a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b0d2d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn direct_texture_button_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0d60;
    loop {
        match pc {
            0x4b0d60 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4b0d71); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b0e6a; continue; }
            pc = 0x4b0d79; continue;
            }
            0x4b0d79 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x4b0d95); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x4b0d9f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xec),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b0e45; continue; }
            pc = 0x4b0db0; continue;
            }
            0x4b0db0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0dba);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xec),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4b0dcb);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xec),32));
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0de1);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x4b0de8);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x130),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x134),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b0e24);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b0e2e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4b0e38);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4b0e42);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            pc = 0x4b0e45; continue;
            }
            0x4b0e45 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b0e6a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4c74c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4b0e6a; continue;
            }
            0x4b0e6a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn direct_texture_button_load_scaled_texture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b1360;
    loop {
        match pc {
            0x4b1360 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661958u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m, 0x4b1392);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x4b13a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb5c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x18c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x4b1481; continue; }
            pc = 0x4b13c0; continue;
            }
            0x4b13c0 => {
            c.push(m, 0x4b13c5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x184); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x4b13e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb2a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4b140b; continue; }
            pc = 0x4b13ee; continue;
            }
            0x4b13ee => {
            c.set(7,32,c.r[6].wrapping_add(0x190));
            c.push(m,0x520au32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4b1400);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x654c58, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[7],32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4b1408);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b140b; continue;
            }
            0x4b140b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4b1419);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b1427);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4b1430);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18c),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4b1443);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x184),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x180),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b1468);
            { let args = []; c.r[0] = host.call(m, 0x4bba80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x4b146e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.push(m, 0x4b1474);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b147a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4b14a3; continue;
            }
            0x4b1481 => {
            c.push(m, 0x4b1486);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x190));
            c.push(m,0x5208u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4b1498);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x654c58, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[7],32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4b14a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b14a3; continue;
            }
            0x4b14a3 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b14ba);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn direct_texture_button_get_class_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b14f0;
    loop {
        match pc {
            0x4b14f0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bb5b8u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x4b150b);
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

pub fn direct_texture_button_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b1520;
    loop {
        match pc {
            0x4b1520 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4b1528);
            { let args = []; c.r[0] = host.call(m, 0x4b0a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4b1538; continue; }
            pc = 0x4b152f; continue;
            }
            0x4b152f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4b1535);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b1538; continue;
            }
            0x4b1538 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_list_release_all_textures(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b4750;
    loop {
        match pc {
            0x4b4750 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4b4770; continue; }
            pc = 0x4b475d; continue;
            }
            0x4b475d => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m, 0x4b4768);
            { let args = []; c.r[0] = host.call(m, 0x4b3890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b475d; continue; }
            pc = 0x4b4770; continue;
            }
            0x4b4770 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_list_reload_all_textures(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b4780;
    loop {
        match pc {
            0x4b4780 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,8,0x1u32);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4b47a6; continue; }
            pc = 0x4b478f; continue;
            }
            0x4b478f => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b47a6; continue; }
            pc = 0x4b4793; continue;
            }
            0x4b4793 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m, 0x4b479e);
            { let args = []; c.r[0] = host.call(m, 0x4b38b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4b478f; continue; }
            pc = 0x4b47a6; continue;
            }
            0x4b47a6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_list_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b47d0;
    loop {
        match pc {
            0x4b47d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x670b30u32); }
            c.push(m, 0x4b47de);
            { let args = []; c.r[0] = host.call(m, 0x4b39e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4b47ee; continue; }
            pc = 0x4b47e5; continue;
            }
            0x4b47e5 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4b47eb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b47ee; continue;
            }
            0x4b47ee => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn present_frame(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b8ff0;
    loop {
        match pc {
            0x4b8ff0 => {
            c.set(0,32,rm(m,0x6dccecu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,0xffffffffu32);
            { let a = 0x6dccecu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4b9004);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xfffffffdu32);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x4b900d);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4b9016);
            { let args = []; c.r[0] = host.call(m, 0x4bb360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x4b901c);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b9022);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x4b9029);
            { let args = []; c.r[0] = host.call(m, 0x4bb460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dcbd1u32,8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbd1u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4b90a7; continue; }
            pc = 0x4b9048; continue;
            }
            0x4b9048 => {
            c.set(0,8,rm(m,0x6dcc13u32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcc13u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4b90a7; continue; }
            pc = 0x4b9064; continue;
            }
            0x4b9064 => {
            c.push(m,0x0u32);
            c.push(m,0x6bbbd4u32);
            { let a = 0x6dcbd1u32; wm(m,a,8,0x0u32); }
            { let a = 0x6dcc13u32; wm(m,a,8,0x0u32); }
            { let target = rm(m,0x66a2b0u32,32); c.push(m, 0x4b907f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,rm(m,0x66a574u32,32));
            { let target = c.get(6,32); c.push(m, 0x4b9087); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,0x2u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4b9092);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let target = c.get(6,32); c.push(m, 0x4b9097); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dccfcu32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = 0x6dccfcu32; wm(m,a,32,c.get(1,32)); }
            pc = 0x4b90a7; continue;
            }
            0x4b90a7 => {
            c.set(0,32,rm(m,0x6dcb94u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(5) { pc = 0x4b90d7; continue; }
            pc = 0x4b90b2; continue;
            }
            0x4b90b2 => {
            c.push(m, 0x4b90b7);
            { let args = []; c.r[0] = host.call(m, 0x4bb740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dcd04u32,32));
            c.set(1,32,rm(m,0x670f6cu32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = 0x6dcd04u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4b90d6; continue; }
            pc = 0x4b90cc; continue;
            }
            0x4b90cc => {
            { let a = 0x6dcd04u32; wm(m,a,32,0x0u32); }
            pc = 0x4b90d6; continue;
            }
            0x4b90d6 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4b90d7 => {
            { let args = []; c.r[0] = host.call(m, 0x4bb740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn rect_clipper_stuff(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b9650;
    loop {
        match pc {
            0x4b9650 => {
            c.set(0,32,rm(m,0x6dcb94u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4b96b3; continue; }
            pc = 0x4b965a; continue;
            }
            0x4b965a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4b96b3; continue; }
            pc = 0x4b9668; continue;
            }
            0x4b9668 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4b96b3; continue; }
            pc = 0x4b9671; continue;
            }
            0x4b9671 => {
            c.set(0,32,rm(m,0x670f6cu32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4b969d; continue; }
            pc = 0x4b967d; continue;
            }
            0x4b967d => {
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6dcd20),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4b968a);
            { let args = []; c.r[0] = host.call(m, 0x4bb780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b9690);
            { let args = []; c.r[0] = host.call(m, 0x4bb140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x670f6cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b967d; continue; }
            pc = 0x4b969d; continue;
            }
            0x4b969d => {
            c.set(0,32,rm(m,0x6dcd04u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd20),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4b96af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4b96b3; continue;
            }
            0x4b96b3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_load_driver_dll(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bad00;
    loop {
        match pc {
            0x4bad00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x404u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0d0u32,32); c.push(m, 0x4bad13); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = 0x6dab20u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4bad25; continue; }
            pc = 0x4bad1c; continue;
            }
            0x4bad1c => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x404u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bad25 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(0,32));
            c.set(3,8,0x1u32);
            c.set(6,32,0x6bbc2cu32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xb4u32); }
            pc = 0x4bad38; continue;
            }
            0x4bad38 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,0x66a0ccu32,32); c.push(m, 0x4bad40); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bad67; continue; }
            pc = 0x4bad46; continue;
            }
            0x4bad46 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x6bebf8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4bad56);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bad65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x656ca5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4bad67; continue;
            }
            0x4bad67 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfffffffc),32));
            { let a = c.get(6,32); let r = c.add(a,0x44u32,32); c.set(6,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4bad38; continue; }
            pc = 0x4bad7a; continue;
            }
            0x4bad7a => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x4bad8a; continue; }
            pc = 0x4bad80; continue;
            }
            0x4bad80 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = 0x6dab20u32; wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x4badc4; continue; }
            pc = 0x4bad8a; continue;
            }
            0x4bad8a => {
            c.set(0,32,rm(m,0x6dab24u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4badb3; continue; }
            pc = 0x4bad95; continue;
            }
            0x4bad95 => {
            c.set(0,32,rm(m,0x6da884u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bada0; continue; }
            pc = 0x4bad9e; continue;
            }
            0x4bad9e => {
            { let target = c.get(0,32); c.push(m, 0x4bada0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bada0; continue;
            }
            0x4bada0 => {
            c.set(1,32,rm(m,0x6dab20u32,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a128u32,32); c.push(m, 0x4badad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6dab24u32; wm(m,a,32,c.get(6,32)); }
            pc = 0x4badb3; continue;
            }
            0x4badb3 => {
            { let a = 0x6dab20u32; wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x404u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4badc4 => {
            { let a = 0x6dab24u32; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x404u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_unload_driver_dll(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bae80;
    loop {
        match pc {
            0x4bae80 => {
            c.set(0,32,rm(m,0x6dab24u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4baeaa; continue; }
            pc = 0x4bae89; continue;
            }
            0x4bae89 => {
            c.set(0,32,rm(m,0x6da884u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bae94; continue; }
            pc = 0x4bae92; continue;
            }
            0x4bae92 => {
            { let target = c.get(0,32); c.push(m, 0x4bae94); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bae94; continue;
            }
            0x4bae94 => {
            c.set(0,32,rm(m,0x6dab20u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a128u32,32); c.push(m, 0x4baea0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6dab24u32; wm(m,a,32,0x0u32); }
            pc = 0x4baeaa; continue;
            }
            0x4baeaa => {
            { let a = 0x6dab20u32; wm(m,a,32,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_AddClipRect_4i(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb130;
    loop {
        match pc {
            0x4bb130 => {
            { let target = rm(m,0x6daa90u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_Begin3D(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb200;
    loop {
        match pc {
            0x4bb200 => {
            { let target = rm(m,0x6daa74u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_ChangePalette(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb210;
    loop {
        match pc {
            0x4bb210 => {
            { let target = rm(m,0x6daa70u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_ChangeTextureFlags(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb220;
    loop {
        match pc {
            0x4bb220 => {
            { let target = rm(m,0x6daa6cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_CleanUp(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb240;
    loop {
        match pc {
            0x4bb240 => {
            { let target = rm(m,0x6daa64u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_ClearBuffer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb260;
    loop {
        match pc {
            0x4bb260 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x6daa5cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_copy_clipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb280;
    loop {
        match pc {
            0x4bb280 => {
            { let target = rm(m,0x6daa54u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_CreateMemoryTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb2a0;
    loop {
        match pc {
            0x4bb2a0 => {
            { let target = rm(m,0x6daa4cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_Blt(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb350;
    loop {
        match pc {
            0x4bb350 => {
            { let target = rm(m,0x6daa20u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_Blt_RC(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb360;
    loop {
        match pc {
            0x4bb360 => {
            { let target = rm(m,0x6daa1cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_sgl_draw_line(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb390;
    loop {
        match pc {
            0x4bb390 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x6daa14u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_DrawScene(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb3c0;
    loop {
        match pc {
            0x4bb3c0 => {
            { let target = rm(m,0x6daa10u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ui_render_text_at(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb3e0;
    loop {
        match pc {
            0x4bb3e0 => {
            { let target = rm(m,0x6daa08u32,32); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_DrawText_Rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb3f0;
    loop {
        match pc {
            0x4bb3f0 => {
            { let target = rm(m,0x6daa04u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_Finish3D(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb440;
    loop {
        match pc {
            0x4bb440 => {
            { let target = rm(m,0x6da9f0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SwapBuffer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb460;
    loop {
        match pc {
            0x4bb460 => {
            { let target = rm(m,0x6da9e8u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_FreeFont(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb480;
    loop {
        match pc {
            0x4bb480 => {
            { let target = rm(m,0x6da9e0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_free_rect_clipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb4c0;
    loop {
        match pc {
            0x4bb4c0 => {
            { let target = rm(m,0x6da9d0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_FreeRectClipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb4d0;
    loop {
        match pc {
            0x4bb4d0 => {
            { let target = rm(m,0x6da9d0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_GetProjectionData(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb510;
    loop {
        match pc {
            0x4bb510 => {
            { let target = rm(m,0x6da9bcu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_get_rect_clipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb520;
    loop {
        match pc {
            0x4bb520 => {
            { let target = rm(m,0x6da9b8u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_CreateClipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb530;
    loop {
        match pc {
            0x4bb530 => {
            { let target = rm(m,0x6da9b4u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_GrabScreen(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb550;
    loop {
        match pc {
            0x4bb550 => {
            { let target = rm(m,0x6da9acu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_InitHardware(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb570;
    loop {
        match pc {
            0x4bb570 => {
            { let target = rm(m,0x6da9a0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_InitScreen(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb590;
    loop {
        match pc {
            0x4bb590 => {
            { let target = rm(m,0x6da99cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_LoadMemoryTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb5c0;
    loop {
        match pc {
            0x4bb5c0 => {
            { let target = rm(m,0x6da990u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_ReLoadMemoryTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb6f0;
    loop {
        match pc {
            0x4bb6f0 => {
            { let target = rm(m,0x6da94cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_ResetClipper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb740;
    loop {
        match pc {
            0x4bb740 => {
            { let target = rm(m,0x6da938u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_set_2d_clipping_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb760;
    loop {
        match pc {
            0x4bb760 => {
            { let target = rm(m,0x6da930u32,32); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_Get2dClippingMode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb770;
    loop {
        match pc {
            0x4bb770 => {
            { let target = rm(m,0x6da92cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetArchive(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb790;
    loop {
        match pc {
            0x4bb790 => {
            { let target = rm(m,0x6da924u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetBlendMode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb7a0;
    loop {
        match pc {
            0x4bb7a0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x6da920u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_IsRectInClippingRegion(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb7c0;
    loop {
        match pc {
            0x4bb7c0 => {
            { let target = rm(m,0x6da904u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_set_clip_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb800;
    loop {
        match pc {
            0x4bb800 => {
            { let target = rm(m,0x6da918u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetClipRect_4i(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb810;
    loop {
        match pc {
            0x4bb810 => {
            { let target = rm(m,0x6da914u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ddraw_set_clip_rect_r(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb820;
    loop {
        match pc {
            0x4bb820 => {
            { let target = rm(m,0x6da910u32,32); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_GetClipRect_r(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb840;
    loop {
        match pc {
            0x4bb840 => {
            { let target = rm(m,0x6da908u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetColorKey(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb850;
    loop {
        match pc {
            0x4bb850 => {
            { let target = rm(m,0x6da8f4u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_GetColorKey(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb860;
    loop {
        match pc {
            0x4bb860 => {
            { let target = rm(m,0x6da8f0u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_GetConstantColor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb880;
    loop {
        match pc {
            0x4bb880 => {
            { let target = rm(m,0x6da8e8u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetScreenDimensions(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bb9d0;
    loop {
        match pc {
            0x4bb9d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x6da8a4u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SetZoomFactor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bba50;
    loop {
        match pc {
            0x4bba50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x6da888u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_StretchBitmapRect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bba80;
    loop {
        match pc {
            0x4bba80 => {
            { let target = rm(m,0x6da880u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_StrLen(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bba90;
    loop {
        match pc {
            0x4bba90 => {
            { let target = rm(m,0x6da87cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SubClipRect_4i(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbaa0;
    loop {
        match pc {
            0x4bbaa0 => {
            { let target = rm(m,0x6da878u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SubClipRect_r(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbab0;
    loop {
        match pc {
            0x4bbab0 => {
            { let target = rm(m,0x6da874u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_SwitchScreenMode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbac0;
    loop {
        match pc {
            0x4bbac0 => {
            { let target = rm(m,0x6da86cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_PrefetchTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbaf0;
    loop {
        match pc {
            0x4bbaf0 => {
            { let target = rm(m,0x6da860u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_LockTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbb00;
    loop {
        match pc {
            0x4bbb00 => {
            { let target = rm(m,0x6da85cu32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_UnlockTexture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbb10;
    loop {
        match pc {
            0x4bbb10 => {
            { let target = rm(m,0x6da864u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_MessageBox(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbb30;
    loop {
        match pc {
            0x4bbb30 => {
            { let target = rm(m,0x6da854u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn report_win32_error_at_file_line(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbe40;
    loop {
        match pc {
            0x4bbe40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d3bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x204u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4]; wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,0x200u32);
            c.push(m,c.get(1,32));
            c.push(m,0x400u32);
            { let a = c.r[4].wrapping_add(0x21c); wm(m,a,32,0x0u32); }
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x4bbe86); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x1200u32);
            { let target = rm(m,0x66a294u32,32); c.push(m, 0x4bbe94); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x218),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x214),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x6bed7cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4bbeb8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4]);
            { let a = c.r[4].wrapping_add(0x20c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4bbecf);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x204),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x210u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_gl_cfg_and_init_graphics(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc010;
    loop {
        match pc {
            0x4bc010 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d5du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,0x6bec68u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4bc047);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,0x6bec70u32,32));
            c.push(m,0x6bedb6u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4bc061);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc105; continue; }
            pc = 0x4bc06e; continue;
            }
            0x4bc06e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc0f4; continue; }
            pc = 0x4bc077; continue;
            }
            0x4bc077 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc08e);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc0ea; continue; }
            pc = 0x4bc09a; continue;
            }
            0x4bc09a => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc0a9);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc0d1; continue; }
            pc = 0x4bc0b0; continue;
            }
            0x4bc0b0 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc0b7);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc0d5; continue; }
            pc = 0x4bc0be; continue;
            }
            0x4bc0be => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc0ca);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc0b0; continue; }
            pc = 0x4bc0d1; continue;
            }
            0x4bc0d1 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc0ea; continue;
            }
            0x4bc0d5 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc0e1);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc0f2; continue; }
            pc = 0x4bc0ea; continue;
            }
            0x4bc0ea => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc077; continue; }
            pc = 0x4bc0f0; continue;
            }
            0x4bc0f0 => {
            pc = 0x4bc0f4; continue;
            }
            0x4bc0f2 => {
            c.set(3,8,0x1u32);
            pc = 0x4bc0f4; continue;
            }
            0x4bc0f4 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc0fa);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc105; continue; }
            pc = 0x4bc101; continue;
            }
            0x4bc101 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc10e; continue; }
            pc = 0x4bc105; continue;
            }
            0x4bc105 => {
            c.set(1,32,rm(m,0x6beec4u32,32));
            c.push(m,c.get(1,32));
            pc = 0x4bc10f; continue;
            }
            0x4bc10e => {
            c.push(m,c.get(7,32));
            pc = 0x4bc10f; continue;
            }
            0x4bc10f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4bc11a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6bef00u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bc130);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x2u32); }
            c.push(m, 0x4bc13f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.push(m, 0x4bc14d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed10u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bc167);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc174; continue;
            }
            0x4bc174 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc204; continue; }
            pc = 0x4bc184; continue;
            }
            0x4bc184 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc19b);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc1f7; continue; }
            pc = 0x4bc1a7; continue;
            }
            0x4bc1a7 => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc1b6);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc1de; continue; }
            pc = 0x4bc1bd; continue;
            }
            0x4bc1bd => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc1c4);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc1e2; continue; }
            pc = 0x4bc1cb; continue;
            }
            0x4bc1cb => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc1d7);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc1bd; continue; }
            pc = 0x4bc1de; continue;
            }
            0x4bc1de => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc1f7; continue;
            }
            0x4bc1e2 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc1ee);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc1ff; continue; }
            pc = 0x4bc1f7; continue;
            }
            0x4bc1f7 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc184; continue; }
            pc = 0x4bc1fd; continue;
            }
            0x4bc1fd => {
            pc = 0x4bc204; continue;
            }
            0x4bc1ff => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x4bc204; continue;
            }
            0x4bc204 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc20a);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc215; continue;
            }
            0x4bc215 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc219; continue;
            }
            0x4bc219 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,0x6bedb8u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc229);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc245; continue; }
            pc = 0x4bc235; continue;
            }
            0x4bc235 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc268; continue;
            }
            0x4bc245 => {
            c.set(6,32,0x6becccu32);
            pc = 0x4bc24a; continue;
            }
            0x4bc24a => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bc253);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc25d; continue; }
            pc = 0x4bc25a; continue;
            }
            0x4bc25a => {
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc25d; continue;
            }
            0x4bc25d => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc24a; continue; }
            pc = 0x4bc268; continue;
            }
            0x4bc268 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[3].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed20u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc282);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc28f; continue;
            }
            0x4bc28f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc315; continue; }
            pc = 0x4bc298; continue;
            }
            0x4bc298 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc2af);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc30b; continue; }
            pc = 0x4bc2bb; continue;
            }
            0x4bc2bb => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc2ca);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc2f2; continue; }
            pc = 0x4bc2d1; continue;
            }
            0x4bc2d1 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc2d8);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc2f6; continue; }
            pc = 0x4bc2df; continue;
            }
            0x4bc2df => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc2eb);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc2d1; continue; }
            pc = 0x4bc2f2; continue;
            }
            0x4bc2f2 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc30b; continue;
            }
            0x4bc2f6 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc302);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc313; continue; }
            pc = 0x4bc30b; continue;
            }
            0x4bc30b => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc298; continue; }
            pc = 0x4bc311; continue;
            }
            0x4bc311 => {
            pc = 0x4bc315; continue;
            }
            0x4bc313 => {
            c.set(3,8,0x1u32);
            pc = 0x4bc315; continue;
            }
            0x4bc315 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc31b);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc322; continue;
            }
            0x4bc322 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc326; continue;
            }
            0x4bc326 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0x6bedbcu32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc336);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc356; continue; }
            pc = 0x4bc342; continue;
            }
            0x4bc342 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc37d; continue;
            }
            0x4bc356 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(6,32,0x6becccu32);
            pc = 0x4bc35f; continue;
            }
            0x4bc35f => {
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bc368);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc372; continue; }
            pc = 0x4bc36f; continue;
            }
            0x4bc36f => {
            { let a = c.r[3].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc372; continue;
            }
            0x4bc372 => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc35f; continue; }
            pc = 0x4bc37d; continue;
            }
            0x4bc37d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[2].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed2cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc39b);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc3a8; continue;
            }
            0x4bc3a8 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc42e; continue; }
            pc = 0x4bc3b1; continue;
            }
            0x4bc3b1 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc3c8);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc424; continue; }
            pc = 0x4bc3d4; continue;
            }
            0x4bc3d4 => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc3e3);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc40b; continue; }
            pc = 0x4bc3ea; continue;
            }
            0x4bc3ea => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc3f1);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc40f; continue; }
            pc = 0x4bc3f8; continue;
            }
            0x4bc3f8 => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc404);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc3ea; continue; }
            pc = 0x4bc40b; continue;
            }
            0x4bc40b => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc424; continue;
            }
            0x4bc40f => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc41b);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc42c; continue; }
            pc = 0x4bc424; continue;
            }
            0x4bc424 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc3b1; continue; }
            pc = 0x4bc42a; continue;
            }
            0x4bc42a => {
            pc = 0x4bc42e; continue;
            }
            0x4bc42c => {
            c.set(3,8,0x1u32);
            pc = 0x4bc42e; continue;
            }
            0x4bc42e => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc434);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc43b; continue;
            }
            0x4bc43b => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc43f; continue;
            }
            0x4bc43f => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0x6bedc0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc44f);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc46f; continue; }
            pc = 0x4bc45f; continue;
            }
            0x4bc45f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[3].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc498; continue;
            }
            0x4bc46f => {
            c.set(6,32,0x6becccu32);
            pc = 0x4bc474; continue;
            }
            0x4bc474 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc47d);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc487; continue; }
            pc = 0x4bc484; continue;
            }
            0x4bc484 => {
            { let a = c.r[3].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc487; continue;
            }
            0x4bc487 => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc474; continue; }
            pc = 0x4bc492; continue;
            }
            0x4bc492 => {
            pc = 0x4bc498; continue;
            }
            0x4bc494 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x4bc498; continue;
            }
            0x4bc498 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x670f7cu32,32));
            c.set(0,32,rm(m,0x6bed44u32,32));
            c.set(6,32,rm(m,0x66a2c8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = c.get(6,32); c.push(m, 0x4bc4b9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x2b11u32,32); }
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc4cc; continue; }
            pc = 0x4bc4c3; continue;
            }
            0x4bc4c3 => {
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,0x2b11u32); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4cc => {
            { let a = c.get(0,32); c.sub(a,0xac44u32,32); }
            if c.cond(14) { pc = 0x4bc4dc; continue; }
            pc = 0x4bc4d3; continue;
            }
            0x4bc4d3 => {
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,0xac44u32); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4dc => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x2b11u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4bc4f1; continue; }
            pc = 0x4bc4e8; continue;
            }
            0x4bc4e8 => {
            c.set(2,32,rm(m,0x670f7cu32,32));
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4f1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x670f80u32,32));
            c.set(2,32,rm(m,0x6bed50u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(6,32); c.push(m, 0x4bc50c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc51d; continue; }
            pc = 0x4bc514; continue;
            }
            0x4bc514 => {
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,0x8u32); }
            pc = 0x4bc542; continue;
            }
            0x4bc51d => {
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(14) { pc = 0x4bc52b; continue; }
            pc = 0x4bc522; continue;
            }
            0x4bc522 => {
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,0x10u32); }
            pc = 0x4bc542; continue;
            }
            0x4bc52b => {
            { let a = c.get(0,32); let r = c.and(a,0x80000007u32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x4bc537; continue; }
            pc = 0x4bc532; continue;
            }
            0x4bc532 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x4bc537; continue;
            }
            0x4bc537 => {
            if c.cond(4) { pc = 0x4bc542; continue; }
            pc = 0x4bc539; continue;
            }
            0x4bc539 => {
            c.set(1,32,rm(m,0x670f80u32,32));
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc542; continue;
            }
            0x4bc542 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x670f84u32,32));
            c.set(1,32,rm(m,0x6bed60u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(6,32); c.push(m, 0x4bc55d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc56e; continue; }
            pc = 0x4bc565; continue;
            }
            0x4bc565 => {
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,0x1u32); }
            pc = 0x4bc57a; continue;
            }
            0x4bc56e => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x4bc57a; continue; }
            pc = 0x4bc573; continue;
            }
            0x4bc573 => {
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,0x2u32); }
            pc = 0x4bc57a; continue;
            }
            0x4bc57a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6beef0u32,32));
            c.set(2,32,rm(m,0x6beca4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6bec5cu32,32));
            c.push(m,0x400u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a2c4u32,32); c.push(m, 0x4bc5a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6bedc4u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc5bc);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4bc5d2; continue; }
            pc = 0x4bc5c4; continue;
            }
            0x4bc5c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc5d2; continue;
            }
            0x4bc5d2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x6becb4u32,32));
            c.set(1,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = c.get(6,32); c.push(m, 0x4bc5e8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(6,32,rm(m,0x66a528u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x10u32);
            { let a = c.r[3].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let target = c.get(6,32); c.push(m, 0x4bc604); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4bc610; continue; }
            pc = 0x4bc609; continue;
            }
            0x4bc609 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bc610; continue;
            }
            0x4bc610 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x54u32);
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let target = c.get(6,32); c.push(m, 0x4bc620); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4bc62c; continue; }
            pc = 0x4bc625; continue;
            }
            0x4bc625 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc62c; continue;
            }
            0x4bc62c => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4bc632);
            { let args = []; c.r[0] = host.call(m, 0x4bc680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,c.get(0,8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc64f; continue; }
            pc = 0x4bc63b; continue;
            }
            0x4bc63b => {
            c.push(m, 0x4bc640);
            { let args = []; c.r[0] = host.call(m, 0x4bb240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x4bc645);
            { let args = []; c.r[0] = host.call(m, 0x4bae80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6dcb8cu32; wm(m,a,32,0x0u32); }
            pc = 0x4bc64f; continue;
            }
            0x4bc64f => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc65d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4bc66a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_shutdown(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc770;
    loop {
        match pc {
            0x4bc770 => {
            c.set(0,32,rm(m,0x670f6cu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4bc79d; continue; }
            pc = 0x4bc77f; continue;
            }
            0x4bc77f => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6dcd20),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc793; continue; }
            pc = 0x4bc78a; continue;
            }
            0x4bc78a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc790);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bc793; continue;
            }
            0x4bc793 => {
            c.set(0,32,rm(m,0x670f6cu32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4bc77f; continue; }
            pc = 0x4bc79d; continue;
            }
            0x4bc79d => {
            { let a = rm(m,0x6dcb8cu32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc7e1; continue; }
            pc = 0x4bc7a5; continue;
            }
            0x4bc7a5 => {
            c.set(0,32,rm(m,0x6dcb9cu32,32));
            c.set(6,32,rm(m,0x66a528u32,32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4bc7bd; continue; }
            pc = 0x4bc7b5; continue;
            }
            0x4bc7b5 => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            { let target = c.get(6,32); c.push(m, 0x4bc7bd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4bc7bd; continue;
            }
            0x4bc7bd => {
            c.set(0,32,rm(m,0x6dcba0u32,32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4bc7cf; continue; }
            pc = 0x4bc7c7; continue;
            }
            0x4bc7c7 => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x56u32);
            { let target = c.get(6,32); c.push(m, 0x4bc7cf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4bc7cf; continue;
            }
            0x4bc7cf => {
            { let a = rm(m,0x6dcb8cu32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc7e1; continue; }
            pc = 0x4bc7d7; continue;
            }
            0x4bc7d7 => {
            c.push(m, 0x4bc7dc);
            { let args = []; c.r[0] = host.call(m, 0x4bb240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x4bc7e1);
            { let args = []; c.r[0] = host.call(m, 0x4bae80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bc7e1; continue;
            }
            0x4bc7e1 => {
            { let a = 0x6dccc0u32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6dccc4u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_open_texture_archives(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc880;
    loop {
        match pc {
            0x4bc880 => {
            c.set(0,32,rm(m,0x6dcb8cu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x400u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x4bc903; continue; }
            pc = 0x4bc890; continue;
            }
            0x4bc890 => {
            c.push(m,0x1u32);
            c.push(m,0x6bee08u32);
            c.push(m, 0x4bc89c);
            { let args = []; c.r[0] = host.call(m, 0x6389e8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc903; continue; }
            pc = 0x4bc8a5; continue;
            }
            0x4bc8a5 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40c),32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4bc8b0; continue;
            }
            0x4bc8b0 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc8bf);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc8ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6389e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc8b0; continue; }
            pc = 0x4bc8d1; continue;
            }
            0x4bc8d1 => {
            c.set(0,32,rm(m,0x6dcb8cu32,32));
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x4bc90d; continue; }
            pc = 0x4bc8dd; continue;
            }
            0x4bc8dd => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc8fa; continue; }
            pc = 0x4bc8e1; continue;
            }
            0x4bc8e1 => {
            c.push(m,0x0u32);
            c.push(m, 0x4bc8e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6dccccu32; wm(m,a,32,c.get(0,32)); }
            { let a = 0x6dccc8u32; wm(m,a,32,0x0u32); }
            pc = 0x4bc8fa; continue;
            }
            0x4bc8fa => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc900);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389dc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bc903; continue;
            }
            0x4bc903 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x400u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bc90d => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc903; continue; }
            pc = 0x4bc911; continue;
            }
            0x4bc911 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc917);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6dccc8u32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6dccccu32; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x400u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_close_texture_archives(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc930;
    loop {
        match pc {
            0x4bc930 => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc967; continue; }
            pc = 0x4bc939; continue;
            }
            0x4bc939 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dcb8cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc95d; continue; }
            pc = 0x4bc945; continue;
            }
            0x4bc945 => {
            c.set(0,32,rm(m,0x6dccccu32,32));
            c.push(m,c.get(0,32));
            { let a = 0x6dccc8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4bc955);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6dccccu32; wm(m,a,32,c.get(0,32)); }
            pc = 0x4bc95d; continue;
            }
            0x4bc95d => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc963);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389dc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4bc967; continue;
            }
            0x4bc967 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_get_texture_archive(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc970;
    loop {
        match pc {
            0x4bc970 => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_load_memory_texture_from_archive(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bca00;
    loop {
        match pc {
            0x4bca00 => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bca0d);
            { let args = []; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(6,32,c.get(0,32));
            c.push(m, 0x4bca1e);
            { let args = []; c.r[0] = host.call(m, 0x4bb5c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4bca26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_reload_memory_texture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bca70;
    loop {
        match pc {
            0x4bca70 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m, 0x4bca7b);
            { let args = []; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4bca8c);
            { let args = []; c.r[0] = host.call(m, 0x4bb6f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bcab0; continue; }
            pc = 0x4bca95; continue;
            }
            0x4bca95 => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bcab0; continue; }
            pc = 0x4bca9e; continue;
            }
            0x4bca9e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bcaa4);
            { let args = []; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4bcaab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb6f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x4bcab0; continue;
            }
            0x4bcab0 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4bcab6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(6,32));
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

pub fn sgl2d_load_texture_image(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bcb90;
    loop {
        match pc {
            0x4bcb90 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m, 0x4bcb9a);
            { let args = []; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bcba6);
            { let args = []; c.r[0] = host.call(m, 0x4bb640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bcbc9; continue; }
            pc = 0x4bcbaf; continue;
            }
            0x4bcbaf => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bcbc9; continue; }
            pc = 0x4bcbb8; continue;
            }
            0x4bcbb8 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bcbbe);
            { let args = []; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bcbc4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x4bcbc9; continue;
            }
            0x4bcbc9 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4bcbcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_update_frame_timer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd180;
    loop {
        match pc {
            0x4bd180 => {
            c.set(1,32,rm(m,0x6dccecu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a574u32,32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = 0x6dccecu32; wm(m,a,32,c.get(1,32)); }
            { let target = c.get(7,32); c.push(m, 0x4bd197); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dccfcu32,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(3) { pc = 0x4bd1c6; continue; }
            pc = 0x4bd1ae; continue;
            }
            0x4bd1ae => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x4bd1ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let target = c.get(7,32); c.push(m, 0x4bd1bc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6dccfcu32,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x4bd1ce; continue;
            }
            0x4bd1c6 => {
            c.push(m,0x1u32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x4bd1ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bd1ce; continue;
            }
            0x4bd1ce => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,c.get(6,32));
            { let a = 0x6dccf0u32; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = 0x6dccf4u32; wm(m,a,32,c.get(0,32)); }
            { let a = 0x6dccf8u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_3d_object_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd7a0;
    loop {
        match pc {
            0x4bd7a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d78u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bee2cu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4bd7c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[6]; wm(m,a,32,0x670de8u32); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            c.push(m, 0x4bd7e7);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
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

pub fn csgl2d_3dobject_ctor_with_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd810;
    loop {
        match pc {
            0x4bd810 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d8au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4bd837);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[6]; wm(m,a,32,0x670de8u32); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            c.push(m, 0x4bd856);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd880;
    loop {
        match pc {
            0x4bd880 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d9cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x670de8u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x4bd8b0);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4bd8c9);
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

pub fn csgl2d_3dobject_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd8e0;
    loop {
        match pc {
            0x4bd8e0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4bd8e8);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_slot2_return_false(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd900;
    loop {
        match pc {
            0x4bd900 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4bd909);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_slot3_return_false(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd910;
    loop {
        match pc {
            0x4bd910 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4bd919);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_get_client_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd9c0;
    loop {
        match pc {
            0x4bd9c0 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4]);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4bd9cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4],32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_set_size_from_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bda00;
    loop {
        match pc {
            0x4bda00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[6].wrapping_add(0x4c),32); c.push(m, 0x4bda14); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_set_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bda20;
    loop {
        match pc {
            0x4bda20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_draw_stub(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bda40;
    loop {
        match pc {
            0x4bda40 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x94),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bda4f; continue; }
            pc = 0x4bda4a; continue;
            }
            0x4bda4a => {
            c.push(m, 0x4bda4f);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bda4f; continue;
            }
            0x4bda4f => {
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_get_class_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bdb00;
    loop {
        match pc {
            0x4bdb00 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bee2cu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x4bdb1b);
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

pub fn csgl2d_3dobject_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bdb30;
    loop {
        match pc {
            0x4bdb30 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4bdb38);
            { let args = []; c.r[0] = host.call(m, 0x4bd880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4bdb48; continue; }
            pc = 0x4bdb3f; continue;
            }
            0x4bdb3f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bdb45);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bdb48; continue;
            }
            0x4bdb48 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_set_visible(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bdb50;
    loop {
        match pc {
            0x4bdb50 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x94),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(4) { pc = 0x4bdb85; continue; }
            pc = 0x4bdb5f; continue;
            }
            0x4bdb5f => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0xd8),32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4bdb7c; continue; }
            pc = 0x4bdb69; continue;
            }
            0x4bdb69 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffffefu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0xd8); wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b21a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4bdb7c => {
            { let a = c.get(6,32); let r = c.or(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(0xd8); wm(m,a,32,c.get(6,32)); }
            pc = 0x4bdb85; continue;
            }
            0x4bdb85 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b21a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn csgl2d_3dobject_set_object_vector(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bdd00;
    loop {
        match pc {
            0x4bdd00 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x94),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4bdd3b; continue; }
            pc = 0x4bdd0a; continue;
            }
            0x4bdd0a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.r[2].wrapping_add(0xc0); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x94),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0xc4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x94),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,8,0x1u32);
            { let a = c.r[1].wrapping_add(0xc8); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4bdd3b => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5750;
    loop {
        match pc {
            0x4c5750 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x671498u32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c5765);
            { let args = []; c.r[0] = host.call(m, 0x4c5780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5770;
    loop {
        match pc {
            0x4c5770 => {
            { let a = c.r[1]; wm(m,a,32,0x671498u32); }
            { let args = []; c.r[0] = host.call(m, 0x4c5780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5780;
    loop {
        match pc {
            0x4c5780 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c5830; continue; }
            pc = 0x4c5791; continue;
            }
            0x4c5791 => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4c5826; continue; }
            pc = 0x4c579a; continue;
            }
            0x4c579a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4c57a0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            if c.cond(8) { pc = 0x4c57fa; continue; }
            pc = 0x4c57af; continue;
            }
            0x4c57af => {
            c.push(m,c.get(7,32));
            pc = 0x4c57b0; continue;
            }
            0x4c57b0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4c57de; continue; }
            pc = 0x4c57bd; continue;
            }
            0x4c57bd => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c57cc);
            { let args = []; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2]),32),32); }
            if c.cond(12) { pc = 0x4c57bd; continue; }
            pc = 0x4c57de; continue;
            }
            0x4c57de => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c57ed);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x4c57b0; continue; }
            pc = 0x4c57f9; continue;
            }
            0x4c57f9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4c57fa; continue;
            }
            0x4c57fa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c580a; continue; }
            pc = 0x4c5801; continue;
            }
            0x4c5801 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5807);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c580a; continue;
            }
            0x4c580a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c5833; continue; }
            pc = 0x4c5811; continue;
            }
            0x4c5811 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5817);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c5826 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c5830; continue; }
            pc = 0x4c582a; continue;
            }
            0x4c582a => {
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c5830; continue;
            }
            0x4c5830 => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4c5833; continue;
            }
            0x4c5833 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_get_group_count(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5840;
    loop {
        match pc {
            0x4c5840 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_get_texture(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5870;
    loop {
        match pc {
            0x4c5870 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4c5880; continue; }
            pc = 0x4c587b; continue;
            }
            0x4c587b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4c5880 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(12) { pc = 0x4c5897; continue; }
            pc = 0x4c5892; continue;
            }
            0x4c5892 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4c5897 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_load(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c58d0;
    loop {
        match pc {
            0x4c58d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6620c0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(2,32,rm(m,0x6bf558u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,8,0x1u32); }
            c.push(m, 0x4c5920);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,32,rm(m,0x6bf574u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c5943);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4c5954);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_mul(4));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5967);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            c.set(1,32,rm(m,0x6bf594u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c598a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6bf584u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c59ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6bf5a4u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.push(m,c.get(0,32));
            c.push(m,0xffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4c59d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6bf5b8u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c59f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4c5a0d; continue; }
            pc = 0x4c5a00; continue;
            }
            0x4c5a00 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4c5a06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c5a0d; continue;
            }
            0x4c5a0d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x4c5bd3; continue; }
            pc = 0x4c5a1c; continue;
            }
            0x4c5a1c => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6bf568u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x84));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,8,0x2u32); }
            c.push(m, 0x4c5a48);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,32,rm(m,0x6beee0u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c5a7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c5c2d; continue; }
            pc = 0x4c5a83; continue;
            }
            0x4c5a83 => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4c5a91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x3u32); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c5aaa);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c5ac3);
            { let args = []; c.r[0] = host.call(m, 0x4aebc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x4u32); }
            c.push(m, 0x4c5ad6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4aecd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c5b95; continue; }
            pc = 0x4c5ade; continue;
            }
            0x4c5ade => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sbb(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sbb(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x7u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.or(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x4c5aff; continue;
            }
            0x4c5aff => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4c5b08);
            { let args = []; c.r[0] = host.call(m, 0x4aee50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1]),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c5b34);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x4c5b57);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4aeea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,8,0x5u32); }
            c.push(m, 0x4c5b67);
            { let args = []; c.r[0] = host.call(m, 0x4bca00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4c5b85);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4c5aff; continue; }
            pc = 0x4c5b8d; continue;
            }
            0x4c5b8d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4c5b95; continue;
            }
            0x4c5b95 => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x3u32); }
            c.push(m, 0x4c5ba3);
            { let args = []; c.r[0] = host.call(m, 0x4aec40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x2u32); }
            c.push(m, 0x4c5bb1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x1u32); }
            c.push(m, 0x4c5bc2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4c5a1c; continue; }
            pc = 0x4c5bd3; continue;
            }
            0x4c5bd3 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c5be4; continue; }
            pc = 0x4c5bd7; continue;
            }
            0x4c5bd7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c5be1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c5be4; continue;
            }
            0x4c5be4 => {
            c.push(m,0x4u32);
            c.push(m, 0x4c5beb);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[0]; wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x0u32); }
            c.push(m, 0x4c5c05);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4c5c16);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            pc = 0x4c5c18; continue;
            }
            0x4c5c18 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x68u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4c5c2d => {
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x1u32); }
            c.push(m, 0x4c5c3e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x0u32); }
            c.push(m, 0x4c5c4c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4c5c5d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x4c5c18; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn texture_collection_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5cc0;
    loop {
        match pc {
            0x4c5cc0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,0x671498u32); }
            c.push(m, 0x4c5cce);
            { let args = []; c.r[0] = host.call(m, 0x4c5780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4c5cde; continue; }
            pc = 0x4c5cd5; continue;
            }
            0x4c5cd5 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4c5cdb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c5cde; continue;
            }
            0x4c5cde => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "dyn_gfx_text_box_ctor" => dyn_gfx_text_box_ctor(m, host, regs),
        "dyn_gfx_text_box_dtor" => dyn_gfx_text_box_dtor(m, host, regs),
        "dyn_gfx_text_box_reset" => dyn_gfx_text_box_reset(m, host, regs),
        "dyn_gfx_text_box_load_texture" => dyn_gfx_text_box_load_texture(m, host, regs),
        "dyn_gfx_text_box_set_size" => dyn_gfx_text_box_set_size(m, host, regs),
        "dyn_gfx_text_box_set_clip_rect" => dyn_gfx_text_box_set_clip_rect(m, host, regs),
        "dyn_gfx_text_box_deleting_dtor" => dyn_gfx_text_box_deleting_dtor(m, host, regs),
        "atlas_texture_get_frame" => atlas_texture_get_frame(m, host, regs),
        "draw_texture_excluding_rect" => draw_texture_excluding_rect(m, host, regs),
        "direct_texture_button_ctor_default" => direct_texture_button_ctor_default(m, host, regs),
        "direct_texture_button_ctor_named" => direct_texture_button_ctor_named(m, host, regs),
        "direct_texture_button_dtor" => direct_texture_button_dtor(m, host, regs),
        "direct_texture_button_reset" => direct_texture_button_reset(m, host, regs),
        "direct_texture_button_load_layout" => direct_texture_button_load_layout(m, host, regs),
        "direct_texture_button_slot3_release_string" => direct_texture_button_slot3_release_string(m, host, regs),
        "direct_texture_button_draw" => direct_texture_button_draw(m, host, regs),
        "direct_texture_button_load_scaled_texture" => direct_texture_button_load_scaled_texture(m, host, regs),
        "direct_texture_button_get_class_name" => direct_texture_button_get_class_name(m, host, regs),
        "direct_texture_button_scalar_deleting_dtor" => direct_texture_button_scalar_deleting_dtor(m, host, regs),
        "texture_list_release_all_textures" => texture_list_release_all_textures(m, host, regs),
        "texture_list_reload_all_textures" => texture_list_reload_all_textures(m, host, regs),
        "texture_list_scalar_deleting_dtor" => texture_list_scalar_deleting_dtor(m, host, regs),
        "present_frame" => present_frame(m, host, regs),
        "rect_clipper_stuff" => rect_clipper_stuff(m, host, regs),
        "sgl_load_driver_dll" => sgl_load_driver_dll(m, host, regs),
        "sgl_unload_driver_dll" => sgl_unload_driver_dll(m, host, regs),
        "sgl_AddClipRect_4i" => sgl_AddClipRect_4i(m, host, regs),
        "sgl_Begin3D" => sgl_Begin3D(m, host, regs),
        "sgl_ChangePalette" => sgl_ChangePalette(m, host, regs),
        "sgl_ChangeTextureFlags" => sgl_ChangeTextureFlags(m, host, regs),
        "sgl_CleanUp" => sgl_CleanUp(m, host, regs),
        "sgl_ClearBuffer" => sgl_ClearBuffer(m, host, regs),
        "ddraw_copy_clipper" => ddraw_copy_clipper(m, host, regs),
        "sgl_CreateMemoryTexture" => sgl_CreateMemoryTexture(m, host, regs),
        "sgl_Blt" => sgl_Blt(m, host, regs),
        "sgl_Blt_RC" => sgl_Blt_RC(m, host, regs),
        "ddraw_sgl_draw_line" => ddraw_sgl_draw_line(m, host, regs),
        "sgl_DrawScene" => sgl_DrawScene(m, host, regs),
        "ui_render_text_at" => ui_render_text_at(m, host, regs),
        "sgl_DrawText_Rect" => sgl_DrawText_Rect(m, host, regs),
        "sgl_Finish3D" => sgl_Finish3D(m, host, regs),
        "sgl_SwapBuffer" => sgl_SwapBuffer(m, host, regs),
        "sgl_FreeFont" => sgl_FreeFont(m, host, regs),
        "ddraw_free_rect_clipper" => ddraw_free_rect_clipper(m, host, regs),
        "sgl_FreeRectClipper" => sgl_FreeRectClipper(m, host, regs),
        "sgl_GetProjectionData" => sgl_GetProjectionData(m, host, regs),
        "ddraw_get_rect_clipper" => ddraw_get_rect_clipper(m, host, regs),
        "sgl_CreateClipper" => sgl_CreateClipper(m, host, regs),
        "sgl_GrabScreen" => sgl_GrabScreen(m, host, regs),
        "sgl_InitHardware" => sgl_InitHardware(m, host, regs),
        "sgl_InitScreen" => sgl_InitScreen(m, host, regs),
        "sgl_LoadMemoryTexture" => sgl_LoadMemoryTexture(m, host, regs),
        "sgl_ReLoadMemoryTexture" => sgl_ReLoadMemoryTexture(m, host, regs),
        "sgl_ResetClipper" => sgl_ResetClipper(m, host, regs),
        "ddraw_set_2d_clipping_mode" => ddraw_set_2d_clipping_mode(m, host, regs),
        "sgl_Get2dClippingMode" => sgl_Get2dClippingMode(m, host, regs),
        "sgl_SetArchive" => sgl_SetArchive(m, host, regs),
        "sgl_SetBlendMode" => sgl_SetBlendMode(m, host, regs),
        "sgl_IsRectInClippingRegion" => sgl_IsRectInClippingRegion(m, host, regs),
        "ddraw_set_clip_rect" => ddraw_set_clip_rect(m, host, regs),
        "sgl_SetClipRect_4i" => sgl_SetClipRect_4i(m, host, regs),
        "ddraw_set_clip_rect_r" => ddraw_set_clip_rect_r(m, host, regs),
        "sgl_GetClipRect_r" => sgl_GetClipRect_r(m, host, regs),
        "sgl_SetColorKey" => sgl_SetColorKey(m, host, regs),
        "sgl_GetColorKey" => sgl_GetColorKey(m, host, regs),
        "sgl_GetConstantColor" => sgl_GetConstantColor(m, host, regs),
        "sgl_SetScreenDimensions" => sgl_SetScreenDimensions(m, host, regs),
        "sgl_SetZoomFactor" => sgl_SetZoomFactor(m, host, regs),
        "sgl_StretchBitmapRect" => sgl_StretchBitmapRect(m, host, regs),
        "sgl_StrLen" => sgl_StrLen(m, host, regs),
        "sgl_SubClipRect_4i" => sgl_SubClipRect_4i(m, host, regs),
        "sgl_SubClipRect_r" => sgl_SubClipRect_r(m, host, regs),
        "sgl_SwitchScreenMode" => sgl_SwitchScreenMode(m, host, regs),
        "sgl_PrefetchTexture" => sgl_PrefetchTexture(m, host, regs),
        "sgl_LockTexture" => sgl_LockTexture(m, host, regs),
        "sgl_UnlockTexture" => sgl_UnlockTexture(m, host, regs),
        "sgl_MessageBox" => sgl_MessageBox(m, host, regs),
        "report_win32_error_at_file_line" => report_win32_error_at_file_line(m, host, regs),
        "load_gl_cfg_and_init_graphics" => load_gl_cfg_and_init_graphics(m, host, regs),
        "sgl2d_shutdown" => sgl2d_shutdown(m, host, regs),
        "sgl2d_open_texture_archives" => sgl2d_open_texture_archives(m, host, regs),
        "sgl2d_close_texture_archives" => sgl2d_close_texture_archives(m, host, regs),
        "sgl2d_get_texture_archive" => sgl2d_get_texture_archive(m, host, regs),
        "sgl2d_load_memory_texture_from_archive" => sgl2d_load_memory_texture_from_archive(m, host, regs),
        "sgl2d_reload_memory_texture" => sgl2d_reload_memory_texture(m, host, regs),
        "sgl2d_load_texture_image" => sgl2d_load_texture_image(m, host, regs),
        "sgl2d_update_frame_timer" => sgl2d_update_frame_timer(m, host, regs),
        "sgl2d_3d_object_ctor" => sgl2d_3d_object_ctor(m, host, regs),
        "csgl2d_3dobject_ctor_with_name" => csgl2d_3dobject_ctor_with_name(m, host, regs),
        "csgl2d_3dobject_dtor" => csgl2d_3dobject_dtor(m, host, regs),
        "csgl2d_3dobject_reset" => csgl2d_3dobject_reset(m, host, regs),
        "csgl2d_3dobject_slot2_return_false" => csgl2d_3dobject_slot2_return_false(m, host, regs),
        "csgl2d_3dobject_slot3_return_false" => csgl2d_3dobject_slot3_return_false(m, host, regs),
        "csgl2d_3dobject_get_client_rect" => csgl2d_3dobject_get_client_rect(m, host, regs),
        "csgl2d_3dobject_set_size_from_rect" => csgl2d_3dobject_set_size_from_rect(m, host, regs),
        "csgl2d_3dobject_set_size" => csgl2d_3dobject_set_size(m, host, regs),
        "csgl2d_3dobject_draw_stub" => csgl2d_3dobject_draw_stub(m, host, regs),
        "csgl2d_3dobject_get_class_name" => csgl2d_3dobject_get_class_name(m, host, regs),
        "csgl2d_3dobject_deleting_dtor" => csgl2d_3dobject_deleting_dtor(m, host, regs),
        "csgl2d_3dobject_set_visible" => csgl2d_3dobject_set_visible(m, host, regs),
        "csgl2d_3dobject_set_object_vector" => csgl2d_3dobject_set_object_vector(m, host, regs),
        "texture_collection_ctor" => texture_collection_ctor(m, host, regs),
        "texture_collection_dtor" => texture_collection_dtor(m, host, regs),
        "texture_collection_clear" => texture_collection_clear(m, host, regs),
        "texture_collection_get_group_count" => texture_collection_get_group_count(m, host, regs),
        "texture_collection_get_texture" => texture_collection_get_texture(m, host, regs),
        "texture_collection_load" => texture_collection_load(m, host, regs),
        "texture_collection_scalar_deleting_dtor" => texture_collection_scalar_deleting_dtor(m, host, regs),
        _ => return None,
    })
}
