//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn siege_unit_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564590;
    loop {
        match pc {
            0x564590 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,32,0x6752e0u32); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,0x6752dcu32); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,0x6752d0u32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x6752c4u32); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x31); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x64); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x674e54u32); }
            { let args = []; c.r[0] = host.call(m, 0x557a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_free_variant_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5645e0;
    loop {
        match pc {
            0x5645e0 => {
            c.set(0,32,rm(m,0x6e173cu32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5645f5; continue; }
            pc = 0x5645ec; continue;
            }
            0x5645ec => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5645f2);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5645f5; continue;
            }
            0x5645f5 => {
            c.set(0,32,rm(m,0x6e1740u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x564607; continue; }
            pc = 0x5645fe; continue;
            }
            0x5645fe => {
            c.push(m,c.get(0,32));
            c.push(m, 0x564604);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x564607; continue;
            }
            0x564607 => {
            c.set(0,32,rm(m,0x6e1738u32,32));
            { let a = 0x6e173cu32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = 0x6e1740u32; wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x564656; continue; }
            pc = 0x56461c; continue;
            }
            0x56461c => {
            c.set(1,32,rm(m,0x6e1734u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x564646; continue; }
            pc = 0x564629; continue;
            }
            0x564629 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x56463b; continue; }
            pc = 0x564630; continue;
            }
            0x564630 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x564636); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e1738u32,32));
            pc = 0x56463b; continue;
            }
            0x56463b => {
            c.set(1,32,rm(m,0x6e1734u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x564629; continue; }
            pc = 0x564646; continue;
            }
            0x564646 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x56464c);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e1738u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x564656; continue;
            }
            0x564656 => {
            { let a = 0x6e1734u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_read_variant_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564660;
    loop {
        match pc {
            0x564660 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e1734u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e173cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56467d);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e173cu32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e1734u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e1734u32,32));
            c.set(2,32,rm(m,0x6e1740u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5646a9);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e1740u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e1734u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_draw_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5647c0;
    loop {
        match pc {
            0x5647c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5647f3; continue; }
            pc = 0x5647ca; continue;
            }
            0x5647ca => {
            c.set(2,32,rm(m,0x6e1734u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x31),8));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5647e0; continue; }
            pc = 0x5647dc; continue;
            }
            0x5647dc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5647e9; continue;
            }
            0x5647e0 => {
            c.set(2,32,rm(m,0x6e1738u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5647e9; continue;
            }
            0x5647e9 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5647f0);
            { let args = []; c.r[0] = host.call(m, 0x559520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5647f3; continue;
            }
            0x5647f3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5647ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564a70;
    loop {
        match pc {
            0x564a70 => {
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_load_new_sprite_variant(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x565d10;
    loop {
        match pc {
            0x565d10 => {
            c.set(0,8,rm(m,0x6e1734u32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e1730u32);
            { let a = c.r[6].wrapping_add(0x31); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x565d25);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e1734u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x31),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x565d49; continue; }
            pc = 0x565d34; continue;
            }
            0x565d34 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x565d45);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x565d49 => {
            c.set(1,32,rm(m,0x6e1738u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x565d62);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567000;
    loop {
        match pc {
            0x567000 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x567008);
            { let args = []; c.r[0] = host.call(m, 0x564590, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x567018; continue; }
            pc = 0x56700f; continue;
            }
            0x56700f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x567015);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x567018; continue;
            }
            0x567018 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567020;
    loop {
        match pc {
            0x567020 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x567000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567030;
    loop {
        match pc {
            0x567030 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x567000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_unit_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567040;
    loop {
        match pc {
            0x567040 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x567000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567160;
    loop {
        match pc {
            0x567160 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66447au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x675358u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x675354u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x675348u32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x675344u32); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,0x675338u32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x89); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5671b7);
            { let args = []; c.r[0] = host.call(m, 0x55e050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5671c6);
            { let args = []; c.r[0] = host.call(m, 0x55e010, c.r[1], &args); }
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

pub fn siege_walker_free_variant_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567200;
    loop {
        match pc {
            0x567200 => {
            c.set(0,32,rm(m,0x6e1754u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x567215; continue; }
            pc = 0x56720c; continue;
            }
            0x56720c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x567212);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x567215; continue;
            }
            0x567215 => {
            c.set(0,32,rm(m,0x6e1758u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x567227; continue; }
            pc = 0x56721e; continue;
            }
            0x56721e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x567224);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x567227; continue;
            }
            0x567227 => {
            c.set(0,32,rm(m,0x6e175cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x567239; continue; }
            pc = 0x567230; continue;
            }
            0x567230 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x567236);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x567239; continue;
            }
            0x567239 => {
            c.set(0,32,rm(m,0x6e1760u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x56724b; continue; }
            pc = 0x567242; continue;
            }
            0x567242 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x567248);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x56724b; continue;
            }
            0x56724b => {
            c.set(0,32,rm(m,0x6e1750u32,32));
            { let a = 0x6e1754u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = 0x6e1758u32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6e175cu32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6e1760u32; wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x5672a6; continue; }
            pc = 0x56726c; continue;
            }
            0x56726c => {
            c.set(1,32,rm(m,0x6e174cu32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x567296; continue; }
            pc = 0x567279; continue;
            }
            0x567279 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x56728b; continue; }
            pc = 0x567280; continue;
            }
            0x567280 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x567286); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e1750u32,32));
            pc = 0x56728b; continue;
            }
            0x56728b => {
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x567279; continue; }
            pc = 0x567296; continue;
            }
            0x567296 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x56729c);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e1750u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x5672a6; continue;
            }
            0x5672a6 => {
            { let a = 0x6e174cu32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_read_variant_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5672b0;
    loop {
        match pc {
            0x5672b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e174cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e1754u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5672cd);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e1754u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e174cu32,32));
            c.set(2,32,rm(m,0x6e1758u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5672f9);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e1758u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e174cu32,32));
            c.set(2,32,rm(m,0x6e175cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x567325);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e175cu32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e174cu32,32));
            c.set(2,32,rm(m,0x6e1760u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x567351);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e1760u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_load_new_sprite_variant(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567380;
    loop {
        match pc {
            0x567380 => {
            c.set(0,8,rm(m,0x6e174cu32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e1748u32);
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x567398);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5673bf; continue; }
            pc = 0x5673aa; continue;
            }
            0x5673aa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x5673bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5673bf => {
            c.set(1,32,rm(m,0x6e1750u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5673d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_remove_from_tile_lists(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5674c0;
    loop {
        match pc {
            0x5674c0 => {
            c.set(2,32,rm(m,0x6e174cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5674dc; continue; }
            pc = 0x5674d2; continue;
            }
            0x5674d2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5674db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5674dc => {
            c.set(2,32,rm(m,0x6e1750u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5674ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_draw_sprite(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567960;
    loop {
        match pc {
            0x567960 => {
            c.set(2,32,rm(m,0x6e174cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x567982; continue; }
            pc = 0x567972; continue;
            }
            0x567972 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56797f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x567982 => {
            c.set(2,32,rm(m,0x6e1750u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x567996);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_draw_shadow(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5679a0;
    loop {
        match pc {
            0x5679a0 => {
            c.set(2,32,rm(m,0x6e174cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5679c2; continue; }
            pc = 0x5679b2; continue;
            }
            0x5679b2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5679bf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5679c2 => {
            c.set(2,32,rm(m,0x6e1750u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5679d6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5679e0;
    loop {
        match pc {
            0x5679e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            c.set(1,32,rm(m,0x6e174cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5679f5; continue; }
            pc = 0x5679f2; continue;
            }
            0x5679f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5679f5 => {
            c.set(1,32,rm(m,0x6e1750u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567a00;
    loop {
        match pc {
            0x567a00 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x567a08);
            { let args = []; c.r[0] = host.call(m, 0x567160, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x567a18; continue; }
            pc = 0x567a0f; continue;
            }
            0x567a0f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x567a15);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x567a18; continue;
            }
            0x567a18 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567a20;
    loop {
        match pc {
            0x567a20 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x567a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567a30;
    loop {
        match pc {
            0x567a30 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x567a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x570e20;
    loop {
        match pc {
            0x570e20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6646dau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x675bb0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x675bacu32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x675ba0u32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x675b9cu32); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,0x675b90u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x91); wm(m,a,8,0xffu32); }
            c.push(m, 0x570e73);
            { let args = []; c.r[0] = host.call(m, 0x55e050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x570e82);
            { let args = []; c.r[0] = host.call(m, 0x55e010, c.r[1], &args); }
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

pub fn siege_fighter_release_sprite_variants(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x570eb0;
    loop {
        match pc {
            0x570eb0 => {
            c.set(0,32,rm(m,0x6e2db0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x570ef7; continue; }
            pc = 0x570eb9; continue;
            }
            0x570eb9 => {
            c.set(1,32,rm(m,0x6e2dacu32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x570ee3; continue; }
            pc = 0x570ec6; continue;
            }
            0x570ec6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x570ed8; continue; }
            pc = 0x570ecd; continue;
            }
            0x570ecd => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x570ed3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e2db0u32,32));
            pc = 0x570ed8; continue;
            }
            0x570ed8 => {
            c.set(1,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x570ec6; continue; }
            pc = 0x570ee3; continue;
            }
            0x570ee3 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x570ee9);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2db0u32; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x570ef7; continue;
            }
            0x570ef7 => {
            { let a = 0x6e2dacu32; wm(m,a,32,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_load_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x570f10;
    loop {
        match pc {
            0x570f10 => {
            c.set(0,8,rm(m,0x6e2dacu32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e2da8u32);
            { let a = c.r[6].wrapping_add(0x91); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x570f28);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x570f4f; continue; }
            pc = 0x570f3a; continue;
            }
            0x570f3a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x570f4b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x570f4f => {
            c.set(1,32,rm(m,0x6e2db0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x570f68);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_remove_from_tile_lists(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571070;
    loop {
        match pc {
            0x571070 => {
            c.set(2,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x57108c; continue; }
            pc = 0x571082; continue;
            }
            0x571082 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57108b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57108c => {
            c.set(2,32,rm(m,0x6e2db0u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57109d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_can_engage(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571ed0;
    loop {
        match pc {
            0x571ed0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6a),8));
            c.set(6,32,rm(m,0x6df270u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x571f25; continue; }
            pc = 0x571eee; continue;
            }
            0x571eee => {
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x78),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x571f25; continue; }
            pc = 0x571ef7; continue;
            }
            0x571ef7 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x90),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x571f25; continue; }
            pc = 0x571f13; continue;
            }
            0x571f13 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x571f25; continue; }
            pc = 0x571f1a; continue;
            }
            0x571f1a => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x5a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x571f25; continue; }
            pc = 0x571f21; continue;
            }
            0x571f21 => {
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x571f25 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_draw_sprite(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571f30;
    loop {
        match pc {
            0x571f30 => {
            c.set(2,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x571f52; continue; }
            pc = 0x571f42; continue;
            }
            0x571f42 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571f4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x571f52 => {
            c.set(2,32,rm(m,0x6e2db0u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571f66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_draw_shadow(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571f70;
    loop {
        match pc {
            0x571f70 => {
            c.set(2,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x571f92; continue; }
            pc = 0x571f82; continue;
            }
            0x571f82 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571f8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x571f92 => {
            c.set(2,32,rm(m,0x6e2db0u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571fa6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_get_unit_class(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571fb0;
    loop {
        match pc {
            0x571fb0 => {
            c.set(0,8,0x4u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571fc0;
    loop {
        match pc {
            0x571fc0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x91),8));
            c.set(1,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x571fd5; continue; }
            pc = 0x571fd2; continue;
            }
            0x571fd2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x571fd5 => {
            c.set(1,32,rm(m,0x6e2db0u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571fe0;
    loop {
        match pc {
            0x571fe0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x571fe8);
            { let args = []; c.r[0] = host.call(m, 0x570e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x571ff8; continue; }
            pc = 0x571fef; continue;
            }
            0x571fef => {
            c.push(m,c.get(6,32));
            c.push(m, 0x571ff5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x571ff8; continue;
            }
            0x571ff8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572000;
    loop {
        match pc {
            0x572000 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x571fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572010;
    loop {
        match pc {
            0x572010 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x571fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572020;
    loop {
        match pc {
            0x572020 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x571fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_deleting_dtor_thunk_2c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572030;
    loop {
        match pc {
            0x572030 => {
            { let a = c.get(1,32); let r = c.sub(a,0x2cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x571fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_release_static_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5728b0;
    loop {
        match pc {
            0x5728b0 => {
            c.set(0,32,rm(m,0x6e2dd8u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5728c5; continue; }
            pc = 0x5728bc; continue;
            }
            0x5728bc => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5728c2);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5728c5; continue;
            }
            0x5728c5 => {
            c.set(0,32,rm(m,0x6e2ddcu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5728d7; continue; }
            pc = 0x5728ce; continue;
            }
            0x5728ce => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5728d4);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5728d7; continue;
            }
            0x5728d7 => {
            c.set(0,32,rm(m,0x6e2de0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5728e9; continue; }
            pc = 0x5728e0; continue;
            }
            0x5728e0 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5728e6);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5728e9; continue;
            }
            0x5728e9 => {
            c.set(0,32,rm(m,0x6e2de4u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5728fb; continue; }
            pc = 0x5728f2; continue;
            }
            0x5728f2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5728f8);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5728fb; continue;
            }
            0x5728fb => {
            c.set(0,32,rm(m,0x6e2dd4u32,32));
            { let a = 0x6e2dd8u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = 0x6e2ddcu32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6e2de0u32; wm(m,a,32,c.get(7,32)); }
            { let a = 0x6e2de4u32; wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x572956; continue; }
            pc = 0x57291c; continue;
            }
            0x57291c => {
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x572946; continue; }
            pc = 0x572929; continue;
            }
            0x572929 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x57293b; continue; }
            pc = 0x572930; continue;
            }
            0x572930 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x572936); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e2dd4u32,32));
            pc = 0x57293b; continue;
            }
            0x57293b => {
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x572929; continue; }
            pc = 0x572946; continue;
            }
            0x572946 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x57294c);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2dd4u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x572956; continue;
            }
            0x572956 => {
            { let a = 0x6e2dd0u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_parse_variant_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572960;
    loop {
        match pc {
            0x572960 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e2dd0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e2dd8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57297d);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e2dd8u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e2dd0u32,32));
            c.set(2,32,rm(m,0x6e2ddcu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5729a9);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e2ddcu32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e2dd0u32,32));
            c.set(2,32,rm(m,0x6e2de0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5729d5);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e2de0u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e2dd0u32,32));
            c.set(2,32,rm(m,0x6e2de4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x572a01);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e2de4u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_load_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572a30;
    loop {
        match pc {
            0x572a30 => {
            c.set(0,8,rm(m,0x6e2dd0u32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e2dccu32);
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x572a48);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x572a6f; continue; }
            pc = 0x572a5a; continue;
            }
            0x572a5a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x572a6b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x572a6f => {
            c.set(1,32,rm(m,0x6e2dd4u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x572a88);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_remove_from_tile_lists(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572b70;
    loop {
        match pc {
            0x572b70 => {
            c.set(2,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x572b8c; continue; }
            pc = 0x572b82; continue;
            }
            0x572b82 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x572b8b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x572b8c => {
            c.set(2,32,rm(m,0x6e2dd4u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x572b9d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_draw_sprite(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573030;
    loop {
        match pc {
            0x573030 => {
            c.set(2,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x573052; continue; }
            pc = 0x573042; continue;
            }
            0x573042 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57304f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x573052 => {
            c.set(2,32,rm(m,0x6e2dd4u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x573066);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_draw_shadow(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573070;
    loop {
        match pc {
            0x573070 => {
            c.set(2,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x573092; continue; }
            pc = 0x573082; continue;
            }
            0x573082 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57308f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x573092 => {
            c.set(2,32,rm(m,0x6e2dd4u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5730a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_get_set_header_byte(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5730b0;
    loop {
        match pc {
            0x5730b0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5730c8; continue; }
            pc = 0x5730c2; continue;
            }
            0x5730c2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5730c8 => {
            c.set(1,32,rm(m,0x6e2dd4u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_get_unit_class(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5730e0;
    loop {
        match pc {
            0x5730e0 => {
            c.set(0,8,0x3u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5730f0;
    loop {
        match pc {
            0x5730f0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x88),8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x573105; continue; }
            pc = 0x573102; continue;
            }
            0x573102 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x573105 => {
            c.set(1,32,rm(m,0x6e2dd4u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573110;
    loop {
        match pc {
            0x573110 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x573118);
            { let args = []; c.r[0] = host.call(m, 0x572810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x573128; continue; }
            pc = 0x57311f; continue;
            }
            0x57311f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x573125);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x573128; continue;
            }
            0x573128 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573130;
    loop {
        match pc {
            0x573130 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x573110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573140;
    loop {
        match pc {
            0x573140 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x573110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573150;
    loop {
        match pc {
            0x573150 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x573110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_walker_type0b_deleting_dtor_thunk_2c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573160;
    loop {
        match pc {
            0x573160 => {
            { let a = c.get(1,32); let r = c.sub(a,0x2cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x573110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_cart_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573f80;
    loop {
        match pc {
            0x573f80 => {
            { let a = c.r[1]; wm(m,a,32,0x6760c0u32); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,0x6760bcu32); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,0x6760b0u32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x6760a4u32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x674e54u32); }
            { let args = []; c.r[0] = host.call(m, 0x557a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_cart_draw_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5740a0;
    loop {
        match pc {
            0x5740a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5740d3; continue; }
            pc = 0x5740aa; continue;
            }
            0x5740aa => {
            c.set(2,32,rm(m,0x6e2e14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x35),8));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5740c0; continue; }
            pc = 0x5740bc; continue;
            }
            0x5740bc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5740c9; continue;
            }
            0x5740c0 => {
            c.set(2,32,rm(m,0x6e2e18u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5740c9; continue;
            }
            0x5740c9 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5740d0);
            { let args = []; c.r[0] = host.call(m, 0x559520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5740d3; continue;
            }
            0x5740d3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5740df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_cart_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x574350;
    loop {
        match pc {
            0x574350 => {
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5766e0;
    loop {
        match pc {
            0x5766e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1]; wm(m,a,32,0x676248u32); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,0x676244u32); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,0x676238u32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x67622cu32); }
            { let a = c.r[1].wrapping_add(0x2d); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,16,0x1u32); }
            { let a = c.r[1].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x674e54u32); }
            { let args = []; c.r[0] = host.call(m, 0x557a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_free_variant_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576740;
    loop {
        match pc {
            0x576740 => {
            c.set(0,32,rm(m,0x6e2e4cu32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x57675b; continue; }
            pc = 0x57674c; continue;
            }
            0x57674c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x576752);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2e4cu32; wm(m,a,32,c.get(7,32)); }
            pc = 0x57675b; continue;
            }
            0x57675b => {
            c.set(0,32,rm(m,0x6e2e48u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x57679e; continue; }
            pc = 0x576764; continue;
            }
            0x576764 => {
            c.set(1,32,rm(m,0x6e2e44u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x57678e; continue; }
            pc = 0x576771; continue;
            }
            0x576771 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x576783; continue; }
            pc = 0x576778; continue;
            }
            0x576778 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x57677e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e2e48u32,32));
            pc = 0x576783; continue;
            }
            0x576783 => {
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x576771; continue; }
            pc = 0x57678e; continue;
            }
            0x57678e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x576794);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2e48u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x57679e; continue;
            }
            0x57679e => {
            { let a = 0x6e2e44u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_draw_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5767b0;
    loop {
        match pc {
            0x5767b0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x36),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5767c8; continue; }
            pc = 0x5767c4; continue;
            }
            0x5767c4 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5767d1; continue;
            }
            0x5767c8 => {
            c.set(1,32,rm(m,0x6e2e48u32,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5767d1; continue;
            }
            0x5767d1 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x80u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2d),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(1,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57681e);
            { let args = []; c.r[0] = host.call(m, 0x559520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8));
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x576857);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576d10;
    loop {
        match pc {
            0x576d10 => {
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_read_variant_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5773c0;
    loop {
        match pc {
            0x5773c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0xcu32);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1a),8));
            c.set(1,32,rm(m,0x6e2e4cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(2,32));
            c.push(m, 0x5773e6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_load_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577400;
    loop {
        match pc {
            0x577400 => {
            c.set(0,8,rm(m,0x6e2e44u32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e2e40u32);
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x577415);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6e2e4cu32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x57742e);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = 0x6e2e4cu32; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57745a; continue; }
            pc = 0x577445; continue;
            }
            0x577445 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x577456);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x57745a => {
            c.set(1,32,rm(m,0x6e2e48u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x577473);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_anim_end_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577480;
    loop {
        match pc {
            0x577480 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577490;
    loop {
        match pc {
            0x577490 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x36),8));
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5774a2; continue; }
            pc = 0x57749f; continue;
            }
            0x57749f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5774a2 => {
            c.set(1,32,rm(m,0x6e2e48u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5774b0;
    loop {
        match pc {
            0x5774b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5774b8);
            { let args = []; c.r[0] = host.call(m, 0x5766e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5774c8; continue; }
            pc = 0x5774bf; continue;
            }
            0x5774bf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x5774c5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5774c8; continue;
            }
            0x5774c8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5774d0;
    loop {
        match pc {
            0x5774d0 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5774b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5774e0;
    loop {
        match pc {
            0x5774e0 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5774b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_projectile_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5774f0;
    loop {
        match pc {
            0x5774f0 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5774b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577610;
    loop {
        match pc {
            0x577610 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66479au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x6762c0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x6762b8u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x6762acu32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x6762a8u32); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,0x67629cu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x90); wm(m,a,8,0xffu32); }
            c.push(m, 0x577663);
            { let args = []; c.r[0] = host.call(m, 0x55e050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x577672);
            { let args = []; c.r[0] = host.call(m, 0x55e010, c.r[1], &args); }
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

pub fn siege_fighter_walker_free_variant_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577690;
    loop {
        match pc {
            0x577690 => {
            c.set(0,32,rm(m,0x6e2e60u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5776a5; continue; }
            pc = 0x57769c; continue;
            }
            0x57769c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5776a2);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5776a5; continue;
            }
            0x5776a5 => {
            c.set(0,32,rm(m,0x6e2e5cu32,32));
            { let a = 0x6e2e60u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5776ee; continue; }
            pc = 0x5776b4; continue;
            }
            0x5776b4 => {
            c.set(1,32,rm(m,0x6e2e58u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x5776de; continue; }
            pc = 0x5776c1; continue;
            }
            0x5776c1 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5776d3; continue; }
            pc = 0x5776c8; continue;
            }
            0x5776c8 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x5776ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e2e5cu32,32));
            pc = 0x5776d3; continue;
            }
            0x5776d3 => {
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5776c1; continue; }
            pc = 0x5776de; continue;
            }
            0x5776de => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5776e4);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2e5cu32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x5776ee; continue;
            }
            0x5776ee => {
            { let a = 0x6e2e58u32; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_read_variant_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577710;
    loop {
        match pc {
            0x577710 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6e2e58u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e2e60u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57772d);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6e2e60u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_load_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577750;
    loop {
        match pc {
            0x577750 => {
            c.set(0,8,rm(m,0x6e2e58u32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e2e54u32);
            { let a = c.r[6].wrapping_add(0x90); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x577768);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57778f; continue; }
            pc = 0x57777a; continue;
            }
            0x57777a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x57778b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x57778f => {
            c.set(1,32,rm(m,0x6e2e5cu32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5777a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_remove_from_tile_lists(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577920;
    loop {
        match pc {
            0x577920 => {
            c.set(2,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x57793c; continue; }
            pc = 0x577932; continue;
            }
            0x577932 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57793b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57793c => {
            c.set(2,32,rm(m,0x6e2e5cu32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57794d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_draw_sprite(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579830;
    loop {
        match pc {
            0x579830 => {
            c.set(2,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x579852; continue; }
            pc = 0x579842; continue;
            }
            0x579842 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57984f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x579852 => {
            c.set(2,32,rm(m,0x6e2e5cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x579866);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_draw_shadow(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579870;
    loop {
        match pc {
            0x579870 => {
            c.set(2,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x579892; continue; }
            pc = 0x579882; continue;
            }
            0x579882 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57988f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x579892 => {
            c.set(2,32,rm(m,0x6e2e5cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5798a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_get_set_header_byte(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5798b0;
    loop {
        match pc {
            0x5798b0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x90),8));
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5798c8; continue; }
            pc = 0x5798c2; continue;
            }
            0x5798c2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5798c8 => {
            c.set(1,32,rm(m,0x6e2e5cu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x8),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5798e0;
    loop {
        match pc {
            0x5798e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x90),8));
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5798f5; continue; }
            pc = 0x5798f2; continue;
            }
            0x5798f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5798f5 => {
            c.set(1,32,rm(m,0x6e2e5cu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579900;
    loop {
        match pc {
            0x579900 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x579908);
            { let args = []; c.r[0] = host.call(m, 0x577610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x579918; continue; }
            pc = 0x57990f; continue;
            }
            0x57990f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x579915);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x579918; continue;
            }
            0x579918 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579920;
    loop {
        match pc {
            0x579920 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x579900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579930;
    loop {
        match pc {
            0x579930 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x579900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579940;
    loop {
        match pc {
            0x579940 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x579900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fighter_walker_deleting_dtor_thunk_2c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579950;
    loop {
        match pc {
            0x579950 => {
            { let a = c.get(1,32); let r = c.sub(a,0x2cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x579900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57d310;
    loop {
        match pc {
            0x57d310 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x57d318);
            { let args = []; c.r[0] = host.call(m, 0x557a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x674e54u32); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x29); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x676688u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x676684u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x676678u32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x67666cu32); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,8,0xffu32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57d350;
    loop {
        match pc {
            0x57d350 => {
            { let a = c.r[1]; wm(m,a,32,0x676688u32); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,0x676684u32); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,0x676678u32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x67666cu32); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,0x674e54u32); }
            { let args = []; c.r[0] = host.call(m, 0x557a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_move_to(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57d880;
    loop {
        match pc {
            0x57d880 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x57d8ae; continue; }
            pc = 0x57d895; continue;
            }
            0x57d895 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x28),32),32); }
            if c.cond(5) { pc = 0x57d8ae; continue; }
            pc = 0x57d89f; continue;
            }
            0x57d89f => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57d9b5; continue; }
            pc = 0x57d8ae; continue;
            }
            0x57d8ae => {
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57d8c1; continue; }
            pc = 0x57d8bd; continue;
            }
            0x57d8bd => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x57d8ca; continue;
            }
            0x57d8c1 => {
            c.set(1,32,rm(m,0x6e2eacu32,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x57d8ca; continue;
            }
            0x57d8ca => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x80u32,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xffffffe0u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(0,32); c.sub(a,0xc0u32,32); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(7) { pc = 0x57d9b5; continue; }
            pc = 0x57d902; continue;
            }
            0x57d902 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x57d9cc),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x57d9c0)); continue;
            }
            0x57d911 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57d92a);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,0xbu32);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57d952);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x57d968 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57d981);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(2,32));
            c.set(5,32,0xfffffff5u32);
            c.push(m,c.get(0,32));
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.push(m,c.get(3,32));
            c.push(m, 0x57d9a9);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57d9b5; continue;
            }
            0x57d9b5 => {
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

pub fn siege_fixture_type19_erase(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57da90;
    loop {
        match pc {
            0x57da90 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57daa6; continue; }
            pc = 0x57daa2; continue;
            }
            0x57daa2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57daaf; continue;
            }
            0x57daa6 => {
            c.set(1,32,rm(m,0x6e2eacu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x57daaf; continue;
            }
            0x57daaf => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x80u32,32); c.set(7,32,r); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xffffffe0u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(1,32); c.sub(a,0xc0u32,32); }
            c.set(7,32,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(7,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)));
            if c.cond(7) { pc = 0x57db3f; continue; }
            pc = 0x57dae6; continue;
            }
            0x57dae6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x57db74),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x57db68)); continue;
            }
            0x57daf5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.set(3,32,0xfffffff5u32);
            c.push(m,c.get(1,32));
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.push(m,c.get(0,32));
            c.push(m, 0x57db0d);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57db3f; continue;
            }
            0x57db1b => {
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.set(3,32,0xfffffff5u32);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57db33);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57db3f; continue;
            }
            0x57db3f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x57db63; continue; }
            pc = 0x57db5d; continue;
            }
            0x57db5d => {
            c.push(m,c.get(6,32));
            c.push(m, 0x57db63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x630a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x57db63; continue;
            }
            0x57db63 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57dc40;
    loop {
        match pc {
            0x57dc40 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57dc5c; continue; }
            pc = 0x57dc58; continue;
            }
            0x57dc58 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x57dc65; continue;
            }
            0x57dc5c => {
            c.set(1,32,rm(m,0x6e2eacu32,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x57dc65; continue;
            }
            0x57dc65 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(5,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28),8));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.add(a,0xffffff72u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x60u32,32); }
            if c.cond(7) { pc = 0x57df52; continue; }
            pc = 0x57dc93; continue;
            }
            0x57dc93 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x57e0a0),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x57e088)); continue;
            }
            0x57dca2 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x57dca9);
            { let args = []; c.r[0] = host.call(m, 0x629c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57dd24; continue; }
            pc = 0x57dcad; continue;
            }
            0x57dcad => {
            { let a = rm(m,c.r[7].wrapping_add(0x42),16); c.sub(a,0x200u32,16); }
            if c.cond(2) { pc = 0x57dd24; continue; }
            pc = 0x57dcb5; continue;
            }
            0x57dcb5 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            pc = 0x57dcba; continue;
            }
            0x57dcba => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0xc5c0),16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x57dd16; continue; }
            pc = 0x57dcd6; continue;
            }
            0x57dcd6 => {
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0xc5b8),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x57dd16; continue; }
            pc = 0x57dcf2; continue;
            }
            0x57dcf2 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc5a0),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2f),8));
            pc = 0x57dd02; continue;
            }
            0x57dd02 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = rm(m,c.r[0].wrapping_add(0x68),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x57dd3f; continue; }
            pc = 0x57dd0a; continue;
            }
            0x57dd0a => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x57dd02; continue; }
            pc = 0x57dd16; continue;
            }
            0x57dd16 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x57dcba; continue; }
            pc = 0x57dd24; continue;
            }
            0x57dd24 => {
            c.set(2,32,c.r[4].wrapping_add(0x13));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57dd36);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x557340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57dd3f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            pc = 0x57df52; continue;
            }
            0x57dd4b => {
            c.set(1,32,c.r[4].wrapping_add(0x13));
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57dd5d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x557340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57dd66 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x57dd6d);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57de51; continue; }
            pc = 0x57dd75; continue;
            }
            0x57dd75 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x57de51; continue; }
            pc = 0x57dd80; continue;
            }
            0x57dd80 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57dd93; continue; }
            pc = 0x57dd87; continue;
            }
            0x57dd87 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57dd93 => {
            c.push(m,0x91u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57dda1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            pc = 0x57dda5; continue;
            }
            0x57dda5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x57ddc1);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57de68; continue; }
            pc = 0x57ddc9; continue;
            }
            0x57ddc9 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x57de68; continue; }
            pc = 0x57ddd4; continue;
            }
            0x57ddd4 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x57de68; continue; }
            pc = 0x57dddd; continue;
            }
            0x57dddd => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57de36; continue; }
            pc = 0x57dde4; continue;
            }
            0x57dde4 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x57ddf8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57de32; continue; }
            pc = 0x57ddfc; continue;
            }
            0x57ddfc => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x57de0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x57de13);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x1582u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x57de32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x57de32; continue;
            }
            0x57de32 => {
            { let a = rm(m,c.r[3].wrapping_add(0x10),32); let n = 0x2u32; let r = c.shr(a,n,32); { let a = c.r[3].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x57de36; continue;
            }
            0x57de36 => {
            c.set(0,32,c.r[4].wrapping_add(0x13));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57de48);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x557340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57de51 => {
            c.push(m,0x8eu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57de5f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57de68 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x57de85);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57deec; continue; }
            pc = 0x57de89; continue;
            }
            0x57de89 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x57deec; continue; }
            pc = 0x57de90; continue;
            }
            0x57de90 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57deec; continue; }
            pc = 0x57de97; continue;
            }
            0x57de97 => {
            c.push(m,0x11u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57dea2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[7],32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x7fffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,8); let r = c.sub(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df52; continue;
            }
            0x57deec => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x57df52; continue; }
            pc = 0x57df00; continue;
            }
            0x57df00 => {
            c.set(7,32,c.r[6].wrapping_add(0x20));
            c.push(m,0x11u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x57df0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x57df44; continue; }
            pc = 0x57df33; continue;
            }
            0x57df33 => {
            c.push(m,0x8eu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x57df40);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x57df44; continue;
            }
            0x57df44 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x57df52; continue;
            }
            0x57df52 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x57df65; continue; }
            pc = 0x57df5b; continue;
            }
            0x57df5b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57df65 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57e07e; continue; }
            pc = 0x57df70; continue;
            }
            0x57df70 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x80u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xffffffe0u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.sub(a,0xc0u32,32); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(5,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            if c.cond(7) { pc = 0x57dff8; continue; }
            pc = 0x57dfa5; continue;
            }
            0x57dfa5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x57e110),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x57e104)); continue;
            }
            0x57dfb4 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[5].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57dfcd);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[5].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57dff8; continue;
            }
            0x57dfd7 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[5].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57dff0);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[5].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57dff8; continue;
            }
            0x57dff8 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(0,32); let r = c.add(a,0xffffffe0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc0u32,32); }
            if c.cond(7) { pc = 0x57e07e; continue; }
            pc = 0x57e023; continue;
            }
            0x57e023 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x57e1e0),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x57e1d4)); continue;
            }
            0x57e032 => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57e046);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57e05d => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x57e071);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57e07e; continue;
            }
            0x57e07e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_load_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e2b0;
    loop {
        match pc {
            0x57e2b0 => {
            c.set(0,8,rm(m,0x6e2ea8u32,8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6e2ea4u32);
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x57e2c5);
            { let args = []; c.r[0] = host.call(m, 0x5629d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57e2e9; continue; }
            pc = 0x57e2d4; continue;
            }
            0x57e2d4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m, 0x57e2e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x57e2e9 => {
            c.set(1,32,rm(m,0x6e2eacu32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x57e302);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x558770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_release_static_resources(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e390;
    loop {
        match pc {
            0x57e390 => {
            c.set(0,32,rm(m,0x6e2eacu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57e3d7; continue; }
            pc = 0x57e399; continue;
            }
            0x57e399 => {
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x57e3c3; continue; }
            pc = 0x57e3a6; continue;
            }
            0x57e3a6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x57e3b8; continue; }
            pc = 0x57e3ad; continue;
            }
            0x57e3ad => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x57e3b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e2eacu32,32));
            pc = 0x57e3b8; continue;
            }
            0x57e3b8 => {
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57e3a6; continue; }
            pc = 0x57e3c3; continue;
            }
            0x57e3c3 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x57e3c9);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e2eacu32; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x57e3d7; continue;
            }
            0x57e3d7 => {
            { let a = 0x6e2ea8u32; wm(m,a,32,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_get_sprite_set(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e3f0;
    loop {
        match pc {
            0x57e3f0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c),8));
            c.set(1,32,rm(m,0x6e2ea8u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57e402; continue; }
            pc = 0x57e3ff; continue;
            }
            0x57e3ff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57e402 => {
            c.set(1,32,rm(m,0x6e2eacu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e410;
    loop {
        match pc {
            0x57e410 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x57e418);
            { let args = []; c.r[0] = host.call(m, 0x57d350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x57e428; continue; }
            pc = 0x57e41f; continue;
            }
            0x57e41f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x57e425);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x57e428; continue;
            }
            0x57e428 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_deleting_dtor_thunk_08(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e430;
    loop {
        match pc {
            0x57e430 => {
            { let a = c.get(1,32); let r = c.sub(a,0x8u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x57e410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_deleting_dtor_thunk_1c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e440;
    loop {
        match pc {
            0x57e440 => {
            { let a = c.get(1,32); let r = c.sub(a,0x1cu32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x57e410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_fixture_type19_deleting_dtor_thunk_20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57e450;
    loop {
        match pc {
            0x57e450 => {
            { let a = c.get(1,32); let r = c.sub(a,0x20u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x57e410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_compute_checksum(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fbe90;
    loop {
        match pc {
            0x5fbe90 => {
            c.push(m, 0x5fbe95);
            { let args = []; c.r[0] = host.call(m, 0x604140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x1du32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fbea0;
    loop {
        match pc {
            0x5fbea0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[5].wrapping_add(0x2f18),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5fbeb4; continue; }
            pc = 0x5fbeb0; continue;
            }
            0x5fbeb0 => {
            { let a = c.r[5].wrapping_add(0x5e); wm(m,a,8,0x1u32); }
            pc = 0x5fbeb4; continue;
            }
            0x5fbeb4 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[5].wrapping_add(0x1e98));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5fbec7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x1eb2));
            c.set(3,32,c.r[5].wrapping_add(0x1ec4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            pc = 0x5fbedf; continue;
            }
            0x5fbedf => {
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5fbee8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xfffffffc));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5fbef7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5fbf01);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0xc));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5fbf0e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5fbedf; continue; }
            pc = 0x5fbf21; continue;
            }
            0x5fbf21 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5fbf29);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x604220, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn town_attack_battle_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fbf30;
    loop {
        match pc {
            0x5fbf30 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.set(0,32,c.r[3].wrapping_add(0x1e98));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fbf49);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[3].wrapping_add(0x2f1a); wm(m,a,8,0x1u32); }
            c.set(7,32,c.r[3].wrapping_add(0x1eb2));
            c.set(5,32,c.r[3].wrapping_add(0x2ef8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            pc = 0x5fbf68; continue;
            }
            0x5fbf68 => {
            c.push(m,0x4u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffefcc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5fbf77);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,c.r[7].wrapping_add(0xfffffffc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5fbf86);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5fbf90);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(0,32,c.r[7].wrapping_add(0xc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fbf9d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[7].wrapping_add(0xffffffe7); wm(m,a,8,0xau32); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5fbf68; continue; }
            pc = 0x5fbfc2; continue;
            }
            0x5fbfc2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[3].wrapping_add(0x2f18); wm(m,a,16,0x0u32); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5fbfd8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x604600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_handle_object_collision(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ff8e0;
    loop {
        match pc {
            0x5ff8e0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(3,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3]),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff95c; continue; }
            pc = 0x5ff8fc; continue;
            }
            0x5ff8fc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(3,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = rm(m,c.r[2].wrapping_add(0x12a),8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x5ffc03; continue; }
            pc = 0x5ff91e; continue;
            }
            0x5ff91e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.sub(a,0x7u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5ff94d; continue; }
            pc = 0x5ff936; continue;
            }
            0x5ff936 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5ffc03; continue; }
            pc = 0x5ff93d; continue;
            }
            0x5ff93d => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ff946);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x608850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5ff94d => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ff955);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x600770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5ff95c => {
            { let a = c.get(1,8); c.sub(a,0x8u32,8); }
            if c.cond(5) { pc = 0x5ffc03; continue; }
            pc = 0x5ff965; continue;
            }
            0x5ff965 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5ffc03; continue; }
            pc = 0x5ff97d; continue;
            }
            0x5ff97d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ff98f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x60117b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ffc03; continue; }
            pc = 0x5ff997; continue;
            }
            0x5ff997 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x668),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x24),16));
            { let a = c.get(2,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x5ffa1f; continue; }
            pc = 0x5ff9b4; continue;
            }
            0x5ff9b4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x559),8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0xaca),8));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0xaca));
            if c.cond(3) { pc = 0x5ff9e9; continue; }
            pc = 0x5ff9e5; continue;
            }
            0x5ff9e5 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            pc = 0x5ff9e9; continue;
            }
            0x5ff9e9 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xacc),16); c.sub(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x5ffa1f; continue; }
            pc = 0x5ff9f4; continue;
            }
            0x5ff9f4 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x668),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5ffa15; continue; }
            pc = 0x5ffa09; continue;
            }
            0x5ffa09 => {
            { let a = rm(m,c.r[0].wrapping_add(0x24),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x5ffa15; continue; }
            pc = 0x5ffa0f; continue;
            }
            0x5ffa0f => {
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,16,0xffffu32); }
            pc = 0x5ffa15; continue;
            }
            0x5ffa15 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xacc),16),16); }
            if c.cond(2) { pc = 0x5ff9f4; continue; }
            pc = 0x5ffa1f; continue;
            }
            0x5ffa1f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x668),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ffa40);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x60130b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[6].wrapping_add(0xad2),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5ffa52; continue; }
            pc = 0x5ffa49; continue;
            }
            0x5ffa49 => {
            c.push(m,0xeu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ffa52);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611649, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5ffa52; continue;
            }
            0x5ffa52 => {
            c.set(1,16,rm(m,0x6e59ccu32,8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x662),16),16); }
            if c.cond(5) { pc = 0x5ffbdd; continue; }
            pc = 0x5ffa67; continue;
            }
            0x5ffa67 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            c.push(m,0x100020u32);
            c.push(m,0x12u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5ffac1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xeea2u32);
            c.push(m, 0x5ffad1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5ffb3d; continue; }
            pc = 0x5ffad5; continue;
            }
            0x5ffad5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0xeea2u32);
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.push(m, 0x5ffaee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x5ffaf5);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x115eu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5ffb39);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5ffb3d; continue;
            }
            0x5ffb3d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x668),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,0x5d9f7391u32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x14),32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x5ffb73; continue; }
            pc = 0x5ffb6b; continue;
            }
            0x5ffb6b => {
            c.set(2,8,0x1u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            pc = 0x5ffb73; continue;
            }
            0x5ffb73 => {
            if c.cond(6) { pc = 0x5ffbdd; continue; }
            pc = 0x5ffb75; continue;
            }
            0x5ffb75 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            pc = 0x5ffb7f; continue;
            }
            0x5ffb7f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)),16));
            c.push(m, 0x5ffb91);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            c.push(m,0x0u32);
            c.push(m,0x13u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5ffbd6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5ffb7f; continue; }
            pc = 0x5ffbd9; continue;
            }
            0x5ffbd9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5ffbdd; continue;
            }
            0x5ffbdd => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x668),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5ffc03; continue; }
            pc = 0x5ffbf3; continue;
            }
            0x5ffbf3 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ffbfa);
            { let args = []; c.r[0] = host.call(m, 0x602da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x5ffc00);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5ffc03; continue;
            }
            0x5ffc03 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_is_water_cell(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ffe20;
    loop {
        match pc {
            0x5ffe20 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1e98),8));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x5ffe4f; continue; }
            pc = 0x5ffe39; continue;
            }
            0x5ffe39 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1ed8),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x1edc),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            pc = 0x5ffe53; continue;
            }
            0x5ffe4f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5ffe53; continue;
            }
            0x5ffe53 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x2ee0),32),32); }
            if c.cond(3) { pc = 0x5ffee3; continue; }
            pc = 0x5ffe5f; continue;
            }
            0x5ffe5f => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x2ee4),32),32); }
            if c.cond(3) { pc = 0x5ffee3; continue; }
            pc = 0x5ffe67; continue;
            }
            0x5ffe67 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x5ffeae; continue; }
            pc = 0x5ffe6b; continue;
            }
            0x5ffe6b => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffau32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(7) { pc = 0x5ffea6; continue; }
            pc = 0x5ffe97; continue;
            }
            0x5ffe97 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x5ffef4),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x5ffeec)); continue;
            }
            0x5ffea6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5ffeae => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x6ddba4u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffau32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(7) { pc = 0x5ffea6; continue; }
            pc = 0x5ffed4; continue;
            }
            0x5ffed4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x5fff18),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x5fff10)); continue;
            }
            0x5ffee3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_enter_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fffa0;
    loop {
        match pc {
            0x5fffa0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x5fffaf);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x662),16));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x96u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.push(m, 0x5fffe5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x662),8));
            { let a = 0x6e59ccu32; wm(m,a,8,c.get(0,8)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x600004);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x600014); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x600021); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x600031); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x60003e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x600048);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x600056; continue; }
            pc = 0x60004d; continue;
            }
            0x60004d => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x600056);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b8380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x600056; continue;
            }
            0x600056 => {
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_place_ships(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x600060;
    loop {
        match pc {
            0x600060 => {
            { let a = c.get(4,32); let r = c.sub(a,0x68u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m, 0x600072);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1edc),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x880u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x781u32); }
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m, 0x60009a);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x6000a2);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x6000f0; continue; }
            pc = 0x6000a9; continue;
            }
            0x6000a9 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x6000cc; continue; }
            pc = 0x6000ad; continue;
            }
            0x6000ad => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x880u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x600127; continue;
            }
            0x6000cc => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x1100u32);
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x880u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x600127; continue;
            }
            0x6000f0 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x600105; continue; }
            pc = 0x6000f4; continue;
            }
            0x6000f4 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x600119; continue;
            }
            0x600105 => {
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(0,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(2)));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0xf02u32);
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x600119; continue;
            }
            0x600119 => {
            { let a = c.get(0,32); let r = c.add(a,0x781u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x600127; continue;
            }
            0x600127 => {
            c.set(2,32,c.get(3,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x48));
            c.fstp64(m,c.r[4].wrapping_add(0x54));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x60014e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x600156);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x600169; continue; }
            pc = 0x60015c; continue;
            }
            0x60015c => {
            c.set(5,32,0xb5u32);
            c.set(3,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x600181; continue;
            }
            0x600169 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,c.get(0,32));
            pc = 0x600181; continue;
            }
            0x600181 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x48));
            c.fstp64(m,c.r[4].wrapping_add(0x54));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6001aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x6001b2);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(3,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x60022b; continue; }
            pc = 0x6001fc; continue;
            }
            0x6001fc => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            pc = 0x60022b; continue;
            }
            0x60022b => {
            { let a = rm(m,c.r[6].wrapping_add(0x658),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x6003f0; continue; }
            pc = 0x600241; continue;
            }
            0x600241 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            pc = 0x600286; continue;
            }
            0x600286 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60028d);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,0x40u32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6002b2);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,0x40u32,32); c.set(0,32,r); }
            c.set(2,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,8); let r = c.sub(a,0x5u32,8); c.set(1,8,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(1,8); let n = 0x4u32; let r = c.shl(a,n,8); c.set(1,8,r); }
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x600367; continue; }
            pc = 0x60030e; continue;
            }
            0x60030e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x3c),16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x600367; continue; }
            pc = 0x600344; continue;
            }
            0x600344 => {
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0xc8u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x600367; continue;
            }
            0x600367 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x6003a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6003b3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5ffe20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x6003c9; continue; }
            pc = 0x6003b7; continue;
            }
            0x6003b7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x6003c9; continue;
            }
            0x6003c9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x600286; continue; }
            pc = 0x6003f0; continue;
            }
            0x6003f0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x65a),16));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x44));
            c.fstp64(m,c.r[4].wrapping_add(0x5c));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x600415);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x60041d);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x600467; continue; }
            pc = 0x60043c; continue;
            }
            0x60043c => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(0xffffffff));
            c.set(2,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,0xc8u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,0xc8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            pc = 0x600467; continue;
            }
            0x600467 => {
            { let a = rm(m,c.r[6].wrapping_add(0x65a),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x600657; continue; }
            pc = 0x60047d; continue;
            }
            0x60047d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.imul(a,0xc8u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.imul(a,0xc8u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            pc = 0x6004cc; continue;
            }
            0x6004c8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x6004cc; continue;
            }
            0x6004cc => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6004d3);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[2].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x600503);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x658),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(2,8); let r = c.sub(a,0x5u32,8); c.set(2,8,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,8); let n = 0x4u32; let r = c.shl(a,n,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x6005d2; continue; }
            pc = 0x600579; continue;
            }
            0x600579 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x3c),16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x6005d2; continue; }
            pc = 0x6005af; continue;
            }
            0x6005af => {
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0xc8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,0xc8u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x9u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x6005d2; continue;
            }
            0x6005d2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x60060d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60061e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5ffe20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x600634; continue; }
            pc = 0x600622; continue;
            }
            0x600622 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x600634; continue;
            }
            0x600634 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x65a),16));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            if c.cond(2) { pc = 0x6004c8; continue; }
            pc = 0x600657; continue;
            }
            0x600657 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x68u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_open_for_player(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x600660;
    loop {
        match pc {
            0x600660 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x60067b; continue; }
            pc = 0x60066d; continue;
            }
            0x60066d => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x1c),32); c.push(m, 0x600674); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x60067b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x2f18); wm(m,a,16,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x600761; continue; }
            pc = 0x600690; continue;
            }
            0x600690 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x600697); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            if c.cond(6) { pc = 0x600761; continue; }
            pc = 0x6006ab; continue;
            }
            0x6006ab => {
            c.set(7,32,rm(m,0x6dfc14u32,32));
            pc = 0x6006b1; continue;
            }
            0x6006b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6006dc; continue; }
            pc = 0x6006cb; continue;
            }
            0x6006cb => {
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            if c.cond(2) { pc = 0x6006b1; continue; }
            pc = 0x6006d5; continue;
            }
            0x6006d5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x6006dc => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x5d),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x600761; continue; }
            pc = 0x6006e9; continue;
            }
            0x6006e9 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6006fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x60070d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x60071a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x60072a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x600737); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x600741);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x60074e; continue; }
            pc = 0x600745; continue;
            }
            0x600745 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x60074e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b8380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x60074e; continue;
            }
            0x60074e => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x600755);
            { let args = []; c.r[0] = host.call(m, 0x6107b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x662),8));
            { let a = 0x6e59ccu32; wm(m,a,8,c.get(1,8)); }
            pc = 0x600761; continue;
            }
            0x600761 => {
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

pub fn town_attack_battle_get_land_mask_cell(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x60114b;
    loop {
        match pc {
            0x60114b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x601174; continue; }
            pc = 0x601153; continue;
            }
            0x601153 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x601174; continue; }
            pc = 0x601158; continue;
            }
            0x601158 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x601174; continue; }
            pc = 0x601160; continue;
            }
            0x601160 => {
            { let a = c.get(2,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x601174; continue; }
            pc = 0x601165; continue;
            }
            0x601165 => {
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1ee0),8));
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x601174 => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_projectile_reaches_defense_piece(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x60117b;
    loop {
        match pc {
            0x60117b => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x668),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(7,32));
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x6011b6; continue; }
            pc = 0x6011b3; continue;
            }
            0x6011b3 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x6011b9; continue;
            }
            0x6011b6 => {
            if c.cond(13) { pc = 0x6011bd; continue; }
            pc = 0x6011b8; continue;
            }
            0x6011b8 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            pc = 0x6011b9; continue;
            }
            0x6011b9 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x6011bd; continue;
            }
            0x6011bd => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x6011cb; continue; }
            pc = 0x6011c4; continue;
            }
            0x6011c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x6011d2; continue;
            }
            0x6011cb => {
            if c.cond(13) { pc = 0x6011d6; continue; }
            pc = 0x6011cd; continue;
            }
            0x6011cd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x6011d2; continue;
            }
            0x6011d2 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x6011d6; continue;
            }
            0x6011d6 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xac8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6011f2; continue; }
            pc = 0x6011e0; continue;
            }
            0x6011e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x6011f2; continue;
            }
            0x6011f2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            c.set(5,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(7,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.get(5,32));
            c.set(6,32,c.r[0].wrapping_add(0xffffffde));
            pc = 0x601228; continue;
            }
            0x601224 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x601228; continue;
            }
            0x601228 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x60125c; continue; }
            pc = 0x60123d; continue;
            }
            0x60123d => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x6012f4)); continue;
            }
            0x601244 => {
            c.set(6,32,c.get(0,32));
            c.set(7,32,c.r[5].wrapping_add(0xffffffde));
            pc = 0x60125c; continue;
            }
            0x60124b => {
            c.set(6,32,c.r[0].wrapping_add(0x22));
            pc = 0x60125a; continue;
            }
            0x601250 => {
            c.set(6,32,c.get(0,32));
            c.set(7,32,c.r[5].wrapping_add(0x22));
            pc = 0x60125c; continue;
            }
            0x601257 => {
            c.set(6,32,c.r[0].wrapping_add(0xffffffde));
            pc = 0x60125a; continue;
            }
            0x60125a => {
            c.set(7,32,c.get(5,32));
            pc = 0x60125c; continue;
            }
            0x60125c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(6,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x601286; continue; }
            pc = 0x601282; continue;
            }
            0x601282 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x601286; continue;
            }
            0x601286 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x6012d0; continue; }
            pc = 0x60128a; continue;
            }
            0x60128a => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x6012d0; continue; }
            pc = 0x60128e; continue;
            }
            0x60128e => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(7,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x6012bc; continue; }
            pc = 0x6012b8; continue;
            }
            0x6012b8 => {
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x6012bc; continue;
            }
            0x6012bc => {
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x6012d4; continue; }
            pc = 0x6012c0; continue;
            }
            0x6012c0 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x6012d4; continue; }
            pc = 0x6012c4; continue;
            }
            0x6012c4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6012d0 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6012d4; continue;
            }
            0x6012d4 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x601224; continue; }
            pc = 0x6012e6; continue;
            }
            0x6012e6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_apply_damage_to_defense_piece(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x60130b;
    loop {
        match pc {
            0x60130b => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1ec4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x6013ef; continue; }
            pc = 0x60132e; continue;
            }
            0x60132e => {
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1ec4); wm(m,a,32,c.get(5,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(2) { pc = 0x6015d0; continue; }
            pc = 0x60134d; continue;
            }
            0x60134d => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x6ddba4u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[1].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x6015d0; continue; }
            pc = 0x601369; continue;
            }
            0x601369 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6015d0; continue; }
            pc = 0x601374; continue;
            }
            0x601374 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x601378; continue;
            }
            0x601378 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(2,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x601385; continue; }
            pc = 0x601380; continue;
            }
            0x601380 => {
            { let a = c.get(2,8); c.sub(a,0x31u32,8); }
            if c.cond(5) { pc = 0x60139c; continue; }
            pc = 0x601385; continue;
            }
            0x601385 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x1eae),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0],8),8); }
            if c.cond(5) { pc = 0x60139c; continue; }
            pc = 0x601390; continue;
            }
            0x601390 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x1eb2),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x1),8),8); }
            if c.cond(4) { pc = 0x6013db; continue; }
            pc = 0x60139c; continue;
            }
            0x60139c => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0x8),16),16); }
            if c.cond(3) { pc = 0x6015d0; continue; }
            pc = 0x6013b1; continue;
            }
            0x6013b1 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x6015d0; continue; }
            pc = 0x6013ca; continue;
            }
            0x6013ca => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x601378; continue; }
            pc = 0x6013d4; continue;
            }
            0x6013d4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6013db => {
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6013e8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x515d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6013ef => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x60142b; continue; }
            pc = 0x601401; continue;
            }
            0x601401 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x768),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.push(m, 0x601425);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x601446; continue;
            }
            0x60142b => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x6ddba4u32);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x601444; continue; }
            pc = 0x60143f; continue;
            }
            0x60143f => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            pc = 0x601446; continue;
            }
            0x601444 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x601446; continue;
            }
            0x601446 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            pc = 0x60144b; continue;
            }
            0x60144b => {
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6014c2; continue; }
            pc = 0x60144f; continue;
            }
            0x60144f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x6015de; continue; }
            pc = 0x60145a; continue;
            }
            0x60145a => {
            { let a = c.get(0,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x6015de; continue; }
            pc = 0x601462; continue;
            }
            0x601462 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x601494; continue; }
            pc = 0x601473; continue;
            }
            0x601473 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.push(m, 0x601490);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            pc = 0x6014be; continue;
            }
            0x601494 => {
            c.set(2,32,c.r[1].wrapping_mul(4));
            c.set(1,32,0x6ddba4u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1],32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x6015d7; continue; }
            pc = 0x6014b8; continue;
            }
            0x6014b8 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x6014be; continue;
            }
            0x6014be => {
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x60144f; continue; }
            pc = 0x6014c2; continue;
            }
            0x6014c2 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            pc = 0x6014c7; continue;
            }
            0x6014c7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x60144b; continue; }
            pc = 0x6014d9; continue;
            }
            0x6014d9 => {
            c.set(2,16,rm(m,0x6e59ccu32,8));
            { let a = rm(m,c.r[7].wrapping_add(0x662),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x6015a8; continue; }
            pc = 0x6014ee; continue;
            }
            0x6014ee => {
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,0x78787879u32);
            c.push(m,0x400030u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.push(m,0x12u32);
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x60153b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xeeacu32);
            c.push(m, 0x60154b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6015a8; continue; }
            pc = 0x60154f; continue;
            }
            0x60154f => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xeeacu32);
            c.push(m, 0x60155f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x601566);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(2,32); let r = c.sub(a,0x1154u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x6015a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x6015a8; continue;
            }
            0x6015a8 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1eb6),16));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1e9e); wm(m,a,16,0xffffu32); }
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1ec4); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x6015c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x619560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0xac4); wm(m,a,8,0x1u32); }
            pc = 0x6015d0; continue;
            }
            0x6015d0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6015d7 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x6014c2; continue;
            }
            0x6015de => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x1eae),8));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x60165e; continue; }
            pc = 0x6015eb; continue;
            }
            0x6015eb => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x1eb2),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x60165e; continue; }
            pc = 0x6015f9; continue;
            }
            0x6015f9 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x601609);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x25u32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x4u32); }
            c.set(1,16,rm(m,c.r[6],8));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1e98),8));
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,0x1u32); }
            { let a = rm(m,0x6dfc18u32,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6014c7; continue; }
            pc = 0x601648; continue;
            }
            0x601648 => {
            c.set(2,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x601659);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d85c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6014c7; continue;
            }
            0x60165e => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x601692; continue; }
            pc = 0x60166f; continue;
            }
            0x60166f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x60168b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            pc = 0x6014c7; continue;
            }
            0x601692 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x6ddba4u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x6016b9; continue; }
            pc = 0x6016ae; continue;
            }
            0x6016ae => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x6014c7; continue;
            }
            0x6016b9 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x6014c7; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_order_selected_ships_attack_piece(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6016cb;
    loop {
        match pc {
            0x6016cb => {
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x6016f9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x6018be; continue; }
            pc = 0x60170c; continue;
            }
            0x60170c => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,0x1u32);
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x601723);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            if c.cond(3) { pc = 0x601755; continue; }
            pc = 0x601739; continue;
            }
            0x601739 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1ed8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1edc),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            pc = 0x601755; continue;
            }
            0x601755 => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            pc = 0x60175b; continue;
            }
            0x60175b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x1eae),8));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x601782; continue; }
            pc = 0x601772; continue;
            }
            0x601772 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x1eb2),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x60178d; continue; }
            pc = 0x601782; continue;
            }
            0x601782 => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x60175b; continue; }
            pc = 0x60178d; continue;
            }
            0x60178d => {
            { let a = c.get(2,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x6018c7; continue; }
            pc = 0x601796; continue;
            }
            0x601796 => {
            c.set(1,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x6017bb; continue; }
            pc = 0x60179f; continue;
            }
            0x60179f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6017a1; continue;
            }
            0x6017a1 => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x6017a1; continue; }
            pc = 0x6017bb; continue;
            }
            0x6017bb => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            if c.cond(6) { pc = 0x60181f; continue; }
            pc = 0x6017cb; continue;
            }
            0x6017cb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(6,32,0x1u32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(3,32); let r = c.or(a,c.get(6,32),32); c.set(3,32,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            c.set(1,8,c.get(2,8));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x6017cb; continue; }
            pc = 0x60181f; continue;
            }
            0x60181f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x6018be; continue; }
            pc = 0x60182d; continue;
            }
            0x60182d => {
            c.set(2,8,c.get(3,8));
            { let a = c.get(2,8); let r = c.add(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x60184d; continue; }
            pc = 0x601845; continue;
            }
            0x601845 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6018b0; continue; }
            pc = 0x60184d; continue;
            }
            0x60184d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x34u32);
            c.set(0,16,rm(m,0x6df346u32,16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x6018b0; continue; }
            pc = 0x601861; continue;
            }
            0x601861 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x60186a);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x662),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,0xfffeu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x93u32); }
            { let a = c.r[4].wrapping_add(0x29); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x6018b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6018b0; continue;
            }
            0x6018b0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x60182d; continue; }
            pc = 0x6018be; continue;
            }
            0x6018be => {
            c.push(m,c.get(7,32));
            c.push(m, 0x6018c4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x6018c7; continue;
            }
            0x6018c7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_update_interface(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6018db;
    loop {
        match pc {
            0x6018db => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(5,32,c.r[3].wrapping_add(0x1e9e));
            c.set(7,32,c.r[3].wrapping_add(0x1ec4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x4u32); }
            pc = 0x6018fc; continue;
            }
            0x6018fc => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x601c20; continue; }
            pc = 0x601906; continue;
            }
            0x601906 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); c.sub(a,0x5fb40u32,32); }
            if c.cond(15) { pc = 0x601b36; continue; }
            pc = 0x601915; continue;
            }
            0x601915 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x1024),32));
            c.set(6,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x601b08; continue; }
            pc = 0x601929; continue;
            }
            0x601929 => {
            { let a = c.r[7].wrapping_add(0x1034); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,0x677929u32,8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x400024u32);
            c.push(m,0x12u32);
            c.push(m, 0x601946);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x14u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffff6));
            c.set(0,32,0x78787879u32);
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(1,32));
            c.push(m, 0x601978);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x14u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffff6));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x601999);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,32,rm(m,c.r[7],32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xea0ea0ebu32);
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xc8));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x1024); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x1566464du32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,8); let r = c.add(a,0x19u32,8); c.set(2,8,r); }
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x670),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x671),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x40));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x601a52; continue; }
            pc = 0x601a36; continue;
            }
            0x601a36 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x601a44; continue; }
            pc = 0x601a3b; continue;
            }
            0x601a3b => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67a920),32));
            pc = 0x601a71; continue;
            }
            0x601a44 => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,0x67ab20u32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            pc = 0x601a71; continue;
            }
            0x601a52 => {
            c.set(6,32,c.r[0].wrapping_add(0xffffff80));
            { let a = c.get(6,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x601a63; continue; }
            pc = 0x601a5a; continue;
            }
            0x601a5a => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x67a920),32));
            pc = 0x601a6f; continue;
            }
            0x601a63 => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,0x67ad20u32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            pc = 0x601a6f; continue;
            }
            0x601a6f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x601a71; continue;
            }
            0x601a71 => {
            { let a = c.get(2,8); c.sub(a,0x80u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x601a96; continue; }
            pc = 0x601a7a; continue;
            }
            0x601a7a => {
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x601a88; continue; }
            pc = 0x601a7f; continue;
            }
            0x601a7f => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67a920),32));
            pc = 0x601ab9; continue;
            }
            0x601a88 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x67ab20u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x601ab9; continue;
            }
            0x601a96 => {
            c.set(0,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x601aa7; continue; }
            pc = 0x601a9e; continue;
            }
            0x601a9e => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67a920),32));
            pc = 0x601ab7; continue;
            }
            0x601aa7 => {
            c.set(0,32,c.r[1].wrapping_mul(4));
            c.set(1,32,0x67ad20u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1],32));
            pc = 0x601ab7; continue;
            }
            0x601ab7 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            pc = 0x601ab9; continue;
            }
            0x601ab9 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,0x10624dd3u32);
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x1f4u32,32); c.set(2,32,r); }
            c.set(1,32,0x1f4u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x601b36; continue;
            }
            0x601b08 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x1566464du32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0xc8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x601b36; continue; }
            pc = 0x601b30; continue;
            }
            0x601b30 => {
            { let a = c.r[7].wrapping_add(0x1024); wm(m,a,32,c.get(2,32)); }
            pc = 0x601b36; continue;
            }
            0x601b36 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); c.sub(a,0x2fda0u32,32); }
            if c.cond(13) { pc = 0x601c4a; continue; }
            pc = 0x601b47; continue;
            }
            0x601b47 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0x1566464du32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,8,0x2u32);
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1044),32));
            { let a = c.get(1,8); let r = c.sub(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x601bb8; continue; }
            pc = 0x601b73; continue;
            }
            0x601b73 => {
            c.set(2,32,rm(m,0x6e2e6cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x601b86; continue; }
            pc = 0x601b82; continue;
            }
            0x601b82 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x601b8f; continue;
            }
            0x601b86 => {
            c.set(2,32,rm(m,0x6e2e70u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x601b8f; continue;
            }
            0x601b8f => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x601bb8; continue; }
            pc = 0x601b9c; continue;
            }
            0x601b9c => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x601bae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x1044); wm(m,a,32,0x0u32); }
            pc = 0x601bb8; continue;
            }
            0x601bb8 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1044),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x601c4a; continue; }
            pc = 0x601bc6; continue;
            }
            0x601bc6 => {
            c.set(0,8,rm(m,0x677929u32,8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(6,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let r = c.add(a,0x4000b0u32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,0x12u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5],16));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x601c18);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[7].wrapping_add(0x1044); wm(m,a,32,c.get(0,32)); }
            pc = 0x601c4a; continue;
            }
            0x601c20 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1044),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x601c4a; continue; }
            pc = 0x601c2a; continue;
            }
            0x601c2a => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x601c40); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x1044); wm(m,a,32,0x0u32); }
            pc = 0x601c4a; continue;
            }
            0x601c4a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6018fc; continue; }
            pc = 0x601c5f; continue;
            }
            0x601c5f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,c.get(3,32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x60ed39, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_add_reinforcement(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x60234c;
    loop {
        match pc {
            0x60234c => {
            c.push(m,0xffffffffu32);
            c.push(m,0x668214u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0],8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x35a),16),16); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x8),16));
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x559),8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(1,8)); }
            if c.cond(3) { pc = 0x602459; continue; }
            pc = 0x6023c1; continue;
            }
            0x6023c1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x2f20),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2f20); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2f20),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2f1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6023e3);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x2f1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2f20),16));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(7,16)); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x602461; continue; }
            pc = 0x602422; continue;
            }
            0x602422 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            pc = 0x602430; continue;
            }
            0x602430 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[0],8));
            { let a = c.get(3,16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x60244a; continue; }
            pc = 0x602446; continue;
            }
            0x602446 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x60244a; continue;
            }
            0x60244a => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x602430; continue; }
            pc = 0x602457; continue;
            }
            0x602457 => {
            pc = 0x602461; continue;
            }
            0x602459 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x602461; continue;
            }
            0x602461 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x658),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x65a),16),16); c.set(1,16,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x60248c);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa74),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6024a4);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.r[6].wrapping_add(0xa74); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x6024bf);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xa78); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6024d7);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6024ec);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(3,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x65a),16),16); c.set(3,16,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x602652; continue; }
            pc = 0x60250d; continue;
            }
            0x60250d => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x602700; continue; }
            pc = 0x602531; continue;
            }
            0x602531 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,c.r[3].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x60253e; continue;
            }
            0x60253e => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x35a),16),16); }
            if c.cond(5) { pc = 0x60263b; continue; }
            pc = 0x602564; continue;
            }
            0x602564 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(5,32,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m, 0x60257e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa74),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(3,32));
            c.push(m,0x6u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60259a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            c.push(m,0x1168u32);
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x6025ae);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x6025fa; continue; }
            pc = 0x6025c3; continue;
            }
            0x6025c3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6025d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x61a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,0xffffu32);
            { let a = c.r[7]; wm(m,a,32,0x67aad0u32); }
            { let a = c.r[7].wrapping_add(0x1164); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x1166); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x127); wm(m,a,8,0x0u32); }
            pc = 0x6025fc; continue;
            }
            0x6025fa => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x6025fc; continue;
            }
            0x6025fc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[7].wrapping_add(0xfffffffe));
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(7,16)); }
            c.push(m, 0x602627);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6027cc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            pc = 0x60263b; continue;
            }
            0x60263b => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x60253e; continue; }
            pc = 0x60264d; continue;
            }
            0x60264d => {
            pc = 0x602700; continue;
            }
            0x602652 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(5,32,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m, 0x60266c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa74),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(3,32));
            c.push(m,0x6u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x602688);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            c.push(m,0x1168u32);
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x60269c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x6026e8; continue; }
            pc = 0x6026b1; continue;
            }
            0x6026b1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6026c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x61a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,0xffffu32);
            { let a = c.r[7]; wm(m,a,32,0x67aad0u32); }
            { let a = c.r[7].wrapping_add(0x1164); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x1166); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x127); wm(m,a,8,0x0u32); }
            pc = 0x6026ea; continue;
            }
            0x6026e8 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x6026ea; continue;
            }
            0x6026ea => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x602700);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6027cc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x602700; continue;
            }
            0x602700 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x38),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x60275b; continue; }
            pc = 0x60270b; continue;
            }
            0x60270b => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); let r = c.dec(a,16); c.set(0,16,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x65a),16),16); c.set(0,16,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,0xffffu32); }
            { let a = rm(m,c.r[6].wrapping_add(0x658),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x658); wm(m,a,16,r); } }
            pc = 0x6027b2; continue;
            }
            0x60275b => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.get(0,16); let r = c.dec(a,16); c.set(0,16,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x65a),16),16); c.set(0,16,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.r[6].wrapping_add(0x65c); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,0xffffu32); }
            { let a = rm(m,c.r[6].wrapping_add(0x65a),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x65a); wm(m,a,16,r); } }
            pc = 0x6027b2; continue;
            }
            0x6027b2 => {
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

pub fn town_attack_battle_place_reinforcement_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6027cc;
    loop {
        match pc {
            0x6027cc => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x1edc),32));
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x1ed8),32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m, 0x6027ed);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x6027f5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x602830; continue; }
            pc = 0x6027fc; continue;
            }
            0x6027fc => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x60280c; continue; }
            pc = 0x602800; continue;
            }
            0x602800 => {
            c.set(5,32,0x10ecu32);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0xc0u32); }
            pc = 0x602816; continue;
            }
            0x60280c => {
            c.set(5,32,0x14u32);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x40u32); }
            pc = 0x602816; continue;
            }
            0x602816 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x781u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x32u32); }
            pc = 0x602865; continue;
            }
            0x602830 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(5,32,0x880u32);
            if c.cond(13) { pc = 0x602848; continue; }
            pc = 0x602839; continue;
            }
            0x602839 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xeeeu32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x602855; continue;
            }
            0x602848 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x14u32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x80u32); }
            pc = 0x602855; continue;
            }
            0x602855 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x32u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            pc = 0x602865; continue;
            }
            0x602865 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x602869; continue;
            }
            0x602869 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            { let a = c.get(7,16); c.and(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x60293b; continue; }
            pc = 0x602879; continue;
            }
            0x602879 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xa78),32));
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x602886; continue;
            }
            0x602886 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(0,16); let r = c.add(a,c.get(0,16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x6028d3; continue; }
            pc = 0x6028a6; continue;
            }
            0x6028a6 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x6028d3; continue; }
            pc = 0x6028af; continue;
            }
            0x6028af => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x38),32));
            c.set(3,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16),16)&0xffffffffu32);
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            if c.cond(15) { pc = 0x6028d3; continue; }
            pc = 0x6028c0; continue;
            }
            0x6028c0 => {
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(12) { pc = 0x6028d3; continue; }
            pc = 0x6028cb; continue;
            }
            0x6028cb => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.set(2,32,c.r[7].wrapping_add(0xffffffff));
            pc = 0x6028d3; continue;
            }
            0x6028d3 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x602886; continue; }
            pc = 0x6028d9; continue;
            }
            0x6028d9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x60293b; continue; }
            pc = 0x6028e1; continue;
            }
            0x6028e1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            if c.cond(14) { pc = 0x602900; continue; }
            pc = 0x6028f9; continue;
            }
            0x6028f9 => {
            c.set(2,32,0xffffffceu32);
            pc = 0x602907; continue;
            }
            0x602900 => {
            if c.cond(13) { pc = 0x60290f; continue; }
            pc = 0x602902; continue;
            }
            0x602902 => {
            c.set(2,32,0x32u32);
            pc = 0x602907; continue;
            }
            0x602907 => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x60290f; continue;
            }
            0x60290f => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x602923; continue; }
            pc = 0x602913; continue;
            }
            0x602913 => {
            c.set(0,32,0xffffffceu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x602869; continue;
            }
            0x602923 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x602869; continue; }
            pc = 0x60292b; continue;
            }
            0x60292b => {
            c.set(2,32,0x32u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x602869; continue;
            }
            0x60293b => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            { let a = c.r[2].wrapping_add(0xa); wm(m,a,16,c.get(5,16)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x8),16));
            { let a = c.r[2].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x7),8));
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0]),32));
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_is_land_attack(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x602980;
    loop {
        match pc {
            0x602980 => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_attack_battle_delete_this(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x602990;
    loop {
        match pc {
            0x602990 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x602998);
            { let args = []; c.r[0] = host.call(m, 0x5fb9f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x6029a8; continue; }
            pc = 0x60299f; continue;
            }
            0x60299f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x6029a5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x6029a8; continue;
            }
            0x6029a8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_projectile_serialized_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x602e10;
    loop {
        match pc {
            0x602e10 => {
            c.set(0,32,0x1fu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_projectile_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x602e20;
    loop {
        match pc {
            0x602e20 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e31);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x2));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e40);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e4f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x6));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e5e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e6d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xa));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0xc));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e8e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x10));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602e9d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x14));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602eac);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x18));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602ebb);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x1c));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602ec8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x24u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x602ed7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x2));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_projectile_update_sprite(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6038c0;
    loop {
        match pc {
            0x6038c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,sx(rm(m,c.r[6],16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x6),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x603903);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x60390b);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(0x32),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x9u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x6fu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x3159721fu32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(5,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x3159721fu32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x78787879u32);
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x603a0f; continue; }
            pc = 0x60399b; continue;
            }
            0x60399b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x6039c0; continue; }
            pc = 0x6039ae; continue;
            }
            0x6039ae => {
            { let a = c.get(0,32); let r = c.add(a,0xffffff80u32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            pc = 0x6039d4; continue;
            }
            0x6039c0 => {
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x6039d4; continue;
            }
            0x6039d4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x30),8));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x34),16));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0xeu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x603a04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x603a0f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x603a32; continue; }
            pc = 0x603a20; continue;
            }
            0x603a20 => {
            { let a = c.get(0,32); let r = c.add(a,0xffffff80u32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            pc = 0x603a46; continue;
            }
            0x603a32 => {
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x603a46; continue;
            }
            0x603a46 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x34),16));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x30),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x603a66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x568840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_order_selected_ships_attack_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x608360;
    loop {
        match pc {
            0x608360 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x60838e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x608522; continue; }
            pc = 0x6083a1; continue;
            }
            0x6083a1 => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x6083ec; continue; }
            pc = 0x6083ad; continue;
            }
            0x6083ad => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x30),16));
            pc = 0x6083b5; continue;
            }
            0x6083b5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x134),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6083d7; continue; }
            pc = 0x6083cd; continue;
            }
            0x6083cd => {
            { let a = rm(m,c.r[0].wrapping_add(0x16),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x608535; continue; }
            pc = 0x6083d7; continue;
            }
            0x6083d7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x14),16),16); }
            if c.cond(2) { pc = 0x6083b5; continue; }
            pc = 0x6083ec; continue;
            }
            0x6083ec => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            pc = 0x6083f4; continue;
            }
            0x6083f4 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x608412; continue; }
            pc = 0x6083fd; continue;
            }
            0x6083fd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6083ff; continue;
            }
            0x6083ff => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x6083ff; continue; }
            pc = 0x608412; continue;
            }
            0x608412 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x60848a; continue; }
            pc = 0x60841d; continue;
            }
            0x60841d => {
            c.set(3,8,0x8u32);
            pc = 0x60841f; continue;
            }
            0x60841f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(6,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x608480; continue; }
            pc = 0x60844b; continue;
            }
            0x60844b => {
            { let a = rm(m,c.r[0].wrapping_add(0x12a),8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x608480; continue; }
            pc = 0x608453; continue;
            }
            0x608453 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x608480; continue;
            }
            0x608480 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            if c.cond(2) { pc = 0x60841f; continue; }
            pc = 0x60848a; continue;
            }
            0x60848a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x608522; continue; }
            pc = 0x608498; continue;
            }
            0x608498 => {
            c.set(6,16,c.get(3,8));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6084b2; continue; }
            pc = 0x6084aa; continue;
            }
            0x6084aa => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x608514; continue; }
            pc = 0x6084b2; continue;
            }
            0x6084b2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x608514; continue; }
            pc = 0x6084c7; continue;
            }
            0x6084c7 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x6084d0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x662),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x93u32); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x608514);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x608514; continue;
            }
            0x608514 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x608498; continue; }
            pc = 0x608522; continue;
            }
            0x608522 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x608528);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x608535 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x6083f4; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_order_selected_ships_move_to(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x608550;
    loop {
        match pc {
            0x608550 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x60857e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(3,32,c.get(0,32));
            if c.cond(4) { pc = 0x608765; continue; }
            pc = 0x608591; continue;
            }
            0x608591 => {
            c.set(2,32,c.r[7].wrapping_add(c.r[7]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x6085af; continue; }
            pc = 0x60859a; continue;
            }
            0x60859a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x60859c; continue;
            }
            0x60859c => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x60859c; continue; }
            pc = 0x6085af; continue;
            }
            0x6085af => {
            { let a = rm(m,c.r[6].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x6086b2; continue; }
            pc = 0x6085c9; continue;
            }
            0x6085c9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x60869c; continue; }
            pc = 0x6085fb; continue;
            }
            0x6085fb => {
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0xae0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0xae4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x12d8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.r[6].wrapping_add(0x12dc); wm(m,a,32,c.get(2,32)); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x38),8));
            { let a = c.r[6].wrapping_add(0x12e0); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,0x1u32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x60869c; continue;
            }
            0x60869c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xad0),16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x6085c9; continue; }
            pc = 0x6086b2; continue;
            }
            0x6086b2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x608765; continue; }
            pc = 0x6086be; continue;
            }
            0x6086be => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(0,8)); }
            if c.cond(6) { pc = 0x608765; continue; }
            pc = 0x6086d0; continue;
            }
            0x6086d0 => {
            c.set(5,16,c.get(0,8));
            { let a = c.get(5,32); let r = c.add(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6086ea; continue; }
            pc = 0x6086e2; continue;
            }
            0x6086e2 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x608753; continue; }
            pc = 0x6086ea; continue;
            }
            0x6086ea => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x608753; continue; }
            pc = 0x6086ff; continue;
            }
            0x6086ff => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x608708);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x34),8));
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x662),8));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x30),16));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x95u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(7,16)); }
            c.push(m, 0x60874f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x34),8));
            pc = 0x608753; continue;
            }
            0x608753 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x6086d0; continue; }
            pc = 0x608765; continue;
            }
            0x608765 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x60876b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn sea_battle_find_local_ship_by_ship_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6087f0;
    loop {
        match pc {
            0x6087f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x14),16));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(6,16); c.and(a,c.get(6,16),16); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x608834; continue; }
            pc = 0x608803; continue;
            }
            0x608803 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            pc = 0x608810; continue;
            }
            0x608810 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = rm(m,c.r[7].wrapping_add(0x4),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x60883d; continue; }
            pc = 0x608824; continue;
            }
            0x608824 => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,16); c.sub(a,c.get(6,16),16); }
            if c.cond(2) { pc = 0x608810; continue; }
            pc = 0x608834; continue;
            }
            0x608834 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x60883d => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_generate_random_obstacles(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x60f479;
    loop {
        match pc {
            0x60f479 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x510),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            { let a = c.get(0,16); let n = 0x1u32; let r = c.sar(a,n,16); c.set(0,16,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xa205b063u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f4bb; continue; }
            pc = 0x60f4b2; continue;
            }
            0x60f4b2 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f4bd; continue;
            }
            0x60f4bb => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f4bd; continue;
            }
            0x60f4bd => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            c.set(0,32,0xa205b063u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f4e7; continue; }
            pc = 0x60f4de; continue;
            }
            0x60f4de => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f4e9; continue;
            }
            0x60f4e7 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f4e9; continue;
            }
            0x60f4e9 => {
            c.set(2,32,c.get(0,32));
            c.set(1,32,0xa205b063u32);
            { let a = c.get(2,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x60f513; continue; }
            pc = 0x60f509; continue;
            }
            0x60f509 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0x80000000u32,32); c.set(1,32,r); }
            pc = 0x60f515; continue;
            }
            0x60f513 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            pc = 0x60f515; continue;
            }
            0x60f515 => {
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x41u32);
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,c.get(7,32));
            c.set(7,32,0x41u32);
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,0x60u32,16); }
            if c.cond(13) { pc = 0x60f550; continue; }
            pc = 0x60f545; continue;
            }
            0x60f545 => {
            c.set(2,32,0x40u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            pc = 0x60f556; continue;
            }
            0x60f550 => {
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            pc = 0x60f556; continue;
            }
            0x60f556 => {
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x60f565; continue; }
            pc = 0x60f55f; continue;
            }
            0x60f55f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x60f565; continue;
            }
            0x60f565 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x60f572; continue;
            }
            0x60f572 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,16); c.sub(a,c.get(1,16),16); }
            if c.cond(13) { pc = 0x60f98e; continue; }
            pc = 0x60f57f; continue;
            }
            0x60f57f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            c.set(0,32,0xa205b063u32);
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f5a5; continue; }
            pc = 0x60f59c; continue;
            }
            0x60f59c => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f5a7; continue;
            }
            0x60f5a5 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f5a7; continue;
            }
            0x60f5a7 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x5u32);
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.get(0,32));
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(5,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x67acfc),16));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x67acfe),16));
            c.set(0,32,0xa205b063u32);
            { let a = c.get(1,16); let r = c.dec(a,16); c.set(1,16,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); let r = c.dec(a,16); c.set(7,16,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f5fa; continue; }
            pc = 0x60f5f1; continue;
            }
            0x60f5f1 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f5fc; continue;
            }
            0x60f5fa => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f5fc; continue;
            }
            0x60f5fc => {
            c.set(5,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,32,sx(c.get(3,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(3,32,0x100u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            c.set(0,32,0xa205b063u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(2,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f66d; continue; }
            pc = 0x60f664; continue;
            }
            0x60f664 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f66f; continue;
            }
            0x60f66d => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f66f; continue;
            }
            0x60f66f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,0x1c4u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            c.set(0,32,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x876));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x874); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x878); wm(m,a,16,c.get(7,16)); }
            if c.cond(14) { pc = 0x60f73d; continue; }
            pc = 0x60f6ca; continue;
            }
            0x60f6ca => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,sx(c.get(2,16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x874));
            pc = 0x60f6ee; continue;
            }
            0x60f6ee => {
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(2,8,0x1u32);
            c.set(3,32,c.r[7].wrapping_add(0x8));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x60f705; continue; }
            pc = 0x60f703; continue;
            }
            0x60f703 => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            pc = 0x60f705; continue;
            }
            0x60f705 => {
            c.set(7,32,sx(rm(m,c.r[1],16),16)&0xffffffffu32);
            c.set(3,32,c.r[7].wrapping_add(0x10));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x874),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x60f719; continue; }
            pc = 0x60f717; continue;
            }
            0x60f717 => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            pc = 0x60f719; continue;
            }
            0x60f719 => {
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x28),32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x60f725; continue; }
            pc = 0x60f723; continue;
            }
            0x60f723 => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            pc = 0x60f725; continue;
            }
            0x60f725 => {
            c.set(7,32,sx(rm(m,c.r[1],16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x60f737; continue; }
            pc = 0x60f72e; continue;
            }
            0x60f72e => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x60f737; continue; }
            pc = 0x60f732; continue;
            }
            0x60f732 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x60f737; continue;
            }
            0x60f737 => {
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x60f6ee; continue; }
            pc = 0x60f73d; continue;
            }
            0x60f73d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x60f84a; continue; }
            pc = 0x60f757; continue;
            }
            0x60f757 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x60f75f; continue;
            }
            0x60f75f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x1u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(0,16); let r = c.add(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(c.r[1]),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[1]),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,sx(rm(m,c.r[3].wrapping_add(c.r[0]),16),16)&0xffffffffu32);
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0]),16));
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x60f7bf; continue; }
            pc = 0x60f7ba; continue;
            }
            0x60f7ba => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60f7bf; continue;
            }
            0x60f7bf => {
            c.set(1,32,sx(c.get(1,16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x874),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x60f7e4; continue; }
            pc = 0x60f7df; continue;
            }
            0x60f7df => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60f7e4; continue;
            }
            0x60f7e4 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x34),16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x60f80b; continue; }
            pc = 0x60f806; continue;
            }
            0x60f806 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60f80b; continue;
            }
            0x60f80b => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x38),16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x60f832; continue; }
            pc = 0x60f825; continue;
            }
            0x60f825 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x60f832; continue; }
            pc = 0x60f82d; continue;
            }
            0x60f82d => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x60f832; continue;
            }
            0x60f832 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x60f75f; continue; }
            pc = 0x60f84a; continue;
            }
            0x60f84a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x60f96c; continue; }
            pc = 0x60f856; continue;
            }
            0x60f856 => {
            c.set(1,16,rm(m,0x6e59ccu32,8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x662),16),16); }
            if c.cond(5) { pc = 0x60f890; continue; }
            pc = 0x60f867; continue;
            }
            0x60f867 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x24),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x2c),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x874),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x60f890);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5944e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x60f890; continue;
            }
            0x60f890 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x874),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x872),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(2,16); let r = c.add(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(8)).wrapping_add(0x878),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(5,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            c.set(7,16,rm(m,c.r[7],16));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60f8ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28),32));
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2),16));
            c.set(1,16,rm(m,c.r[1],16));
            { let a = c.get(2,16); let r = c.add(a,0xc8u32,16); c.set(2,16,r); }
            { let a = c.get(1,16); let r = c.sub(a,0xc8u32,16); c.set(1,16,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.get(2,16); let r = c.add(a,0xc8u32,16); c.set(2,16,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(2,16); let r = c.sub(a,0xc8u32,16); c.set(2,16,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60f92a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x60f92f; continue;
            }
            0x60f92f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x60f933; continue;
            }
            0x60f933 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x60f572; continue; }
            pc = 0x60f946; continue;
            }
            0x60f946 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            c.set(0,32,0xa205b063u32);
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f99d; continue; }
            pc = 0x60f963; continue;
            }
            0x60f963 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f99f; continue;
            }
            0x60f96c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x60f92f; continue; }
            pc = 0x60f983; continue;
            }
            0x60f983 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x60f933; continue;
            }
            0x60f98e => {
            c.set(0,32,sx(c.get(3,16),16)&0xffffffffu32);
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x872); wm(m,a,16,0xffffu32); }
            pc = 0x60f933; continue;
            }
            0x60f99d => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f99f; continue;
            }
            0x60f99f => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            c.set(0,32,0xa205b063u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60f9c9; continue; }
            pc = 0x60f9c0; continue;
            }
            0x60f9c0 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60f9cb; continue;
            }
            0x60f9c9 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60f9cb; continue;
            }
            0x60f9cb => {
            c.set(2,32,c.get(0,32));
            c.set(1,32,0xa205b063u32);
            { let a = c.get(2,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x60f9f5; continue; }
            pc = 0x60f9eb; continue;
            }
            0x60f9eb => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0x80000000u32,32); c.set(1,32,r); }
            pc = 0x60f9f7; continue;
            }
            0x60f9f5 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            pc = 0x60f9f7; continue;
            }
            0x60f9f7 => {
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x19u32);
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,c.get(7,32));
            c.set(7,32,0x19u32);
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,0x24u32,16); }
            if c.cond(13) { pc = 0x60fa32; continue; }
            pc = 0x60fa27; continue;
            }
            0x60fa27 => {
            c.set(2,32,0x30u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            pc = 0x60fa35; continue;
            }
            0x60fa32 => {
            { let a = c.get(1,32); let r = c.sub(a,0x18u32,32); c.set(1,32,r); }
            pc = 0x60fa35; continue;
            }
            0x60fa35 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x60fa44; continue;
            }
            0x60fa44 => {
            { let a = c.get(5,16); c.sub(a,c.get(1,16),16); }
            if c.cond(13) { pc = 0x60feb7; continue; }
            pc = 0x60fa4d; continue;
            }
            0x60fa4d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            c.set(0,32,0xa205b063u32);
            { let a = c.get(1,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60fa73; continue; }
            pc = 0x60fa6a; continue;
            }
            0x60fa6a => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60fa75; continue;
            }
            0x60fa73 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60fa75; continue;
            }
            0x60fa75 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x3u32);
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,c.get(0,32));
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(3,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x67ad20),16));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x67ad22),16));
            c.set(0,32,0xa205b063u32);
            { let a = c.get(1,16); let r = c.dec(a,16); c.set(1,16,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); let r = c.dec(a,16); c.set(7,16,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60fac8; continue; }
            pc = 0x60fabf; continue;
            }
            0x60fabf => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60faca; continue;
            }
            0x60fac8 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60faca; continue;
            }
            0x60faca => {
            c.set(3,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,32,sx(c.get(5,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(5,32,0x100u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(5,32); c.div32(b); }
            c.set(0,32,0xa205b063u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let a = c.get(2,32); let r = c.imul(a,0x44bf19b3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x60fb3b; continue; }
            pc = 0x60fb32; continue;
            }
            0x60fb32 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0x80000000u32,32); c.set(0,32,r); }
            pc = 0x60fb3d; continue;
            }
            0x60fb3b => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            pc = 0x60fb3d; continue;
            }
            0x60fb3d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,0x1c4u32);
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xadc),32));
            { let b = c.get(5,32); c.div32(b); }
            c.set(0,32,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x676));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x40u32); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x678); wm(m,a,16,c.get(7,16)); }
            c.set(7,32,c.r[6].wrapping_add(0x872));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            pc = 0x60fba1; continue;
            }
            0x60fba1 => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(14) { pc = 0x60fbf5; continue; }
            pc = 0x60fba9; continue;
            }
            0x60fba9 => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(1,8,0x1u32);
            c.set(5,32,c.r[0].wrapping_add(0x8));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x60fbbf; continue; }
            pc = 0x60fbbd; continue;
            }
            0x60fbbd => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fbbf; continue;
            }
            0x60fbbf => {
            c.set(7,32,sx(rm(m,c.r[7].wrapping_add(0x2),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x60fbd4; continue; }
            pc = 0x60fbd2; continue;
            }
            0x60fbd2 => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fbd4; continue;
            }
            0x60fbd4 => {
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x60fbdd; continue; }
            pc = 0x60fbdb; continue;
            }
            0x60fbdb => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fbdd; continue;
            }
            0x60fbdd => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.add(a,0x10u32,32); c.set(5,32,r); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x60fbf5; continue; }
            pc = 0x60fbec; continue;
            }
            0x60fbec => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x60fbf5; continue; }
            pc = 0x60fbf0; continue;
            }
            0x60fbf0 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x60fbf5; continue;
            }
            0x60fbf5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x60fba1; continue; }
            pc = 0x60fc07; continue;
            }
            0x60fc07 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(14) { pc = 0x60fc89; continue; }
            pc = 0x60fc10; continue;
            }
            0x60fc10 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x674));
            pc = 0x60fc3a; continue;
            }
            0x60fc3a => {
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(1,8,0x1u32);
            c.set(5,32,c.r[7].wrapping_add(0x8));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x60fc51; continue; }
            pc = 0x60fc4f; continue;
            }
            0x60fc4f => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fc51; continue;
            }
            0x60fc51 => {
            c.set(7,32,sx(rm(m,c.r[0],16),16)&0xffffffffu32);
            c.set(5,32,c.r[7].wrapping_add(0x10));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x60fc65; continue; }
            pc = 0x60fc63; continue;
            }
            0x60fc63 => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fc65; continue;
            }
            0x60fc65 => {
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x3c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x60fc71; continue; }
            pc = 0x60fc6f; continue;
            }
            0x60fc6f => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x60fc71; continue;
            }
            0x60fc71 => {
            c.set(7,32,sx(rm(m,c.r[0],16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x38),32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x60fc83; continue; }
            pc = 0x60fc7a; continue;
            }
            0x60fc7a => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x60fc83; continue; }
            pc = 0x60fc7e; continue;
            }
            0x60fc7e => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x60fc83; continue;
            }
            0x60fc83 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x60fc3a; continue; }
            pc = 0x60fc89; continue;
            }
            0x60fc89 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x60fd96; continue; }
            pc = 0x60fca3; continue;
            }
            0x60fca3 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x60fcab; continue;
            }
            0x60fcab => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa78),32));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x1u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(0,16); let r = c.add(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[5]),16));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[7]),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(c.r[5]),16),16)&0xffffffffu32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7]),16));
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x60fd0b; continue; }
            pc = 0x60fd06; continue;
            }
            0x60fd06 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60fd0b; continue;
            }
            0x60fd0b => {
            c.set(1,32,sx(c.get(1,16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x60fd30; continue; }
            pc = 0x60fd2b; continue;
            }
            0x60fd2b => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60fd30; continue;
            }
            0x60fd30 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x3c),16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x60fd57; continue; }
            pc = 0x60fd52; continue;
            }
            0x60fd52 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x60fd57; continue;
            }
            0x60fd57 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x38),16),16)&0xffffffffu32);
            c.set(0,32,0x78787879u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x60fd7e; continue; }
            pc = 0x60fd71; continue;
            }
            0x60fd71 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x60fd7e; continue; }
            pc = 0x60fd79; continue;
            }
            0x60fd79 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x60fd7e; continue;
            }
            0x60fd7e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x60fcab; continue; }
            pc = 0x60fd96; continue;
            }
            0x60fd96 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x60fe95; continue; }
            pc = 0x60fda2; continue;
            }
            0x60fda2 => {
            c.set(1,16,rm(m,0x6e59ccu32,8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x662),16),16); }
            if c.cond(5) { pc = 0x60fddc; continue; }
            pc = 0x60fdb3; continue;
            }
            0x60fdb3 => {
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x24),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[4].wrapping_add(0x2c),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x60fddc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x594640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x60fddc; continue;
            }
            0x60fddc => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x674),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x672),16));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,16); let r = c.add(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x678),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(5,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            c.set(7,16,rm(m,c.r[7],16));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60fe36);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28),32));
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2),16));
            c.set(1,16,rm(m,c.r[1],16));
            { let a = c.get(2,16); let r = c.add(a,0xc8u32,16); c.set(2,16,r); }
            { let a = c.get(1,16); let r = c.sub(a,0xc8u32,16); c.set(1,16,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.get(2,16); let r = c.add(a,0xc8u32,16); c.set(2,16,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(2,16); let r = c.sub(a,0xc8u32,16); c.set(2,16,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,0x5u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x60fe76);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x60fe76; continue;
            }
            0x60fe76 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x60fe7a; continue;
            }
            0x60fe7a => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x60fe7e; continue;
            }
            0x60fe7e => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,0x40u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x60fa44; continue; }
            pc = 0x60fe8d; continue;
            }
            0x60fe8d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x60fe95 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x60fe76; continue; }
            pc = 0x60feac; continue;
            }
            0x60feac => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x60fe7a; continue;
            }
            0x60feb7 => {
            c.set(0,32,sx(c.get(5,16),16)&0xffffffffu32);
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x672); wm(m,a,16,0xffffu32); }
            pc = 0x60fe7e; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_leave_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6100d9;
    loop {
        match pc {
            0x6100d9 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x6100e9);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12ea),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x610121; continue; }
            pc = 0x6100f5; continue;
            }
            0x6100f5 => {
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x610109); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x61011b; continue; }
            pc = 0x61010d; continue;
            }
            0x61010d => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x61011b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x61011b; continue;
            }
            0x61011b => {
            { let a = 0x6e59cdu32; wm(m,a,8,c.get(3,8)); }
            pc = 0x610121; continue;
            }
            0x610121 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = 0x6e59ccu32; wm(m,a,8,0xffu32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x662),16));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x96u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.push(m, 0x610160);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x6101b1; continue; }
            pc = 0x61016b; continue;
            }
            0x61016b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x134),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x610197; continue; }
            pc = 0x610181; continue;
            }
            0x610181 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x610197); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610197; continue;
            }
            0x610197 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(0x134); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x61016b; continue; }
            pc = 0x6101b1; continue;
            }
            0x6101b1 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x610205; continue; }
            pc = 0x6101bf; continue;
            }
            0x6101bf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x134),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x6101eb; continue; }
            pc = 0x6101d5; continue;
            }
            0x6101d5 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x6101eb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6101eb; continue;
            }
            0x6101eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(0x134); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x6101bf; continue; }
            pc = 0x610205; continue;
            }
            0x610205 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xacc),16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x610257; continue; }
            pc = 0x610210; continue;
            }
            0x610210 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x668),32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x61024d; continue; }
            pc = 0x610228; continue;
            }
            0x610228 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x610241; continue; }
            pc = 0x61022f; continue;
            }
            0x61022f => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x610241); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610241; continue;
            }
            0x610241 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x668),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            pc = 0x61024d; continue;
            }
            0x61024d => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xacc),16),16); }
            if c.cond(2) { pc = 0x610210; continue; }
            pc = 0x610257; continue;
            }
            0x610257 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x610295; continue; }
            pc = 0x610263; continue;
            }
            0x610263 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x61026d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x61027a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x610289); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x610295); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610295; continue;
            }
            0x610295 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_restamp_obstacles(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610379;
    loop {
        match pc {
            0x610379 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[1].wrapping_add(0x876));
            c.set(7,32,0x40u32);
            pc = 0x610386; continue;
            }
            0x610386 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xfffffffc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(12) { pc = 0x6103ac; continue; }
            pc = 0x61038f; continue;
            }
            0x61038f => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x2),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6],16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6103ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5944e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x6103ac; continue;
            }
            0x6103ac => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xfffffdfc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(12) { pc = 0x6103df; continue; }
            pc = 0x6103b8; continue;
            }
            0x6103b8 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xfffffe02),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xfffffe00),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xfffffdfe),16),16)&0xffffffffu32);
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6103df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x594640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x6103df; continue;
            }
            0x6103df => {
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x610386; continue; }
            pc = 0x6103e5; continue;
            }
            0x6103e5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_send_order_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6103e9;
    loop {
        match pc {
            0x6103e9 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x610417);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x610564; continue; }
            pc = 0x61042a; continue;
            }
            0x61042a => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x610448; continue; }
            pc = 0x610433; continue;
            }
            0x610433 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x610435; continue;
            }
            0x610435 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x610435; continue; }
            pc = 0x610448; continue;
            }
            0x610448 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x6104b7; continue; }
            pc = 0x610453; continue;
            }
            0x610453 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(6,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x6104ad; continue; }
            pc = 0x610480; continue;
            }
            0x610480 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x6104ad; continue;
            }
            0x6104ad => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            if c.cond(2) { pc = 0x610453; continue; }
            pc = 0x6104b7; continue;
            }
            0x6104b7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x610564; continue; }
            pc = 0x6104c5; continue;
            }
            0x6104c5 => {
            c.set(6,16,c.get(3,8));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6104df; continue; }
            pc = 0x6104d7; continue;
            }
            0x6104d7 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x610556; continue; }
            pc = 0x6104df; continue;
            }
            0x6104df => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x610556; continue; }
            pc = 0x6104f4; continue;
            }
            0x6104f4 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x6104fd);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x662),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x97u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(3,8)); }
            if c.cond(5) { pc = 0x610527; continue; }
            pc = 0x61051b; continue;
            }
            0x61051b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x610527; continue; }
            pc = 0x610523; continue;
            }
            0x610523 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x61052c; continue;
            }
            0x610527 => {
            c.set(0,32,0x1u32);
            pc = 0x61052c; continue;
            }
            0x61052c => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x34),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x610556);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610556; continue;
            }
            0x610556 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x6104c5; continue; }
            pc = 0x610564; continue;
            }
            0x610564 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x61056a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn sea_battle_send_sail_level(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610579;
    loop {
        match pc {
            0x610579 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x6105a6);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x6106ee; continue; }
            pc = 0x6105b9; continue;
            }
            0x6105b9 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x6105d7; continue; }
            pc = 0x6105c2; continue;
            }
            0x6105c2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6105c4; continue;
            }
            0x6105c4 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x6105c4; continue; }
            pc = 0x6105d7; continue;
            }
            0x6105d7 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x610646; continue; }
            pc = 0x6105e2; continue;
            }
            0x6105e2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(6,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x61063c; continue; }
            pc = 0x61060f; continue;
            }
            0x61060f => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x61063c; continue;
            }
            0x61063c => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            if c.cond(2) { pc = 0x6105e2; continue; }
            pc = 0x610646; continue;
            }
            0x610646 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.push(m,c.get(3,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x6106cf; continue; }
            pc = 0x610651; continue;
            }
            0x610651 => {
            c.set(6,16,c.get(3,8));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x61066b; continue; }
            pc = 0x610663; continue;
            }
            0x610663 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6106c5; continue; }
            pc = 0x61066b; continue;
            }
            0x61066b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x6106c5; continue; }
            pc = 0x610680; continue;
            }
            0x610680 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x610689);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x662),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x98u32); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x6106c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6106c5; continue;
            }
            0x6106c5 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x610651; continue; }
            pc = 0x6106cf; continue;
            }
            0x6106cf => {
            c.set(0,32,rm(m,0x6ce6d0u32,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6106ee; continue; }
            pc = 0x6106d9; continue;
            }
            0x6106d9 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.r[0].wrapping_add(0x5829); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x6106ee);
            { let args = []; c.r[0] = host.call(m, 0x494580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6106ee; continue;
            }
            0x6106ee => {
            c.push(m,c.get(7,32));
            c.push(m, 0x6106f4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_draw_drift_sprites(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610709;
    loop {
        match pc {
            0x610709 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(3,32,c.r[1].wrapping_add(0x1b10));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(7,32,c.get(3,32));
            c.set(5,32,0xe0u32);
            pc = 0x61071c; continue;
            }
            0x61071c => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x80000007u32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x61072a; continue; }
            pc = 0x610725; continue;
            }
            0x610725 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x61072a; continue;
            }
            0x61072a => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,8,0x32u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x610746);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x610756; continue; }
            pc = 0x61074f; continue;
            }
            0x61074f => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xc),32); c.push(m, 0x610756); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610756; continue;
            }
            0x610756 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x61071c; continue; }
            pc = 0x61075d; continue;
            }
            0x61075d => {
            c.set(7,32,c.get(3,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,32,0xe0u32);
            pc = 0x610766; continue;
            }
            0x610766 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x80000007u32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x610774; continue; }
            pc = 0x61076f; continue;
            }
            0x61076f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x610774; continue;
            }
            0x610774 => {
            { let a = c.get(0,8); let n = 0x2u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            c.set(1,8,0x80u32);
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xffffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x610790);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6107a0; continue; }
            pc = 0x610799; continue;
            }
            0x610799 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x6107a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6107a0; continue;
            }
            0x6107a0 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x610766; continue; }
            pc = 0x6107a7; continue;
            }
            0x6107a7 => {
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

pub fn sea_battle_spawn_drift_sprites(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6107b9;
    loop {
        match pc {
            0x6107b9 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x6107ca); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1b10));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x610980; continue; }
            pc = 0x6107da; continue;
            }
            0x6107da => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x38u32); }
            pc = 0x6107e2; continue;
            }
            0x6107e2 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x610966; continue; }
            pc = 0x6107eb; continue;
            }
            0x6107eb => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x670),8));
            { let a = c.get(1,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x610822; continue; }
            pc = 0x6107f6; continue;
            }
            0x6107f6 => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x61080a; continue; }
            pc = 0x6107fb; continue;
            }
            0x6107fb => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x61084d; continue;
            }
            0x61080a => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,0x67adb0u32);
            c.set(2,32,c.r[1].wrapping_mul(4));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[0],32));
            pc = 0x61084d; continue;
            }
            0x610822 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x610839; continue; }
            pc = 0x610830; continue;
            }
            0x610830 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x610849; continue;
            }
            0x610839 => {
            c.set(2,32,c.r[1].wrapping_mul(4));
            c.set(0,32,0x67afb0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x610849; continue;
            }
            0x610849 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x61084d; continue;
            }
            0x61084d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x610896; continue; }
            pc = 0x61087a; continue;
            }
            0x61087a => {
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x610888; continue; }
            pc = 0x61087f; continue;
            }
            0x61087f => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x6108b5; continue;
            }
            0x610888 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x67adb0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x6108b5; continue;
            }
            0x610896 => {
            c.set(2,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(2,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x6108a7; continue; }
            pc = 0x61089e; continue;
            }
            0x61089e => {
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x6108b3; continue;
            }
            0x6108a7 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x67afb0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x6108b3; continue;
            }
            0x6108b3 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            pc = 0x6108b5; continue;
            }
            0x6108b5 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffff0000u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,0x14u32);
            c.push(m, 0x6108ec);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x119u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8d));
            c.push(m,c.get(1,32));
            c.push(m, 0x610918);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x116u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0x80000007u32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x61092e; continue; }
            pc = 0x610929; continue;
            }
            0x610929 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x61092e; continue;
            }
            0x61092e => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8b));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x610952);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x34); wm(m,a,16,0x11c8u32); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[0].wrapping_add(0x36); wm(m,a,16,0xfcau32); }
            pc = 0x610966; continue;
            }
            0x610966 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6107e2; continue; }
            pc = 0x610979; continue;
            }
            0x610979 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x610980 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xe0u32); }
            pc = 0x610988; continue;
            }
            0x610988 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x610b0c; continue; }
            pc = 0x610991; continue;
            }
            0x610991 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x670),8));
            { let a = c.get(1,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x6109c8; continue; }
            pc = 0x61099c; continue;
            }
            0x61099c => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x6109b0; continue; }
            pc = 0x6109a1; continue;
            }
            0x6109a1 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x6109f3; continue;
            }
            0x6109b0 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,0x67adb0u32);
            c.set(2,32,c.r[1].wrapping_mul(4));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[0],32));
            pc = 0x6109f3; continue;
            }
            0x6109c8 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x6109df; continue; }
            pc = 0x6109d6; continue;
            }
            0x6109d6 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x6109ef; continue;
            }
            0x6109df => {
            c.set(2,32,c.r[1].wrapping_mul(4));
            c.set(0,32,0x67afb0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x6109ef; continue;
            }
            0x6109ef => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x6109f3; continue;
            }
            0x6109f3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x610a3c; continue; }
            pc = 0x610a20; continue;
            }
            0x610a20 => {
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x610a2e; continue; }
            pc = 0x610a25; continue;
            }
            0x610a25 => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x610a5b; continue;
            }
            0x610a2e => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x67adb0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x610a5b; continue;
            }
            0x610a3c => {
            c.set(2,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(2,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x610a4d; continue; }
            pc = 0x610a44; continue;
            }
            0x610a44 => {
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x67abb0),32));
            pc = 0x610a59; continue;
            }
            0x610a4d => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x67afb0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x610a59; continue;
            }
            0x610a59 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            pc = 0x610a5b; continue;
            }
            0x610a5b => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,0x78787879u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffff0000u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,0x14u32);
            c.push(m, 0x610a92);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x119u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8d));
            c.push(m,c.get(1,32));
            c.push(m, 0x610abe);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x116u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0x8000000fu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x610ad4; continue; }
            pc = 0x610acf; continue;
            }
            0x610acf => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x610ad4; continue;
            }
            0x610ad4 => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8b));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x610af8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x34); wm(m,a,16,0x22c8u32); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[0].wrapping_add(0x36); wm(m,a,16,0x1eccu32); }
            pc = 0x610b0c; continue;
            }
            0x610b0c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x610988; continue; }
            pc = 0x610b1f; continue;
            }
            0x610b1f => {
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

pub fn sea_battle_send_return_to_ai(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610c09;
    loop {
        match pc {
            0x610c09 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x610c37);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x610d57; continue; }
            pc = 0x610c4a; continue;
            }
            0x610c4a => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x610c68; continue; }
            pc = 0x610c53; continue;
            }
            0x610c53 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x610c55; continue;
            }
            0x610c55 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x610c55; continue; }
            pc = 0x610c68; continue;
            }
            0x610c68 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x610cd7; continue; }
            pc = 0x610c73; continue;
            }
            0x610c73 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(6,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x610ccd; continue; }
            pc = 0x610ca0; continue;
            }
            0x610ca0 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x610ccd; continue;
            }
            0x610ccd => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            if c.cond(2) { pc = 0x610c73; continue; }
            pc = 0x610cd7; continue;
            }
            0x610cd7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x610d57; continue; }
            pc = 0x610ce1; continue;
            }
            0x610ce1 => {
            c.set(6,16,c.get(3,8));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x610cfb; continue; }
            pc = 0x610cf3; continue;
            }
            0x610cf3 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x610d4d; continue; }
            pc = 0x610cfb; continue;
            }
            0x610cfb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x610d4d; continue; }
            pc = 0x610d10; continue;
            }
            0x610d10 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x610d19);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x662),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x9au32); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x610d4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x610d4d; continue;
            }
            0x610d4d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x610ce1; continue; }
            pc = 0x610d57; continue;
            }
            0x610d57 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x610d5d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn sea_battle_get_order_button_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610d69;
    loop {
        match pc {
            0x610d69 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0xad0),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x610da8; continue; }
            pc = 0x610d7b; continue;
            }
            0x610d7b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x610da8; continue; }
            pc = 0x610d9f; continue;
            }
            0x610d9f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x610da8 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x28),32); c.push(m, 0x610daf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x610e1f; continue; }
            pc = 0x610db3; continue;
            }
            0x610db3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x65a),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x658),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,0x8000u32);
            c.set(7,32,c.r[0].wrapping_add(c.r[2]));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x610e1f; continue; }
            pc = 0x610dd3; continue;
            }
            0x610dd3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x610ddc; continue;
            }
            0x610ddc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(5,16,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x610e13; continue; }
            pc = 0x610dff; continue;
            }
            0x610dff => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(0,8); c.and(a,0x8u32,8); }
            if c.cond(5) { pc = 0x610e0d; continue; }
            pc = 0x610e09; continue;
            }
            0x610e09 => {
            { let a = c.get(0,8); c.and(a,0x27u32,8); }
            if c.cond(5) { pc = 0x610e13; continue; }
            pc = 0x610e0d; continue;
            }
            0x610e0d => {
            { let a = c.get(3,32); let r = c.and(a,0x7fffu32,32); c.set(3,32,r); }
            pc = 0x610e13; continue;
            }
            0x610e13 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x610ddc; continue; }
            pc = 0x610e1f; continue;
            }
            0x610e1f => {
            { let a = rm(m,c.r[6].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x610fc7; continue; }
            pc = 0x610e2d; continue;
            }
            0x610e2d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.get(3,32); let r = c.or(a,0x3022u32,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x610e4c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x610e9f; continue; }
            pc = 0x610e50; continue;
            }
            0x610e50 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x559),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x610e9f; continue; }
            pc = 0x610e65; continue;
            }
            0x610e65 => {
            { let a = rm(m,c.r[6].wrapping_add(0xacb),8); c.sub(a,0x3u32,8); }
            if c.cond(7) { pc = 0x610e9f; continue; }
            pc = 0x610e6e; continue;
            }
            0x610e6e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x610e9f; continue; }
            pc = 0x610e78; continue;
            }
            0x610e78 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,8,0x66u32);
            pc = 0x610e7d; continue;
            }
            0x610e7d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = rm(m,c.r[7].wrapping_add(0x12a),8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x610e9c; continue; }
            pc = 0x610e8d; continue;
            }
            0x610e8d => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x610e7d; continue; }
            pc = 0x610e9a; continue;
            }
            0x610e9a => {
            pc = 0x610e9f; continue;
            }
            0x610e9c => {
            { let a = c.get(3,32); let r = c.or(a,0x1u32,32); c.set(3,32,r); }
            pc = 0x610e9f; continue;
            }
            0x610e9f => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x28),32); c.push(m, 0x610ea6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x610ef8; continue; }
            pc = 0x610eaa; continue;
            }
            0x610eaa => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x610ef8; continue; }
            pc = 0x610eb7; continue;
            }
            0x610eb7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x559));
            c.set(7,32,c.r[6].wrapping_add(0x35a));
            c.set(5,32,c.get(0,32));
            pc = 0x610eca; continue;
            }
            0x610eca => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x610ef1; continue; }
            pc = 0x610ed9; continue;
            }
            0x610ed9 => {
            c.set(1,8,rm(m,c.r[2],8));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xad2),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x610ef1; continue; }
            pc = 0x610eee; continue;
            }
            0x610eee => {
            { let a = c.get(7,8); let r = c.or(a,0x3u32,8); c.set(7,8,r); }
            pc = 0x610ef1; continue;
            }
            0x610ef1 => {
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x610eca; continue; }
            pc = 0x610ef8; continue;
            }
            0x610ef8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x13d),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x559),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x610f1d; continue; }
            pc = 0x610f13; continue;
            }
            0x610f13 => {
            { let a = rm(m,c.r[6].wrapping_add(0x65a),16); c.sub(a,0x0u32,16); }
            pc = 0x610f25; continue;
            }
            0x610f1d => {
            { let a = rm(m,c.r[6].wrapping_add(0x658),16); c.sub(a,0x0u32,16); }
            pc = 0x610f25; continue;
            }
            0x610f25 => {
            if c.cond(4) { pc = 0x610f2a; continue; }
            pc = 0x610f27; continue;
            }
            0x610f27 => {
            { let a = c.get(3,32); let r = c.or(a,0x10u32,32); c.set(3,32,r); }
            pc = 0x610f2a; continue;
            }
            0x610f2a => {
            c.set(6,16,rm(m,c.r[6].wrapping_add(0xad0),16));
            { let a = c.get(6,16); c.and(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x610f93; continue; }
            pc = 0x610f36; continue;
            }
            0x610f36 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            pc = 0x610f3e; continue;
            }
            0x610f3e => {
            c.set(1,32,rm(m,c.r[2],32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x12a),8));
            { let a = c.get(0,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x610f4f; continue; }
            pc = 0x610f4a; continue;
            }
            0x610f4a => {
            { let a = c.get(3,32); let r = c.or(a,0x4u32,32); c.set(3,32,r); }
            pc = 0x610f55; continue;
            }
            0x610f4f => {
            { let a = c.get(3,32); let r = c.and(a,0xfffdu32,32); c.set(3,32,r); }
            pc = 0x610f55; continue;
            }
            0x610f55 => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x610f5e; continue; }
            pc = 0x610f59; continue;
            }
            0x610f59 => {
            { let a = c.get(7,8); let r = c.or(a,0x40u32,8); c.set(7,8,r); }
            pc = 0x610f64; continue;
            }
            0x610f5e => {
            { let a = c.get(3,32); let r = c.and(a,0xdfffu32,32); c.set(3,32,r); }
            pc = 0x610f64; continue;
            }
            0x610f64 => {
            { let a = rm(m,c.r[1].wrapping_add(0x13e),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x610f87; continue; }
            pc = 0x610f6d; continue;
            }
            0x610f6d => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x610f7c; continue; }
            pc = 0x610f71; continue;
            }
            0x610f71 => {
            { let a = c.get(3,32); let r = c.and(a,0xfdffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.or(a,0x40u32,32); c.set(3,32,r); }
            pc = 0x610f8d; continue;
            }
            0x610f7c => {
            { let a = c.get(3,32); let r = c.and(a,0xffdfu32,32); c.set(3,32,r); }
            { let a = c.get(7,8); let r = c.or(a,0x4u32,8); c.set(7,8,r); }
            pc = 0x610f8d; continue;
            }
            0x610f87 => {
            { let a = c.get(3,32); let r = c.and(a,0xfddfu32,32); c.set(3,32,r); }
            pc = 0x610f8d; continue;
            }
            0x610f8d => {
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x610f3e; continue; }
            pc = 0x610f93; continue;
            }
            0x610f93 => {
            { let a = c.get(3,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x610fa6; continue; }
            pc = 0x610f98; continue;
            }
            0x610f98 => {
            c.set(3,32,0x2u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,16,c.get(3,16));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x610fa6 => {
            { let a = c.get(3,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x610fb1; continue; }
            pc = 0x610fab; continue;
            }
            0x610fab => {
            { let a = c.get(3,32); let r = c.and(a,0xffbfu32,32); c.set(3,32,r); }
            pc = 0x610fb1; continue;
            }
            0x610fb1 => {
            { let a = c.get(7,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x610fbc; continue; }
            pc = 0x610fb6; continue;
            }
            0x610fb6 => {
            { let a = c.get(3,32); let r = c.and(a,0xfbffu32,32); c.set(3,32,r); }
            pc = 0x610fbc; continue;
            }
            0x610fbc => {
            { let a = c.get(7,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x610fc7; continue; }
            pc = 0x610fc1; continue;
            }
            0x610fc1 => {
            { let a = c.get(3,32); let r = c.and(a,0xbfffu32,32); c.set(3,32,r); }
            pc = 0x610fc7; continue;
            }
            0x610fc7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,16,c.get(3,16));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_send_set_retreat(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x610fd9;
    loop {
        match pc {
            0x610fd9 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x3fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            c.push(m, 0x611007);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x61112f; continue; }
            pc = 0x61101a; continue;
            }
            0x61101a => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x611038; continue; }
            pc = 0x611023; continue;
            }
            0x611023 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x611025; continue;
            }
            0x611025 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x611025; continue; }
            pc = 0x611038; continue;
            }
            0x611038 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xad0),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x6110a7; continue; }
            pc = 0x611043; continue;
            }
            0x611043 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x66c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            c.set(6,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x61109d; continue; }
            pc = 0x611070; continue;
            }
            0x611070 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(6,32,0x1u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x61109d; continue;
            }
            0x61109d => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xad0),16),16); }
            if c.cond(2) { pc = 0x611043; continue; }
            pc = 0x6110a7; continue;
            }
            0x6110a7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x61112f; continue; }
            pc = 0x6110b1; continue;
            }
            0x6110b1 => {
            c.set(6,16,c.get(3,8));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6110cb; continue; }
            pc = 0x6110c3; continue;
            }
            0x6110c3 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x611125; continue; }
            pc = 0x6110cb; continue;
            }
            0x6110cb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,0x6df346u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1au32,32); }
            if c.cond(6) { pc = 0x611125; continue; }
            pc = 0x6110e0; continue;
            }
            0x6110e0 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x6110e9);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x662),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x9bu32); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x611125);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x611125; continue;
            }
            0x611125 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x6110b1; continue; }
            pc = 0x61112f; continue;
            }
            0x61112f => {
            c.push(m,c.get(7,32));
            c.push(m, 0x611135);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_raise_event_level(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611649;
    loop {
        match pc {
            0x611649 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x1e94),16));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(3,16); let n = 0xau32; let r = c.shr(a,n,16); c.set(3,16,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x61165d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            if c.cond(4) { pc = 0x611673; continue; }
            pc = 0x611665; continue;
            }
            0x611665 => {
            { let a = c.get(0,8); c.sub(a,0x9u32,8); }
            if c.cond(4) { pc = 0x611671; continue; }
            pc = 0x611669; continue;
            }
            0x611669 => {
            { let a = c.get(0,8); c.sub(a,0xau32,8); }
            if c.cond(6) { pc = 0x611673; continue; }
            pc = 0x61166d; continue;
            }
            0x61166d => {
            { let a = c.get(0,8); c.sub(a,0xcu32,8); }
            if c.cond(7) { pc = 0x611673; continue; }
            pc = 0x611671; continue;
            }
            0x611671 => {
            c.set(0,8,0xeu32);
            pc = 0x611673; continue;
            }
            0x611673 => {
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(6) { pc = 0x611695; continue; }
            pc = 0x611677; continue;
            }
            0x611677 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1e94),16));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,32); let r = c.and(a,0x3ffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0xau32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1e94); wm(m,a,16,c.get(1,16)); }
            pc = 0x611695; continue;
            }
            0x611695 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn sea_battle_is_town_attack(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611730;
    loop {
        match pc {
            0x611730 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_battles_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6118f0;
    loop {
        match pc {
            0x6118f0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,0x1u32);
            c.set(6,32,c.get(3,32));
            c.set(5,32,0xffu32);
            pc = 0x611902; continue;
            }
            0x611902 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x611911; continue; }
            pc = 0x611908; continue;
            }
            0x611908 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x61190d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1));
            pc = 0x611911; continue;
            }
            0x611911 => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x611902; continue; }
            pc = 0x611917; continue;
            }
            0x611917 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc34),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x61193f; continue; }
            pc = 0x611924; continue;
            }
            0x611924 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xc30),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x61192f; continue;
            }
            0x61192f => {
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x611939; continue; }
            pc = 0x611935; continue;
            }
            0x611935 => {
            c.set(7,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x8));
            pc = 0x611939; continue;
            }
            0x611939 => {
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x61192f; continue; }
            pc = 0x61193f; continue;
            }
            0x61193f => {
            c.set(0,32,c.get(7,32));
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

pub fn queue_battles_time_advance(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x612db0;
    loop {
        match pc {
            0x612db0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x612dbf);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x612dc8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6127c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x3fe),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x612ece; continue; }
            pc = 0x612dd6; continue;
            }
            0x612dd6 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc44),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x6dccf8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,0x6dfc1cu32,32));
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x612e00; continue; }
            pc = 0x612df4; continue;
            }
            0x612df4 => {
            { let a = c.get(7,32); c.sub(a,0x64u32,32); }
            if c.cond(14) { pc = 0x612e00; continue; }
            pc = 0x612df9; continue;
            }
            0x612df9 => {
            c.set(7,32,0x64u32);
            pc = 0x612e15; continue;
            }
            0x612e00 => {
            { let a = c.get(7,32); c.sub(a,0x186a0u32,32); }
            if c.cond(14) { pc = 0x612e0f; continue; }
            pc = 0x612e08; continue;
            }
            0x612e08 => {
            c.set(7,32,0x186a0u32);
            pc = 0x612e15; continue;
            }
            0x612e0f => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x612e15; continue; }
            pc = 0x612e13; continue;
            }
            0x612e13 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x612e15; continue;
            }
            0x612e15 => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            c.set(6,32,c.get(2,32));
            if c.cond(5) { pc = 0x612e4c; continue; }
            pc = 0x612e2d; continue;
            }
            0x612e2d => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let b = rm(m,0x6dfbc8u32,32); c.div32(b); }
            pc = 0x612e76; continue;
            }
            0x612e4c => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x612e78; continue; }
            pc = 0x612e50; continue;
            }
            0x612e50 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x612e78; continue; }
            pc = 0x612e59; continue;
            }
            0x612e59 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let b = rm(m,0x6dfbc4u32,32); c.div32(b); }
            pc = 0x612e76; continue;
            }
            0x612e76 => {
            c.set(6,32,c.get(0,32));
            pc = 0x612e78; continue;
            }
            0x612e78 => {
            { let a = c.get(6,16); c.sub(a,0x5460u32,16); }
            if c.cond(6) { pc = 0x612e86; continue; }
            pc = 0x612e7f; continue;
            }
            0x612e7f => {
            c.set(6,32,0x5460u32);
            pc = 0x612e8b; continue;
            }
            0x612e86 => {
            { let a = c.get(6,16); c.and(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x612eb5; continue; }
            pc = 0x612e8b; continue;
            }
            0x612e8b => {
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x94u32); }
            c.push(m, 0x612eae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[3].wrapping_add(0x3fe); wm(m,a,8,0x0u32); }
            pc = 0x612eb5; continue;
            }
            0x612eb5 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0xc44); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x612ece; continue;
            }
            0x612ece => {
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_state_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6278f0;
    loop {
        match pc {
            0x6278f0 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6f),8));
            c.set(0,32,0x51u32);
            { let a = c.get(2,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x627911; continue; }
            pc = 0x6278fd; continue;
            }
            0x6278fd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6e),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_mul(8).wrapping_add(0x51));
            pc = 0x627911; continue;
            }
            0x627911 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_state_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x627920;
    loop {
        match pc {
            0x627920 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627936);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627942);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x57c));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627954);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x580));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627966);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x584));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627978);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x42));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627987);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x66));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627999);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x3c));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279a8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x68));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279b7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x3a));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279c6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x6a));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279d5);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x44));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x78));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6279f6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x6d));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a03);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x5d4));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a13);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x61c));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a23);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x62c));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a33);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x230));
            c.set(5,32,0x5u32);
            pc = 0x627a42; continue;
            }
            0x627a42 => {
            c.set(1,32,c.r[3].wrapping_add(0xffffffec));
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a4e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a5a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x627a42; continue; }
            pc = 0x627a66; continue;
            }
            0x627a66 => {
            c.set(5,32,c.r[7].wrapping_add(0x6e));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a72);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[7].wrapping_add(0x6f));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627a7f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[3],8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x627ac2; continue; }
            pc = 0x627a8a; continue;
            }
            0x627a8a => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(3) { pc = 0x627ac2; continue; }
            pc = 0x627a95; continue;
            }
            0x627a95 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x254));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x627aaf);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            if c.cond(2) { pc = 0x627a95; continue; }
            pc = 0x627ac2; continue;
            }
            0x627ac2 => {
            c.set(0,32,c.get(6,32));
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

pub fn siege_state_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x627ad0;
    loop {
        match pc {
            0x627ad0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m, 0x627adc);
            { let args = []; c.r[0] = host.call(m, 0x627f70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x4u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627aec);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x627af8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(1,32,c.r[7].wrapping_add(0x57c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x627b0a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(2,32,c.r[7].wrapping_add(0x580));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x627b1c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,0x57u32,32); }
            if c.cond(3) { pc = 0x627b54; continue; }
            pc = 0x627b2b; continue;
            }
            0x627b2b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x4u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m, 0x627b4e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            pc = 0x627b54; continue;
            }
            0x627b54 => {
            c.push(m,0x4u32);
            c.set(2,32,c.r[7].wrapping_add(0x584));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x627b63);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x42));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627b72);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0x66));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x627b81);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x3c));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627b90);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(2,32,c.r[7].wrapping_add(0x68));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x627b9f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x3a));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627bae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x6a));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627bc0);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x44));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627bcf);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x78));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627bde);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.r[7].wrapping_add(0x6d));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x627beb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,0x4au32,32); }
            if c.cond(6) { pc = 0x627c09; continue; }
            pc = 0x627bf4; continue;
            }
            0x627bf4 => {
            c.push(m,0x1u32);
            c.set(2,32,c.r[7].wrapping_add(0x5d4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x627c03);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x627c10; continue;
            }
            0x627c09 => {
            { let a = c.r[7].wrapping_add(0x5d4); wm(m,a,8,0x2u32); }
            pc = 0x627c10; continue;
            }
            0x627c10 => {
            { let a = c.get(3,32); c.sub(a,0x55u32,32); }
            if c.cond(6) { pc = 0x627c85; continue; }
            pc = 0x627c15; continue;
            }
            0x627c15 => {
            { let a = c.get(3,32); c.sub(a,0x57u32,32); }
            if c.cond(3) { pc = 0x627c70; continue; }
            pc = 0x627c1a; continue;
            }
            0x627c1a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x2u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1018));
            c.push(m,c.get(1,32));
            c.push(m, 0x627c41);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(2,32,c.get(0,32));
            c.push(m,0x1u32);
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x101a));
            c.push(m,c.get(2,32));
            c.push(m, 0x627c6c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x627c70; continue;
            }
            0x627c70 => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[7].wrapping_add(0x61c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627c7f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x627ccc; continue;
            }
            0x627c85 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1018); wm(m,a,16,0x3fffu32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x101a); wm(m,a,8,0x2u32); }
            { let a = c.r[7].wrapping_add(0x61c); wm(m,a,8,0x1u32); }
            pc = 0x627ccc; continue;
            }
            0x627ccc => {
            { let a = c.get(3,32); c.sub(a,0x5fu32,32); }
            if c.cond(6) { pc = 0x627ce6; continue; }
            pc = 0x627cd1; continue;
            }
            0x627cd1 => {
            c.push(m,0x1u32);
            c.set(1,32,c.r[7].wrapping_add(0x62c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x627ce0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x627ced; continue;
            }
            0x627ce6 => {
            { let a = c.r[7].wrapping_add(0x62c); wm(m,a,8,0xffu32); }
            pc = 0x627ced; continue;
            }
            0x627ced => {
            c.set(3,32,c.r[7].wrapping_add(0x230));
            c.set(5,32,0x5u32);
            pc = 0x627cf8; continue;
            }
            0x627cf8 => {
            c.push(m,0x4u32);
            c.set(2,32,c.r[3].wrapping_add(0xffffffec));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x627d04);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x627d10);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x627cf8; continue; }
            pc = 0x627d1c; continue;
            }
            0x627d1c => {
            c.set(5,32,c.r[7].wrapping_add(0x6e));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x627d28);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x6f));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x627d35);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[3],8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x627d77; continue; }
            pc = 0x627d40; continue;
            }
            0x627d40 => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(3) { pc = 0x627d77; continue; }
            pc = 0x627d4b; continue;
            }
            0x627d4b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x8u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x254));
            c.push(m,c.get(1,32));
            c.push(m, 0x627d64);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(3,8)); }
            if c.cond(2) { pc = 0x627d4b; continue; }
            pc = 0x627d77; continue;
            }
            0x627d77 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x3a),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x627d86; continue; }
            pc = 0x627d80; continue;
            }
            0x627d80 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x6a),16),16); }
            pc = 0x627d8e; continue;
            }
            0x627d86 => {
            { let a = rm(m,c.r[7].wrapping_add(0x21e),16); c.sub(a,0x0u32,16); }
            pc = 0x627d8e; continue;
            }
            0x627d8e => {
            c.set(1,8,u32::from(c.cond(7)));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x627d9e; continue; }
            pc = 0x627d95; continue;
            }
            0x627d95 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x627ee1; continue; }
            pc = 0x627d9e; continue;
            }
            0x627d9e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x627dbd);
            { let args = []; c.r[0] = host.call(m, 0x5252f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x627dd0; continue; }
            pc = 0x627dc5; continue;
            }
            0x627dc5 => {
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            pc = 0x627ed3; continue;
            }
            0x627dd0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x768),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[7].wrapping_add(0x244); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,c.r[7].wrapping_add(0x248));
            pc = 0x627df4; continue;
            }
            0x627df4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x627e87; continue; }
            pc = 0x627e06; continue;
            }
            0x627e06 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x627e11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x37u32,8); }
            if c.cond(4) { pc = 0x627e28; continue; }
            pc = 0x627e19; continue;
            }
            0x627e19 => {
            { let a = c.get(1,8); c.sub(a,0x38u32,8); }
            if c.cond(4) { pc = 0x627e28; continue; }
            pc = 0x627e1e; continue;
            }
            0x627e1e => {
            { let a = c.get(1,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x627e28; continue; }
            pc = 0x627e23; continue;
            }
            0x627e23 => {
            { let a = c.get(1,8); c.sub(a,0x35u32,8); }
            if c.cond(5) { pc = 0x627e42; continue; }
            pc = 0x627e28; continue;
            }
            0x627e28 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,0xffu32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x627e83; continue; }
            pc = 0x627e42; continue;
            }
            0x627e42 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x627e6a; continue; }
            pc = 0x627e4a; continue;
            }
            0x627e4a => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x627e6a; continue; }
            pc = 0x627e57; continue;
            }
            0x627e57 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x762),16));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            pc = 0x627e73; continue;
            }
            0x627e6a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            pc = 0x627e73; continue;
            }
            0x627e73 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x627e06; continue; }
            pc = 0x627e83; continue;
            }
            0x627e83 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            pc = 0x627e87; continue;
            }
            0x627e87 => {
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x627eba; continue; }
            pc = 0x627e8c; continue;
            }
            0x627e8c => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x627eb1; continue; }
            pc = 0x627e94; continue;
            }
            0x627e94 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x627eb1; continue; }
            pc = 0x627ea1; continue;
            }
            0x627ea1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x762),16));
            pc = 0x627eb7; continue;
            }
            0x627eb1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            pc = 0x627eb7; continue;
            }
            0x627eb7 => {
            { let a = c.r[5]; wm(m,a,32,c.get(2,32)); }
            pc = 0x627eba; continue;
            }
            0x627eba => {
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(1,8); c.sub(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(1,8)); }
            if c.cond(2) { pc = 0x627df4; continue; }
            pc = 0x627ecc; continue;
            }
            0x627ecc => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x627ed3);
            { let args = []; c.r[0] = host.call(m, 0x632780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x627ed3; continue;
            }
            0x627ed3 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x627eda);
            { let args = []; c.r[0] = host.call(m, 0x62b2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x627ee1);
            { let args = []; c.r[0] = host.call(m, 0x62e810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x627ee1; continue;
            }
            0x627ee1 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x3c),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x627f07; continue; }
            pc = 0x627eea; continue;
            }
            0x627eea => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x627f07; continue; }
            pc = 0x627ef0; continue;
            }
            0x627ef0 => {
            { let a = c.r[7].wrapping_add(0x78); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x79); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x578); wm(m,a,8,0xffu32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x627f07 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[7].wrapping_add(0x78); wm(m,a,8,0x1u32); }
            if c.cond(5) { pc = 0x627f4e; continue; }
            pc = 0x627f10; continue;
            }
            0x627f10 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5d3),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.add(a,0x1e00u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x627f4e; continue; }
            pc = 0x627f39; continue;
            }
            0x627f39 => {
            { let a = c.r[7].wrapping_add(0x79); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x578); wm(m,a,8,0xffu32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x627f4e => {
            { let a = c.r[7].wrapping_add(0x79); wm(m,a,8,0x1u32); }
            { let a = c.r[7].wrapping_add(0x578); wm(m,a,8,0xffu32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_state_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x627f70;
    loop {
        match pc {
            0x627f70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,0x5u32);
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,c.r[6].wrapping_add(0x230));
            pc = 0x627f91; continue;
            }
            0x627f91 => {
            { let a = c.r[0].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x627f91; continue; }
            pc = 0x627f9c; continue;
            }
            0x627f9c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5d8),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x57c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x580); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x627fc0; continue; }
            pc = 0x627fb7; continue;
            }
            0x627fb7 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x627fbd);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x627fc0; continue;
            }
            0x627fc0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x59c),32));
            { let a = c.r[6].wrapping_add(0x5d8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x590); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x592); wm(m,a,16,c.get(3,16)); }
            if c.cond(4) { pc = 0x627fe7; continue; }
            pc = 0x627fde; continue;
            }
            0x627fde => {
            c.push(m,c.get(0,32));
            c.push(m, 0x627fe4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x627fe7; continue;
            }
            0x627fe7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.r[6].wrapping_add(0x59c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x77); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x628003; continue; }
            pc = 0x627ffa; continue;
            }
            0x627ffa => {
            c.push(m,c.get(0,32));
            c.push(m, 0x628000);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x628003; continue;
            }
            0x628003 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x250); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x5dc));
            c.set(5,32,0x8u32);
            pc = 0x628016; continue;
            }
            0x628016 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x628038; continue; }
            pc = 0x62801c; continue;
            }
            0x62801c => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x628036; continue; }
            pc = 0x628026; continue;
            }
            0x628026 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x3f8),32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x628036); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x628036; continue;
            }
            0x628036 => {
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            pc = 0x628038; continue;
            }
            0x628038 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x628016; continue; }
            pc = 0x62803e; continue;
            }
            0x62803e => {
            c.set(7,32,c.r[6].wrapping_add(0x5fc));
            c.set(5,32,0x3u32);
            pc = 0x628049; continue;
            }
            0x628049 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x62806b; continue; }
            pc = 0x62804f; continue;
            }
            0x62804f => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x628069; continue; }
            pc = 0x628059; continue;
            }
            0x628059 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x3f8),32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x628069); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x628069; continue;
            }
            0x628069 => {
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            pc = 0x62806b; continue;
            }
            0x62806b => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x628049; continue; }
            pc = 0x628071; continue;
            }
            0x628071 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[6].wrapping_add(0x22));
            c.set(1,32,0x5u32);
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x62807b; continue;
            }
            0x62807b => {
            { let a = c.r[0].wrapping_add(0xfffffff6); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0]; wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x62807b; continue; }
            pc = 0x62808c; continue;
            }
            0x62808c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x608); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x60c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x610); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x614); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x218); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x6280c0; continue;
            }
            0x6280c0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x28u32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x7c); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x6280c0; continue; }
            pc = 0x6280d8; continue;
            }
            0x6280d8 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x6280de; continue;
            }
            0x6280de => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x16u32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x11c); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x6280de; continue; }
            pc = 0x6280f9; continue;
            }
            0x6280f9 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x6280ff; continue;
            }
            0x6280ff => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x12u32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x174); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x6280ff; continue; }
            pc = 0x62811a; continue;
            }
            0x62811a => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x628120; continue;
            }
            0x628120 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0xfu32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1bc); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x628120; continue; }
            pc = 0x62813b; continue;
            }
            0x62813b => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x628141; continue;
            }
            0x628141 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1f8); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x628141; continue; }
            pc = 0x62815c; continue;
            }
            0x62815c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x70),32));
            { let a = c.r[6].wrapping_add(0x574); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x628172; continue; }
            pc = 0x628169; continue;
            }
            0x628169 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x62816f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x628172; continue;
            }
            0x628172 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x628),32));
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x6f); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x588); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x7a); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x79); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x6281a7; continue; }
            pc = 0x62818f; continue;
            }
            0x62818f => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x6281a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x628); wm(m,a,32,c.get(3,32)); }
            pc = 0x6281a7; continue;
            }
            0x6281a7 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_state_free_resource_5d8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6281b0;
    loop {
        match pc {
            0x6281b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5d8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6281c6; continue; }
            pc = 0x6281bd; continue;
            }
            0x6281bd => {
            c.push(m,c.get(0,32));
            c.push(m, 0x6281c3);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x6281c6; continue;
            }
            0x6281c6 => {
            { let a = c.r[6].wrapping_add(0x5d8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x590); wm(m,a,16,0x0u32); }
            { let a = c.r[6].wrapping_add(0x592); wm(m,a,16,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_list_defense_towers_near_gate(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62b2b0;
    loop {
        match pc {
            0x62b2b0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x242),16));
            c.set(3,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            if c.cond(6) { pc = 0x62b3fb; continue; }
            pc = 0x62b2e5; continue;
            }
            0x62b2e5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x62b2fd);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x250); wm(m,a,32,c.get(0,32)); }
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x768),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x62b3e9; continue; }
            pc = 0x62b320; continue;
            }
            0x62b320 => {
            c.push(m,c.get(7,32));
            pc = 0x62b321; continue;
            }
            0x62b321 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x62b329);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62b3bf; continue; }
            pc = 0x62b336; continue;
            }
            0x62b336 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x32u32,8); }
            if c.cond(4) { pc = 0x62b341; continue; }
            pc = 0x62b33d; continue;
            }
            0x62b33d => {
            { let a = c.get(0,8); c.sub(a,0x33u32,8); }
            if c.cond(5) { pc = 0x62b3bf; continue; }
            pc = 0x62b341; continue;
            }
            0x62b341 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6d),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x244),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x62b355);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x48u32,32); }
            if c.cond(15) { pc = 0x62b3bf; continue; }
            pc = 0x62b383; continue;
            }
            0x62b383 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x250),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x250),32));
            c.set(5,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            pc = 0x62b3bf; continue;
            }
            0x62b3bf => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x62b321; continue; }
            pc = 0x62b3d6; continue;
            }
            0x62b3d6 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.r[6].wrapping_add(0x242); wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x62b3e9 => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0xc),8));
            { let a = c.r[6].wrapping_add(0x242); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x62b3fb => {
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(1,8));
            { let a = c.r[6].wrapping_add(0x242); wm(m,a,16,c.get(0,16)); }
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

pub fn town_siege_add_flow_field_seed_cell(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62b960;
    loop {
        match pc {
            0x62b960 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(0xc314),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x62b9ba; continue; }
            pc = 0x62b980; continue;
            }
            0x62b980 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0xc318),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x62b9ba; continue; }
            pc = 0x62b996; continue;
            }
            0x62b996 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x62b9b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62be90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(3,16)); }
            pc = 0x62b9ba; continue;
            }
            0x62b9ba => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_choose_active_tower(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62e810;
    loop {
        match pc {
            0x62e810 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x242),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(4) { pc = 0x62e960; continue; }
            pc = 0x62e841; continue;
            }
            0x62e841 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x62e909; continue; }
            pc = 0x62e85a; continue;
            }
            0x62e85a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62e878);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62e88f; continue; }
            pc = 0x62e87f; continue;
            }
            0x62e87f => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x32u32,8); }
            if c.cond(5) { pc = 0x62e88b; continue; }
            pc = 0x62e885; continue;
            }
            0x62e885 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,r); } }
            pc = 0x62e88f; continue;
            }
            0x62e88b => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,r); } }
            pc = 0x62e88f; continue;
            }
            0x62e88f => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x242),16),16); }
            if c.cond(2) { pc = 0x62e85a; continue; }
            pc = 0x62e8a3; continue;
            }
            0x62e8a3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x62e909; continue; }
            pc = 0x62e8ab; continue;
            }
            0x62e8ab => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2e0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,8,c.get(2,8)); }
            pc = 0x62e8c7; continue;
            }
            0x62e8c7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x62e8e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62e8f2; continue; }
            pc = 0x62e8ec; continue;
            }
            0x62e8ec => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x32u32,8); }
            if c.cond(4) { pc = 0x62e8f5; continue; }
            pc = 0x62e8f2; continue;
            }
            0x62e8f2 => {
            { let a = rm(m,c.r[6].wrapping_add(0x6c),8); let r = c.inc(a,8); { let a = c.r[6].wrapping_add(0x6c); wm(m,a,8,r); } }
            pc = 0x62e8f5; continue;
            }
            0x62e8f5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6c),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x62e8c7; continue; }
            pc = 0x62e902; continue;
            }
            0x62e902 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x62e909 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2e0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,8,c.get(2,8)); }
            pc = 0x62e925; continue;
            }
            0x62e925 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x62e943);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62e950; continue; }
            pc = 0x62e94a; continue;
            }
            0x62e94a => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x33u32,8); }
            if c.cond(4) { pc = 0x62e953; continue; }
            pc = 0x62e950; continue;
            }
            0x62e950 => {
            { let a = rm(m,c.r[6].wrapping_add(0x6c),8); let r = c.inc(a,8); { let a = c.r[6].wrapping_add(0x6c); wm(m,a,8,r); } }
            pc = 0x62e953; continue;
            }
            0x62e953 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6c),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x62e925; continue; }
            pc = 0x62e960; continue;
            }
            0x62e960 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_update_tower_fire(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62e970;
    loop {
        match pc {
            0x62e970 => {
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x242),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x62ed9e; continue; }
            pc = 0x62e986; continue;
            }
            0x62e986 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x77),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x62ed9e; continue; }
            pc = 0x62e991; continue;
            }
            0x62e991 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x62ed9e; continue; }
            pc = 0x62e9a0; continue;
            }
            0x62e9a0 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            pc = 0x62e9a3; continue;
            }
            0x62e9a3 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x250),32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62e9dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62ed85; continue; }
            pc = 0x62e9e7; continue;
            }
            0x62e9e7 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x250),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x4),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[6]));
            { let a = c.get(2,32); c.sub(a,rm(m,0x6dccf8u32,32),32); }
            if c.cond(7) { pc = 0x62ed85; continue; }
            pc = 0x62ea00; continue;
            }
            0x62ea00 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.set(7,32,0x7fffffffu32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ea27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x250),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ea58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x62ea7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x2a),16));
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x20),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x62eb2c; continue; }
            pc = 0x62eaa2; continue;
            }
            0x62eaa2 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x578),8));
            c.set(5,16,c.get(1,8));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x62eb2c; continue; }
            pc = 0x62eab1; continue;
            }
            0x62eab1 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1f8),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62eb2c; continue; }
            pc = 0x62eac5; continue;
            }
            0x62eac5 => {
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xac440u32,32); }
            if c.cond(15) { pc = 0x62eb2c; continue; }
            pc = 0x62eb0b; continue;
            }
            0x62eb0b => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x1f8),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x12),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x62ed5c; continue;
            }
            0x62eb2c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x28),16));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1e),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x26),16));
            c.set(0,32,c.r[3].wrapping_add(0x1a));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x24),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x62ec79; continue; }
            pc = 0x62eb64; continue;
            }
            0x62eb64 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x3u32); }
            pc = 0x62eb78; continue;
            }
            0x62eb78 => {
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x62ec25; continue; }
            pc = 0x62eb84; continue;
            }
            0x62eb84 => {
            c.set(1,32,c.r[3].wrapping_add(0x174));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x62eb97; continue;
            }
            0x62eb97 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x62ebb5; continue; }
            pc = 0x62eb9e; continue;
            }
            0x62eb9e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x62ebad; continue; }
            pc = 0x62eba1; continue;
            }
            0x62eba1 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x62ebbc; continue; }
            pc = 0x62eba4; continue;
            }
            0x62eba4 => {
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x48),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            pc = 0x62ebc0; continue;
            }
            0x62ebad => {
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x62ebbc; continue;
            }
            0x62ebb5 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xffffffa8),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x62ebbc; continue;
            }
            0x62ebbc => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x62ebc0; continue;
            }
            0x62ebc0 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62ec0f; continue; }
            pc = 0x62ebc7; continue;
            }
            0x62ebc7 => {
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x62ec0f; continue; }
            pc = 0x62ec09; continue;
            }
            0x62ec09 => {
            c.set(7,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            pc = 0x62ec0f; continue;
            }
            0x62ec0f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x62eb97; continue; }
            pc = 0x62ec25; continue;
            }
            0x62ec25 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x62eb78; continue; }
            pc = 0x62ec48; continue;
            }
            0x62ec48 => {
            { let a = c.get(7,32); c.sub(a,0xac440u32,32); }
            if c.cond(15) { pc = 0x62ec85; continue; }
            pc = 0x62ec50; continue;
            }
            0x62ec50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ec5f; continue; }
            pc = 0x62ec58; continue;
            }
            0x62ec58 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            pc = 0x62ec63; continue;
            }
            0x62ec5f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x62ec63; continue;
            }
            0x62ec63 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x62ec79);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62edb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x62ec79; continue;
            }
            0x62ec79 => {
            { let a = c.get(7,32); c.sub(a,0xac440u32,32); }
            if c.cond(14) { pc = 0x62ed63; continue; }
            pc = 0x62ec85; continue;
            }
            0x62ec85 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x18),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x22),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x62ed63; continue; }
            pc = 0x62ec9f; continue;
            }
            0x62ec9f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x62ed22; continue; }
            pc = 0x62eca6; continue;
            }
            0x62eca6 => {
            c.set(2,32,c.r[3].wrapping_add(0x7c));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x62ecad; continue;
            }
            0x62ecad => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62ed10; continue; }
            pc = 0x62ecba; continue;
            }
            0x62ecba => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ecc3; continue; }
            pc = 0x62ecbe; continue;
            }
            0x62ecbe => {
            c.set(1,32,c.r[0].wrapping_add(0x8));
            pc = 0x62ecc5; continue;
            }
            0x62ecc3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x62ecc5; continue;
            }
            0x62ecc5 => {
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ecd6; continue; }
            pc = 0x62ecd1; continue;
            }
            0x62ecd1 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62ecd8; continue;
            }
            0x62ecd6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62ecd8; continue;
            }
            0x62ecd8 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x62ed10; continue; }
            pc = 0x62ed0a; continue;
            }
            0x62ed0a => {
            c.set(7,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            pc = 0x62ed10; continue;
            }
            0x62ed10 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x18),16),16); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x62ecad; continue; }
            pc = 0x62ed22; continue;
            }
            0x62ed22 => {
            { let a = c.get(7,32); c.sub(a,0xac440u32,32); }
            if c.cond(15) { pc = 0x62ed63; continue; }
            pc = 0x62ed2a; continue;
            }
            0x62ed2a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ed40; continue; }
            pc = 0x62ed3b; continue;
            }
            0x62ed3b => {
            c.set(1,32,c.r[0].wrapping_add(0x8));
            pc = 0x62ed42; continue;
            }
            0x62ed40 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x62ed42; continue;
            }
            0x62ed42 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ed4b; continue; }
            pc = 0x62ed46; continue;
            }
            0x62ed46 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62ed4d; continue;
            }
            0x62ed4b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62ed4d; continue;
            }
            0x62ed4d => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0xa),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x62ed5c; continue;
            }
            0x62ed5c => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x62ed63);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62edb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x62ed63; continue;
            }
            0x62ed63 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x250),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x4));
            c.set(2,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x62ed85; continue; }
            pc = 0x62ed7d; continue;
            }
            0x62ed7d => {
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x62ed85; continue;
            }
            0x62ed85 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x242),16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x62e9a3; continue; }
            pc = 0x62ed9b; continue;
            }
            0x62ed9b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x62ed9e; continue;
            }
            0x62ed9e => {
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_fire_tower_at_target(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62edb0;
    loop {
        match pc {
            0x62edb0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x250),32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62edf3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x250),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ee24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x62ee47);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x62ee7b; continue; }
            pc = 0x62ee76; continue;
            }
            0x62ee76 => {
            { let a = c.get(0,32); let r = c.add(a,0x11u32,32); c.set(0,32,r); }
            pc = 0x62ee7e; continue;
            }
            0x62ee7b => {
            { let a = c.get(0,32); let r = c.sub(a,0x11u32,32); c.set(0,32,r); }
            pc = 0x62ee7e; continue;
            }
            0x62ee7e => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x62ee8f; continue; }
            pc = 0x62ee8a; continue;
            }
            0x62ee8a => {
            { let a = c.get(0,32); let r = c.add(a,0x1au32,32); c.set(0,32,r); }
            pc = 0x62ee92; continue;
            }
            0x62ee8f => {
            { let a = c.get(0,32); let r = c.sub(a,0x1au32,32); c.set(0,32,r); }
            pc = 0x62ee92; continue;
            }
            0x62ee92 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x62ee9d);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x62eea5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x62eed0; continue; }
            pc = 0x62eeac; continue;
            }
            0x62eeac => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x62eebf; continue; }
            pc = 0x62eeb0; continue;
            }
            0x62eeb0 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(14)));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0x90u32,8); c.set(2,8,r); }
            pc = 0x62ef37; continue;
            }
            0x62eebf => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xf0u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            pc = 0x62ef3b; continue;
            }
            0x62eed0 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x62eed6);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[0].wrapping_mul(8));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m, 0x62eee5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x62ef13; continue; }
            pc = 0x62eeef; continue;
            }
            0x62eeef => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x62ef02; continue; }
            pc = 0x62eef3; continue;
            }
            0x62eef3 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(14)));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0xa0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0xb0u32,8); c.set(2,8,r); }
            pc = 0x62ef37; continue;
            }
            0x62ef02 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x60u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xd0u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            pc = 0x62ef3b; continue;
            }
            0x62ef13 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x62ef2a; continue; }
            pc = 0x62ef17; continue;
            }
            0x62ef17 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.and(a,0xc0u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,0xa0u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(1,8)); }
            pc = 0x62ef3b; continue;
            }
            0x62ef2a => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(14)));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x40u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0xe0u32,8); c.set(2,8,r); }
            pc = 0x62ef37; continue;
            }
            0x62ef37 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(2,8)); }
            pc = 0x62ef3b; continue;
            }
            0x62ef3b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xf0u32,32); }
            if c.cond(7) { pc = 0x62f062; continue; }
            pc = 0x62ef52; continue;
            }
            0x62ef52 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x62f198),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x62f154)); continue;
            }
            0x62ef61 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.r[2].wrapping_add(0xfffffff0));
            pc = 0x62f066; continue;
            }
            0x62ef6d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x7u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xfffffff1u32,32); c.set(1,32,r); }
            pc = 0x62f06a; continue;
            }
            0x62ef80 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffff4u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xc));
            pc = 0x62f06a; continue;
            }
            0x62ef93 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffffau32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0x10));
            pc = 0x62f06a; continue;
            }
            0x62efa6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.r[2].wrapping_add(0x11));
            pc = 0x62f06a; continue;
            }
            0x62efb6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x6u32,32); c.set(1,32,r); }
            pc = 0x62f06a; continue;
            }
            0x62efc9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xc));
            pc = 0x62f06a; continue;
            }
            0x62efdc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0x7));
            pc = 0x62f06a; continue;
            }
            0x62efec => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.r[2].wrapping_add(0x10));
            pc = 0x62f066; continue;
            }
            0x62eff5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffff9u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xfu32,32); c.set(1,32,r); }
            pc = 0x62f06a; continue;
            }
            0x62f005 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xfffffff4));
            pc = 0x62f06a; continue;
            }
            0x62f015 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x6u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xfffffff0));
            pc = 0x62f06a; continue;
            }
            0x62f025 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.r[2].wrapping_add(0xffffffef));
            pc = 0x62f06a; continue;
            }
            0x62f032 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffff0u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xfffffffau32,32); c.set(1,32,r); }
            pc = 0x62f06a; continue;
            }
            0x62f042 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffff4u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xfffffff4));
            pc = 0x62f06a; continue;
            }
            0x62f052 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffff1u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(0xfffffff9));
            pc = 0x62f06a; continue;
            }
            0x62f062 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x62f066; continue;
            }
            0x62f066 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x62f06a; continue;
            }
            0x62f06a => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0x300025u32);
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62f089);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x62f0a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.add(a,0x3008u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x17u32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62f0dc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x5d3),8));
            c.push(m,0xee5cu32);
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x62f0f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62f12f; continue; }
            pc = 0x62f0f9; continue;
            }
            0x62f0f9 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee5cu32);
            c.push(m, 0x62f109);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x62f110);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x11a4u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62f12f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x62f12f; continue;
            }
            0x62f12f => {
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x250),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0x1388u32,32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_forget_gate_marker(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x630a30;
    loop {
        match pc {
            0x630a30 => {
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(3,8)); }
            pc = 0x630a3d; continue;
            }
            0x630a3d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x5fc),32),32); }
            if c.cond(4) { pc = 0x630a63; continue; }
            pc = 0x630a52; continue;
            }
            0x630a52 => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(3,8)); }
            if c.cond(2) { pc = 0x630a3d; continue; }
            pc = 0x630a5d; continue;
            }
            0x630a5d => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x630a63 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x5fc); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_get_distance_to_tracked_entity(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6319d0;
    loop {
        match pc {
            0x6319d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x574),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6319e9; continue; }
            pc = 0x6319e0; continue;
            }
            0x6319e0 => {
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6319e9 => {
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x631a03);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x8),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0xc),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x10),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x4));
            c.fstp64(m,c.r[4].wrapping_add(0x4));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x631a42);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x63aae4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_get_unit_pitch_demand(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x632690;
    loop {
        match pc {
            0x632690 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x6326d3; continue; }
            pc = 0x632697; continue;
            }
            0x632697 => {
            { let a = rm(m,c.r[1].wrapping_add(0x42),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x6326d3; continue; }
            pc = 0x63269e; continue;
            }
            0x63269e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x23e),16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x23a),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x236),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x21e),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x6326cc; continue; }
            pc = 0x6326ca; continue;
            }
            0x6326ca => {
            c.set(0,32,c.get(2,32));
            pc = 0x6326cc; continue;
            }
            0x6326cc => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6326d3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_get_building_pitch_demand(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6326e0;
    loop {
        match pc {
            0x6326e0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x632771; continue; }
            pc = 0x6326ee; continue;
            }
            0x6326ee => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x632771; continue; }
            pc = 0x6326f5; continue;
            }
            0x6326f5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x42),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x632771; continue; }
            pc = 0x6326fc; continue;
            }
            0x6326fc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6d),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x244),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x632771; continue; }
            pc = 0x63272e; continue;
            }
            0x63272e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x632734);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x63276a; continue; }
            pc = 0x63273a; continue;
            }
            0x63273a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6d),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x244),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x632764);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x35u32,8); }
            if c.cond(5) { pc = 0x632771; continue; }
            pc = 0x63276a; continue;
            }
            0x63276a => {
            c.set(0,32,0x190u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x632771 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_tick_timer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6329a0;
    loop {
        match pc {
            0x6329a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x6329b5; continue; }
            pc = 0x6329ad; continue;
            }
            0x6329ad => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x6329b5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x77),8));
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x6329e2; continue; }
            pc = 0x6329c6; continue;
            }
            0x6329c6 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6329d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6329e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62dd70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6329e2; continue;
            }
            0x6329e2 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6334b0;
    loop {
        match pc {
            0x6334b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,0xffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1013); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1012); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1011); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1010); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1018); wm(m,a,16,0x3fffu32); }
            { let a = c.r[6].wrapping_add(0x101a); wm(m,a,8,0x2u32); }
            { let a = c.r[6].wrapping_add(0x1017); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x1016); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x1015); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x1014); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x101c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1020); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x101b); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x633518);
            { let args = []; c.r[0] = host.call(m, 0x6339c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6335a0;
    loop {
        match pc {
            0x6335a0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x6335be);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6335cd);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x1018));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6335dd);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x101a));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6335ef);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x101b));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6335ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x633608; continue;
            }
            0x633608 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x1010));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x633622);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x1014));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x63363d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x633608; continue; }
            pc = 0x63364f; continue;
            }
            0x63364f => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x101c),32));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x633664; continue; }
            pc = 0x63365b; continue;
            }
            0x63365b => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63365b; continue; }
            pc = 0x633664; continue;
            }
            0x633664 => {
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x633676);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6336a2; continue; }
            pc = 0x633682; continue;
            }
            0x633682 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x101c),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x6336a2; continue; }
            pc = 0x63368c; continue;
            }
            0x63368c => {
            c.push(m,0x8u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x633695);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x63368c; continue; }
            pc = 0x6336a2; continue;
            }
            0x6336a2 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1020),32));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6336b7; continue; }
            pc = 0x6336ae; continue;
            }
            0x6336ae => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6336ae; continue; }
            pc = 0x6336b7; continue;
            }
            0x6336b7 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6336c8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6336f4; continue; }
            pc = 0x6336d4; continue;
            }
            0x6336d4 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x1020),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x6336f4; continue; }
            pc = 0x6336de; continue;
            }
            0x6336de => {
            c.push(m,0x10u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6336e7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6336de; continue; }
            pc = 0x6336f4; continue;
            }
            0x6336f4 => {
            c.set(2,32,c.r[4].wrapping_add(0xf));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x633701);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1b),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x63371d; continue; }
            pc = 0x63370e; continue;
            }
            0x63370e => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x633717);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x627920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x63371d => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x633730;
    loop {
        match pc {
            0x633730 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6684d6u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m, 0x633752);
            { let args = []; c.r[0] = host.call(m, 0x6339c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(6,32); c.sub(a,0x56u32,32); }
            if c.cond(6) { pc = 0x63393e; continue; }
            pc = 0x63375f; continue;
            }
            0x63375f => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x63376c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x63377b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x1018));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x63378b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.r[7].wrapping_add(0x101a));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x63379d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,c.r[7].wrapping_add(0x101b));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6337ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            pc = 0x6337b7; continue;
            }
            0x6337b7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x1010));
            c.push(m,c.get(1,32));
            c.push(m, 0x6337d0);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x1014));
            c.push(m,c.get(0,32));
            c.push(m, 0x6337eb);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x6337b7; continue; }
            pc = 0x6337fd; continue;
            }
            0x6337fd => {
            c.push(m,c.get(5,32));
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x63380b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            { let a = c.r[7].wrapping_add(0x101c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x633865; continue; }
            pc = 0x63381d; continue;
            }
            0x63381d => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x633821; continue;
            }
            0x633821 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x101c),32));
            c.push(m,0xcu32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x633837; continue; }
            pc = 0x63382d; continue;
            }
            0x63382d => {
            c.push(m, 0x633832);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x633842; continue;
            }
            0x633837 => {
            c.push(m, 0x63383c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(0x101c); wm(m,a,32,c.get(0,32)); }
            pc = 0x633842; continue;
            }
            0x633842 => {
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            c.push(m,0x8u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x633850);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x633821; continue; }
            pc = 0x633862; continue;
            }
            0x633862 => {
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x633865; continue;
            }
            0x633865 => {
            c.push(m,0x1u32);
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x633872);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6338c6; continue; }
            pc = 0x63387e; continue;
            }
            0x63387e => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x633882; continue;
            }
            0x633882 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1020),32));
            c.push(m,0x14u32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x633898; continue; }
            pc = 0x63388e; continue;
            }
            0x63388e => {
            c.push(m, 0x633893);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x6338a3; continue;
            }
            0x633898 => {
            c.push(m, 0x63389d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(0x1020); wm(m,a,32,c.get(0,32)); }
            pc = 0x6338a3; continue;
            }
            0x6338a3 => {
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            c.push(m,0x10u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x6338b1);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x633882; continue; }
            pc = 0x6338c3; continue;
            }
            0x6338c3 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x6338c6; continue;
            }
            0x6338c6 => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[4].wrapping_add(0x17));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6338d3);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x63399e; continue; }
            pc = 0x6338e4; continue;
            }
            0x6338e4 => {
            c.push(m,0x650u32);
            c.push(m, 0x6338ee);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x633906; continue; }
            pc = 0x6338fd; continue;
            }
            0x6338fd => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x633904);
            { let args = []; c.r[0] = host.call(m, 0x627880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x633908; continue;
            }
            0x633906 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x633908; continue;
            }
            0x633908 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x5d3); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x63392a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x627ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x63393e => {
            c.push(m,0x650u32);
            c.push(m, 0x633948);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x633966; continue; }
            pc = 0x63395d; continue;
            }
            0x63395d => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x633964);
            { let args = []; c.r[0] = host.call(m, 0x627880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x633968; continue;
            }
            0x633966 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x633968; continue;
            }
            0x633968 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x5d3); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x63398a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x627ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x63399e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_update_scene(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x633a90;
    loop {
        match pc {
            0x633a90 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x633ad1; continue; }
            pc = 0x633a9b; continue;
            }
            0x633a9b => {
            c.push(m, 0x633aa0);
            { let args = []; c.r[0] = host.call(m, 0x6281f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m, 0x633aa8);
            { let args = []; c.r[0] = host.call(m, 0x629c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x633ad1; continue; }
            pc = 0x633aac; continue;
            }
            0x633aac => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x633ad1; continue; }
            pc = 0x633ab6; continue;
            }
            0x633ab6 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x633aca; continue; }
            pc = 0x633aba; continue;
            }
            0x633aba => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x633ac1);
            { let args = []; c.r[0] = host.call(m, 0x6278e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x633ac7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x633aca; continue;
            }
            0x633aca => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            pc = 0x633ad1; continue;
            }
            0x633ad1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_plunder_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x633db0;
    loop {
        match pc {
            0x633db0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6684fdu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x64u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xeu32,32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x633e03; continue; }
            pc = 0x633de5; continue;
            }
            0x633de5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x70u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x633e03 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d8),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2dc),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2e0),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x700),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x633ec3; continue; }
            pc = 0x633e67; continue;
            }
            0x633e67 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[7],8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x633ea0; continue; }
            pc = 0x633e98; continue;
            }
            0x633e98 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            pc = 0x633ea0; continue;
            }
            0x633ea0 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x11c),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x118),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x633ecb; continue;
            }
            0x633ec3 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x4e20u32); }
            pc = 0x633ecb; continue;
            }
            0x633ecb => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            c.set(5,32,0x673a18u32);
            pc = 0x633ed2; continue;
            }
            0x633ed2 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            if c.cond(6) { pc = 0x633f13; continue; }
            pc = 0x633eda; continue;
            }
            0x633eda => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.fld(c.ld(m,c.r[5],4));
            c.set(3,8,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x5f),8));
            c.set(6,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x60));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let v = c.ld(m,c.r[0].wrapping_mul(4).wrapping_add(0x673a18),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x633f17; continue; }
            pc = 0x633f07; continue;
            }
            0x633f07 => {
            c.set(0,8,c.get(2,8));
            { let a = c.r[6]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x633eda; continue; }
            pc = 0x633f13; continue;
            }
            0x633f13 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x633f17; continue;
            }
            0x633f17 => {
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(c.r[2]).wrapping_add(0x60); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x14u32,8); }
            if c.cond(2) { pc = 0x633ed2; continue; }
            pc = 0x633f2b; continue;
            }
            0x633f2b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.r[3]; wm(m,a,16,0x0u32); }
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x6341e7; continue; }
            pc = 0x633f77; continue;
            }
            0x633f77 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x633f82);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x8c),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x633fb0; continue; }
            pc = 0x633f97; continue;
            }
            0x633f97 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6341ba; continue; }
            pc = 0x633f9f; continue;
            }
            0x633f9f => {
            c.set(1,16,rm(m,c.r[7],8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x2c4),16),16); }
            if c.cond(4) { pc = 0x63414a; continue; }
            pc = 0x633fb0; continue;
            }
            0x633fb0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[2],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x63414a; continue; }
            pc = 0x633fcb; continue;
            }
            0x633fcb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2c4),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x633fdf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x63413b; continue; }
            pc = 0x633fea; continue;
            }
            0x633fea => {
            c.set(0,32,c.r[4].wrapping_add(0x60));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x14u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x633ffa; continue;
            }
            0x633ffa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x634107; continue; }
            pc = 0x63401c; continue;
            }
            0x63401c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(6,32)); }
            c.set(3,32,rm(m,c.r[1],32));
            c.fild64(m,c.r[4].wrapping_add(0x40));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[5].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x634051);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x634098; continue; }
            pc = 0x634061; continue;
            }
            0x634061 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[2]));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x63419a; continue; }
            pc = 0x63406c; continue;
            }
            0x63406c => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x48));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[5].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('/',v); }
            c.push(m, 0x634096);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            pc = 0x634098; continue;
            }
            0x634098 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x6340ab; continue; }
            pc = 0x6340a0; continue;
            }
            0x6340a0 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(6,32,c.get(0,32));
            if c.cond(4) { pc = 0x6340ab; continue; }
            pc = 0x6340a6; continue;
            }
            0x6340a6 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x6340ab; continue;
            }
            0x6340ab => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0xc8u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            if c.cond(4) { pc = 0x634107; continue; }
            pc = 0x6340ba; continue;
            }
            0x6340ba => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6340c2; continue; }
            pc = 0x6340be; continue;
            }
            0x6340be => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54),32); let r = c.add(a,c.get(6,32),32); { let a = c.r[7].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,r); } }
            pc = 0x6340c2; continue;
            }
            0x6340c2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x0u32); }
            c.fild64(m,c.r[4].wrapping_add(0x50));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(6,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let a = c.r[0].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let v = c.ld(m,c.r[5].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x634103);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x634107; continue;
            }
            0x634107 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x633ffa; continue; }
            pc = 0x63411f; continue;
            }
            0x63411f => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6341ba; continue; }
            pc = 0x634127; continue;
            }
            0x634127 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63413b; continue; }
            pc = 0x63412f; continue;
            }
            0x63412f => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x634136);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x63413b; continue;
            }
            0x63413b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x88),32));
            pc = 0x63414a; continue;
            }
            0x63414a => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6341ba; continue; }
            pc = 0x63414e; continue;
            }
            0x63414e => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x6341ba; continue; }
            pc = 0x634156; continue;
            }
            0x634156 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x6341a1; continue; }
            pc = 0x634163; continue;
            }
            0x634163 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x11c),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x118),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x6341d5; continue;
            }
            0x63419a => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x6340ab; continue;
            }
            0x6341a1 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            { let a = rm(m,c.r[3],16); let r = c.inc(a,16); { let a = c.r[3]; wm(m,a,16,r); } }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x6341d5; continue;
            }
            0x6341ba => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(0x2ca),16));
            { let a = rm(m,c.r[3],16); let r = c.inc(a,16); { let a = c.r[3]; wm(m,a,16,r); } }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            pc = 0x6341d5; continue;
            }
            0x6341d5 => {
            c.set(0,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x633f77; continue; }
            pc = 0x6341e7; continue;
            }
            0x6341e7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x63444b; continue; }
            pc = 0x634202; continue;
            }
            0x634202 => {
            c.set(0,32,c.r[4].wrapping_add(0x60));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x14u32); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            pc = 0x634215; continue;
            }
            0x634215 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6343ba; continue; }
            pc = 0x634252; continue;
            }
            0x634252 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6343ba; continue; }
            pc = 0x634260; continue;
            }
            0x634260 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x0u32); }
            c.fild64(m,c.r[4].wrapping_add(0x50));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            c.set(3,32,rm(m,c.r[2],32));
            c.push(m, 0x63428d);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x6342da; continue; }
            pc = 0x63429d; continue;
            }
            0x63429d => {
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[3].wrapping_add(c.r[0]));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x63431f; continue; }
            pc = 0x6342aa; continue;
            }
            0x6342aa => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x48));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('/',v); }
            c.push(m, 0x6342d8);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            pc = 0x6342da; continue;
            }
            0x6342da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x6342ed; continue; }
            pc = 0x6342e2; continue;
            }
            0x6342e2 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(5,32,c.get(0,32));
            if c.cond(4) { pc = 0x6342ed; continue; }
            pc = 0x6342e8; continue;
            }
            0x6342e8 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x6342ed; continue;
            }
            0x6342ed => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0xc8u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            if c.cond(4) { pc = 0x6343ba; continue; }
            pc = 0x634300; continue;
            }
            0x634300 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x634361; continue; }
            pc = 0x634306; continue;
            }
            0x634306 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x634323; continue; }
            pc = 0x634312; continue;
            }
            0x634312 => {
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            pc = 0x634361; continue;
            }
            0x63431f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x6342ed; continue;
            }
            0x634323 => {
            c.fild32(m,c.r[4].wrapping_add(0x38));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4),4); c.fop('*',v); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.fstp32(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4));
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),4); c.fop('/',v); }
            c.fstp32(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xb4));
            pc = 0x634361; continue;
            }
            0x634361 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            c.fild64(m,c.r[4].wrapping_add(0x40));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let v = c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x6343b6);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x6343ba; continue;
            }
            0x6343ba => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x634215; continue; }
            pc = 0x6343d8; continue;
            }
            0x6343d8 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x63444b; continue; }
            pc = 0x6343dc; continue;
            }
            0x6343dc => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6343eb; continue; }
            pc = 0x6343e4; continue;
            }
            0x6343e4 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6343eb);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6343eb; continue;
            }
            0x6343eb => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63444b; continue; }
            pc = 0x6343f3; continue;
            }
            0x6343f3 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x63444b; continue; }
            pc = 0x634400; continue;
            }
            0x634400 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x11c),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x118),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x634202; continue; }
            pc = 0x63444b; continue;
            }
            0x63444b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x6345e9; continue; }
            pc = 0x63445f; continue;
            }
            0x63445f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15d),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x6345e9; continue; }
            pc = 0x63449f; continue;
            }
            0x63449f => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x6345e9; continue; }
            pc = 0x6344b0; continue;
            }
            0x6344b0 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6344c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,16,rm(m,c.r[7].wrapping_add(0x42),16));
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4b8),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x16),16));
            { let a = c.r[6].wrapping_add(0x4b8); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,0x672de4u32,16));
            { let a = c.get(5,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,16,c.get(0,16)); }
            if c.cond(14) { pc = 0x63454b; continue; }
            pc = 0x634523; continue;
            }
            0x634523 => {
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,0x7fffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(c.get(1,16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x634543; continue; }
            pc = 0x634534; continue;
            }
            0x634534 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            pc = 0x634572; continue;
            }
            0x634543 => {
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,0x7fffu32); }
            pc = 0x634572; continue;
            }
            0x63454b => {
            c.set(2,32,sx(c.get(1,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,0x8000u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x63456c; continue; }
            pc = 0x63455d; continue;
            }
            0x63455d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            pc = 0x634572; continue;
            }
            0x63456c => {
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,0x8000u32); }
            pc = 0x634572; continue;
            }
            0x634572 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x6345e9; continue; }
            pc = 0x634576; continue;
            }
            0x634576 => {
            c.set(1,32,c.r[4].wrapping_add(0x50));
            c.push(m, 0x63457f);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15e),8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x59); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,8); let n = 0x2u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x5b); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x15d),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x6au32); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x62); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x6345d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x50));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x6345e9);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6345e9; continue;
            }
            0x6345e9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,rm(m,c.r[2],32));
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x634614; continue; }
            pc = 0x634600; continue;
            }
            0x634600 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x70u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x634614 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x634636);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x529f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x68),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x70u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_refill_goodwill(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x634c50;
    loop {
        match pc {
            0x634c50 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(6,32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x634cb6; continue; }
            pc = 0x634c59; continue;
            }
            0x634c59 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x634cb6; continue; }
            pc = 0x634c67; continue;
            }
            0x634c67 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x634c6a; continue;
            }
            0x634c6a => {
            c.set(6,32,c.get(0,32));
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x4),8));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(8)));
            c.set(6,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1020),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x634c98; continue; }
            pc = 0x634c88; continue;
            }
            0x634c88 => {
            { let a = rm(m,c.r[0].wrapping_add(0x6),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x634c91; continue; }
            pc = 0x634c8d; continue;
            }
            0x634c8d => {
            { let a = c.r[0].wrapping_add(0x9); wm(m,a,8,0x1u32); }
            pc = 0x634c91; continue;
            }
            0x634c91 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x634c88; continue; }
            pc = 0x634c98; continue;
            }
            0x634c98 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x634c6a; continue; }
            pc = 0x634cb5; continue;
            }
            0x634cb5 => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x634cb6; continue;
            }
            0x634cb6 => {
            { let a = rm(m,c.r[1].wrapping_add(0x101b),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x634cf9; continue; }
            pc = 0x634cbf; continue;
            }
            0x634cbf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,0xa001u32);
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,0x6de4b4u32,32),32); c.set(0,32,r); }
            { let b = c.get(6,32); c.div32(b); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(2,32); let r = c.add(a,0x2800u32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x1018); wm(m,a,16,c.get(2,16)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x634cf9 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x1018),16));
            { let a = c.get(0,16); c.sub(a,0x4fffu32,16); }
            if c.cond(7) { pc = 0x634d3e; continue; }
            pc = 0x634d06; continue;
            }
            0x634d06 => {
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x1000u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x1fffu32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4fffu32,32); }
            if c.cond(6) { pc = 0x634d35; continue; }
            pc = 0x634d28; continue;
            }
            0x634d28 => {
            { let a = c.r[1].wrapping_add(0x1018); wm(m,a,16,0x4fffu32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x634d35 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x1018); wm(m,a,16,c.get(0,16)); }
            pc = 0x634d3e; continue;
            }
            0x634d3e => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_process_delivery_agreements(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x635440;
    loop {
        match pc {
            0x635440 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x101c),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x635470; continue; }
            pc = 0x635457; continue;
            }
            0x635457 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            pc = 0x635459; continue;
            }
            0x635459 => {
            { let a = rm(m,c.r[6].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x635474; continue; }
            pc = 0x63545f; continue;
            }
            0x63545f => {
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,0x0u32); }
            pc = 0x635463; continue;
            }
            0x635463 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x8),32));
            pc = 0x63546a; continue;
            }
            0x63546a => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x635459; continue; }
            pc = 0x63546e; continue;
            }
            0x63546e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x635470; continue;
            }
            0x635470 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x635474 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x4),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(3) { pc = 0x635608; continue; }
            pc = 0x63549c; continue;
            }
            0x63549c => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x6354ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x6354ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = rm(m,c.r[0].wrapping_add(0x2c4),16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x6354dd; continue; }
            pc = 0x6354c8; continue;
            }
            0x6354c8 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x2ca),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(2) { pc = 0x63549c; continue; }
            pc = 0x6354d8; continue;
            }
            0x6354d8 => {
            pc = 0x635608; continue;
            }
            0x6354dd => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6],8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6354fc; continue; }
            pc = 0x6354eb; continue;
            }
            0x6354eb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x63550e; continue;
            }
            0x6354fc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x63550e; continue;
            }
            0x63550e => {
            { let a = rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x6355e4; continue; }
            pc = 0x635518; continue;
            }
            0x635518 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x635528);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4c8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x4c8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(5,32,0x4fffu32);
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(7,16,rm(m,c.r[3].wrapping_add(0x1018),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(5,16),16); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            if c.cond(7) { pc = 0x6355df; continue; }
            pc = 0x635581; continue;
            }
            0x635581 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6],8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x635592; continue; }
            pc = 0x63558f; continue;
            }
            0x63558f => {
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x635592; continue;
            }
            0x635592 => {
            { let a = c.get(1,32); c.sub(a,0x3e8u32,32); }
            if c.cond(2) { pc = 0x6355ab; continue; }
            pc = 0x63559a; continue;
            }
            0x63559a => {
            { let a = c.r[3].wrapping_add(0x1018); wm(m,a,16,c.get(5,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,c.get(0,8)); }
            pc = 0x635608; continue;
            }
            0x6355ab => {
            { let a = c.get(1,32); let n = 0x9u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x6355d6; continue; }
            pc = 0x6355c5; continue;
            }
            0x6355c5 => {
            { let a = c.r[3].wrapping_add(0x1018); wm(m,a,16,c.get(5,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,c.get(0,8)); }
            pc = 0x635608; continue;
            }
            0x6355d6 => {
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[3].wrapping_add(0x1018); wm(m,a,16,c.get(7,16)); }
            pc = 0x6355df; continue;
            }
            0x6355df => {
            { let a = rm(m,c.r[6].wrapping_add(0x2),8); let r = c.inc(a,8); { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,r); } }
            pc = 0x635608; continue;
            }
            0x6355e4 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1018),16));
            { let a = c.get(0,16); c.sub(a,0x19au32,16); }
            if c.cond(6) { pc = 0x6355ff; continue; }
            pc = 0x6355f1; continue;
            }
            0x6355f1 => {
            { let a = c.get(0,32); let r = c.add(a,0xfffffe66u32,32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x1018); wm(m,a,16,c.get(0,16)); }
            pc = 0x635608; continue;
            }
            0x6355ff => {
            { let a = c.r[3].wrapping_add(0x1018); wm(m,a,16,0x1u32); }
            pc = 0x635608; continue;
            }
            0x635608 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x635463; continue; }
            pc = 0x635616; continue;
            }
            0x635616 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x635635; continue; }
            pc = 0x63561f; continue;
            }
            0x63561f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x63562a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x63546a; continue;
            }
            0x635635 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[3].wrapping_add(0x101c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x635643);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x101c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x63546a; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_ship_surplus_wares(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x635740;
    loop {
        match pc {
            0x635740 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x101b),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x635765; continue; }
            pc = 0x635750; continue;
            }
            0x635750 => {
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x635896; continue; }
            pc = 0x635758; continue;
            }
            0x635758 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[5].wrapping_add(0x101b); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x635765 => {
            { let a = rm(m,c.r[5].wrapping_add(0x1018),16); c.sub(a,0x2000u32,16); }
            if c.cond(3) { pc = 0x635896; continue; }
            pc = 0x635774; continue;
            }
            0x635774 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x4),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x1010));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,c.r[3].wrapping_add(0x4f0));
            pc = 0x6357a5; continue;
            }
            0x6357a5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x635878; continue; }
            pc = 0x6357b3; continue;
            }
            0x6357b3 => {
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(6,32,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x635878; continue; }
            pc = 0x6357ca; continue;
            }
            0x6357ca => {
            c.set(0,32,rm(m,c.r[1],32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6357ef; continue; }
            pc = 0x6357da; continue;
            }
            0x6357da => {
            c.set(0,32,0x51eb851fu32);
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x635805; continue;
            }
            0x6357ef => {
            c.set(0,32,0x10624dd3u32);
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x635805; continue;
            }
            0x635805 => {
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.r[3].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x1018),16));
            { let a = c.get(6,16); c.sub(a,0x4fffu32,16); }
            if c.cond(7) { pc = 0x635871; continue; }
            pc = 0x635819; continue;
            }
            0x635819 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = rm(m,c.r[0].wrapping_add(0x672c14),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x63582d; continue; }
            pc = 0x63582a; continue;
            }
            0x63582a => {
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            pc = 0x63582d; continue;
            }
            0x63582d => {
            { let a = c.get(2,32); c.sub(a,0x3e8u32,32); }
            if c.cond(2) { pc = 0x635840; continue; }
            pc = 0x635835; continue;
            }
            0x635835 => {
            { let a = c.r[5].wrapping_add(0x1018); wm(m,a,16,0x4fffu32); }
            pc = 0x635871; continue;
            }
            0x635840 => {
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(6,32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4fffu32,32); }
            if c.cond(6) { pc = 0x635868; continue; }
            pc = 0x63585d; continue;
            }
            0x63585d => {
            { let a = c.r[5].wrapping_add(0x1018); wm(m,a,16,0x4fffu32); }
            pc = 0x635871; continue;
            }
            0x635868 => {
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0x1018); wm(m,a,16,c.get(6,16)); }
            pc = 0x635871; continue;
            }
            0x635871 => {
            { let a = c.r[5].wrapping_add(0x101b); wm(m,a,8,0xcu32); }
            pc = 0x635878; continue;
            }
            0x635878 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6357a5; continue; }
            pc = 0x635893; continue;
            }
            0x635893 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x635896; continue;
            }
            0x635896 => {
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "siege_unit_dtor" => siege_unit_dtor(m, host, regs),
        "siege_unit_free_variant_tables" => siege_unit_free_variant_tables(m, host, regs),
        "siege_unit_read_variant_record" => siege_unit_read_variant_record(m, host, regs),
        "siege_unit_draw_mask" => siege_unit_draw_mask(m, host, regs),
        "siege_unit_noop" => siege_unit_noop(m, host, regs),
        "siege_unit_load_new_sprite_variant" => siege_unit_load_new_sprite_variant(m, host, regs),
        "siege_unit_deleting_dtor" => siege_unit_deleting_dtor(m, host, regs),
        "siege_unit_deleting_dtor_thunk_08" => siege_unit_deleting_dtor_thunk_08(m, host, regs),
        "siege_unit_deleting_dtor_thunk_1c" => siege_unit_deleting_dtor_thunk_1c(m, host, regs),
        "siege_unit_deleting_dtor_thunk_20" => siege_unit_deleting_dtor_thunk_20(m, host, regs),
        "siege_walker_dtor" => siege_walker_dtor(m, host, regs),
        "siege_walker_free_variant_tables" => siege_walker_free_variant_tables(m, host, regs),
        "siege_walker_read_variant_record" => siege_walker_read_variant_record(m, host, regs),
        "siege_walker_load_new_sprite_variant" => siege_walker_load_new_sprite_variant(m, host, regs),
        "siege_walker_remove_from_tile_lists" => siege_walker_remove_from_tile_lists(m, host, regs),
        "siege_walker_draw_sprite" => siege_walker_draw_sprite(m, host, regs),
        "siege_walker_draw_shadow" => siege_walker_draw_shadow(m, host, regs),
        "siege_walker_get_sprite_set" => siege_walker_get_sprite_set(m, host, regs),
        "siege_walker_deleting_dtor" => siege_walker_deleting_dtor(m, host, regs),
        "siege_walker_deleting_dtor_thunk_08" => siege_walker_deleting_dtor_thunk_08(m, host, regs),
        "siege_walker_deleting_dtor_thunk_1c" => siege_walker_deleting_dtor_thunk_1c(m, host, regs),
        "siege_fighter_dtor" => siege_fighter_dtor(m, host, regs),
        "siege_fighter_release_sprite_variants" => siege_fighter_release_sprite_variants(m, host, regs),
        "siege_fighter_load_sprite_set" => siege_fighter_load_sprite_set(m, host, regs),
        "siege_fighter_remove_from_tile_lists" => siege_fighter_remove_from_tile_lists(m, host, regs),
        "siege_fighter_can_engage" => siege_fighter_can_engage(m, host, regs),
        "siege_fighter_draw_sprite" => siege_fighter_draw_sprite(m, host, regs),
        "siege_fighter_draw_shadow" => siege_fighter_draw_shadow(m, host, regs),
        "siege_fighter_get_unit_class" => siege_fighter_get_unit_class(m, host, regs),
        "siege_fighter_get_sprite_set" => siege_fighter_get_sprite_set(m, host, regs),
        "siege_fighter_deleting_dtor" => siege_fighter_deleting_dtor(m, host, regs),
        "siege_fighter_deleting_dtor_thunk_08" => siege_fighter_deleting_dtor_thunk_08(m, host, regs),
        "siege_fighter_deleting_dtor_thunk_1c" => siege_fighter_deleting_dtor_thunk_1c(m, host, regs),
        "siege_fighter_deleting_dtor_thunk_20" => siege_fighter_deleting_dtor_thunk_20(m, host, regs),
        "siege_fighter_deleting_dtor_thunk_2c" => siege_fighter_deleting_dtor_thunk_2c(m, host, regs),
        "siege_walker_type0b_release_static_resources" => siege_walker_type0b_release_static_resources(m, host, regs),
        "siege_walker_type0b_parse_variant_record" => siege_walker_type0b_parse_variant_record(m, host, regs),
        "siege_walker_type0b_load_sprite_set" => siege_walker_type0b_load_sprite_set(m, host, regs),
        "siege_walker_type0b_remove_from_tile_lists" => siege_walker_type0b_remove_from_tile_lists(m, host, regs),
        "siege_walker_type0b_draw_sprite" => siege_walker_type0b_draw_sprite(m, host, regs),
        "siege_walker_type0b_draw_shadow" => siege_walker_type0b_draw_shadow(m, host, regs),
        "siege_walker_type0b_get_set_header_byte" => siege_walker_type0b_get_set_header_byte(m, host, regs),
        "siege_walker_type0b_get_unit_class" => siege_walker_type0b_get_unit_class(m, host, regs),
        "siege_walker_type0b_get_sprite_set" => siege_walker_type0b_get_sprite_set(m, host, regs),
        "siege_walker_type0b_deleting_dtor" => siege_walker_type0b_deleting_dtor(m, host, regs),
        "siege_walker_type0b_deleting_dtor_thunk_08" => siege_walker_type0b_deleting_dtor_thunk_08(m, host, regs),
        "siege_walker_type0b_deleting_dtor_thunk_1c" => siege_walker_type0b_deleting_dtor_thunk_1c(m, host, regs),
        "siege_walker_type0b_deleting_dtor_thunk_20" => siege_walker_type0b_deleting_dtor_thunk_20(m, host, regs),
        "siege_walker_type0b_deleting_dtor_thunk_2c" => siege_walker_type0b_deleting_dtor_thunk_2c(m, host, regs),
        "siege_cart_dtor" => siege_cart_dtor(m, host, regs),
        "siege_cart_draw_mask" => siege_cart_draw_mask(m, host, regs),
        "siege_cart_noop" => siege_cart_noop(m, host, regs),
        "siege_projectile_dtor" => siege_projectile_dtor(m, host, regs),
        "siege_projectile_free_variant_tables" => siege_projectile_free_variant_tables(m, host, regs),
        "siege_projectile_draw_mask" => siege_projectile_draw_mask(m, host, regs),
        "siege_projectile_noop" => siege_projectile_noop(m, host, regs),
        "siege_projectile_read_variant_record" => siege_projectile_read_variant_record(m, host, regs),
        "siege_projectile_load_sprite_set" => siege_projectile_load_sprite_set(m, host, regs),
        "siege_projectile_anim_end_noop" => siege_projectile_anim_end_noop(m, host, regs),
        "siege_projectile_get_sprite_set" => siege_projectile_get_sprite_set(m, host, regs),
        "siege_projectile_deleting_dtor" => siege_projectile_deleting_dtor(m, host, regs),
        "siege_projectile_deleting_dtor_thunk_08" => siege_projectile_deleting_dtor_thunk_08(m, host, regs),
        "siege_projectile_deleting_dtor_thunk_1c" => siege_projectile_deleting_dtor_thunk_1c(m, host, regs),
        "siege_projectile_deleting_dtor_thunk_20" => siege_projectile_deleting_dtor_thunk_20(m, host, regs),
        "siege_fighter_walker_dtor" => siege_fighter_walker_dtor(m, host, regs),
        "siege_fighter_walker_free_variant_tables" => siege_fighter_walker_free_variant_tables(m, host, regs),
        "siege_fighter_walker_read_variant_record" => siege_fighter_walker_read_variant_record(m, host, regs),
        "siege_fighter_walker_load_sprite_set" => siege_fighter_walker_load_sprite_set(m, host, regs),
        "siege_fighter_walker_remove_from_tile_lists" => siege_fighter_walker_remove_from_tile_lists(m, host, regs),
        "siege_fighter_walker_draw_sprite" => siege_fighter_walker_draw_sprite(m, host, regs),
        "siege_fighter_walker_draw_shadow" => siege_fighter_walker_draw_shadow(m, host, regs),
        "siege_fighter_walker_get_set_header_byte" => siege_fighter_walker_get_set_header_byte(m, host, regs),
        "siege_fighter_walker_get_sprite_set" => siege_fighter_walker_get_sprite_set(m, host, regs),
        "siege_fighter_walker_deleting_dtor" => siege_fighter_walker_deleting_dtor(m, host, regs),
        "siege_fighter_walker_deleting_dtor_thunk_08" => siege_fighter_walker_deleting_dtor_thunk_08(m, host, regs),
        "siege_fighter_walker_deleting_dtor_thunk_1c" => siege_fighter_walker_deleting_dtor_thunk_1c(m, host, regs),
        "siege_fighter_walker_deleting_dtor_thunk_20" => siege_fighter_walker_deleting_dtor_thunk_20(m, host, regs),
        "siege_fighter_walker_deleting_dtor_thunk_2c" => siege_fighter_walker_deleting_dtor_thunk_2c(m, host, regs),
        "siege_fixture_type19_ctor" => siege_fixture_type19_ctor(m, host, regs),
        "siege_fixture_type19_dtor" => siege_fixture_type19_dtor(m, host, regs),
        "siege_fixture_type19_move_to" => siege_fixture_type19_move_to(m, host, regs),
        "siege_fixture_type19_erase" => siege_fixture_type19_erase(m, host, regs),
        "siege_fixture_type19_update" => siege_fixture_type19_update(m, host, regs),
        "siege_fixture_type19_load_sprite_set" => siege_fixture_type19_load_sprite_set(m, host, regs),
        "siege_fixture_type19_release_static_resources" => siege_fixture_type19_release_static_resources(m, host, regs),
        "siege_fixture_type19_get_sprite_set" => siege_fixture_type19_get_sprite_set(m, host, regs),
        "siege_fixture_type19_deleting_dtor" => siege_fixture_type19_deleting_dtor(m, host, regs),
        "siege_fixture_type19_deleting_dtor_thunk_08" => siege_fixture_type19_deleting_dtor_thunk_08(m, host, regs),
        "siege_fixture_type19_deleting_dtor_thunk_1c" => siege_fixture_type19_deleting_dtor_thunk_1c(m, host, regs),
        "siege_fixture_type19_deleting_dtor_thunk_20" => siege_fixture_type19_deleting_dtor_thunk_20(m, host, regs),
        "town_attack_battle_compute_checksum" => town_attack_battle_compute_checksum(m, host, regs),
        "town_attack_battle_serialize" => town_attack_battle_serialize(m, host, regs),
        "town_attack_battle_deserialize" => town_attack_battle_deserialize(m, host, regs),
        "town_attack_battle_handle_object_collision" => town_attack_battle_handle_object_collision(m, host, regs),
        "town_attack_battle_is_water_cell" => town_attack_battle_is_water_cell(m, host, regs),
        "town_attack_battle_enter_view" => town_attack_battle_enter_view(m, host, regs),
        "town_attack_battle_place_ships" => town_attack_battle_place_ships(m, host, regs),
        "town_attack_battle_open_for_player" => town_attack_battle_open_for_player(m, host, regs),
        "town_attack_battle_get_land_mask_cell" => town_attack_battle_get_land_mask_cell(m, host, regs),
        "town_attack_battle_projectile_reaches_defense_piece" => town_attack_battle_projectile_reaches_defense_piece(m, host, regs),
        "town_attack_battle_apply_damage_to_defense_piece" => town_attack_battle_apply_damage_to_defense_piece(m, host, regs),
        "town_attack_battle_order_selected_ships_attack_piece" => town_attack_battle_order_selected_ships_attack_piece(m, host, regs),
        "town_attack_battle_update_interface" => town_attack_battle_update_interface(m, host, regs),
        "town_attack_battle_add_reinforcement" => town_attack_battle_add_reinforcement(m, host, regs),
        "town_attack_battle_place_reinforcement_ship" => town_attack_battle_place_reinforcement_ship(m, host, regs),
        "town_attack_battle_is_land_attack" => town_attack_battle_is_land_attack(m, host, regs),
        "town_attack_battle_delete_this" => town_attack_battle_delete_this(m, host, regs),
        "sea_battle_projectile_serialized_size" => sea_battle_projectile_serialized_size(m, host, regs),
        "sea_battle_projectile_serialize" => sea_battle_projectile_serialize(m, host, regs),
        "sea_battle_projectile_update_sprite" => sea_battle_projectile_update_sprite(m, host, regs),
        "sea_battle_order_selected_ships_attack_ship" => sea_battle_order_selected_ships_attack_ship(m, host, regs),
        "sea_battle_order_selected_ships_move_to" => sea_battle_order_selected_ships_move_to(m, host, regs),
        "sea_battle_find_local_ship_by_ship_index" => sea_battle_find_local_ship_by_ship_index(m, host, regs),
        "sea_battle_generate_random_obstacles" => sea_battle_generate_random_obstacles(m, host, regs),
        "sea_battle_leave_view" => sea_battle_leave_view(m, host, regs),
        "sea_battle_restamp_obstacles" => sea_battle_restamp_obstacles(m, host, regs),
        "sea_battle_send_order_mode" => sea_battle_send_order_mode(m, host, regs),
        "sea_battle_send_sail_level" => sea_battle_send_sail_level(m, host, regs),
        "sea_battle_draw_drift_sprites" => sea_battle_draw_drift_sprites(m, host, regs),
        "sea_battle_spawn_drift_sprites" => sea_battle_spawn_drift_sprites(m, host, regs),
        "sea_battle_send_return_to_ai" => sea_battle_send_return_to_ai(m, host, regs),
        "sea_battle_get_order_button_mask" => sea_battle_get_order_button_mask(m, host, regs),
        "sea_battle_send_set_retreat" => sea_battle_send_set_retreat(m, host, regs),
        "sea_battle_raise_event_level" => sea_battle_raise_event_level(m, host, regs),
        "sea_battle_is_town_attack" => sea_battle_is_town_attack(m, host, regs),
        "get_battles_save_size" => get_battles_save_size(m, host, regs),
        "queue_battles_time_advance" => queue_battles_time_advance(m, host, regs),
        "siege_state_get_save_size" => siege_state_get_save_size(m, host, regs),
        "siege_state_serialize" => siege_state_serialize(m, host, regs),
        "siege_state_deserialize" => siege_state_deserialize(m, host, regs),
        "siege_state_reset" => siege_state_reset(m, host, regs),
        "siege_state_free_resource_5d8" => siege_state_free_resource_5d8(m, host, regs),
        "town_siege_list_defense_towers_near_gate" => town_siege_list_defense_towers_near_gate(m, host, regs),
        "town_siege_add_flow_field_seed_cell" => town_siege_add_flow_field_seed_cell(m, host, regs),
        "town_siege_choose_active_tower" => town_siege_choose_active_tower(m, host, regs),
        "town_siege_update_tower_fire" => town_siege_update_tower_fire(m, host, regs),
        "town_siege_fire_tower_at_target" => town_siege_fire_tower_at_target(m, host, regs),
        "town_siege_forget_gate_marker" => town_siege_forget_gate_marker(m, host, regs),
        "town_siege_get_distance_to_tracked_entity" => town_siege_get_distance_to_tracked_entity(m, host, regs),
        "town_siege_get_unit_pitch_demand" => town_siege_get_unit_pitch_demand(m, host, regs),
        "town_siege_get_building_pitch_demand" => town_siege_get_building_pitch_demand(m, host, regs),
        "town_siege_tick_timer" => town_siege_tick_timer(m, host, regs),
        "siege_record_ctor" => siege_record_ctor(m, host, regs),
        "siege_record_serialize" => siege_record_serialize(m, host, regs),
        "siege_record_deserialize" => siege_record_deserialize(m, host, regs),
        "siege_record_update_scene" => siege_record_update_scene(m, host, regs),
        "siege_record_plunder_town" => siege_record_plunder_town(m, host, regs),
        "siege_record_refill_goodwill" => siege_record_refill_goodwill(m, host, regs),
        "siege_record_process_delivery_agreements" => siege_record_process_delivery_agreements(m, host, regs),
        "siege_record_ship_surplus_wares" => siege_record_ship_surplus_wares(m, host, regs),
        _ => return None,
    })
}
