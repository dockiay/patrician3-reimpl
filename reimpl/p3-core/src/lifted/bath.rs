//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn operation_record_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x535740;
    loop {
        match pc {
            0x535740 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,0xffffffffu32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn schedule_operation(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543f10;
    loop {
        match pc {
            0x543f10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6636fbu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xe),16));
            { let a = c.get(1,16); c.sub(a,0x34u32,16); }
            if c.cond(3) { pc = 0x543f68; continue; }
            pc = 0x543f33; continue;
            }
            0x543f33 => {
            c.set(0,32,c.get(1,32));
            c.push(m,0x14u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x543f53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x543f68 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x420),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x543fdd; continue; }
            pc = 0x543f72; continue;
            }
            0x543f72 => {
            c.push(m,c.get(6,32));
            c.push(m,0x424u32);
            c.push(m, 0x543f7d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x543fc3; continue; }
            pc = 0x543f92; continue;
            }
            0x543f92 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x10));
            c.set(5,32,0x34u32);
            pc = 0x543f9c; continue;
            }
            0x543f9c => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x543fa3);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x543f9c; continue; }
            pc = 0x543fa9; continue;
            }
            0x543fa9 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x543fb7);
            { let args = []; c.r[0] = host.call(m, 0x543da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x420); wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x543fc5; continue;
            }
            0x543fc3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x543fc5; continue;
            }
            0x543fc5 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[3].wrapping_add(0x420); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x543fed; continue; }
            pc = 0x543fd8; continue;
            }
            0x543fd8 => {
            c.push(m, 0x543fdd);
            { let args = []; c.r[0] = host.call(m, 0x543da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x543fdd; continue;
            }
            0x543fdd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x420),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x543fed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x543f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x543fed; continue;
            }
            0x543fed => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "operation_record_init" => operation_record_init(m, host, regs),
        "schedule_operation" => schedule_operation(m, host, regs),
        _ => return None,
    })
}
