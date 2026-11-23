//! Generado por `reimpl/tools/ui_rt_grp.py snd` con `tools/lift.py` (no editar a mano): grupo `snd`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_SND: [(u32, super::core_table::CoreFn); 11] = [
    (0x4419a0, f_004419a0),
    (0x441cd0, f_00441cd0),
    (0x441e10, f_00441e10),
    (0x4426c0, f_004426c0),
    (0x442d90, f_00442d90),
    (0x442e30, f_00442e30),
    (0x442e90, f_00442e90),
    (0x4429d0, f_004429d0),
    (0x4436e0, f_004436e0),
    (0x4439b0, f_004439b0),
    (0x443150, f_00443150),
];
pub fn f_004419a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4419a0;
    loop {
        match pc {
            0x4419a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x50u32);
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[6].wrapping_add(0xd0));
            { let a = c.r[6].wrapping_add(0x130); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x128); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1b4); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4419eb);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x14u32);
            c.set(1,32,c.r[6].wrapping_add(0x18c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4419fa);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x14u32);
            c.set(2,32,c.r[6].wrapping_add(0x1a0));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x441a09);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x1b8); wm(m,a,32,0x50u32); }
            { let a = c.r[6].wrapping_add(0x1bc); wm(m,a,32,0x50u32); }
            { let a = c.r[6].wrapping_add(0x1c0); wm(m,a,32,0x50u32); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x12c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c5); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1dd); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,8,0x3u32); }
            { let a = c.r[6].wrapping_add(0x120); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1de); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1df); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1e0); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1d4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1d8); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1d9); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1db); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1dc); wm(m,a,8,0x1u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00441cd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441cd0;
    loop {
        match pc {
            0x441cd0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e543u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x697bf0u32);
            c.push(m,0x0u32);
            c.push(m,0x697c04u32);
            c.push(m,0x697c0cu32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.push(m, 0x441d19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x441d2e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x441d58; continue; }
            pc = 0x441d3e; continue;
            }
            0x441d3e => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(6,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x441d56);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x441d5a; continue;
            }
            0x441d58 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x441d5a; continue;
            }
            0x441d5a => {
            { let a = c.r[5].wrapping_add(0xcc); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            if c.cond(14) { pc = 0x441dbe; continue; }
            pc = 0x441d6b; continue;
            }
            0x441d6b => {
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x697c18u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x441d7b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,0x697c20u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbdbcu32);
            c.push(m,c.get(0,32));
            c.push(m,0x697c34u32);
            c.push(m, 0x441da6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xcc),32));
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            c.push(m, 0x441db9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x441d6b; continue; }
            pc = 0x441dbe; continue;
            }
            0x441dbe => {
            c.push(m,0x697c40u32);
            c.push(m,0x0u32);
            c.push(m,0x697c54u32);
            c.push(m,0x697c70u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x441dd9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[5].wrapping_add(0x1c4); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x441dea; continue; }
            pc = 0x441de3; continue;
            }
            0x441de3 => {
            { let a = c.r[5].wrapping_add(0x1c4); wm(m,a,8,0x2u32); }
            pc = 0x441dea; continue;
            }
            0x441dea => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x441dfb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00441e10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441e10;
    loop {
        match pc {
            0x441e10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e55bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x90u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x697c7cu32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a5e8u32,32); c.push(m, 0x441e40); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x441e45);
            { let args = []; c.r[0] = host.call(m, 0x638980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let target = rm(m,0x66a5ecu32,32); c.push(m, 0x441e4b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x441e5e; continue; }
            pc = 0x441e4f; continue;
            }
            0x441e4f => {
            { let target = rm(m,0x66a5f0u32,32); c.push(m, 0x441e55); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x441e5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441e5e; continue;
            }
            0x441e5e => {
            c.push(m,0x0u32);
            c.push(m,0xfu32);
            { let target = rm(m,0x66a5f4u32,32); c.push(m, 0x441e68); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,rm(m,0x697be4u32,32));
            { let a = c.get(6,32); c.sub(a,0x2b11u32,32); }
            if c.cond(12) { pc = 0x442061; continue; }
            pc = 0x441e7a; continue;
            }
            0x441e7a => {
            c.set(3,32,rm(m,0x66a5f8u32,32));
            { let a = c.get(7,32); let r = c.add(a,0x68u32,32); c.set(7,32,r); }
            c.set(5,32,0x1u32);
            pc = 0x441e88; continue;
            }
            0x441e88 => {
            c.set(0,32,rm(m,0x697be8u32,32));
            c.set(1,32,rm(m,0x697becu32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.push(m,0x6cbda0u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0xffffffffu32);
            { let a = c.get(0,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = 0x6cbdacu32; wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,0x697be8u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            { let a = 0x6cbda0u32; wm(m,a,16,c.get(5,16)); }
            { let a = 0x6cbda2u32; wm(m,a,16,c.get(1,16)); }
            { let a = 0x6cbda4u32; wm(m,a,32,c.get(6,32)); }
            { let a = 0x6cbda8u32; wm(m,a,32,c.get(2,32)); }
            { let a = 0x6cbdaeu32; wm(m,a,16,c.get(0,16)); }
            { let target = c.get(3,32); c.push(m, 0x441edf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x441f4f; continue; }
            pc = 0x441ee3; continue;
            }
            0x441ee3 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x697c84u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x441ef2);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.push(m, 0x441efc);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5fcu32,32); c.push(m, 0x441f11); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0xfu32);
            { let target = rm(m,0x66a600u32,32); c.push(m, 0x441f19); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x441fc0; continue; }
            pc = 0x441f21; continue;
            }
            0x441f21 => {
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x697c90u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x441f30);
            { let args = []; c.r[0] = host.call(m, 0x63ad50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x441fc0; continue; }
            pc = 0x441f3b; continue;
            }
            0x441f3b => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a624u32,32); c.push(m, 0x441f44); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.push(m,0xfu32);
            { let target = rm(m,0x66a5f4u32,32); c.push(m, 0x441f4d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x441f5b; continue;
            }
            0x441f4f => {
            c.push(m,0xfu32);
            { let target = rm(m,0x66a600u32,32); c.push(m, 0x441f57); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x441f7f; continue; }
            pc = 0x441f5b; continue;
            }
            0x441f5b => {
            c.push(m,c.get(5,32));
            c.push(m,0xfu32);
            { let target = rm(m,0x66a5f4u32,32); c.push(m, 0x441f64); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,rm(m,0x697be4u32,32));
            pc = 0x441f6a; continue;
            }
            0x441f6a => {
            { let a = c.get(6,32); c.sub(a,0x2b11u32,32); }
            if c.cond(13) { pc = 0x441e88; continue; }
            pc = 0x441f76; continue;
            }
            0x441f76 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x442061; continue;
            }
            0x441f7f => {
            c.set(0,32,rm(m,0x697be4u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x2b11u32,32); }
            { let a = 0x697be4u32; wm(m,a,32,c.get(6,32)); }
            if c.cond(13) { pc = 0x441e88; continue; }
            pc = 0x441f9d; continue;
            }
            0x441f9d => {
            { let a = rm(m,0x697be8u32,32); c.sub(a,0x10u32,32); }
            if c.cond(5) { pc = 0x441f6a; continue; }
            pc = 0x441fa6; continue;
            }
            0x441fa6 => {
            c.set(6,32,0xac44u32);
            { let a = 0x697be8u32; wm(m,a,32,0x8u32); }
            { let a = 0x697be4u32; wm(m,a,32,c.get(6,32)); }
            pc = 0x441e88; continue;
            }
            0x441fc0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,0x66a604u32,32));
            c.set(3,32,0x14u32);
            c.set(6,32,c.r[1].wrapping_add(0x18));
            pc = 0x441fd2; continue;
            }
            0x441fd2 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x441fd7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x441fd2; continue; }
            pc = 0x441fdf; continue;
            }
            0x441fdf => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = 0x6dccd0u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dccdcu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x44201d; continue; }
            pc = 0x441ff1; continue;
            }
            0x441ff1 => {
            c.push(m,0x84u32);
            c.push(m, 0x441ffb);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x442016; continue; }
            pc = 0x44200d; continue;
            }
            0x44200d => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x442014);
            { let args = []; c.r[0] = host.call(m, 0x4c4130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x442018; continue;
            }
            0x442016 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x442018; continue;
            }
            0x442018 => {
            { let a = 0x6dccdcu32; wm(m,a,32,c.get(0,32)); }
            pc = 0x44201d; continue;
            }
            0x44201d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(6,32,c.r[1].wrapping_add(0x6c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a608u32,32); c.push(m, 0x442039); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x697c9cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x442048);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442053; continue; }
            pc = 0x44204f; continue;
            }
            0x44204f => {
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            pc = 0x442068; continue;
            }
            0x442053 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a60cu32,32); c.push(m, 0x44205f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442068; continue;
            }
            0x442061 => {
            { let a = c.r[7].wrapping_add(0x68); wm(m,a,32,0x0u32); }
            pc = 0x442068; continue;
            }
            0x442068 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xa0),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x9cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004426c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4426c0;
    loop {
        match pc {
            0x4426c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e57fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442896; continue; }
            pc = 0x4426ec; continue;
            }
            0x4426ec => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44271d; continue; }
            pc = 0x4426f4; continue;
            }
            0x4426f4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x442896; continue; }
            pc = 0x44271d; continue;
            }
            0x44271d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x120),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442896; continue; }
            pc = 0x44272b; continue;
            }
            0x44272b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e0),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x442896; continue; }
            pc = 0x442739; continue;
            }
            0x442739 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x442741);
            { let args = []; c.r[0] = host.call(m, 0x442180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44288e; continue; }
            pc = 0x44274d; continue;
            }
            0x44274d => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a610u32,32));
            c.push(m,c.get(5,32));
            c.set(7,32,c.r[6].wrapping_add(0x18));
            c.set(5,32,0x14u32);
            pc = 0x44275d; continue;
            }
            0x44275d => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let target = c.get(3,32); c.push(m, 0x442762); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x44275d; continue; }
            pc = 0x442768; continue;
            }
            0x442768 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x4427be; continue; }
            pc = 0x442771; continue;
            }
            0x442771 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44278b; continue; }
            pc = 0x442778; continue;
            }
            0x442778 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x442781); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x44278b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x44278b; continue;
            }
            0x44278b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1da),8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1d4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x1db); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x1dc); wm(m,a,8,0x1u32); }
            pc = 0x4427be; continue;
            }
            0x4427be => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,0x3e800u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5ccu32,32); c.push(m, 0x4427d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x44288e; continue; }
            pc = 0x4427df; continue;
            }
            0x4427df => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x44280e; continue; }
            pc = 0x44280a; continue;
            }
            0x44280a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x442818; continue;
            }
            0x44280e => {
            { let a = c.get(2,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442818; continue; }
            pc = 0x442813; continue;
            }
            0x442813 => {
            c.set(2,32,0x7fu32);
            pc = 0x442818; continue;
            }
            0x442818 => {
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442820); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5c0u32,32); c.push(m, 0x44282a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5a8u32,32); c.push(m, 0x44283b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44284f; continue; }
            pc = 0x442843; continue;
            }
            0x442843 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5c4u32,32); c.push(m, 0x44284f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44284f; continue;
            }
            0x44284f => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44286d; continue; }
            pc = 0x442855; continue;
            }
            0x442855 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x44285e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x442867); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            pc = 0x44286d; continue;
            }
            0x44286d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44288e; continue; }
            pc = 0x442874; continue;
            }
            0x442874 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x44287d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x442887); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x44288e; continue;
            }
            0x44288e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4428bb; continue; }
            pc = 0x442896; continue;
            }
            0x442896 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4428a7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x4428bb => {
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x442903; continue; }
            pc = 0x4428c0; continue;
            }
            0x4428c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4428da; continue; }
            pc = 0x4428c7; continue;
            }
            0x4428c7 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x4428d0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x4428da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4428da; continue;
            }
            0x4428da => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1d8),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1d9); wm(m,a,8,c.get(1,8)); }
            pc = 0x442903; continue;
            }
            0x442903 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,0x3e800u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5ccu32,32); c.push(m, 0x442917); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x44299d; continue; }
            pc = 0x44291e; continue;
            }
            0x44291e => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442927); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x44295b; continue; }
            pc = 0x442938; continue;
            }
            0x442938 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x44296e; continue; }
            pc = 0x44293f; continue;
            }
            0x44293f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x44295b; continue; }
            pc = 0x442943; continue;
            }
            0x442943 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1db),8));
            c.set(2,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            pc = 0x44295b; continue;
            }
            0x44295b => {
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1d0); wm(m,a,32,c.get(1,32)); }
            pc = 0x44296e; continue;
            }
            0x44296e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5c0u32,32); c.push(m, 0x442978); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44298c; continue; }
            pc = 0x442980; continue;
            }
            0x442980 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c4u32,32); c.push(m, 0x44298c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44298c; continue;
            }
            0x44298c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5a8u32,32); c.push(m, 0x44299d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44299d; continue;
            }
            0x44299d => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4429b2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442d90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442d90;
    loop {
        match pc {
            0x442d90 => {
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442e20; continue; }
            pc = 0x442d9f; continue;
            }
            0x442d9f => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x1b8),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442e20; continue; }
            pc = 0x442dad; continue;
            }
            0x442dad => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1da),8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x442dd1; continue; }
            pc = 0x442dc0; continue;
            }
            0x442dc0 => {
            c.set(7,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x442e18; continue; }
            pc = 0x442dc7; continue;
            }
            0x442dc7 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1db),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            pc = 0x442dd1; continue;
            }
            0x442dd1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let target = rm(m,0x66a5a0u32,32); c.push(m, 0x442df8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x442e18; continue; }
            pc = 0x442dfc; continue;
            }
            0x442dfc => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,32,c.get(6,32));
            if c.cond(13) { pc = 0x442e06; continue; }
            pc = 0x442e02; continue;
            }
            0x442e02 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x442e10; continue;
            }
            0x442e06 => {
            { let a = c.get(6,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442e10; continue; }
            pc = 0x442e0b; continue;
            }
            0x442e0b => {
            c.set(0,32,0x7fu32);
            pc = 0x442e10; continue;
            }
            0x442e10 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442e18); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442e18; continue;
            }
            0x442e18 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[3].wrapping_add(0x1b8); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x442e20; continue;
            }
            0x442e20 => {
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442e30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442e30;
    loop {
        match pc {
            0x442e30 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442e81; continue; }
            pc = 0x442e3b; continue;
            }
            0x442e3b => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x1c0); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x442e81; continue; }
            pc = 0x442e4c; continue;
            }
            0x442e4c => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5a0u32,32); c.push(m, 0x442e53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x442e81; continue; }
            pc = 0x442e57; continue;
            }
            0x442e57 => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x442e6f; continue; }
            pc = 0x442e60; continue;
            }
            0x442e60 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442e6a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x442e6f => {
            { let a = c.get(6,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442e79; continue; }
            pc = 0x442e74; continue;
            }
            0x442e74 => {
            c.set(0,32,0x7fu32);
            pc = 0x442e79; continue;
            }
            0x442e79 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442e81); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442e81; continue;
            }
            0x442e81 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442e90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442e90;
    loop {
        match pc {
            0x442e90 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442fed; continue; }
            pc = 0x442ea1; continue;
            }
            0x442ea1 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[3].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x14u32); }
            pc = 0x442eb1; continue;
            }
            0x442eb1 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x134),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x64),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x442ed2; continue; }
            pc = 0x442ec0; continue;
            }
            0x442ec0 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x124),32));
            pc = 0x442ec6; continue;
            }
            0x442ec6 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(4) { pc = 0x442ed2; continue; }
            pc = 0x442eca; continue;
            }
            0x442eca => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x442ec6; continue; }
            pc = 0x442ed2; continue;
            }
            0x442ed2 => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x442f3d; continue; }
            pc = 0x442ed6; continue;
            }
            0x442ed6 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a598u32,32); c.push(m, 0x442edf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442f06; continue; }
            pc = 0x442ee3; continue;
            }
            0x442ee3 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x78),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(2)),16));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            pc = 0x442f12; continue;
            }
            0x442f06 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[3].wrapping_add(0x1bc),32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            pc = 0x442f12; continue;
            }
            0x442f12 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a598u32,32); c.push(m, 0x442f1b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x442f3d; continue; }
            pc = 0x442f1f; continue;
            }
            0x442f1f => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x442f2b; continue; }
            pc = 0x442f27; continue;
            }
            0x442f27 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x442f35; continue;
            }
            0x442f2b => {
            { let a = c.get(6,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442f35; continue; }
            pc = 0x442f30; continue;
            }
            0x442f30 => {
            c.set(0,32,0x7fu32);
            pc = 0x442f35; continue;
            }
            0x442f35 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a59cu32,32); c.push(m, 0x442f3d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442f3d; continue;
            }
            0x442f3d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x442eb1; continue; }
            pc = 0x442f4f; continue;
            }
            0x442f4f => {
            c.set(7,32,rm(m,c.r[3],32));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1d8),8));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x442f70; continue; }
            pc = 0x442f5f; continue;
            }
            0x442f5f => {
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x442fb7; continue; }
            pc = 0x442f66; continue;
            }
            0x442f66 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1d9),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            pc = 0x442f70; continue;
            }
            0x442f70 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let target = rm(m,0x66a5a0u32,32); c.push(m, 0x442f97); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x442fb7; continue; }
            pc = 0x442f9b; continue;
            }
            0x442f9b => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,32,c.get(6,32));
            if c.cond(13) { pc = 0x442fa5; continue; }
            pc = 0x442fa1; continue;
            }
            0x442fa1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x442faf; continue;
            }
            0x442fa5 => {
            { let a = c.get(6,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442faf; continue; }
            pc = 0x442faa; continue;
            }
            0x442faa => {
            c.set(0,32,0x7fu32);
            pc = 0x442faf; continue;
            }
            0x442faf => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442fb7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442fb7; continue;
            }
            0x442fb7 => {
            { let a = c.r[3].wrapping_add(0x1bc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,0x6dccdcu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x442fed; continue; }
            pc = 0x442fc8; continue;
            }
            0x442fc8 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.push(m,0xffffffffu32);
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x81020409u32);
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x442fed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c4540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442fed; continue;
            }
            0x442fed => {
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

pub fn f_004429d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4429d0;
    loop {
        match pc {
            0x4429d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442a08; continue; }
            pc = 0x4429da; continue;
            }
            0x4429da => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0x442090u32);
            c.push(m, 0x4429ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x654296, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x120); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x442a08; continue; }
            pc = 0x4429f7; continue;
            }
            0x4429f7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442a08; continue; }
            pc = 0x4429fe; continue;
            }
            0x4429fe => {
            { let a = c.r[6].wrapping_add(0x120); wm(m,a,32,0x0u32); }
            pc = 0x442a08; continue;
            }
            0x442a08 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004436e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4436e0;
    loop {
        match pc {
            0x4436e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x68),32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443891; continue; }
            pc = 0x4436f3; continue;
            }
            0x4436f3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x1c5); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x4437f9; continue; }
            pc = 0x443705; continue;
            }
            0x443705 => {
            c.set(0,32,rm(m,0x66d640u32,32));
            c.set(7,32,rm(m,0x66a5b4u32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x443718; continue;
            }
            0x443718 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x443728); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x443718; continue; }
            pc = 0x44372c; continue;
            }
            0x44372c => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,0x66a5b8u32,32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a5bcu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44375d; continue; }
            pc = 0x44373f; continue;
            }
            0x44373f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x44374b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x44375d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44375d; continue;
            }
            0x44375d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443783; continue; }
            pc = 0x443764; continue;
            }
            0x443764 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x443770); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x443783); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x443783; continue;
            }
            0x443783 => {
            { let a = rm(m,c.r[6].wrapping_add(0x68),32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4437df; continue; }
            pc = 0x443788; continue;
            }
            0x443788 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443797; continue; }
            pc = 0x443792; continue;
            }
            0x443792 => {
            c.push(m, 0x443797);
            { let args = []; c.r[0] = host.call(m, 0x59c010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x443797; continue;
            }
            0x443797 => {
            c.set(3,32,0xfffffe60u32);
            c.set(7,32,c.r[6].wrapping_add(0x1a0));
            c.set(5,32,c.r[6].wrapping_add(0x18));
            { let a = c.get(3,32); let r = c.sub(a,c.get(6,32),32); c.set(3,32,r); }
            pc = 0x4437a7; continue;
            }
            0x4437a7 => {
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x4437b1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x4437ca; continue; }
            pc = 0x4437b6; continue;
            }
            0x4437b6 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5acu32,32); c.push(m, 0x4437c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a610u32,32); c.push(m, 0x4437ca); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4437ca; continue;
            }
            0x4437ca => {
            { let a = c.r[7].wrapping_add(0xffffffec); wm(m,a,8,0x0u32); }
            { let a = c.r[7]; wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[7]));
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4437a7; continue; }
            pc = 0x4437dd; continue;
            }
            0x4437dd => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x4437df; continue;
            }
            0x4437df => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4437ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x442280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4437f9 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,0x66a5b8u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44380e; continue; }
            pc = 0x443805; continue;
            }
            0x443805 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(5,32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x44380e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44380e; continue;
            }
            0x44380e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44381e; continue; }
            pc = 0x443815; continue;
            }
            0x443815 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(5,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            { let target = c.get(7,32); c.push(m, 0x44381e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44381e; continue;
            }
            0x44381e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x443844; continue; }
            pc = 0x443825; continue;
            }
            0x443825 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443857; continue; }
            pc = 0x44382c; continue;
            }
            0x44382c => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x443844; continue; }
            pc = 0x443830; continue;
            }
            0x443830 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1db),8));
            c.set(2,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            pc = 0x443844; continue;
            }
            0x443844 => {
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1d0); wm(m,a,32,c.get(1,32)); }
            pc = 0x443857; continue;
            }
            0x443857 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1de),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x443883; continue; }
            pc = 0x443861; continue;
            }
            0x443861 => {
            { let a = rm(m,c.r[6].wrapping_add(0x68),32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443883; continue; }
            pc = 0x443866; continue;
            }
            0x443866 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x443883; continue; }
            pc = 0x44386d; continue;
            }
            0x44386d => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x443876); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x443880); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            pc = 0x443883; continue;
            }
            0x443883 => {
            { let a = c.r[6].wrapping_add(0x1df); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x1e0); wm(m,a,8,0x0u32); }
            pc = 0x443891; continue;
            }
            0x443891 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004439b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4439b0;
    loop {
        match pc {
            0x4439b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e5fdu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x444008; continue; }
            pc = 0x4439df; continue;
            }
            0x4439df => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x697d10u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x3u32); }
            c.push(m, 0x443a09);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,0x697d1cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x443a29);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbdbdu32);
            c.push(m,0x697d24u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x443a53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x443a5d; continue; }
            pc = 0x443a57; continue;
            }
            0x443a57 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            pc = 0x443a9b; continue;
            }
            0x443a5d => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x443a6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbdbeu32);
            c.push(m,0x697d2cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x443a93);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            if c.cond(5) { pc = 0x443aca; continue; }
            pc = 0x443a9b; continue;
            }
            0x443a9b => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x2u32); }
            c.push(m, 0x443aa5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x1u32); }
            c.push(m, 0x443ab3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            c.push(m, 0x443ac1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            pc = 0x444010; continue;
            }
            0x443aca => {
            c.push(m,0x6dbb58u32);
            c.push(m, 0x443ad4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[7].wrapping_mul(4));
            c.push(m,c.get(3,32));
            c.push(m, 0x443ae1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x443af2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x4u32); }
            if c.cond(4) { pc = 0x443b1a; continue; }
            pc = 0x443b02; continue;
            }
            0x443b02 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(5,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x443b1a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x443b1a; continue;
            }
            0x443b1a => {
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x3u32); }
            c.push(m, 0x443b29);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7]));
            c.push(m,c.get(0,32));
            c.push(m, 0x443b36);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x12c),32));
            c.set(2,32,c.r[1].wrapping_mul(4).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x443b57);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x443b83; continue; }
            pc = 0x443b69; continue;
            }
            0x443b69 => {
            c.push(m,0x8u32);
            c.push(m, 0x443b70);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x443b69; continue; }
            pc = 0x443b83; continue;
            }
            0x443b83 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x443bb3; continue; }
            pc = 0x443b8f; continue;
            }
            0x443b8f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)),32));
            c.push(m,0x8u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x443ba5);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x443b8f; continue; }
            pc = 0x443bb3; continue;
            }
            0x443bb3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x138),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x138),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x12c); wm(m,a,32,c.get(5,32)); }
            if c.cond(14) { pc = 0x443bfd; continue; }
            pc = 0x443be2; continue;
            }
            0x443be2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x443bef);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x443be2; continue; }
            pc = 0x443bfd; continue;
            }
            0x443bfd => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x138),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x443c14);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x443c86; continue; }
            pc = 0x443c1d; continue;
            }
            0x443c1d => {
            c.push(m,0x1u32);
            c.push(m,0x697d30u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x443c2d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x443c60; continue; }
            pc = 0x443c34; continue;
            }
            0x443c34 => {
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x443c43);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x443c4b);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x443c5e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x443c74; continue;
            }
            0x443c60 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x443c6a);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x443c74; continue;
            }
            0x443c74 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.push(m, 0x443c81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x443c1d; continue; }
            pc = 0x443c86; continue;
            }
            0x443c86 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x443d1e; continue; }
            pc = 0x443c90; continue;
            }
            0x443c90 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            pc = 0x443ca4; continue;
            }
            0x443ca0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            pc = 0x443ca4; continue;
            }
            0x443ca4 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,0x697d34u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x443cb7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbdbfu32);
            c.push(m,0x697d40u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x443ce2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x443cf2; continue; }
            pc = 0x443ce6; continue;
            }
            0x443ce6 => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x443cf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x443cf2; continue;
            }
            0x443cf2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x64u32);
            c.push(m,0x697d4cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x443d0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[2].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(0,16)); }
            if c.cond(12) { pc = 0x443ca0; continue; }
            pc = 0x443d1e; continue;
            }
            0x443d1e => {
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x443d27);
            { let args = []; c.r[0] = host.call(m, 0x402740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x5u32); }
            if c.cond(14) { pc = 0x443d7c; continue; }
            pc = 0x443d30; continue;
            }
            0x443d30 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(7,32)); }
            pc = 0x443d3e; continue;
            }
            0x443d3e => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[5]),32));
            c.push(m,0x8000u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m, 0x443d50);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x402760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x443d59; continue; }
            pc = 0x443d54; continue;
            }
            0x443d54 => {
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            pc = 0x443d65; continue;
            }
            0x443d59 => {
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x443d62);
            { let args = []; c.r[0] = host.call(m, 0x402860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            pc = 0x443d65; continue;
            }
            0x443d65 => {
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x443d6e);
            { let args = []; c.r[0] = host.call(m, 0x402780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x443d3e; continue; }
            pc = 0x443d7c; continue;
            }
            0x443d7c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x124),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x128),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x443daf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.r[6].wrapping_add(0x124); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[5].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x443dca);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x6u32); }
            if c.cond(4) { pc = 0x443dfa; continue; }
            pc = 0x443dda; continue;
            }
            0x443dda => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(5,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x443df2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x443dfc; continue;
            }
            0x443dfa => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x443dfc; continue;
            }
            0x443dfc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x5u32); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x443e15);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x128); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x443e2b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x124),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x443e49);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(0,32,c.r[7].wrapping_mul(4));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x124),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m, 0x443e6b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x443e9a; continue; }
            pc = 0x443e7a; continue;
            }
            0x443e7a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(0,32,c.r[5].wrapping_mul(4));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[3]));
            c.push(m,c.get(2,32));
            c.push(m, 0x443e8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x443e7a; continue; }
            pc = 0x443e9a; continue;
            }
            0x443e9a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x443ecc; continue; }
            pc = 0x443ea7; continue;
            }
            0x443ea7 => {
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)));
            c.push(m, 0x443ebe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x443ea7; continue; }
            pc = 0x443ecc; continue;
            }
            0x443ecc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x128),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x443ee7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.r[7].wrapping_mul(4));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x128),32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m, 0x443f09);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x443f20);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7]));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.push(m,c.get(1,32));
            c.push(m, 0x443f3b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x443f53);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x443f7b; continue; }
            pc = 0x443f5e; continue;
            }
            0x443f5e => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x443f72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x443f78);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x443f7b; continue;
            }
            0x443f7b => {
            c.push(m,c.get(3,32));
            c.push(m, 0x443f81);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.push(m, 0x443f87);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x443f91);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x443fb9; continue; }
            pc = 0x443f9c; continue;
            }
            0x443f9c => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x443fb0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x443fb6);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x443fb9; continue;
            }
            0x443fb9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x443fc3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x443fcd);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x3u32); }
            c.push(m, 0x443fde);
            { let args = []; c.r[0] = host.call(m, 0x402750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x2u32); }
            c.push(m, 0x443fec);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x1u32); }
            c.push(m, 0x443ffa);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            c.push(m, 0x444008);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x444008; continue;
            }
            0x444008 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0xffffffffu32); }
            pc = 0x444010; continue;
            }
            0x444010 => {
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m, 0x444019);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00443150(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x443150;
    loop {
        match pc {
            0x443150 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e5b5u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44347c; continue; }
            pc = 0x443179; continue;
            }
            0x443179 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x54),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1df),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x44347c; continue; }
            pc = 0x443192; continue;
            }
            0x443192 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x1de),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e0),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x44347c; continue; }
            pc = 0x4431af; continue;
            }
            0x4431af => {
            { let a = rm(m,c.r[4].wrapping_add(0x48),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4431be; continue; }
            pc = 0x4431b6; continue;
            }
            0x4431b6 => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x7fu32); }
            pc = 0x4431be; continue;
            }
            0x4431be => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(5,32,rm(m,0x66a5c8u32,32));
            { let a = c.get(0,32); c.sub(a,0x1770u32,32); }
            if c.cond(5) { pc = 0x4431fc; continue; }
            pc = 0x4431cf; continue;
            }
            0x4431cf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x7c));
            pc = 0x4431d4; continue;
            }
            0x4431d4 => {
            { let a = rm(m,c.r[1],32); c.sub(a,0x1770u32,32); }
            if c.cond(4) { pc = 0x4431e7; continue; }
            pc = 0x4431dc; continue;
            }
            0x4431dc => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4431d4; continue; }
            pc = 0x4431e5; continue;
            }
            0x4431e5 => {
            pc = 0x4431fc; continue;
            }
            0x4431e7 => {
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4431fc; continue; }
            pc = 0x4431ec; continue;
            }
            0x4431ec => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x4431f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(4) { pc = 0x44347c; continue; }
            pc = 0x4431fc; continue;
            }
            0x4431fc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44322b; continue; }
            pc = 0x443203; continue;
            }
            0x443203 => {
            { let a = c.r[6].wrapping_add(0x1b4); wm(m,a,16,0x0u32); }
            c.set(7,32,c.r[6].wrapping_add(0x18));
            c.set(3,32,0x14u32);
            pc = 0x443214; continue;
            }
            0x443214 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x443219); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x443225; continue; }
            pc = 0x44321e; continue;
            }
            0x44321e => {
            { let a = rm(m,c.r[6].wrapping_add(0x1b4),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x1b4); wm(m,a,16,r); } }
            pc = 0x443225; continue;
            }
            0x443225 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x443214; continue; }
            pc = 0x44322b; continue;
            }
            0x44322b => {
            { let a = rm(m,c.r[6].wrapping_add(0x1b4),16); c.sub(a,0x14u32,16); }
            if c.cond(4) { pc = 0x44347c; continue; }
            pc = 0x443239; continue;
            }
            0x443239 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x13c));
            pc = 0x443245; continue;
            }
            0x443245 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(4) { pc = 0x4432d4; continue; }
            pc = 0x44324d; continue;
            }
            0x44324d => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x443245; continue; }
            pc = 0x443256; continue;
            }
            0x443256 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x443274; continue; }
            pc = 0x443262; continue;
            }
            0x443262 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x124),32));
            pc = 0x443268; continue;
            }
            0x443268 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(4) { pc = 0x443274; continue; }
            pc = 0x44326c; continue;
            }
            0x44326c => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x443268; continue; }
            pc = 0x443274; continue;
            }
            0x443274 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x44347c; continue; }
            pc = 0x44327c; continue;
            }
            0x44327c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x128),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e0u32,32); c.push(m, 0x443290); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44347c; continue; }
            pc = 0x44329a; continue;
            }
            0x44329a => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4432a3);
            { let args = []; c.r[0] = host.call(m, 0x402740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.push(m,0x8000u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4432c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x402760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x443356; continue; }
            pc = 0x4432c8; continue;
            }
            0x4432c8 => {
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a618u32,32); c.push(m, 0x4432cf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x44346b; continue;
            }
            0x4432d4 => {
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x443256; continue; }
            pc = 0x4432dd; continue;
            }
            0x4432dd => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x13c),32));
            c.set(3,32,c.r[7].wrapping_mul(4));
            c.set(5,32,c.r[6].wrapping_add(0x13c));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[6].wrapping_add(0x140));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x443303);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(3,32));
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xd0),32));
            c.set(5,32,c.r[6].wrapping_add(0xd0));
            c.set(2,32,c.r[6].wrapping_add(0xd4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x443325);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x134),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x44334d; continue; }
            pc = 0x443337; continue;
            }
            0x443337 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x124),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x443341; continue;
            }
            0x443341 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[2],32),32); }
            if c.cond(4) { pc = 0x44334d; continue; }
            pc = 0x443345; continue;
            }
            0x443345 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x443341; continue; }
            pc = 0x44334d; continue;
            }
            0x44334d => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x44340a; continue;
            }
            0x443356 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x128),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x44336a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4027a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x128),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32),32); }
            if c.cond(4) { pc = 0x44338a; continue; }
            pc = 0x443375; continue;
            }
            0x443375 => {
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a618u32,32); c.push(m, 0x44337c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x443385);
            { let args = []; c.r[0] = host.call(m, 0x402780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x44346b; continue;
            }
            0x44338a => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x443393);
            { let args = []; c.r[0] = host.call(m, 0x402780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x188),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x7c));
            pc = 0x44339e; continue;
            }
            0x44339e => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(4) { pc = 0x443443; continue; }
            pc = 0x4433a6; continue;
            }
            0x4433a6 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x44339e; continue; }
            pc = 0x4433af; continue;
            }
            0x4433af => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x11c),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a618u32,32); c.push(m, 0x4433bc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0xd0));
            c.push(m,0x4cu32);
            c.set(2,32,c.r[6].wrapping_add(0xd4));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4433d1);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x13c));
            c.push(m,0x4cu32);
            c.set(0,32,c.r[6].wrapping_add(0x140));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[5]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4433e9);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x1b4),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x1b4); wm(m,a,16,r); } }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x44340a);
            { let args = []; c.r[0] = host.call(m, 0x402750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x44340a; continue;
            }
            0x44340a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44347c; continue; }
            pc = 0x443412; continue;
            }
            0x443412 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            pc = 0x44341b; continue;
            }
            0x44341b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x18c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x443434; continue; }
            pc = 0x443426; continue;
            }
            0x443426 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x44342f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x443493; continue; }
            pc = 0x443434; continue;
            }
            0x443434 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x44341b; continue; }
            pc = 0x44343d; continue;
            }
            0x44343d => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            pc = 0x44347c; continue;
            }
            0x443443 => {
            { let a = c.get(5,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4433af; continue; }
            pc = 0x44344c; continue;
            }
            0x44344c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x443457); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(4) { pc = 0x44346b; continue; }
            pc = 0x44345c; continue;
            }
            0x44345c => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x18c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4433af; continue; }
            pc = 0x44346b; continue;
            }
            0x44346b => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x44347c);
            { let args = []; c.r[0] = host.call(m, 0x402750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x44347c; continue;
            }
            0x44347c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x44347e; continue;
            }
            0x44347e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(32); return c.r;
            }
            0x443493 => {
            { let a = c.get(3,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            if c.cond(13) { pc = 0x44347c; continue; }
            pc = 0x44349c; continue;
            }
            0x44349c => {
            c.set(7,32,c.get(3,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x18),32));
            c.set(3,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5dcu32,32); c.push(m, 0x4434ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x128),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5d8u32,32); c.push(m, 0x4434d9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44353a; continue; }
            pc = 0x4434ea; continue;
            }
            0x4434ea => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x44353a; continue; }
            pc = 0x4434ee; continue;
            }
            0x4434ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x18));
            pc = 0x4434f3; continue;
            }
            0x4434f3 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(4) { pc = 0x443500; continue; }
            pc = 0x4434f7; continue;
            }
            0x4434f7 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4434f3; continue; }
            pc = 0x443500; continue;
            }
            0x443500 => {
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(4) { pc = 0x44353a; continue; }
            pc = 0x443505; continue;
            }
            0x443505 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = rm(m,c.r[6].wrapping_add(0x1dd),8); c.sub(a,0x0u32,8); }
            c.set(0,32,c.get(1,32));
            if c.cond(4) { pc = 0x44351b; continue; }
            pc = 0x443514; continue;
            }
            0x443514 => {
            c.set(0,32,0x7fu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x44351b; continue;
            }
            0x44351b => {
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let target = rm(m,0x66a5d0u32,32); c.push(m, 0x44352a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44353a; continue; }
            pc = 0x44352e; continue;
            }
            0x44352e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a634u32,32); c.push(m, 0x44353a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44353a; continue;
            }
            0x44353a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4435bf; continue; }
            pc = 0x443542; continue;
            }
            0x443542 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,0x81020409u32);
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bc),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x48),32),32); c.set(1,32,r); }
            c.set(7,32,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x443587; continue; }
            pc = 0x443583; continue;
            }
            0x443583 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x443591; continue;
            }
            0x443587 => {
            { let a = c.get(2,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x443591; continue; }
            pc = 0x44358c; continue;
            }
            0x44358c => {
            c.set(2,32,0x7fu32);
            pc = 0x443591; continue;
            }
            0x443591 => {
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a59cu32,32); c.push(m, 0x443599); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5b0u32,32); c.push(m, 0x4435a2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,0x66a594u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4435b3; continue; }
            pc = 0x4435ae; continue;
            }
            0x4435ae => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x4435b3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4435b3; continue;
            }
            0x4435b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4435bf; continue; }
            pc = 0x4435ba; continue;
            }
            0x4435ba => {
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x4435bf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4435bf; continue;
            }
            0x4435bf => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x40),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44362b; continue; }
            pc = 0x4435c7; continue;
            }
            0x4435c7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1a0); wm(m,a,8,0x1u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c5),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44362b; continue; }
            pc = 0x4435dd; continue;
            }
            0x4435dd => {
            c.set(1,32,rm(m,0x66d640u32,32));
            c.set(7,32,rm(m,0x66a5b4u32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            pc = 0x4435f5; continue;
            }
            0x4435f5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = c.get(7,32); c.push(m, 0x443605); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4435f5; continue; }
            pc = 0x443609; continue;
            }
            0x443609 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(2,32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5b8u32,32); c.push(m, 0x443618); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5bcu32,32); c.push(m, 0x44362b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x44362b; continue;
            }
            0x44362b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x44),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44364a; continue; }
            pc = 0x44363e; continue;
            }
            0x44363e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x18c); wm(m,a,8,0x1u32); }
            pc = 0x44364a; continue;
            }
            0x44364a => {
            c.set(0,32,c.get(3,32));
            pc = 0x44347e; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
