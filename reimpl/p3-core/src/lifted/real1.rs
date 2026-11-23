//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn game_screen_host_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x401060;
    loop {
        match pc {
            0x401060 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65aea8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x401083);
            { let args = []; c.r[0] = host.call(m, 0x4b4920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x401094);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x23c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0x240); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x250); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x254); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x258); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x25c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66a658u32); }
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

pub fn game_screen_host_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4010e0;
    loop {
        match pc {
            0x4010e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65aed6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x66a658u32); }
            c.set(1,32,c.r[6].wrapping_add(0x240));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x2u32); }
            c.push(m, 0x401116);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.push(m, 0x401126);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x401136);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x401145);
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

pub fn game_screen_host_release_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x401160;
    loop {
        match pc {
            0x401160 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x40117f; continue; }
            pc = 0x401170; continue;
            }
            0x401170 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x401176);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x250); wm(m,a,32,c.get(7,32)); }
            pc = 0x40117f; continue;
            }
            0x40117f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x254),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x401198; continue; }
            pc = 0x401189; continue;
            }
            0x401189 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40118f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x254); wm(m,a,32,c.get(7,32)); }
            pc = 0x401198; continue;
            }
            0x401198 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x258),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4011b1; continue; }
            pc = 0x4011a2; continue;
            }
            0x4011a2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4011a8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x258); wm(m,a,32,c.get(7,32)); }
            pc = 0x4011b1; continue;
            }
            0x4011b1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x25c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4011ca; continue; }
            pc = 0x4011bb; continue;
            }
            0x4011bb => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4011c1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x25c); wm(m,a,32,c.get(7,32)); }
            pc = 0x4011ca; continue;
            }
            0x4011ca => {
            c.set(7,32,c.r[6].wrapping_add(0x164));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4011d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_host_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4011f0;
    loop {
        match pc {
            0x4011f0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65aef0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x401228);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40122f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e0acu32,32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x94),32); c.push(m, 0x40123f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x69293cu32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x40125e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x692948u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x40127c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x40128e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x164),32));
            c.set(7,32,c.r[6].wrapping_add(0x164));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x40129f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x692954u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4012ba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x264); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4012ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x4012d8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4012e9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_host_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x401350;
    loop {
        match pc {
            0x401350 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65af02u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x4cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4013a4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4018dc; continue; }
            pc = 0x4013ac; continue;
            }
            0x4013ac => {
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.push(m, 0x4013b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4018dc; continue; }
            pc = 0x4013c1; continue;
            }
            0x4013c1 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4013ca);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,0x0u32); }
            c.push(m, 0x4013dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4013e6);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            { let a = rm(m,c.r[2].wrapping_add(0x7e4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x401416; continue; }
            pc = 0x4013f4; continue;
            }
            0x4013f4 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,c.get(4,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.push(m,c.get(7,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x401414);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b58c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x40141a; continue;
            }
            0x401416 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            pc = 0x40141a; continue;
            }
            0x40141a => {
            c.push(m,c.get(7,32));
            c.push(m, 0x401420);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.push(m, 0x401432);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x40143b);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            { let a = rm(m,c.r[2].wrapping_add(0x7e4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40144d; continue; }
            pc = 0x401449; continue;
            }
            0x401449 => {
            c.push(m,0xffffffffu32);
            pc = 0x401464; continue;
            }
            0x40144d => {
            c.push(m, 0x401452);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x40145c);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x401464; continue;
            }
            0x401464 => {
            c.push(m, 0x401469);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x500u32,32); }
            c.set(1,32,0x6da820u32);
            if c.cond(4) { pc = 0x40148e; continue; }
            pc = 0x40147d; continue;
            }
            0x40147d => {
            c.push(m,0x7555u32);
            c.push(m, 0x401487);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            pc = 0x40149d; continue;
            }
            0x40148e => {
            c.push(m,0x7545u32);
            c.push(m, 0x401498);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x3u32); }
            pc = 0x40149d; continue;
            }
            0x40149d => {
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4014aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4014b3);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x7e4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4014c5; continue; }
            pc = 0x4014c1; continue;
            }
            0x4014c1 => {
            c.push(m,0xffffffffu32);
            pc = 0x4014dc; continue;
            }
            0x4014c5 => {
            c.push(m, 0x4014ca);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.push(m, 0x4014d4);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            pc = 0x4014dc; continue;
            }
            0x4014dc => {
            c.push(m, 0x4014e1);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x401634; continue; }
            pc = 0x4014fd; continue;
            }
            0x4014fd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x40151c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x401563; continue; }
            pc = 0x401529; continue;
            }
            0x401529 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            pc = 0x401532; continue;
            }
            0x401532 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[3].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x4015ae; continue; }
            pc = 0x401537; continue;
            }
            0x401537 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4015ae; continue; }
            pc = 0x40153e; continue;
            }
            0x40153e => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x4015ae; continue;
            }
            0x401563 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x28cc),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x401582; continue; }
            pc = 0x401575; continue;
            }
            0x401575 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x401532; continue;
            }
            0x401582 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4015ae; continue; }
            pc = 0x401589; continue;
            }
            0x401589 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4015ae; continue; }
            pc = 0x401590; continue;
            }
            0x401590 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            pc = 0x4015ae; continue;
            }
            0x4015ae => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4015b8);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.fild32(m,c.r[7].wrapping_add(0x6ddb94));
            c.fstp64(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x34),8));
            { let v = c.ld(m,c.r[6].wrapping_add(0x270),8); c.fop('*',v); }
            c.push(m, 0x4015e9);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,c.r[7].wrapping_add(0x6ddb90));
            c.push(m,c.get(0,32));
            c.fstp64(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),8));
            { let v = c.ld(m,c.r[6].wrapping_add(0x268),8); c.fop('*',v); }
            c.push(m, 0x401603);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x401613);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4014fd; continue; }
            pc = 0x401630; continue;
            }
            0x401630 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            pc = 0x401634; continue;
            }
            0x401634 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40163b);
            { let args = []; c.r[0] = host.call(m, 0x401900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x7556u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x40164a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x401658);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x401662);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x78000000u32);
            c.push(m, 0x40166c);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.sub(a,0x24u32,32); c.set(2,32,r); }
            c.push(m,0x2cu32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x40168a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x0u32);
            c.push(m, 0x401698);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4016a1);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x7e4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4016b3; continue; }
            pc = 0x4016af; continue;
            }
            0x4016af => {
            c.push(m,0xffffffffu32);
            pc = 0x4016ca; continue;
            }
            0x4016b3 => {
            c.push(m, 0x4016b8);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x4016c2);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x4016ca; continue;
            }
            0x4016ca => {
            c.push(m, 0x4016cf);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,0x28u32,32); c.set(1,32,r); }
            c.push(m,0x28u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4016f0);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4016fc);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0x31u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x11u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x40173b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4c1780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x401746);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0x32u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x12u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x401785);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4c1780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,0x6cc37cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4017b4; continue; }
            pc = 0x401791; continue;
            }
            0x401791 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4017b4; continue; }
            pc = 0x401798; continue;
            }
            0x401798 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x4017b8; continue;
            }
            0x4017b4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4017b8; continue;
            }
            0x4017b8 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x4017be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x0u32);
            c.push(m, 0x4017cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4017d5);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x7e4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4017e7; continue; }
            pc = 0x4017e3; continue;
            }
            0x4017e3 => {
            c.push(m,0xffffffffu32);
            pc = 0x4017fe; continue;
            }
            0x4017e7 => {
            c.push(m, 0x4017ec);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x4017f6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x4017fe; continue;
            }
            0x4017fe => {
            c.push(m, 0x401803);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x34u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x32u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x401829);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x401835);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x240),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0x31u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x32u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x401874);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4c1780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40187f);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x240),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0x32u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x32u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4018be);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4c1780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4018c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4018dc);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4018dc; continue;
            }
            0x4018dc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_host_draw_ships(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x401900;
    loop {
        match pc {
            0x401900 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x401911);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x260),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x401a75; continue; }
            pc = 0x401928; continue;
            }
            0x401928 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x401939; continue;
            }
            0x401935 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x401939; continue;
            }
            0x401939 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x258),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x254),32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x25c),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x250),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x401a5d; continue; }
            pc = 0x40196a; continue;
            }
            0x40196a => {
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x401a7c)); continue;
            }
            0x401971 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xb4),32));
            pc = 0x401999; continue;
            }
            0x40197f => {
            c.set(2,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xb0),32));
            pc = 0x4019fc; continue;
            }
            0x40198d => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xac),32));
            pc = 0x401999; continue;
            }
            0x401999 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x4019c4; continue; }
            pc = 0x40199e; continue;
            }
            0x40199e => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4019c4; continue; }
            pc = 0x4019a5; continue;
            }
            0x4019a5 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(0xc),32));
            pc = 0x4019c4; continue;
            }
            0x4019c4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4019ce);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x401a51; continue;
            }
            0x4019f0 => {
            c.set(2,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xb8),32));
            pc = 0x4019fc; continue;
            }
            0x4019fc => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x401a27; continue; }
            pc = 0x401a01; continue;
            }
            0x401a01 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x401a27; continue; }
            pc = 0x401a08; continue;
            }
            0x401a08 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc),32));
            pc = 0x401a27; continue;
            }
            0x401a27 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x401a31);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            c.push(m,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x401a51; continue;
            }
            0x401a51 => {
            c.push(m, 0x401a56);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x401a5d; continue;
            }
            0x401a5d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x260),32),32); }
            if c.cond(12) { pc = 0x401935; continue; }
            pc = 0x401a72; continue;
            }
            0x401a72 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x401a75; continue;
            }
            0x401a75 => {
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_host_activate(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402390;
    loop {
        match pc {
            0x402390 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x40239d);
            { let args = []; c.r[0] = host.call(m, 0x504900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4023a7);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x4b6e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_host_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402720;
    loop {
        match pc {
            0x402720 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x402728);
            { let args = []; c.r[0] = host.call(m, 0x4010e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x402738; continue; }
            pc = 0x40272f; continue;
            }
            0x40272f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x402735);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x402738; continue;
            }
            0x402738 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_convoy_ship_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4110d0;
    loop {
        match pc {
            0x4110d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b792u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(7,32));
            c.set(7,32,0xffffffu32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x694b48u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x41112e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x122u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411152); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x41115f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(3,32,0x146u32);
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x411245; continue; }
            pc = 0x411182; continue;
            }
            0x411182 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x1f2u32,32); }
            if c.cond(13) { pc = 0x411229; continue; }
            pc = 0x41119c; continue;
            }
            0x41119c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.push(m, 0x4111ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4111b2);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x160));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0cb4),32));
            c.push(m,c.get(2,32));
            c.push(m,0x694b70u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4111d7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x411205; continue; }
            pc = 0x4111ef; continue;
            }
            0x4111ef => {
            c.push(m,0x694b80u32);
            c.push(m, 0x4111f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x41120f; continue;
            }
            0x411205 => {
            c.push(m,0x694b88u32);
            c.push(m, 0x41120f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41120f; continue;
            }
            0x41120f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411226); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x12u32,32); c.set(3,32,r); }
            pc = 0x411229; continue;
            }
            0x411229 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x411182; continue; }
            pc = 0x411241; continue;
            }
            0x411241 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x411245; continue;
            }
            0x411245 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x16),16));
            c.push(m,c.get(1,32));
            c.push(m,0x694b90u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41125f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x26cu32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411279); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4112e1; continue; }
            pc = 0x411285; continue;
            }
            0x411285 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x3),8));
            c.set(6,32,c.r[7].wrapping_add(c.r[1]));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4112a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4112b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,0x694b94u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4112c3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x134u32);
            c.push(m,0x28au32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4112e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4112e1; continue;
            }
            0x4112e1 => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4112f2);
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

pub fn reposition_ui_elements_top_bar_patch(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
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
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a0d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42a0e4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1438),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42a0f5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42a11f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,c.r[6].wrapping_add(0x1520));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42a13a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x42a171); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn game_screen_load_ini_wrapper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42da50;
    loop {
        match pc {
            0x42da50 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d221u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.push(m, 0x42da84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x42da8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,8,c.get(0,8));
            c.set(0,32,rm(m,0x6963a4u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42daa0; continue; }
            pc = 0x42da9a; continue;
            }
            0x42da9a => {
            { let target = rm(m,0x6963acu32,32); c.push(m, 0x42daa0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42daa0; continue;
            }
            0x42daa0 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42dab1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,8,c.get(3,8));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn side_menu_system_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441240;
    loop {
        match pc {
            0x441240 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e51eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x4e20u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697a98u32);
            c.push(m, 0x441272);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x44127c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x697ab0u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x44128a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697abcu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x4412a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x697ad4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x1u32); }
            c.push(m, 0x4412cf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x441306; continue; }
            pc = 0x4412d3; continue;
            }
            0x4412d3 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4412e0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4412f1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x441306 => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x441314);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x697adcu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m, 0x441334);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x697aecu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x441351);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x697afcu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x44136e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x697b08u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x44138b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x697b18u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4413aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,0x697b24u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6cbbb0u32);
            { let a = 0x6cbd98u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4413c8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4413da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cbcb4u32);
            c.push(m, 0x4413e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.set(3,8,c.get(0,8));
            c.push(m, 0x4413fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cbce4u32);
            c.push(m, 0x441404);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            { let a = c.get(3,8); let r = c.and(a,c.get(0,8),8); c.set(3,8,r); }
            c.set(1,32,c.get(4,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x441418);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cbd60u32);
            c.push(m, 0x441422);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.get(3,8); let r = c.and(a,c.get(0,8),8); c.set(3,8,r); }
            c.push(m, 0x44143a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cbd24u32);
            c.push(m, 0x441444);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697b34u32);
            { let a = c.get(3,8); let r = c.and(a,c.get(0,8),8); c.set(3,8,r); }
            c.push(m, 0x441459);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cbbc8u32);
            c.push(m, 0x441463);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4af260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6cbbc8u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x441472);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x441479);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbd74u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4414b3; continue; }
            pc = 0x441482; continue;
            }
            0x441482 => {
            c.set(0,32,rm(m,0x6cbd64u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4414b3; continue; }
            pc = 0x44148b; continue;
            }
            0x44148b => {
            c.set(0,32,rm(m,0x6cbd6cu32,32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = 0x6cbd50u32; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = 0x6cbd54u32; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = 0x6cbd58u32; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = 0x6cbd5cu32; wm(m,a,32,c.get(2,32)); }
            pc = 0x4414b3; continue;
            }
            0x4414b3 => {
            c.set(0,32,rm(m,0x6cbd38u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4414ed; continue; }
            pc = 0x4414bc; continue;
            }
            0x4414bc => {
            c.set(0,32,rm(m,0x6cbd28u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4414ed; continue; }
            pc = 0x4414c5; continue;
            }
            0x4414c5 => {
            c.set(0,32,rm(m,0x6cbd30u32,32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = 0x6cbd14u32; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = 0x6cbd18u32; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = 0x6cbd1cu32; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = 0x6cbd20u32; wm(m,a,32,c.get(2,32)); }
            pc = 0x4414ed; continue;
            }
            0x4414ed => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4414fb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x441509);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x44151a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn side_menu_get_host_height(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4416f0;
    loop {
        match pc {
            0x4416f0 => {
            c.set(2,32,rm(m,0x6cbbc8u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4]);
            c.set(1,32,0x6cbbc8u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x441706); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn side_menu_force_close_current(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441810;
    loop {
        match pc {
            0x441810 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,0x6cbd90u32,32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            { let a = 0x697a54u32; wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x44182b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbd90u32,32));
            { let a = c.r[1].wrapping_add(0x98); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,0x6cbd90u32,32));
            { let a = c.r[2].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd90u32,32));
            { let a = c.r[0].wrapping_add(0x94); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd90u32,32));
            c.set(1,32,rm(m,0x6cbd98u32,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x9c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x9c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd90u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x128),32); c.push(m, 0x441875); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbd90u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4418a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,0x6cbbc8u32);
            c.push(m, 0x4418b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4af970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x697a54u32; wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scroll_map_check_ship_list_problems(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x454b80;
    loop {
        match pc {
            0x454b80 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,8,0x1u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x454c47; continue; }
            pc = 0x454b9a; continue;
            }
            0x454b9a => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x454bb0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,0x1u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,16,rm(m,0x6dd894u32,16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x454bf5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,16,c.get(5,16)); }
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x454c2f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x454cbf; continue;
            }
            0x454c47 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x454c5e);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,16,0x1u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,16,rm(m,0x6dd894u32,16));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x454c8f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,16,c.get(5,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x454cb3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x454cbf; continue;
            }
            0x454cbf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x454eea; continue; }
            pc = 0x454cd8; continue;
            }
            0x454cd8 => {
            c.push(m,c.get(7,32));
            pc = 0x454cd9; continue;
            }
            0x454cd9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x454ed6; continue; }
            pc = 0x454cee; continue;
            }
            0x454cee => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x454ed6; continue; }
            pc = 0x454d14; continue;
            }
            0x454d14 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x454d2c; continue; }
            pc = 0x454d25; continue;
            }
            0x454d25 => {
            c.set(3,8,0x1u32);
            pc = 0x454ed6; continue;
            }
            0x454d2c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[7].wrapping_add(0x40),16); c.sub(a,c.get(2,16),16); }
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x454d85; continue; }
            pc = 0x454d51; continue;
            }
            0x454d51 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,0x0u32); }
            pc = 0x454eca; continue;
            }
            0x454d85 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x454d90);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x454ddd; continue; }
            pc = 0x454da9; continue;
            }
            0x454da9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,0x1u32); }
            pc = 0x454eca; continue;
            }
            0x454ddd => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x454e4a; continue; }
            pc = 0x454e16; continue;
            }
            0x454e16 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,0x2u32); }
            pc = 0x454eca; continue;
            }
            0x454e4a => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3f),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x454e8a; continue; }
            pc = 0x454e59; continue;
            }
            0x454e59 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,0x3u32); }
            pc = 0x454eca; continue;
            }
            0x454e8a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x7u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x454ed6; continue; }
            pc = 0x454e9b; continue;
            }
            0x454e9b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,0x5u32); }
            pc = 0x454eca; continue;
            }
            0x454eca => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = rm(m,c.r[0].wrapping_add(0x4),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,r); } }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x454ed6; continue;
            }
            0x454ed6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x454cd9; continue; }
            pc = 0x454ee3; continue;
            }
            0x454ee3 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x454f5a; continue; }
            pc = 0x454ee8; continue;
            }
            0x454ee8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x454eea; continue;
            }
            0x454eea => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x454eff; continue; }
            pc = 0x454ef6; continue;
            }
            0x454ef6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x454efc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x454eff; continue;
            }
            0x454eff => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x454f1d; continue; }
            pc = 0x454f14; continue;
            }
            0x454f14 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x454f1a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x454f1d; continue;
            }
            0x454f1d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x454f4c);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x454f5a => {
            { let a = rm(m,c.r[6].wrapping_add(0x292c),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x454f6a; continue; }
            pc = 0x454f63; continue;
            }
            0x454f63 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x454f6a);
            { let args = []; c.r[0] = host.call(m, 0x456f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x454f6a; continue;
            }
            0x454f6a => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_icon_button_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x478590;
    loop {
        match pc {
            0x478590 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x478598);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6]; wm(m,a,32,0x66ee20u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_icon_button_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4785b0;
    loop {
        match pc {
            0x4785b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4785ed); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47864f; continue; }
            pc = 0x4785f1; continue;
            }
            0x4785f1 => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m, 0x4785fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47864f; continue; }
            pc = 0x478602; continue;
            }
            0x478602 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47860c);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x478615);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478621);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478646);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x47864c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            pc = 0x47864f; continue;
            }
            0x47864f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_icon_button_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x478670;
    loop {
        match pc {
            0x478670 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x478678);
            { let args = []; c.r[0] = host.call(m, 0x478660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x478688; continue; }
            pc = 0x47867f; continue;
            }
            0x47867f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x478685);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x478688; continue;
            }
            0x478688 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_sub_panel_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x486b80;
    loop {
        match pc {
            0x486b80 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x486b88);
            { let args = []; c.r[0] = host.call(m, 0x486ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x486b98; continue; }
            pc = 0x486b8f; continue;
            }
            0x486b8f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x486b95);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x486b98; continue;
            }
            0x486b98 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_sub_panel_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x486ba0;
    loop {
        match pc {
            0x486ba0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65fbc6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x1290));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x8u32); }
            c.push(m, 0x486bd0);
            { let args = []; c.r[0] = host.call(m, 0x4ace40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4ace40u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x1008));
            c.push(m,0xd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x7u32); }
            c.push(m, 0x486bed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4ace40u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0xd80));
            c.push(m,0xd8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x6u32); }
            c.push(m, 0x486c0a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0xc98));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x5u32); }
            c.push(m, 0x486c1a);
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0x9e0));
            c.push(m,0xe8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x4u32); }
            c.push(m, 0x486c37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(0,32,c.r[6].wrapping_add(0x728));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3u32); }
            c.push(m, 0x486c54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[6].wrapping_add(0x640));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2u32); }
            c.push(m, 0x486c64);
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(1,32,c.r[6].wrapping_add(0x388));
            c.push(m,0xe8u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x486c81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4c6a30u32);
            c.push(m,0x3u32);
            c.set(2,32,c.r[6].wrapping_add(0xd0));
            c.push(m,0xe8u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.push(m, 0x486c9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x486cad);
            { let args = []; c.r[0] = host.call(m, 0x41c3a0, c.r[1], &args); }
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

pub fn combo_list_box_set_size_from_triple(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0010;
    loop {
        match pc {
            0x4b0010 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x314),32));
            c.set(1,32,c.r[6].wrapping_add(0x314));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x54),32); c.push(m, 0x4b0028); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b0030);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4ce6d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn combo_list_box_set_position_from_triple(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0040;
    loop {
        match pc {
            0x4b0040 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x314),32));
            c.set(1,32,c.r[7].wrapping_add(0x314));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4b0058); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl_status_bar_ctor_default(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4820;
    loop {
        match pc {
            0x4c4820 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662028u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bf530u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4c4848);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x671378u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[6].wrapping_add(0xa4));
            c.set(1,32,0x4u32);
            pc = 0x4c485f; continue;
            }
            0x4c485f => {
            { let a = c.r[0].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x4c485f; continue; }
            pc = 0x4c486a; continue;
            }
            0x4c486a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c4871);
            { let args = []; c.r[0] = host.call(m, 0x4c4950, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
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

pub fn sgl_status_bar_ctor_named(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4890;
    loop {
        match pc {
            0x4c4890 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66203au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4c48b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b15f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x671378u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[6].wrapping_add(0xa4));
            c.set(1,32,0x4u32);
            pc = 0x4c48ce; continue;
            }
            0x4c48ce => {
            { let a = c.r[0].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x4c48ce; continue; }
            pc = 0x4c48d9; continue;
            }
            0x4c48d9 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c48e0);
            { let args = []; c.r[0] = host.call(m, 0x4c4950, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
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

pub fn sgl_status_bar_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4900;
    loop {
        match pc {
            0x4c4900 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66204cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x671378u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c4930);
            { let args = []; c.r[0] = host.call(m, 0x4c4950, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4c493f);
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

pub fn sgl_status_bar_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4950;
    loop {
        match pc {
            0x4c4950 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,c.r[7].wrapping_add(0xa4));
            pc = 0x4c4960; continue;
            }
            0x4c4960 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4c496c; continue; }
            pc = 0x4c4966; continue;
            }
            0x4c4966 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x4c496c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c496c; continue;
            }
            0x4c496c => {
            c.set(0,8,c.get(3,8));
            c.set(1,8,0x3fu32);
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(1,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xfffffff0); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(2,32); let r = c.or(a,0xffffff00u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(3,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4c4960; continue; }
            pc = 0x4c499a; continue;
            }
            0x4c499a => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[7].wrapping_add(0xc4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c49a9);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.set(1,32,c.r[7].wrapping_add(0xd0));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c49b8);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(0xe4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(0xe0); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(0xdc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(0xe8); wm(m,a,32,0x2u32); }
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

pub fn sgl2d_status_bar_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4a50;
    loop {
        match pc {
            0x4c4a50 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662081u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            if c.cond(13) { pc = 0x4c4aaf; continue; }
            pc = 0x4c4a9f; continue;
            }
            0x4c4a9f => {
            c.set(2,32,rm(m,0x6beee0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c4aaf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c4aaf; continue;
            }
            0x4c4aaf => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(6,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4c4aba; continue; }
            pc = 0x4c4ab8; continue;
            }
            0x4c4ab8 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4c4aba; continue;
            }
            0x4c4aba => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x4c4ac5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6bf498u32,32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c4ad7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,0x6bf4acu32,32));
            c.set(2,32,rm(m,0x6bf4a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c4b06);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c4b3a; continue; }
            pc = 0x4c4b0a; continue;
            }
            0x4c4b0a => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6bf534u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4c4b23);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4c4b3f; continue; }
            pc = 0x4c4b2b; continue;
            }
            0x4c4b2b => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x54),32); c.push(m, 0x4c4b38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c4b3f; continue;
            }
            0x4c4b3a => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x4c4b3f; continue;
            }
            0x4c4b3f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,0x6bf4bcu32,32));
            c.set(1,32,rm(m,0x6bf4b4u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4c4b72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c4b92; continue; }
            pc = 0x4c4b76; continue;
            }
            0x4c4b76 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6bf53cu32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4c4b8f);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4c4b92; continue;
            }
            0x4c4b92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x7c),32); c.push(m, 0x4c4bb3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0xa4));
            pc = 0x4c4bd6; continue;
            }
            0x4c4bd6 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c4cbb; continue; }
            pc = 0x4c4be2; continue;
            }
            0x4c4be2 => {
            c.set(2,32,rm(m,0x6bf4ecu32,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c4bf4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c4c12);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6bf4f8u32,32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4c4c26);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c4c44);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x4c4c50; continue; }
            pc = 0x4c4c48; continue;
            }
            0x4c4c48 => {
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c4c50; continue; }
            pc = 0x4c4c4d; continue;
            }
            0x4c4c4d => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c4c50; continue;
            }
            0x4c4c50 => {
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4c4cac; continue; }
            pc = 0x4c4c55; continue;
            }
            0x4c4c55 => {
            c.push(m,0xd8u32);
            c.push(m, 0x4c4c5f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x4c4c78; continue; }
            pc = 0x4c4c6f; continue;
            }
            0x4c4c6f => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4c4c76);
            { let args = []; c.r[0] = host.call(m, 0x4acdd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c4c7a; continue;
            }
            0x4c4c78 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4c4c7a; continue;
            }
            0x4c4c7a => {
            c.push(m,c.get(3,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x2u32); }
            c.push(m, 0x4c4c93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4c4c9a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x4c4cac; continue; }
            pc = 0x4c4ca4; continue;
            }
            0x4c4ca4 => {
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c4cac; continue; }
            pc = 0x4c4ca9; continue;
            }
            0x4c4ca9 => {
            { let a = c.r[7].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c4cac; continue;
            }
            0x4c4cac => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4c4bd6; continue; }
            pc = 0x4c4cbb; continue;
            }
            0x4c4cbb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6dd5a8u32,32));
            c.set(2,32,rm(m,0x6bf51cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c4ce7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4c4ce9; continue;
            }
            0x4c4ce9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c4e52; continue; }
            pc = 0x4c4cf5; continue;
            }
            0x4c4cf5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6bf478),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m, 0x4c4d06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x4u32); }
            c.push(m, 0x4c4d1a);
            { let args = []; c.r[0] = host.call(m, 0x63c47b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c4d37; continue; }
            pc = 0x4c4d21; continue;
            }
            0x4c4d21 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4c4d37; continue; }
            pc = 0x4c4d25; continue;
            }
            0x4c4d25 => {
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(15) { pc = 0x4c4d37; continue; }
            pc = 0x4c4d2a; continue;
            }
            0x4c4d2a => {
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0xe8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4c4d37);
            { let args = []; c.r[0] = host.call(m, 0x4c5330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c4d37; continue;
            }
            0x4c4d37 => {
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m, 0x4c4d45);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x4c4ce9; continue; }
            pc = 0x4c4d4b; continue;
            }
            0x4c4d4b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6bf510u32,32));
            c.set(2,32,rm(m,0x6bf504u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4c4d77);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c4e30; continue; }
            pc = 0x4c4d7f; continue;
            }
            0x4c4d7f => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6bf544u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4c4d98);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4c4e30; continue; }
            pc = 0x4c4da4; continue;
            }
            0x4c4da4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4c4e04; continue; }
            pc = 0x4c4db0; continue;
            }
            0x4c4db0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            { let a = c.r[5].wrapping_add(0xdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xe0); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x4c4de0; continue; }
            pc = 0x4c4dc6; continue;
            }
            0x4c4dc6 => {
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0xe4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4c4dd3);
            { let args = []; c.r[0] = host.call(m, 0x4c5330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x4c4dde); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c4e00; continue;
            }
            0x4c4de0 => {
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4c4e04; continue; }
            pc = 0x4c4de4; continue;
            }
            0x4c4de4 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4c4e04; continue; }
            pc = 0x4c4de8; continue;
            }
            0x4c4de8 => {
            { let a = c.r[5].wrapping_add(0xe4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4c4df5);
            { let args = []; c.r[0] = host.call(m, 0x4c5330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x4c4e00); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c4e00; continue;
            }
            0x4c4e00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4c4e04; continue;
            }
            0x4c4e04 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xdc),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe0),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4c4e30; continue; }
            pc = 0x4c4e14; continue;
            }
            0x4c4e14 => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x4c4e30; continue; }
            pc = 0x4c4e18; continue;
            }
            0x4c4e18 => {
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0xe4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4c4e25);
            { let args = []; c.r[0] = host.call(m, 0x4c5330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x4c4e30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c4e30; continue;
            }
            0x4c4e30 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(2,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4c4e41);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4c4e52);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b2080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4c4e52; continue;
            }
            0x4c4e52 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4c4e60);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4c4e6e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4c4e7f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_status_bar_slot3_release_string(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4ea0;
    loop {
        match pc {
            0x4c4ea0 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4c4ea9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_status_bar_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4ec0;
    loop {
        match pc {
            0x4c4ec0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x4c4ed1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c515a; continue; }
            pc = 0x4c4ed9; continue;
            }
            0x4c4ed9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c4efe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b18f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x28),32); c.push(m, 0x4c4f05); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c4f11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(0,32,c.get(5,32));
            if c.cond(12) { pc = 0x4c4f23; continue; }
            pc = 0x4c4f1e; continue;
            }
            0x4c4f1e => {
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4c4f25; continue; }
            pc = 0x4c4f23; continue;
            }
            0x4c4f23 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4c4f25; continue;
            }
            0x4c4f25 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xb4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c4f32);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c4f45); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m, 0x4c4f4c);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4c5132; continue; }
            pc = 0x4c4f62; continue;
            }
            0x4c4f62 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4c5164)); continue;
            }
            0x4c4f69 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x4c50c7; continue;
            }
            0x4c4f92 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c4f9e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c4fb3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4c5132; continue; }
            pc = 0x4c4fbe; continue;
            }
            0x4c4fbe => {
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c5132; continue; }
            pc = 0x4c4fc7; continue;
            }
            0x4c4fc7 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x94),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4c5132; continue; }
            pc = 0x4c4fd6; continue;
            }
            0x4c4fd6 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(4,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x4c500f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4c5132; continue;
            }
            0x4c5014 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c5020); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c5035);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4c5132; continue; }
            pc = 0x4c5040; continue;
            }
            0x4c5040 => {
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c5132; continue; }
            pc = 0x4c5049; continue;
            }
            0x4c5049 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x94),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4c5132; continue; }
            pc = 0x4c5058; continue;
            }
            0x4c5058 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7]; wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x4c508e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4c5132; continue;
            }
            0x4c5093 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            pc = 0x4c50c7; continue;
            }
            0x4c50c7 => {
            c.push(m, 0x4c50cc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4c5105; continue; }
            pc = 0x4c50d3; continue;
            }
            0x4c50d3 => {
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c5105; continue; }
            pc = 0x4c50d8; continue;
            }
            0x4c50d8 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x94),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c5105; continue; }
            pc = 0x4c50e3; continue;
            }
            0x4c50e3 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x4c5103); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4c5132; continue;
            }
            0x4c5105 => {
            c.push(m,0x1u32);
            c.push(m, 0x4c510c);
            { let args = []; c.r[0] = host.call(m, 0x4bb7a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c5125);
            { let args = []; c.r[0] = host.call(m, 0x4bb430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c512f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb7a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x4c5132; continue;
            }
            0x4c5132 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c513c);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5146);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c515a; continue; }
            pc = 0x4c5151; continue;
            }
            0x4c5151 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5157);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c515a; continue;
            }
            0x4c515a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_status_bar_recalc_fill_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5330;
    loop {
        match pc {
            0x4c5330 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xdc),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xe0),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4c5359); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4c55b4; continue; }
            pc = 0x4c5368; continue;
            }
            0x4c5368 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4c564c)); continue;
            }
            0x4c536f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0xcu32);
            c.set(1,32,c.r[6].wrapping_add(0xc4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c5380);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4c5388);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c53cd; continue; }
            pc = 0x4c538f; continue;
            }
            0x4c538f => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4c539d);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x4c53c5);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c53d3; continue;
            }
            0x4c53cd => {
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c53d3; continue;
            }
            0x4c53d3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4c53de); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(5,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c540b => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0xcu32);
            c.set(1,32,c.r[6].wrapping_add(0xc4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c541c);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4c5424);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c5469; continue; }
            pc = 0x4c542b; continue;
            }
            0x4c542b => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4c5439);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x4c5461);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c546f; continue;
            }
            0x4c5469 => {
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c546f; continue;
            }
            0x4c546f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c5487 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0xcu32);
            c.set(0,32,c.r[6].wrapping_add(0xc4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c5498);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4c54a0);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c54e5; continue; }
            pc = 0x4c54a7; continue;
            }
            0x4c54a7 => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4c54b5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x4c54dd);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c54eb; continue;
            }
            0x4c54e5 => {
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c54eb; continue;
            }
            0x4c54eb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c5503 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4c5513);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c5558; continue; }
            pc = 0x4c551a; continue;
            }
            0x4c551a => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4c5528);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x4c5550);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c5562; continue;
            }
            0x4c5558 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c5562; continue;
            }
            0x4c5562 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4c5593); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c55b4 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4c55c4);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c5611; continue; }
            pc = 0x4c55cb; continue;
            }
            0x4c55cb => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4c55d9);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x4c5601);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c561b; continue;
            }
            0x4c5611 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c561b; continue;
            }
            0x4c561b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_status_bar_get_class_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c5680;
    loop {
        match pc {
            0x4c5680 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bf530u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c569b);
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

pub fn sgl2d_status_bar_add_dirty_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c56b0;
    loop {
        match pc {
            0x4c56b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x4c56bb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c56c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sgl2d_status_bar_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c56d0;
    loop {
        match pc {
            0x4c56d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4c56d8);
            { let args = []; c.r[0] = host.call(m, 0x4c4900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4c56e8; continue; }
            pc = 0x4c56df; continue;
            }
            0x4c56df => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4c56e5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c56e8; continue;
            }
            0x4c56e8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
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
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x58a2f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn sea_battle_is_side_human(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x607990;
    loop {
        match pc {
            0x607990 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x6079af; continue; }
            pc = 0x6079aa; continue;
            }
            0x6079aa => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x6079af => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6079bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            c.set(0,8,u32::from(c.cond(4)));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn show_battle_ship_list_banner(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611fc0;
    loop {
        match pc {
            0x611fc0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dfc18u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x611fdd; continue; }
            pc = 0x611fd7; continue;
            }
            0x611fd7 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x611fdd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6127c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x611fdd; continue;
            }
            0x611fdd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6127b5; continue; }
            pc = 0x611ff9; continue;
            }
            0x611ff9 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x5c),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x6127b5; continue; }
            pc = 0x612004; continue;
            }
            0x612004 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3fd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x6127b5; continue; }
            pc = 0x612012; continue;
            }
            0x612012 => {
            { let a = c.r[6].wrapping_add(0x42d); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(6) { pc = 0x612074; continue; }
            pc = 0x61202b; continue;
            }
            0x61202b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(7,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x61205b; continue; }
            pc = 0x61204b; continue;
            }
            0x61204b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            if c.cond(2) { pc = 0x61202b; continue; }
            pc = 0x612059; continue;
            }
            0x612059 => {
            pc = 0x61206e; continue;
            }
            0x61205b => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(3,8,0x1u32);
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            pc = 0x61206e; continue;
            }
            0x61206e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x612074; continue;
            }
            0x612074 => {
            { let a = c.r[6].wrapping_add(0x40c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x410); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x412); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x414); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x418); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x41c); wm(m,a,32,0x3fb00000u32); }
            { let a = c.r[6].wrapping_add(0x420); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x422); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x424); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x428); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x41b); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x419); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x41a); wm(m,a,8,0x0u32); }
            { let a = c.get(6,32); let r = c.add(a,0x42eu32,32); c.set(6,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6123b7; continue; }
            pc = 0x6120dc; continue;
            }
            0x6120dc => {
            c.set(2,32,rm(m,0x6a16d0u32,32));
            c.set(0,32,rm(m,0x6a16ccu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6c7714u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x6120f4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x558),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x61229b; continue; }
            pc = 0x612111; continue;
            }
            0x612111 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x61211d; continue;
            }
            0x61211d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x559),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x6121a4; continue; }
            pc = 0x612135; continue;
            }
            0x612135 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa84),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x6121a4; continue; }
            pc = 0x61214d; continue;
            }
            0x61214d => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x61218c; continue; }
            pc = 0x612162; continue;
            }
            0x612162 => {
            c.set(1,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612170);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c771au32);
            c.push(m,c.get(6,32));
            c.push(m, 0x61217b);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612191; continue;
            }
            0x61218c => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            pc = 0x612191; continue;
            }
            0x612191 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x61214d; continue; }
            pc = 0x6121a4; continue;
            }
            0x6121a4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[1].wrapping_add(0x558),8),8); }
            if c.cond(2) { pc = 0x61211d; continue; }
            pc = 0x6121c0; continue;
            }
            0x6121c0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x61229b; continue; }
            pc = 0x6121cc; continue;
            }
            0x6121cc => {
            c.set(2,32,rm(m,0x6a16d8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6121d9);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(7,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x558),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x61229b; continue; }
            pc = 0x6121f6; continue;
            }
            0x6121f6 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x612202; continue;
            }
            0x612202 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x559),8));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x612283; continue; }
            pc = 0x61221b; continue;
            }
            0x61221b => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa84),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x612283; continue; }
            pc = 0x612233; continue;
            }
            0x612233 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x612270; continue; }
            pc = 0x612248; continue;
            }
            0x612248 => {
            c.set(1,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612256);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c771cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x612261);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612270; continue;
            }
            0x612270 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x612233; continue; }
            pc = 0x612283; continue;
            }
            0x612283 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(7,32,rm(m,c.r[0],32));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[7].wrapping_add(0x558),8),8); }
            if c.cond(2) { pc = 0x612202; continue; }
            pc = 0x61229b; continue;
            }
            0x61229b => {
            c.set(1,32,rm(m,0x6a16dcu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6122a8);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1b),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[2],32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            if c.cond(5) { pc = 0x6122c7; continue; }
            pc = 0x6122be; continue;
            }
            0x6122be => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x65c),16));
            pc = 0x6122cb; continue;
            }
            0x6122c7 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            pc = 0x6122cb; continue;
            }
            0x6122cb => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x61239f; continue; }
            pc = 0x6122f6; continue;
            }
            0x6122f6 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x612302; continue;
            }
            0x612302 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x612383; continue; }
            pc = 0x61231b; continue;
            }
            0x61231b => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xa84),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x612383; continue; }
            pc = 0x612334; continue;
            }
            0x612334 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x612338; continue;
            }
            0x612338 => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[5]),8));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x612370; continue; }
            pc = 0x612348; continue;
            }
            0x612348 => {
            c.set(1,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612356);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c771eu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x612361);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612370; continue;
            }
            0x612370 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x612338; continue; }
            pc = 0x612383; continue;
            }
            0x612383 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x558),8),8); }
            if c.cond(2) { pc = 0x612302; continue; }
            pc = 0x61239f; continue;
            }
            0x61239f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x412); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x414); wm(m,a,32,c.get(0,32)); }
            pc = 0x612776; continue;
            }
            0x6123b7 => {
            c.set(1,32,rm(m,0x6a16d4u32,32));
            c.set(2,32,rm(m,0x6a16ccu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c7720u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x6123d0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x61258e; continue; }
            pc = 0x61240f; continue;
            }
            0x61240f => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x61241b; continue;
            }
            0x61241b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x559),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x6124a7; continue; }
            pc = 0x61242f; continue;
            }
            0x61242f => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa84),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x6124a7; continue; }
            pc = 0x612441; continue;
            }
            0x612441 => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),16),16); }
            if c.cond(5) { pc = 0x612480; continue; }
            pc = 0x612456; continue;
            }
            0x612456 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612464);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c7726u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x61246f);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612494; continue;
            }
            0x612480 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x612494; continue; }
            pc = 0x61248f; continue;
            }
            0x61248f => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            pc = 0x612494; continue;
            }
            0x612494 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x612441; continue; }
            pc = 0x6124a7; continue;
            }
            0x6124a7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x558),8),8); }
            if c.cond(2) { pc = 0x61241b; continue; }
            pc = 0x6124bf; continue;
            }
            0x6124bf => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x61258e; continue; }
            pc = 0x6124cb; continue;
            }
            0x6124cb => {
            c.set(1,32,rm(m,0x6a16d0u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6124d8);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x61258e; continue; }
            pc = 0x6124f4; continue;
            }
            0x6124f4 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x612500; continue;
            }
            0x612500 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x559),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x612576; continue; }
            pc = 0x612514; continue;
            }
            0x612514 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa84),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x612576; continue; }
            pc = 0x612526; continue;
            }
            0x612526 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x612563; continue; }
            pc = 0x61253b; continue;
            }
            0x61253b => {
            c.set(1,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612549);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c7728u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x612554);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612563; continue;
            }
            0x612563 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x612526; continue; }
            pc = 0x612576; continue;
            }
            0x612576 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x558),8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x612500; continue; }
            pc = 0x61258e; continue;
            }
            0x61258e => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x65c),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,32,rm(m,0x6a16dcu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x6125c5);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x612761; continue; }
            pc = 0x6125e2; continue;
            }
            0x6125e2 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x6125ee; continue;
            }
            0x6125ee => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x559),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x61267a; continue; }
            pc = 0x612602; continue;
            }
            0x612602 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa88),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x61267a; continue; }
            pc = 0x612614; continue;
            }
            0x612614 => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),16),16); }
            if c.cond(5) { pc = 0x612653; continue; }
            pc = 0x612629; continue;
            }
            0x612629 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x612637);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c772au32);
            c.push(m,c.get(6,32));
            c.push(m, 0x612642);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612667; continue;
            }
            0x612653 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x612667; continue; }
            pc = 0x612662; continue;
            }
            0x612662 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            pc = 0x612667; continue;
            }
            0x612667 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x612614; continue; }
            pc = 0x61267a; continue;
            }
            0x61267a => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x558),8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x6125ee; continue; }
            pc = 0x612692; continue;
            }
            0x612692 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x612761; continue; }
            pc = 0x61269e; continue;
            }
            0x61269e => {
            c.set(0,32,rm(m,0x6a16d0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6126aa);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x612761; continue; }
            pc = 0x6126c7; continue;
            }
            0x6126c7 => {
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x6126d3; continue;
            }
            0x6126d3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x559),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x612749; continue; }
            pc = 0x6126e7; continue;
            }
            0x6126e7 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa88),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x612749; continue; }
            pc = 0x6126f9; continue;
            }
            0x6126f9 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x612736; continue; }
            pc = 0x61270e; continue;
            }
            0x61270e => {
            c.set(1,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x61271c);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c772cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x612727);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x612736; continue;
            }
            0x612736 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x6126f9; continue; }
            pc = 0x612749; continue;
            }
            0x612749 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x558),8),8); }
            if c.cond(2) { pc = 0x6126d3; continue; }
            pc = 0x612761; continue;
            }
            0x612761 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.r[0].wrapping_add(0x412); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x414); wm(m,a,32,c.get(1,32)); }
            pc = 0x612776; continue;
            }
            0x612776 => {
            c.set(0,32,c.r[7].wrapping_add(0x40c));
            { let a = c.r[7].wrapping_add(0x410); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x418); wm(m,a,8,0xffu32); }
            { let a = c.r[7].wrapping_add(0x422); wm(m,a,16,0xc8u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            { let a = c.r[7].wrapping_add(0x420); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6127a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x42c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x3fd); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x6127b5; continue;
            }
            0x6127b5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x618f10;
    loop {
        match pc {
            0x618f10 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0xffffu32);
            { let a = c.r[0]; wm(m,a,32,0x67af70u32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x34); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x44); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x54); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x56); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x58); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x5a); wm(m,a,16,c.get(1,16)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_get_serialized_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x619120;
    loop {
        match pc {
            0x619120 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,32,0x2u32);
            { let a = rm(m,c.r[7].wrapping_add(0x14),16); c.sub(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x61914d; continue; }
            pc = 0x619132; continue;
            }
            0x619132 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m, 0x619144);
            { let args = []; c.r[0] = host.call(m, 0x61a780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x14),16),16); }
            if c.cond(2) { pc = 0x619132; continue; }
            pc = 0x61914d; continue;
            }
            0x61914d => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6191b0;
    loop {
        match pc {
            0x6191b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66840bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x6191d0);
            { let args = []; c.r[0] = host.call(m, 0x618f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,16); c.sub(a,0x5bu32,16); }
            if c.cond(6) { pc = 0x6191f2; continue; }
            pc = 0x6191de; continue;
            }
            0x6191de => {
            c.set(0,32,c.r[6].wrapping_add(0x14));
            c.push(m,0x2u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6191ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            pc = 0x61920d; continue;
            }
            0x6191f2 => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6191ff);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            pc = 0x61920d; continue;
            }
            0x61920d => {
            c.push(m,c.get(5,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x14),16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x6192a1; continue; }
            pc = 0x61921a; continue;
            }
            0x61921a => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x619225; continue;
            }
            0x619225 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x61922c);
            { let args = []; c.r[0] = host.call(m, 0x61a100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x164u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x619238);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x61925b; continue; }
            pc = 0x61924b; continue;
            }
            0x61924b => {
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x619259);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x61a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x61925d; continue;
            }
            0x61925b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x61925d; continue;
            }
            0x61925d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7]),32));
            c.push(m, 0x619287);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(1,16)); }
            if c.cond(2) { pc = 0x619225; continue; }
            pc = 0x6192a1; continue;
            }
            0x6192a1 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x6192d8; continue; }
            pc = 0x6192ab; continue;
            }
            0x6192ab => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,0xffffu32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x16),16),16); }
            if c.cond(3) { pc = 0x6192d2; continue; }
            pc = 0x6192c4; continue;
            }
            0x6192c4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,0xffffu32); }
            pc = 0x6192d2; continue;
            }
            0x6192d2 => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,16,0x0u32); }
            pc = 0x6192d8; continue;
            }
            0x6192d8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_step_all_ships_repeat(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x619400;
    loop {
        match pc {
            0x619400 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x61944c; continue; }
            pc = 0x61940b; continue;
            }
            0x61940b => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            pc = 0x619413; continue;
            }
            0x619413 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x619447; continue; }
            pc = 0x61941e; continue;
            }
            0x61941e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,0x40u32);
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2],32); c.push(m, 0x619433); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            c.set(1,8,c.get(3,8));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x61941e; continue; }
            pc = 0x619447; continue;
            }
            0x619447 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x619413; continue; }
            pc = 0x61944a; continue;
            }
            0x61944a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x61944c; continue;
            }
            0x61944c => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_on_collision_pair_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x61a0f0;
    loop {
        match pc {
            0x61a0f0 => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn local_map_ship_list_ensure_free_collision_slot(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x61a1d0;
    loop {
        match pc {
            0x61a1d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x20),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x61a4d0; continue; }
            pc = 0x61a1df; continue;
            }
            0x61a1df => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x58),16));
            { let a = c.get(1,16); c.sub(a,0xffbfu32,16); }
            if c.cond(3) { pc = 0x61a4d0; continue; }
            pc = 0x61a1ee; continue;
            }
            0x61a1ee => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x56); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x5a),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x5a); wm(m,a,16,r); } }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x58); wm(m,a,16,c.get(1,16)); }
            c.push(m, 0x61a219);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x61a22e);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a243);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x58),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x61a258);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x58),16));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x61a26d);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x58),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a282);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x58),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x61a297);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x58),16));
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x61a2ac);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x58),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a2c4);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x61a397; continue; }
            pc = 0x61a2db; continue;
            }
            0x61a2db => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x61a2e1; continue;
            }
            0x61a2e1 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a2f6);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.r[2].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x61a312);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x56),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a32e);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x20),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x61a37f; continue; }
            pc = 0x61a344; continue;
            }
            0x61a344 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x48),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x61a344; continue; }
            pc = 0x61a37f; continue;
            }
            0x61a37f => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            c.set(7,32,c.get(3,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x61a2e1; continue; }
            pc = 0x61a397; continue;
            }
            0x61a397 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x61a3aa);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x61a3c0);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x61a3d6);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m, 0x61a3ea);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x61a408);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x61a426);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x56),16); c.sub(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x61a48b; continue; }
            pc = 0x61a43e; continue;
            }
            0x61a43e => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5a),16));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x56),16),16); }
            if c.cond(2) { pc = 0x61a43e; continue; }
            pc = 0x61a48b; continue;
            }
            0x61a48b => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,c.r[2].wrapping_add(0xffffffe0));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x61a4c0; continue; }
            pc = 0x61a4a6; continue;
            }
            0x61a4a6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x56),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x61a4a6; continue; }
            pc = 0x61a4c0; continue;
            }
            0x61a4c0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x56),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,0xffffu32); }
            pc = 0x61a4d0; continue;
            }
            0x61a4d0 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x20),16));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "game_screen_host_ctor" => game_screen_host_ctor(m, host, regs),
        "game_screen_host_dtor" => game_screen_host_dtor(m, host, regs),
        "game_screen_host_release_resources" => game_screen_host_release_resources(m, host, regs),
        "game_screen_host_load_layout" => game_screen_host_load_layout(m, host, regs),
        "game_screen_host_draw" => game_screen_host_draw(m, host, regs),
        "game_screen_host_draw_ships" => game_screen_host_draw_ships(m, host, regs),
        "game_screen_host_activate" => game_screen_host_activate(m, host, regs),
        "game_screen_host_scalar_deleting_dtor" => game_screen_host_scalar_deleting_dtor(m, host, regs),
        "debug_page_convoy_ship_list" => debug_page_convoy_ship_list(m, host, regs),
        "reposition_ui_elements_top_bar_patch" => reposition_ui_elements_top_bar_patch(m, host, regs),
        "game_screen_load_ini_wrapper" => game_screen_load_ini_wrapper(m, host, regs),
        "side_menu_system_init" => side_menu_system_init(m, host, regs),
        "side_menu_get_host_height" => side_menu_get_host_height(m, host, regs),
        "side_menu_force_close_current" => side_menu_force_close_current(m, host, regs),
        "scroll_map_check_ship_list_problems" => scroll_map_check_ship_list_problems(m, host, regs),
        "ship_list_icon_button_ctor" => ship_list_icon_button_ctor(m, host, regs),
        "ship_list_icon_button_draw" => ship_list_icon_button_draw(m, host, regs),
        "ship_list_icon_button_deleting_dtor" => ship_list_icon_button_deleting_dtor(m, host, regs),
        "letter_sub_panel_deleting_dtor" => letter_sub_panel_deleting_dtor(m, host, regs),
        "letter_sub_panel_dtor" => letter_sub_panel_dtor(m, host, regs),
        "combo_list_box_set_size_from_triple" => combo_list_box_set_size_from_triple(m, host, regs),
        "combo_list_box_set_position_from_triple" => combo_list_box_set_position_from_triple(m, host, regs),
        "sgl_status_bar_ctor_default" => sgl_status_bar_ctor_default(m, host, regs),
        "sgl_status_bar_ctor_named" => sgl_status_bar_ctor_named(m, host, regs),
        "sgl_status_bar_dtor" => sgl_status_bar_dtor(m, host, regs),
        "sgl_status_bar_reset" => sgl_status_bar_reset(m, host, regs),
        "sgl2d_status_bar_load_layout" => sgl2d_status_bar_load_layout(m, host, regs),
        "sgl2d_status_bar_slot3_release_string" => sgl2d_status_bar_slot3_release_string(m, host, regs),
        "sgl2d_status_bar_draw" => sgl2d_status_bar_draw(m, host, regs),
        "sgl2d_status_bar_recalc_fill_rect" => sgl2d_status_bar_recalc_fill_rect(m, host, regs),
        "sgl2d_status_bar_get_class_name" => sgl2d_status_bar_get_class_name(m, host, regs),
        "sgl2d_status_bar_add_dirty_rect" => sgl2d_status_bar_add_dirty_rect(m, host, regs),
        "sgl2d_status_bar_scalar_deleting_dtor" => sgl2d_status_bar_scalar_deleting_dtor(m, host, regs),
        "battle_publish_player_side_ships" => battle_publish_player_side_ships(m, host, regs),
        "sea_battle_is_side_human" => sea_battle_is_side_human(m, host, regs),
        "show_battle_ship_list_banner" => show_battle_ship_list_banner(m, host, regs),
        "local_map_ship_list_ctor" => local_map_ship_list_ctor(m, host, regs),
        "local_map_ship_list_get_serialized_size" => local_map_ship_list_get_serialized_size(m, host, regs),
        "local_map_ship_list_deserialize" => local_map_ship_list_deserialize(m, host, regs),
        "local_map_ship_list_step_all_ships_repeat" => local_map_ship_list_step_all_ships_repeat(m, host, regs),
        "local_map_ship_list_on_collision_pair_noop" => local_map_ship_list_on_collision_pair_noop(m, host, regs),
        "local_map_ship_list_ensure_free_collision_slot" => local_map_ship_list_ensure_free_collision_slot(m, host, regs),
        _ => return None,
    })
}
