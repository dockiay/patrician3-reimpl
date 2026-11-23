//! Generado por `reimpl/tools/ui_rt_grp.py cy2` con `tools/lift.py` (no editar a mano): grupo `cy2`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_CY2: [(u32, super::core_table::CoreFn); 47] = [
    (0x4d6fd0, f_004d6fd0),
    (0x4db100, f_004db100),
    (0x4f3af0, f_004f3af0),
    (0x4f4400, f_004f4400),
    (0x4f4550, f_004f4550),
    (0x4f56c0, f_004f56c0),
    (0x4f8ca0, f_004f8ca0),
    (0x4f8d20, f_004f8d20),
    (0x4f8d70, f_004f8d70),
    (0x4f8d90, f_004f8d90),
    (0x4f8e30, f_004f8e30),
    (0x4f8e90, f_004f8e90),
    (0x4f8eb0, f_004f8eb0),
    (0x4f8f10, f_004f8f10),
    (0x4f9280, f_004f9280),
    (0x4f9400, f_004f9400),
    (0x4fac70, f_004fac70),
    (0x4facd0, f_004facd0),
    (0x4fb580, f_004fb580),
    (0x4fd680, f_004fd680),
    (0x5004c0, f_005004c0),
    (0x5008c0, f_005008c0),
    (0x500c30, f_00500c30),
    (0x515c00, f_00515c00),
    (0x515f70, f_00515f70),
    (0x5171e0, f_005171e0),
    (0x51a3c0, f_0051a3c0),
    (0x51a450, f_0051a450),
    (0x51f050, f_0051f050),
    (0x521240, f_00521240),
    (0x524d60, f_00524d60),
    (0x5256f0, f_005256f0),
    (0x52d0b0, f_0052d0b0),
    (0x52dff0, f_0052dff0),
    (0x5306b0, f_005306b0),
    (0x530780, f_00530780),
    (0x5319d0, f_005319d0),
    (0x533ea0, f_00533ea0),
    (0x534600, f_00534600),
    (0x543d10, f_00543d10),
    (0x544590, f_00544590),
    (0x544730, f_00544730),
    (0x4f3ce0, f_004f3ce0),
    (0x534960, f_00534960),
    (0x4f8b50, f_004f8b50),
    (0x4fdce0, f_004fdce0),
    (0x534330, f_00534330),
];
pub fn f_004d6fd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6fd0;
    loop {
        match pc {
            0x4d6fd0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4d701c; continue; }
            pc = 0x4d6fe1; continue;
            }
            0x4d6fe1 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4d6fe3; continue;
            }
            0x4d6fe3 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4d701c; continue; }
            pc = 0x4d6fea; continue;
            }
            0x4d6fea => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x4),8));
            { let a = c.get(2,8); c.sub(a,0x21u32,8); }
            if c.cond(5) { pc = 0x4d7001; continue; }
            pc = 0x4d6ffb; continue;
            }
            0x4d6ffb => {
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,c.get(3,16)); }
            pc = 0x4d7009; continue;
            }
            0x4d7001 => {
            { let a = c.get(2,8); c.sub(a,0x22u32,8); }
            if c.cond(5) { pc = 0x4d7009; continue; }
            pc = 0x4d7006; continue;
            }
            0x4d7006 => {
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,c.get(3,8)); }
            pc = 0x4d7009; continue;
            }
            0x4d7009 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4d6fe3; continue; }
            pc = 0x4d701c; continue;
            }
            0x4d701c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004db100(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4db100;
    loop {
        match pc {
            0x4db100 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            c.set(5,32,c.r[1].wrapping_add(0x8));
            if c.cond(3) { pc = 0x4db210; continue; }
            pc = 0x4db121; continue;
            }
            0x4db121 => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x776));
            c.set(6,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(3) { pc = 0x4db210; continue; }
            pc = 0x4db147; continue;
            }
            0x4db147 => {
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xc),16),16); }
            if c.cond(3) { pc = 0x4db210; continue; }
            pc = 0x4db155; continue;
            }
            0x4db155 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x36u32,16); }
            c.set(6,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4db1a3; continue; }
            pc = 0x4db16a; continue;
            }
            0x4db16a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4db1a3; continue; }
            pc = 0x4db175; continue;
            }
            0x4db175 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(7,32,0x3u32);
            pc = 0x4db17d; continue;
            }
            0x4db17d => {
            c.set(3,16,rm(m,c.r[0],16));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4db18a; continue; }
            pc = 0x4db184; continue;
            }
            0x4db184 => {
            { let a = c.r[0].wrapping_add(0xfffffffe); wm(m,a,16,c.get(3,16)); }
            pc = 0x4db199; continue;
            }
            0x4db18a => {
            { let a = c.get(3,32); let r = c.and(a,0x7fffu32,32); c.set(3,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),16),16); }
            if c.cond(5) { pc = 0x4db199; continue; }
            pc = 0x4db197; continue;
            }
            0x4db197 => {
            c.set(2,8,0x1u32);
            pc = 0x4db199; continue;
            }
            0x4db199 => {
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4db17d; continue; }
            pc = 0x4db19f; continue;
            }
            0x4db19f => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4db1c5; continue; }
            pc = 0x4db1a3; continue;
            }
            0x4db1a3 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(5,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xc),16),16); }
            if c.cond(2) { pc = 0x4db155; continue; }
            pc = 0x4db1bd; continue;
            }
            0x4db1bd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4db1c5 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,0xffffu32);
            { let a = c.get(2,32); let r = c.and(a,0x7fffu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,16,c.get(7,16)); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(2) { pc = 0x4db210; continue; }
            pc = 0x4db1e1; continue;
            }
            0x4db1e1 => {
            c.set(2,16,rm(m,c.r[5],16));
            c.set(6,32,rm(m,c.r[1],32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            { let a = c.r[5]; wm(m,a,16,c.get(6,16)); }
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x6); wm(m,a,16,c.get(7,16)); }
            c.set(6,32,rm(m,c.r[1],32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(2,16)); }
            pc = 0x4db210; continue;
            }
            0x4db210 => {
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

pub fn f_004f3af0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3af0;
    loop {
        match pc {
            0x4f3af0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662e78u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x4f3b2f);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x44),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x2u32); }
            c.set(6,32,c.get(0,32));
            c.push(m, 0x4f3b55);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f3b6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6b1338),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4f3b7d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4f3b92);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4f3ce0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6b1358u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f3bb2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4f3bc3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,0x6de52eu32,8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3du32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.add(a,0x64u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f3be2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4f3bed);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[1].wrapping_add(0xb); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[2].wrapping_add(0xe); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,0xffu32); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x6b1348),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4f3c47; continue;
            }
            0x4f3c47 => {
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4f3c47; continue; }
            pc = 0x4f3c58; continue;
            }
            0x4f3c58 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4f3c78; continue; }
            pc = 0x4f3c63; continue;
            }
            0x4f3c63 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[7]),8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[7].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x4f3c63; continue; }
            pc = 0x4f3c78; continue;
            }
            0x4f3c78 => {
            c.set(2,32,0x4u32);
            pc = 0x4f3c7d; continue;
            }
            0x4f3c7d => {
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            c.set(7,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x4f3c7d; continue; }
            pc = 0x4f3c88; continue;
            }
            0x4f3c88 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4f3c9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x4f3cac);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4f3cb9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4f3cca);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f4400(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f4400;
    loop {
        match pc {
            0x4f4400 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x4f4418; continue; }
            pc = 0x4f440f; continue;
            }
            0x4f440f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4f4418 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f4459; continue; }
            pc = 0x4f4454; continue;
            }
            0x4f4454 => {
            { let a = c.get(2,8); c.and(a,0x6u32,8); }
            if c.cond(4) { pc = 0x4f446d; continue; }
            pc = 0x4f4459; continue;
            }
            0x4f4459 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(2,8); let r = c.and(a,0x2u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4f446d; continue; }
            pc = 0x4f4464; continue;
            }
            0x4f4464 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4f446d => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(1,8));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x4f44b2; continue; }
            pc = 0x4f447b; continue;
            }
            0x4f447b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc),16));
            pc = 0x4f4481; continue;
            }
            0x4f4481 => {
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4f44bc; continue; }
            pc = 0x4f4491; continue;
            }
            0x4f4491 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f449c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x2c6),8));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x4f4481; continue; }
            pc = 0x4f44b2; continue;
            }
            0x4f44b2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x53c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x4f44c5; continue; }
            pc = 0x4f44bc; continue;
            }
            0x4f44bc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4f44c5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x470),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4f44d6; continue; }
            pc = 0x4f44cf; continue;
            }
            0x4f44cf => {
            c.set(6,32,0x4b0u32);
            pc = 0x4f4525; continue;
            }
            0x4f44d6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f4501; continue; }
            pc = 0x4f44dc; continue;
            }
            0x4f44dc => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x546),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,0x320u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4e2u32,32); }
            if c.cond(14) { pc = 0x4f4525; continue; }
            pc = 0x4f44fa; continue;
            }
            0x4f44fa => {
            c.set(6,32,0x4e2u32);
            pc = 0x4f4525; continue;
            }
            0x4f4501 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x547),8));
            c.set(3,32,c.get(0,32));
            c.set(0,32,0x1f4u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x546),8));
            c.set(6,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x190));
            pc = 0x4f4525; continue;
            }
            0x4f4525 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,c.get(5,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x4f4543);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x52a7d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f4550(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f4550;
    loop {
        match pc {
            0x4f4550 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x4f4570; continue; }
            pc = 0x4f4564; continue;
            }
            0x4f4564 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f4570 => {
            { let a = c.get(0,8); c.and(a,0x6u32,8); }
            if c.cond(4) { pc = 0x4f4580; continue; }
            pc = 0x4f4574; continue;
            }
            0x4f4574 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f4580 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.get(3,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f45af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f45c4; continue; }
            pc = 0x4f45b8; continue;
            }
            0x4f45b8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f45c4 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f45db; continue; }
            pc = 0x4f45cf; continue;
            }
            0x4f45cf => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f45db => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x19),8),8); }
            if c.cond(4) { pc = 0x4f461c; continue; }
            pc = 0x4f45e4; continue;
            }
            0x4f45e4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xc),16));
            pc = 0x4f45ea; continue;
            }
            0x4f45ea => {
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4f4856; continue; }
            pc = 0x4f45fe; continue;
            }
            0x4f45fe => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f4609);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4f45ea; continue; }
            pc = 0x4f461c; continue;
            }
            0x4f461c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x7ac),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x80c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4f4649);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4daf10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4f467e; continue; }
            pc = 0x4f4665; continue;
            }
            0x4f4665 => {
            c.set(1,32,c.r[5].wrapping_mul(4));
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            pc = 0x4f46bf; continue;
            }
            0x4f467e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f46bf; continue; }
            pc = 0x4f4682; continue;
            }
            0x4f4682 => {
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.fstp64(m,c.r[4].wrapping_add(0x20));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),8); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x4f46a5);
            { let args = []; c.r[0] = host.call(m, 0x63c654, c.r[1], &args); }
            c.fret();
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),8); c.fop('*',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.ld(m,0x672dc8u32,8); c.fop('*',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x4f46bd);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            pc = 0x4f46bf; continue;
            }
            0x4f46bf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,32,c.r[0].wrapping_add(0xffffffeb));
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(7) { pc = 0x4f4808; continue; }
            pc = 0x4f46d0; continue;
            }
            0x4f46d0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4f492c),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x4f4920)); continue;
            }
            0x4f46df => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2d8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x762),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x2f8),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f48fb; continue; }
            pc = 0x4f4708; continue;
            }
            0x4f4708 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f4710);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x15u32,8); }
            if c.cond(2) { pc = 0x4f472b; continue; }
            pc = 0x4f4718; continue;
            }
            0x4f4718 => {
            { let a = c.get(1,8); c.sub(a,0x17u32,8); }
            if c.cond(7) { pc = 0x4f472b; continue; }
            pc = 0x4f471d; continue;
            }
            0x4f471d => {
            c.set(1,32,rm(m,0x672a18u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x4f472b; continue;
            }
            0x4f472b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f4708; continue; }
            pc = 0x4f473e; continue;
            }
            0x4f473e => {
            pc = 0x4f48fb; continue;
            }
            0x4f4743 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2dc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x762),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x2fa),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f48fb; continue; }
            pc = 0x4f476c; continue;
            }
            0x4f476c => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f4774);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x18u32,8); }
            if c.cond(2) { pc = 0x4f478c; continue; }
            pc = 0x4f477c; continue;
            }
            0x4f477c => {
            { let a = c.get(1,8); c.sub(a,0x1au32,8); }
            if c.cond(7) { pc = 0x4f478c; continue; }
            pc = 0x4f4781; continue;
            }
            0x4f4781 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x672a1au32,16));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x4f478c; continue;
            }
            0x4f478c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f476c; continue; }
            pc = 0x4f479f; continue;
            }
            0x4f479f => {
            pc = 0x4f48fb; continue;
            }
            0x4f47a4 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2e0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x762),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x2fc),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f48fb; continue; }
            pc = 0x4f47cd; continue;
            }
            0x4f47cd => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f47d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x1bu32,8); }
            if c.cond(2) { pc = 0x4f47f0; continue; }
            pc = 0x4f47dd; continue;
            }
            0x4f47dd => {
            { let a = c.get(1,8); c.sub(a,0x1du32,8); }
            if c.cond(7) { pc = 0x4f47f0; continue; }
            pc = 0x4f47e2; continue;
            }
            0x4f47e2 => {
            c.set(1,32,rm(m,0x672a1cu32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x4f47f0; continue;
            }
            0x4f47f0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f47cd; continue; }
            pc = 0x4f4803; continue;
            }
            0x4f4803 => {
            pc = 0x4f48fb; continue;
            }
            0x4f4808 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x672c2c),8));
            c.set(7,32,0x3e8u32);
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f482b; continue; }
            pc = 0x4f4818; continue;
            }
            0x4f4818 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x848),16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x4f483c; continue; }
            pc = 0x4f4827; continue;
            }
            0x4f4827 => {
            c.set(1,8,0xdu32);
            pc = 0x4f483f; continue;
            }
            0x4f482b => {
            { let a = c.get(2,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4f483c; continue; }
            pc = 0x4f4830; continue;
            }
            0x4f4830 => {
            { let a = rm(m,c.r[6].wrapping_add(0x2cc),32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4f483c; continue; }
            pc = 0x4f4838; continue;
            }
            0x4f4838 => {
            c.set(1,8,0xau32);
            pc = 0x4f483f; continue;
            }
            0x4f483c => {
            { let a = c.get(1,8); let r = c.or(a,0xffu32,8); c.set(1,8,r); }
            pc = 0x4f483f; continue;
            }
            0x4f483f => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x848),16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x4f4851; continue; }
            pc = 0x4f484c; continue;
            }
            0x4f484c => {
            { let a = c.get(2,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4f4862; continue; }
            pc = 0x4f4851; continue;
            }
            0x4f4851 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4f4862; continue; }
            pc = 0x4f4856; continue;
            }
            0x4f4856 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f4862 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x784),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f4876);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4f48ff; continue; }
            pc = 0x4f4880; continue;
            }
            0x4f4880 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2c4),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f4894);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x4f48e1; continue; }
            pc = 0x4f489b; continue;
            }
            0x4f489b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2cc),16));
            c.set(1,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4f48e1; continue; }
            pc = 0x4f48b1; continue;
            }
            0x4f48b1 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f48bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xc),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4f48b1; continue; }
            pc = 0x4f48e1; continue;
            }
            0x4f48e1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2ca),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f48f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4f4880; continue; }
            pc = 0x4f48fb; continue;
            }
            0x4f48fb => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4f490b; continue; }
            pc = 0x4f48ff; continue;
            }
            0x4f48ff => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x4f490b => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f56c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f56c0;
    loop {
        match pc {
            0x4f56c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f56d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,8,c.get(0,8));
            c.set(0,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            if c.cond(14) { pc = 0x4f5724; continue; }
            pc = 0x4f56eb; continue;
            }
            0x4f56eb => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[6]),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4f571d; continue; }
            pc = 0x4f5706; continue;
            }
            0x4f5706 => {
            { let a = c.get(5,32); let r = c.add(a,0xffffu32,32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.r[7].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2); wm(m,a,16,c.get(1,16)); }
            pc = 0x4f571d; continue;
            }
            0x4f571d => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4f56eb; continue; }
            pc = 0x4f5722; continue;
            }
            0x4f5722 => {
            pc = 0x4f5728; continue;
            }
            0x4f5724 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x4f5728; continue;
            }
            0x4f5728 => {
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x4f582d; continue; }
            pc = 0x4f5731; continue;
            }
            0x4f5731 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x3c),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f57a4; continue; }
            pc = 0x4f573d; continue;
            }
            0x4f573d => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4f5746);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(6,32,c.get(5,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xbfu32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(3,8)); }
            if c.cond(14) { pc = 0x4f57a4; continue; }
            pc = 0x4f5764; continue;
            }
            0x4f5764 => {
            c.set(1,32,c.r[4].wrapping_add(0x1e));
            c.set(2,32,0x7u32);
            pc = 0x4f576d; continue;
            }
            0x4f576d => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(8) { pc = 0x4f5778; continue; }
            pc = 0x4f5770; continue;
            }
            0x4f5770 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(2)),16));
            pc = 0x4f577d; continue;
            }
            0x4f5778 => {
            c.set(0,32,0xffffu32);
            pc = 0x4f577d; continue;
            }
            0x4f577d => {
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x4f576d; continue; }
            pc = 0x4f5786; continue;
            }
            0x4f5786 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5795);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(1,8)); }
            if c.cond(15) { pc = 0x4f5764; continue; }
            pc = 0x4f57a4; continue;
            }
            0x4f57a4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x60c),16));
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[6]));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f57c5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x60c),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x624),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4f57e6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7]));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x60c),16));
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(2)));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5807);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x624),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f581e; continue; }
            pc = 0x4f5815; continue;
            }
            0x4f5815 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4f581b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4f581e; continue;
            }
            0x4f581e => {
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x624); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x60c); wm(m,a,16,c.get(6,16)); }
            pc = 0x4f582d; continue;
            }
            0x4f582d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8ca0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8ca0;
    loop {
        match pc {
            0x4f8ca0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4f8cc3; continue; }
            pc = 0x4f8caf; continue;
            }
            0x4f8caf => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66fd54u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            pc = 0x4f8d03; continue;
            }
            0x4f8cc3 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4f8d03; continue; }
            pc = 0x4f8cc9; continue;
            }
            0x4f8cc9 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x120));
            pc = 0x4f8cd3; continue;
            }
            0x4f8cd3 => {
            c.set(5,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x2c8),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4f8cf3; continue; }
            pc = 0x4f8ce3; continue;
            }
            0x4f8ce3 => {
            c.fld(c.ld(m,c.r[0],4));
            { let v = c.ld(m,0x66fd54u32,4); c.fop('-',v); }
            c.fstp32(m,c.r[0]);
            c.set(7,32,rm(m,0x6de4b0u32,32));
            pc = 0x4f8cf3; continue;
            }
            0x4f8cf3 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4f8cd3; continue; }
            pc = 0x4f8d01; continue;
            }
            0x4f8d01 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4f8d03; continue;
            }
            0x4f8d03 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1e),8));
            { let a = rm(m,c.r[1].wrapping_add(0x22),16); let r = c.add(a,0xffc4u32,16); { let a = c.r[1].wrapping_add(0x22); wm(m,a,16,r); } }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x4f8d15; continue; }
            pc = 0x4f8d10; continue;
            }
            0x4f8d10 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[1].wrapping_add(0x1e); wm(m,a,8,c.get(0,8)); }
            pc = 0x4f8d15; continue;
            }
            0x4f8d15 => {
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8d20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8d20;
    loop {
        match pc {
            0x4f8d20 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4f8d63; continue; }
            pc = 0x4f8d2a; continue;
            }
            0x4f8d2a => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x48));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            pc = 0x4f8d3d; continue;
            }
            0x4f8d3d => {
            c.set(6,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.sub(a,0x9f8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c8),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4f8d5f; continue; }
            pc = 0x4f8d55; continue;
            }
            0x4f8d55 => {
            c.fld(c.ld(m,c.r[1],4));
            { let v = c.ld(m,0x66fd5cu32,4); c.fop('+',v); }
            c.fstp32(m,c.r[1]);
            pc = 0x4f8d5f; continue;
            }
            0x4f8d5f => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x4f8d3d; continue; }
            pc = 0x4f8d62; continue;
            }
            0x4f8d62 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4f8d63; continue;
            }
            0x4f8d63 => {
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8d70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8d70;
    loop {
        match pc {
            0x4f8d70 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x18u32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8d90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8d90;
    loop {
        match pc {
            0x4f8d90 => {
            c.push(m,c.get(1,32));
            c.set(0,16,rm(m,0x672ddeu32,16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x16),16));
            { let a = c.r[4]; wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            if c.cond(14) { pc = 0x4f8dcb; continue; }
            pc = 0x4f8da9; continue;
            }
            0x4f8da9 => {
            c.set(6,32,0x7fffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,sx(c.get(2,16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x4f8dc3; continue; }
            pc = 0x4f8db7; continue;
            }
            0x4f8db7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f8dec; continue;
            }
            0x4f8dc3 => {
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,0x7fffu32); }
            pc = 0x4f8dec; continue;
            }
            0x4f8dcb => {
            c.set(6,32,sx(c.get(2,16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.add(a,0x8000u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4f8de6; continue; }
            pc = 0x4f8dda; continue;
            }
            0x4f8dda => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f8dec; continue;
            }
            0x4f8de6 => {
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,0x8000u32); }
            pc = 0x4f8dec; continue;
            }
            0x4f8dec => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f8e1c; continue; }
            pc = 0x4f8df4; continue;
            }
            0x4f8df4 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4f8e1c; continue; }
            pc = 0x4f8dff; continue;
            }
            0x4f8dff => {
            c.set(0,32,c.r[1].wrapping_add(0x120));
            pc = 0x4f8e05; continue;
            }
            0x4f8e05 => {
            c.fld(c.ld(m,c.r[0],4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x464),4); c.fop('+',v); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.fstp32(m,c.r[0].wrapping_add(0xfffffff4));
            { let a = c.get(2,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(2) { pc = 0x4f8e05; continue; }
            pc = 0x4f8e1c; continue;
            }
            0x4f8e1c => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8e30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8e30;
    loop {
        match pc {
            0x4f8e30 => {
            c.fild32(m,c.r[4].wrapping_add(0x8));
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,0x672f30u32,4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,0x66fd54u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4f8e61; continue; }
            pc = 0x4f8e59; continue;
            }
            0x4f8e59 => {
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x40400000u32); }
            pc = 0x4f8e61; continue;
            }
            0x4f8e61 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x464),4); c.fop('*',v); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4); c.fop('+',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8e90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8e90;
    loop {
        match pc {
            0x4f8e90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66f4a4u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8eb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8eb0;
    loop {
        match pc {
            0x4f8eb0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f8ee5; continue; }
            pc = 0x4f8ebb; continue;
            }
            0x4f8ebb => {
            { let a = c.r[4]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.fild64(m,c.r[4]);
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x464),4); c.fop('*',v); }
            { let v = c.ld(m,0x66bc58u32,4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            pc = 0x4f8eed; continue;
            }
            0x4f8ee5 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xbf800000u32); }
            pc = 0x4f8eed; continue;
            }
            0x4f8eed => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4); c.fop('+',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8f10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8f10;
    loop {
        match pc {
            0x4f8f10 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(7) { pc = 0x4f8fa6; continue; }
            pc = 0x4f8f1d; continue;
            }
            0x4f8f1d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x4f8fc4),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x4f8fac)); continue;
            }
            0x4f8f2c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f8f45 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66f4a4u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f8f5e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x66f4a0u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f8f77 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),4));
            { let v = c.ld(m,0x672f38u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f8f90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x672f34u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            pc = 0x4f8fa6; continue;
            }
            0x4f8fa6 => {
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f9280(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f9280;
    loop {
        match pc {
            0x4f9280 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.fild32(m,c.r[4].wrapping_add(0x4));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x18u32,32); c.set(0,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,0x672f44u32,4); c.fop('*',v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4); c.fop('+',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f9400(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f9400;
    loop {
        match pc {
            0x4f9400 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x468),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f941d; continue; }
            pc = 0x4f9412; continue;
            }
            0x4f9412 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6df278u32);
            c.push(m, 0x4f941d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x534960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f941d; continue;
            }
            0x4f941d => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x4d8));
            c.push(m, 0x4f9429);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x534960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fac70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fac70;
    loop {
        match pc {
            0x4fac70 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[1].wrapping_add(0x4),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4facc6; continue; }
            pc = 0x4fac83; continue;
            }
            0x4fac83 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(0,32));
            pc = 0x4fac86; continue;
            }
            0x4fac86 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x4facae; continue; }
            pc = 0x4fac8c; continue;
            }
            0x4fac8c => {
            { let a = c.get(0,8); let r = c.and(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4fac97; continue; }
            pc = 0x4fac93; continue;
            }
            0x4fac93 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4facaf; continue;
            }
            0x4fac97 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xbu32,32); }
            if c.cond(4) { pc = 0x4facab; continue; }
            pc = 0x4faca1; continue;
            }
            0x4faca1 => {
            { let a = c.get(0,32); c.sub(a,0x11u32,32); }
            if c.cond(14) { pc = 0x4facaf; continue; }
            pc = 0x4faca6; continue;
            }
            0x4faca6 => {
            { let a = c.get(0,32); c.sub(a,0x1eu32,32); }
            if c.cond(15) { pc = 0x4facaf; continue; }
            pc = 0x4facab; continue;
            }
            0x4facab => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4facaf; continue;
            }
            0x4facae => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x4facaf; continue;
            }
            0x4facaf => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4fac86; continue; }
            pc = 0x4facb2; continue;
            }
            0x4facb2 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(6) { pc = 0x4facc6; continue; }
            pc = 0x4facb7; continue;
            }
            0x4facb7 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let b = c.get(6,32); c.div32(b); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4facc6 => {
            c.set(0,32,0x64u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004facd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4facd0;
    loop {
        match pc {
            0x4facd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,0x32u32,32); }
            if c.cond(12) { pc = 0x4fb149; continue; }
            pc = 0x4face6; continue;
            }
            0x4face6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x37u32,32); }
            c.set(0,32,c.get(1,32));
            if c.cond(4) { pc = 0x4faff4; continue; }
            pc = 0x4facff; continue;
            }
            0x4facff => {
            { let a = c.get(7,32); c.sub(a,0x34u32,32); }
            if c.cond(4) { pc = 0x4faff4; continue; }
            pc = 0x4fad08; continue;
            }
            0x4fad08 => {
            { let a = c.get(7,32); c.sub(a,0x38u32,32); }
            if c.cond(4) { pc = 0x4fae98; continue; }
            pc = 0x4fad11; continue;
            }
            0x4fad11 => {
            { let a = c.get(7,32); c.sub(a,0x35u32,32); }
            if c.cond(4) { pc = 0x4fae98; continue; }
            pc = 0x4fad1a; continue;
            }
            0x4fad1a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x4fad64; continue; }
            pc = 0x4fad5b; continue;
            }
            0x4fad5b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff));
            pc = 0x4fad6c; continue;
            }
            0x4fad64 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            pc = 0x4fad6c; continue;
            }
            0x4fad6c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x2u32,32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x3fu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fada0; continue; }
            pc = 0x4fad98; continue;
            }
            0x4fad98 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fada0; continue;
            }
            0x4fada0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x3fu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fadbe; continue; }
            pc = 0x4fadb6; continue;
            }
            0x4fadb6 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fadbe; continue;
            }
            0x4fadbe => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[7]),8));
            c.set(5,32,c.r[1].wrapping_add(c.r[7]));
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.and(a,0x3fu32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fade0; continue; }
            pc = 0x4fadd7; continue;
            }
            0x4fadd7 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[5]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fade0; continue;
            }
            0x4fade0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.sub(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fadfc; continue; }
            pc = 0x4fadf4; continue;
            }
            0x4fadf4 => {
            { let a = c.get(1,8); let r = c.and(a,0xe0u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.or(a,0x20u32,8); c.set(1,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4fadfc; continue;
            }
            0x4fadfc => {
            { let a = rm(m,c.r[4].wrapping_add(0x40),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4fae0c; continue; }
            pc = 0x4fae03; continue;
            }
            0x4fae03 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1));
            pc = 0x4fae0f; continue;
            }
            0x4fae0c => {
            c.set(1,32,c.r[0].wrapping_add(0x1));
            pc = 0x4fae0f; continue;
            }
            0x4fae0f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x3fu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fae2a; continue; }
            pc = 0x4fae22; continue;
            }
            0x4fae22 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fae2a; continue;
            }
            0x4fae2a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x3fu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fae48; continue; }
            pc = 0x4fae40; continue;
            }
            0x4fae40 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fae48; continue;
            }
            0x4fae48 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.and(a,0x3fu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fae68; continue; }
            pc = 0x4fae60; continue;
            }
            0x4fae60 => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fae68; continue;
            }
            0x4fae68 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.and(a,0x3fu32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fae86; continue; }
            pc = 0x4fae7e; continue;
            }
            0x4fae7e => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fae86; continue;
            }
            0x4fae86 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4fae8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fdce0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fae98 => {
            c.set(3,32,c.get(1,32));
            c.set(5,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x2u32); }
            pc = 0x4faea8; continue;
            }
            0x4faea8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4faec1; continue; }
            pc = 0x4faebe; continue;
            }
            0x4faebe => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faec1; continue;
            }
            0x4faec1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(2,8); let r = c.and(a,0xc4u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4faeda; continue; }
            pc = 0x4faed7; continue;
            }
            0x4faed7 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faeda; continue;
            }
            0x4faeda => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(2,8); let r = c.and(a,0xc3u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x3u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4faef3; continue; }
            pc = 0x4faef0; continue;
            }
            0x4faef0 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faef3; continue;
            }
            0x4faef3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(2,8); let r = c.and(a,0xc3u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x3u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4faf0c; continue; }
            pc = 0x4faf09; continue;
            }
            0x4faf09 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faf0c; continue;
            }
            0x4faf0c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(2,8); let r = c.and(a,0xc4u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4faf25; continue; }
            pc = 0x4faf22; continue;
            }
            0x4faf22 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faf25; continue;
            }
            0x4faf25 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4faf3d; continue; }
            pc = 0x4faf3a; continue;
            }
            0x4faf3a => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4faf3d; continue;
            }
            0x4faf3d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x4faea8; continue; }
            pc = 0x4faf4c; continue;
            }
            0x4faf4c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xfffffffe));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4faf6c; continue; }
            pc = 0x4faf66; continue;
            }
            0x4faf66 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            pc = 0x4faf6c; continue;
            }
            0x4faf6c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.sub(a,0x4u32,32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(0,8,c.get(2,8));
            { let a = c.get(0,8); let r = c.and(a,0x3fu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4faf94; continue; }
            pc = 0x4faf8c; continue;
            }
            0x4faf8c => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4faf94; continue;
            }
            0x4faf94 => {
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4faf9e; continue; }
            pc = 0x4faf99; continue;
            }
            0x4faf99 => {
            c.set(0,32,c.r[1].wrapping_add(0x1));
            pc = 0x4fafa6; continue;
            }
            0x4faf9e => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x4fafa6; continue;
            }
            0x4fafa6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fafbf; continue; }
            pc = 0x4fafb9; continue;
            }
            0x4fafb9 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            pc = 0x4fafbf; continue;
            }
            0x4fafbf => {
            c.set(0,32,0x2u32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x7u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb1bc; continue; }
            pc = 0x4fafe4; continue;
            }
            0x4fafe4 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4faff4 => {
            c.set(5,32,c.get(1,32));
            c.set(2,32,0x2u32);
            { let a = c.get(5,32); let r = c.and(a,0x1u32,32); c.set(5,32,r); }
            pc = 0x4faffe; continue;
            }
            0x4faffe => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(3,8); let r = c.and(a,0xe0u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x20u32,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4fb01a; continue; }
            pc = 0x4fb017; continue;
            }
            0x4fb017 => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb01a; continue;
            }
            0x4fb01a => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(3,8); let r = c.and(a,0xc4u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4fb036; continue; }
            pc = 0x4fb033; continue;
            }
            0x4fb033 => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb036; continue;
            }
            0x4fb036 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(3,8); let r = c.and(a,0xc3u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x3u32,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4fb052; continue; }
            pc = 0x4fb04f; continue;
            }
            0x4fb04f => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb052; continue;
            }
            0x4fb052 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(3,8); let r = c.and(a,0xc3u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x3u32,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4fb06e; continue; }
            pc = 0x4fb06b; continue;
            }
            0x4fb06b => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb06e; continue;
            }
            0x4fb06e => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(3,8); let r = c.and(a,0xc4u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4fb08a; continue; }
            pc = 0x4fb087; continue;
            }
            0x4fb087 => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb08a; continue;
            }
            0x4fb08a => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0],8));
            { let a = c.get(3,8); let r = c.and(a,0xe0u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0x20u32,8); c.set(3,8,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x4fb0a2; continue; }
            pc = 0x4fb09d; continue;
            }
            0x4fb09d => {
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            pc = 0x4fb0aa; continue;
            }
            0x4fb0a2 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x4fb0aa; continue;
            }
            0x4fb0aa => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x4faffe; continue; }
            pc = 0x4fb0b1; continue;
            }
            0x4fb0b1 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(0,8,c.get(2,8));
            { let a = c.get(0,8); let r = c.and(a,0x3fu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb0d4; continue; }
            pc = 0x4fb0cc; continue;
            }
            0x4fb0cc => {
            { let a = c.get(2,8); let r = c.and(a,0xe0u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4fb0d4; continue;
            }
            0x4fb0d4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xfffffffc));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb0f7; continue; }
            pc = 0x4fb0f1; continue;
            }
            0x4fb0f1 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            pc = 0x4fb0f7; continue;
            }
            0x4fb0f7 => {
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            c.set(0,32,c.r[1].wrapping_add(0x1));
            if c.cond(5) { pc = 0x4fb102; continue; }
            pc = 0x4fb0ff; continue;
            }
            0x4fb0ff => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb102; continue;
            }
            0x4fb102 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb11b; continue; }
            pc = 0x4fb115; continue;
            }
            0x4fb115 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            pc = 0x4fb11b; continue;
            }
            0x4fb11b => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(5,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0xfffffff9));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb1bc; continue; }
            pc = 0x4fb139; continue;
            }
            0x4fb139 => {
            { let a = c.get(0,8); let r = c.and(a,0xe0u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fb149 => {
            { let a = c.get(7,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x4fb1c6; continue; }
            pc = 0x4fb14e; continue;
            }
            0x4fb14e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,8,0xeu32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(1,8,0x1eu32);
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(1,8)); }
            if c.cond(4) { pc = 0x4fb19c; continue; }
            pc = 0x4fb177; continue;
            }
            0x4fb177 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffffff));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fb19c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffffd); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(1,8)); }
            pc = 0x4fb1b0; continue;
            }
            0x4fb1b0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            pc = 0x4fb1bc; continue;
            }
            0x4fb1bc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fb1c6 => {
            { let a = c.get(7,32); c.sub(a,0x30u32,32); }
            if c.cond(4) { pc = 0x4fb50f; continue; }
            pc = 0x4fb1cf; continue;
            }
            0x4fb1cf => {
            { let a = c.get(7,32); c.sub(a,0x31u32,32); }
            if c.cond(4) { pc = 0x4fb50f; continue; }
            pc = 0x4fb1d8; continue;
            }
            0x4fb1d8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(7,32); c.sub(a,0x2fu32,32); }
            if c.cond(5) { pc = 0x4fb3d3; continue; }
            pc = 0x4fb1e5; continue;
            }
            0x4fb1e5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,8,0xfu32); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(1,8,0x1eu32);
            { let a = c.r[3].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[3].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(1,8)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            if c.cond(4) { pc = 0x4fb3cc; continue; }
            pc = 0x4fb217; continue;
            }
            0x4fb217 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            pc = 0x4fb219; continue;
            }
            0x4fb219 => {
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            pc = 0x4fb21d; continue;
            }
            0x4fb21d => {
            c.set(7,32,sx(rm(m,c.r[7].wrapping_add(0x672f84),8),8)&0xffffffffu32);
            pc = 0x4fb224; continue;
            }
            0x4fb224 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x672f64),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.imul(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x40),32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.get(3,32));
            if c.cond(14) { pc = 0x4fb288; continue; }
            pc = 0x4fb23f; continue;
            }
            0x4fb23f => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fb243; continue;
            }
            0x4fb243 => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x672f74),8),8)&0xffffffffu32);
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,32,c.get(3,32));
            if c.cond(14) { pc = 0x4fb274; continue; }
            pc = 0x4fb250; continue;
            }
            0x4fb250 => {
            c.set(5,32,c.get(2,32));
            pc = 0x4fb252; continue;
            }
            0x4fb252 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); let r = c.and(a,0x80u32,8); { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); let r = c.or(a,0x20u32,8); { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb271; continue; }
            pc = 0x4fb26e; continue;
            }
            0x4fb26e => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            pc = 0x4fb271; continue;
            }
            0x4fb271 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4fb252; continue; }
            pc = 0x4fb274; continue;
            }
            0x4fb274 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fb27d; continue; }
            pc = 0x4fb27a; continue;
            }
            0x4fb27a => {
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(3,32,r); }
            pc = 0x4fb27d; continue;
            }
            0x4fb27d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fb243; continue; }
            pc = 0x4fb288; continue;
            }
            0x4fb288 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x32u32,32); }
            if c.cond(13) { pc = 0x4fb402; continue; }
            pc = 0x4fb297; continue;
            }
            0x4fb297 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x4fb29b; continue;
            }
            0x4fb29b => {
            { let a = c.get(5,32); c.sub(a,0x6u32,32); }
            if c.cond(4) { pc = 0x4fb3b4; continue; }
            pc = 0x4fb2a4; continue;
            }
            0x4fb2a4 => {
            c.set(0,8,0x1u32);
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,8); let n = c.get(1,8); let r = c.shl(a,n,8); c.set(0,8,r); }
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0x672f64),8),8)&0xffffffffu32);
            c.set(3,32,sx(rm(m,c.r[5].wrapping_add(0x672f74),8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x672f64),8),8)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x672f74),8),8)&0xffffffffu32);
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0xffffffff));
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4fb305; continue; }
            pc = 0x4fb2fd; continue;
            }
            0x4fb2fd => {
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fb305; continue; }
            pc = 0x4fb304; continue;
            }
            0x4fb304 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x4fb305; continue;
            }
            0x4fb305 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffff));
            c.set(1,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x4fb331; continue; }
            pc = 0x4fb327; continue;
            }
            0x4fb327 => {
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fb331; continue; }
            pc = 0x4fb32c; continue;
            }
            0x4fb32c => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4fb331; continue;
            }
            0x4fb331 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,32,c.r[1].wrapping_add(c.r[3]).wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x4fb3b4; continue; }
            pc = 0x4fb345; continue;
            }
            0x4fb345 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fb349; continue;
            }
            0x4fb349 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4fb396; continue; }
            pc = 0x4fb351; continue;
            }
            0x4fb351 => {
            c.set(2,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x4fb357; continue;
            }
            0x4fb357 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4fb361);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4fb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4fb378; continue; }
            pc = 0x4fb365; continue;
            }
            0x4fb365 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x44),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); let r = c.or(a,c.get(1,8),8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x4fb378; continue;
            }
            0x4fb378 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4fb37f; continue; }
            pc = 0x4fb37e; continue;
            }
            0x4fb37e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x4fb37f; continue;
            }
            0x4fb37f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fb357; continue; }
            pc = 0x4fb38e; continue;
            }
            0x4fb38e => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fb396; continue;
            }
            0x4fb396 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x4fb3a9; continue; }
            pc = 0x4fb3a4; continue;
            }
            0x4fb3a4 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4fb3a9; continue;
            }
            0x4fb3a9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fb349; continue; }
            pc = 0x4fb3b4; continue;
            }
            0x4fb3b4 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x4fb29b; continue; }
            pc = 0x4fb3c2; continue;
            }
            0x4fb3c2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fb3cc => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            pc = 0x4fb219; continue;
            }
            0x4fb3d3 => {
            { let a = c.get(7,32); c.sub(a,0x15u32,32); }
            if c.cond(12) { pc = 0x4fb21d; continue; }
            pc = 0x4fb3dc; continue;
            }
            0x4fb3dc => {
            { let a = c.get(7,32); c.sub(a,0x1eu32,32); }
            if c.cond(13) { pc = 0x4fb21d; continue; }
            pc = 0x4fb3e5; continue;
            }
            0x4fb3e5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x67792fu32,8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4fb21d; continue; }
            pc = 0x4fb3f8; continue;
            }
            0x4fb3f8 => {
            c.set(7,32,0x1u32);
            pc = 0x4fb224; continue;
            }
            0x4fb402 => {
            { let a = c.get(5,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x4fb444; continue; }
            pc = 0x4fb407; continue;
            }
            0x4fb407 => {
            pc = m.r32(c.r[5].wrapping_mul(4).wrapping_add(0x4fb554)); continue;
            }
            0x4fb40e => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            pc = 0x4fb43f; continue;
            }
            0x4fb418 => {
            { let a = c.get(1,32); let r = c.sub(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x4fb444; continue;
            }
            0x4fb41d => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fb444; continue; }
            pc = 0x4fb423; continue;
            }
            0x4fb423 => {
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(1,32,r); }
            pc = 0x4fb444; continue;
            }
            0x4fb428 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fb444; continue; }
            pc = 0x4fb42e; continue;
            }
            0x4fb42e => {
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(1,32,r); }
            pc = 0x4fb444; continue;
            }
            0x4fb433 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x4fb444; continue;
            }
            0x4fb438 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            pc = 0x4fb43f; continue;
            }
            0x4fb43f => {
            if c.cond(4) { pc = 0x4fb444; continue; }
            pc = 0x4fb441; continue;
            }
            0x4fb441 => {
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(1,32,r); }
            pc = 0x4fb444; continue;
            }
            0x4fb444 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fb463; continue; }
            pc = 0x4fb457; continue;
            }
            0x4fb457 => {
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); let r = c.or(a,0x20u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x4fb463; continue;
            }
            0x4fb463 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x4fb402; continue; }
            pc = 0x4fb469; continue;
            }
            0x4fb469 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(0xfffffffe));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x4fb495; continue; }
            pc = 0x4fb48f; continue;
            }
            0x4fb48f => {
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff));
            pc = 0x4fb498; continue;
            }
            0x4fb495 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x4fb498; continue;
            }
            0x4fb498 => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,0x4u32);
            c.set(2,8,0xdu32);
            pc = 0x4fb4a3; continue;
            }
            0x4fb4a3 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4fb4a5; continue;
            }
            0x4fb4a5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb4fc; continue; }
            pc = 0x4fb4b1; continue;
            }
            0x4fb4b1 => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4fb4d7; continue; }
            pc = 0x4fb4b5; continue;
            }
            0x4fb4b5 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb4c6; continue; }
            pc = 0x4fb4bb; continue;
            }
            0x4fb4bb => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fb4fc; continue; }
            pc = 0x4fb4c6; continue;
            }
            0x4fb4c6 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb4f9; continue; }
            pc = 0x4fb4cc; continue;
            }
            0x4fb4cc => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffffff),8); c.sub(a,c.get(2,8),8); }
            pc = 0x4fb4f7; continue;
            }
            0x4fb4d7 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb4e8; continue; }
            pc = 0x4fb4dd; continue;
            }
            0x4fb4dd => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffffff),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fb4fc; continue; }
            pc = 0x4fb4e8; continue;
            }
            0x4fb4e8 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fb4f9; continue; }
            pc = 0x4fb4ee; continue;
            }
            0x4fb4ee => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            pc = 0x4fb4f7; continue;
            }
            0x4fb4f7 => {
            if c.cond(5) { pc = 0x4fb4fc; continue; }
            pc = 0x4fb4f9; continue;
            }
            0x4fb4f9 => {
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fb4fc; continue;
            }
            0x4fb4fc => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4fb4a5; continue; }
            pc = 0x4fb502; continue;
            }
            0x4fb502 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4fb4a3; continue; }
            pc = 0x4fb505; continue;
            }
            0x4fb505 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4fb50f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,8,0x1cu32); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(1,8,0x1eu32);
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(1,8)); }
            if c.cond(4) { pc = 0x4fb1b0; continue; }
            pc = 0x4fb53d; continue;
            }
            0x4fb53d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fb580(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fb580;
    loop {
        match pc {
            0x4fb580 => {
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d),8));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,8); c.sub(a,0x82u32,8); }
            if c.cond(4) { pc = 0x4fb5bc; continue; }
            pc = 0x4fb5b7; continue;
            }
            0x4fb5b7 => {
            { let a = c.get(2,8); c.sub(a,0x83u32,8); }
            if c.cond(5) { pc = 0x4fb5c7; continue; }
            pc = 0x4fb5bc; continue;
            }
            0x4fb5bc => {
            { let a = c.r[0]; wm(m,a,8,0x81u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x4fb5c7; continue;
            }
            0x4fb5c7 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(5,32,0x1u32);
            c.set(3,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fb5e5; continue;
            }
            0x4fb5dd => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4fb5e5; continue;
            }
            0x4fb5e5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[7].wrapping_add(c.r[5]));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4fb61b; continue; }
            pc = 0x4fb60c; continue;
            }
            0x4fb60c => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fb625; continue;
            }
            0x4fb61b => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x1u32); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4fb625; continue;
            }
            0x4fb625 => {
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4fb63b; continue; }
            pc = 0x4fb629; continue;
            }
            0x4fb629 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fb643; continue;
            }
            0x4fb63b => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            pc = 0x4fb643; continue;
            }
            0x4fb643 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4fb65e; continue; }
            pc = 0x4fb64d; continue;
            }
            0x4fb64d => {
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fb666; continue;
            }
            0x4fb65e => {
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x1u32); }
            pc = 0x4fb666; continue;
            }
            0x4fb666 => {
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4fb685; continue; }
            pc = 0x4fb66a; continue;
            }
            0x4fb66a => {
            c.set(0,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x4fb68d; continue;
            }
            0x4fb685 => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x1u32); }
            pc = 0x4fb68d; continue;
            }
            0x4fb68d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4fb70b; continue; }
            pc = 0x4fb6a1; continue;
            }
            0x4fb6a1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4fb718; continue; }
            pc = 0x4fb6a9; continue;
            }
            0x4fb6a9 => {
            c.set(5,32,c.r[7].wrapping_add(0x1));
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            pc = 0x4fb6af; continue;
            }
            0x4fb6af => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x82u32,8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4fb6be; continue; }
            pc = 0x4fb6ba; continue;
            }
            0x4fb6ba => {
            { let a = c.get(0,8); c.sub(a,0x83u32,8); }
            if c.cond(5) { pc = 0x4fb6f7; continue; }
            pc = 0x4fb6be; continue;
            }
            0x4fb6be => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6]; wm(m,a,8,0x81u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb6e5; continue; }
            pc = 0x4fb6c9; continue;
            }
            0x4fb6c9 => {
            c.set(1,32,c.r[7].wrapping_add(c.r[5]));
            c.push(m,0xffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x17u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4fb6e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4fb6e5; continue;
            }
            0x4fb6e5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4fb8da; continue; }
            pc = 0x4fb6f7; continue;
            }
            0x4fb6f7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(0x8),32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4fb6af; continue; }
            pc = 0x4fb705; continue;
            }
            0x4fb705 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fb718; continue;
            }
            0x4fb70b => {
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(6,32,c.get(1,32));
            pc = 0x4fb718; continue;
            }
            0x4fb718 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb80d; continue; }
            pc = 0x4fb728; continue;
            }
            0x4fb728 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4fb783; continue; }
            pc = 0x4fb72c; continue;
            }
            0x4fb72c => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x82u32,8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4fb73b; continue; }
            pc = 0x4fb737; continue;
            }
            0x4fb737 => {
            { let a = c.get(0,8); c.sub(a,0x83u32,8); }
            if c.cond(5) { pc = 0x4fb779; continue; }
            pc = 0x4fb73b; continue;
            }
            0x4fb73b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6]; wm(m,a,8,0x81u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb767; continue; }
            pc = 0x4fb746; continue;
            }
            0x4fb746 => {
            c.set(0,32,c.r[7].wrapping_add(0x1));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4fb763);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4fb767; continue;
            }
            0x4fb767 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4fb8da; continue; }
            pc = 0x4fb779; continue;
            }
            0x4fb779 => {
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4fb72c; continue; }
            pc = 0x4fb783; continue;
            }
            0x4fb783 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4fb787; continue;
            }
            0x4fb787 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4fb826; continue; }
            pc = 0x4fb793; continue;
            }
            0x4fb793 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4fb839; continue; }
            pc = 0x4fb79f; continue;
            }
            0x4fb79f => {
            c.set(2,32,c.get(3,32));
            c.set(5,32,c.r[7].wrapping_add(0x1));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4fb7ad; continue;
            }
            0x4fb7ad => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x82u32,8); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4fb7bc; continue; }
            pc = 0x4fb7b8; continue;
            }
            0x4fb7b8 => {
            { let a = c.get(0,8); c.sub(a,0x83u32,8); }
            if c.cond(5) { pc = 0x4fb7f9; continue; }
            pc = 0x4fb7bc; continue;
            }
            0x4fb7bc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6]; wm(m,a,8,0x81u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb7e7; continue; }
            pc = 0x4fb7c7; continue;
            }
            0x4fb7c7 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[5]));
            c.push(m,0xffu32);
            c.push(m,c.get(2,32));
            c.push(m,0x17u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4fb7df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4fb7e7; continue;
            }
            0x4fb7e7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4fb8da; continue; }
            pc = 0x4fb7f9; continue;
            }
            0x4fb7f9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4fb7ad; continue; }
            pc = 0x4fb807; continue;
            }
            0x4fb807 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fb839; continue;
            }
            0x4fb80d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            pc = 0x4fb787; continue;
            }
            0x4fb826 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            pc = 0x4fb839; continue;
            }
            0x4fb839 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb898; continue; }
            pc = 0x4fb841; continue;
            }
            0x4fb841 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4fb894; continue; }
            pc = 0x4fb849; continue;
            }
            0x4fb849 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.sub(a,0x82u32,8); }
            if c.cond(4) { pc = 0x4fb854; continue; }
            pc = 0x4fb850; continue;
            }
            0x4fb850 => {
            { let a = c.get(0,8); c.sub(a,0x83u32,8); }
            if c.cond(5) { pc = 0x4fb88a; continue; }
            pc = 0x4fb854; continue;
            }
            0x4fb854 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6]; wm(m,a,8,0x81u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fb87c; continue; }
            pc = 0x4fb85f; continue;
            }
            0x4fb85f => {
            c.set(1,32,c.r[7].wrapping_add(0x1));
            c.push(m,0xffu32);
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x17u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4fb87c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4fb87c; continue;
            }
            0x4fb87c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4fb8da; continue; }
            pc = 0x4fb88a; continue;
            }
            0x4fb88a => {
            { let a = c.get(3,32); let r = c.sub(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x4fb849; continue; }
            pc = 0x4fb894; continue;
            }
            0x4fb894 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fb898; continue;
            }
            0x4fb898 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x4fb5dd; continue; }
            pc = 0x4fb8b2; continue;
            }
            0x4fb8b2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4fb5dd; continue; }
            pc = 0x4fb8be; continue;
            }
            0x4fb8be => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4fb5dd; continue; }
            pc = 0x4fb8ca; continue;
            }
            0x4fb8ca => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4fb5dd; continue; }
            pc = 0x4fb8d6; continue;
            }
            0x4fb8d6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fb8da; continue;
            }
            0x4fb8da => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fd680(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fd680;
    loop {
        match pc {
            0x4fd680 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4fd816; continue; }
            pc = 0x4fd699; continue;
            }
            0x4fd699 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5d),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4fd816; continue; }
            pc = 0x4fd6a9; continue;
            }
            0x4fd6a9 => {
            c.set(3,32,c.get(0,32));
            c.set(5,32,c.r[0].wrapping_add(c.r[1]));
            { let a = c.get(3,32); let r = c.imul(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(7,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5]));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4fd6cf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4fd816; continue; }
            pc = 0x4fd6de; continue;
            }
            0x4fd6de => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4fd6fe; continue; }
            pc = 0x4fd6ee; continue;
            }
            0x4fd6ee => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xffffffff),8));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7fu32,8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x4fd6ee; continue; }
            pc = 0x4fd6fe; continue;
            }
            0x4fd6fe => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(1,16); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),16),16); c.set(1,16,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x5d),8));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],8); let r = c.or(a,0x80u32,8); { let a = c.r[1]; wm(m,a,8,r); } }
            pc = 0x4fd726; continue;
            }
            0x4fd722 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4fd726; continue;
            }
            0x4fd726 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4fd744; continue; }
            pc = 0x4fd73c; continue;
            }
            0x4fd73c => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x4fd744; continue;
            }
            0x4fd744 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4fd746; continue;
            }
            0x4fd746 => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x4fd792; continue; }
            pc = 0x4fd74b; continue;
            }
            0x4fd74b => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4fd820)); continue;
            }
            0x4fd752 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0xffffu32,32); c.set(1,32,r); }
            pc = 0x4fd78e; continue;
            }
            0x4fd75e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x4fd78e; continue;
            }
            0x4fd767 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fd779; continue; }
            pc = 0x4fd76e; continue;
            }
            0x4fd76e => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = rm(m,c.r[4].wrapping_add(0x10),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,r); } }
            pc = 0x4fd792; continue;
            }
            0x4fd779 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = rm(m,c.r[4].wrapping_add(0x10),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,r); } }
            pc = 0x4fd792; continue;
            }
            0x4fd784 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffeu32,32); c.set(1,32,r); }
            pc = 0x4fd78e; continue;
            }
            0x4fd78e => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x4fd792; continue;
            }
            0x4fd792 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7a0; continue;
            }
            0x4fd7a0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(8) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7aa; continue;
            }
            0x4fd7aa => {
            { let a = c.get(1,8); let r = c.and(a,0x3fu32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0xcu32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7b2; continue;
            }
            0x4fd7b2 => {
            { let a = c.get(1,8); c.sub(a,0x6u32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7b7; continue;
            }
            0x4fd7b7 => {
            { let a = c.get(1,8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7bc; continue;
            }
            0x4fd7bc => {
            { let a = c.get(1,8); c.sub(a,0x9u32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7c1; continue;
            }
            0x4fd7c1 => {
            { let a = c.get(1,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7c6; continue;
            }
            0x4fd7c6 => {
            { let a = c.get(1,8); c.sub(a,0x7u32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7cb; continue;
            }
            0x4fd7cb => {
            { let a = c.get(1,8); c.sub(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7d0; continue;
            }
            0x4fd7d0 => {
            { let a = c.get(1,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x4fd7f5; continue; }
            pc = 0x4fd7d5; continue;
            }
            0x4fd7d5 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.r[3].wrapping_add(c.r[7].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4fd7e9; continue; }
            pc = 0x4fd7e7; continue;
            }
            0x4fd7e7 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4fd7e9; continue;
            }
            0x4fd7e9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],8); let r = c.or(a,0x80u32,8); { let a = c.r[1]; wm(m,a,8,r); } }
            pc = 0x4fd7f5; continue;
            }
            0x4fd7f5 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4fd746; continue; }
            pc = 0x4fd7ff; continue;
            }
            0x4fd7ff => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(5) { pc = 0x4fd722; continue; }
            pc = 0x4fd809; continue;
            }
            0x4fd809 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4fd813);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4fd816; continue;
            }
            0x4fd816 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005004c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5004c0;
    loop {
        match pc {
            0x5004c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(6,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[5].wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(1,32); let n = 0xcu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            pc = 0x50050d; continue;
            }
            0x500509 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x50050d; continue;
            }
            0x50050d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6734e8),8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5005c7; continue; }
            pc = 0x500520; continue;
            }
            0x500520 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x500529; continue; }
            pc = 0x500524; continue;
            }
            0x500524 => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x50052b; continue;
            }
            0x500529 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x50052b; continue;
            }
            0x50052b => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x500557; continue; }
            pc = 0x500535; continue;
            }
            0x500535 => {
            c.set(1,32,c.r[0].wrapping_add(0xc7));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x50057a; continue;
            }
            0x500557 => {
            c.set(1,32,c.r[0].wrapping_add(0x7cf));
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
            pc = 0x50057a; continue;
            }
            0x50057a => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,0x6de3d8u32);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x5005a3; continue; }
            pc = 0x500585; continue;
            }
            0x500585 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x500595);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5005bf; continue;
            }
            0x5005a3 => {
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5005b3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[3]; wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x5005bf; continue;
            }
            0x5005bf => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5005c7; continue;
            }
            0x5005c7 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x500509; continue; }
            pc = 0x5005d4; continue;
            }
            0x5005d4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005008c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5008c0;
    loop {
        match pc {
            0x5008c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c6),8));
            c.push(m,c.get(5,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x2ce),16));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(3,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            pc = 0x5008f9; continue;
            }
            0x5008f9 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x500936; continue; }
            pc = 0x50090c; continue;
            }
            0x50090c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x500a2f; continue; }
            pc = 0x500918; continue;
            }
            0x500918 => {
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x762),16));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x500a2f; continue; }
            pc = 0x500928; continue;
            }
            0x500928 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            pc = 0x500936; continue;
            }
            0x500936 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x50093e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x500a24; continue; }
            pc = 0x500950; continue;
            }
            0x500950 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x500a24; continue; }
            pc = 0x500961; continue;
            }
            0x500961 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = rm(m,c.r[1].wrapping_add(0x2c4),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x500a24; continue; }
            pc = 0x500977; continue;
            }
            0x500977 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c4),8));
            { let a = c.r[7].wrapping_add(0x23); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x22); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(0x20); wm(m,a,16,0x0u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2ce),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5009cd; continue; }
            pc = 0x5009a2; continue;
            }
            0x5009a2 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5009aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x22),8));
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5009c0; continue; }
            pc = 0x5009bc; continue;
            }
            0x5009bc => {
            { let a = rm(m,c.r[7].wrapping_add(0x20),16); let r = c.inc(a,16); { let a = c.r[7].wrapping_add(0x20); wm(m,a,16,r); } }
            pc = 0x5009c0; continue;
            }
            0x5009c0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5009a2; continue; }
            pc = 0x5009cd; continue;
            }
            0x5009cd => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5009d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x500a3b; continue; }
            pc = 0x5009dc; continue;
            }
            0x5009dc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2cc),16));
            c.set(1,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x500a24; continue; }
            pc = 0x5009fd; continue;
            }
            0x5009fd => {
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x500a08);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x22),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x500a52; continue; }
            pc = 0x500a12; continue;
            }
            0x500a12 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5009fd; continue; }
            pc = 0x500a24; continue;
            }
            0x500a24 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x4),16));
            pc = 0x5008f9; continue;
            }
            0x500a2f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x500a3b => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x500a43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51d6f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x500a52 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x500a5f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d3b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00500c30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x500c30;
    loop {
        match pc {
            0x500c30 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662f7au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2ec),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2c4),16));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x500c7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x500c8a);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.get(5,32));
            c.set(1,32,0x18u32);
            pc = 0x500c98; continue;
            }
            0x500c98 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x500c98; continue; }
            pc = 0x500ca2; continue;
            }
            0x500ca2 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x500cb0; continue; }
            pc = 0x500ca6; continue;
            }
            0x500ca6 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xc8u32); }
            pc = 0x500cfe; continue;
            }
            0x500cb0 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2e2),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.and(a,0x3fu32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xfff9e580u32,32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2ec),16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x500cfe; continue; }
            pc = 0x500cf6; continue;
            }
            0x500cf6 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x61a80u32); }
            pc = 0x500cfe; continue;
            }
            0x500cfe => {
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(0,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,c.r[6].wrapping_add(0x8));
            pc = 0x500d0e; continue;
            }
            0x500d0e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32),32); }
            if c.cond(13) { pc = 0x500d2a; continue; }
            pc = 0x500d20; continue;
            }
            0x500d20 => {
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(2,16)); }
            pc = 0x500d2a; continue;
            }
            0x500d2a => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x500d0e; continue; }
            pc = 0x500d33; continue;
            }
            0x500d33 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x500d3f; continue;
            }
            0x500d3f => {
            c.set(7,32,rm(m,c.r[5],32));
            c.set(0,32,0x3u32);
            pc = 0x500d47; continue;
            }
            0x500d47 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x22),16));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(3,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x500d6e; continue; }
            pc = 0x500d5b; continue;
            }
            0x500d5b => {
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x26); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(2,8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x24); wm(m,a,16,c.get(1,16)); }
            if c.cond(15) { pc = 0x500d47; continue; }
            pc = 0x500d6e; continue;
            }
            0x500d6e => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x500d3f; continue; }
            pc = 0x500d77; continue;
            }
            0x500d77 => {
            c.set(5,32,0x2u32);
            pc = 0x500d7c; continue;
            }
            0x500d7c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.r[5].wrapping_add(0x1));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(7,16,rm(m,c.r[4].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x24),16));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x500d9f; continue; }
            pc = 0x500d9d; continue;
            }
            0x500d9d => {
            c.set(2,32,c.get(0,32));
            pc = 0x500d9f; continue;
            }
            0x500d9f => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x500db7; continue; }
            pc = 0x500da9; continue;
            }
            0x500da9 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x500dd6; continue;
            }
            0x500db7 => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x500dca; continue; }
            pc = 0x500dbc; continue;
            }
            0x500dbc => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x500dd6; continue;
            }
            0x500dca => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x500dd6; continue;
            }
            0x500dd6 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc8u32,32); }
            if c.cond(14) { pc = 0x500de9; continue; }
            pc = 0x500de4; continue;
            }
            0x500de4 => {
            c.set(0,32,0xc8u32);
            pc = 0x500de9; continue;
            }
            0x500de9 => {
            c.set(2,8,c.get(0,8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x24); wm(m,a,16,c.get(7,16)); }
            if c.cond(5) { pc = 0x500e0d; continue; }
            pc = 0x500e02; continue;
            }
            0x500e02 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x500e1c; continue;
            }
            0x500e0d => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x500e1a; continue; }
            pc = 0x500e12; continue;
            }
            0x500e12 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x500e1c; continue;
            }
            0x500e1a => {
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x500e1c; continue;
            }
            0x500e1c => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x500e3e; continue; }
            pc = 0x500e20; continue;
            }
            0x500e20 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x500e45; continue;
            }
            0x500e3e => {
            { let a = c.r[4].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x24); wm(m,a,16,0xff00u32); }
            pc = 0x500e45; continue;
            }
            0x500e45 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(9) { pc = 0x500d7c; continue; }
            pc = 0x500e4c; continue;
            }
            0x500e4c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x500e94; continue; }
            pc = 0x500e54; continue;
            }
            0x500e54 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c6),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x75u32); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1c),8));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c4),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x500e94);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x500e94; continue;
            }
            0x500e94 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x500ea5);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
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

pub fn f_00515c00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515c00;
    loop {
        match pc {
            0x515c00 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(2,32,c.r[6].wrapping_add(0x4));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x515cee; continue; }
            pc = 0x515c1b; continue;
            }
            0x515c1b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x515c1f; continue;
            }
            0x515c1f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)),8));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x515c48; continue; }
            pc = 0x515c30; continue;
            }
            0x515c30 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); }
            if c.cond(5) { pc = 0x515c48; continue; }
            pc = 0x515c3b; continue;
            }
            0x515c3b => {
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x515c5d; continue; }
            pc = 0x515c43; continue;
            }
            0x515c43 => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x515c5d; continue; }
            pc = 0x515c48; continue;
            }
            0x515c48 => {
            c.set(2,32,c.r[0].wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x515c1f; continue; }
            pc = 0x515c54; continue;
            }
            0x515c54 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x515c5d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(6,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x6));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0xc8u32,8); }
            if c.cond(7) { pc = 0x515c71; continue; }
            pc = 0x515c6c; continue;
            }
            0x515c6c => {
            { let a = c.r[6]; wm(m,a,8,0xc9u32); }
            pc = 0x515c75; continue;
            }
            0x515c71 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            pc = 0x515c75; continue;
            }
            0x515c75 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.sub(a,0xfau32,8); }
            if c.cond(3) { pc = 0x515cc4; continue; }
            pc = 0x515c87; continue;
            }
            0x515c87 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x515cbb; continue; }
            pc = 0x515c91; continue;
            }
            0x515c91 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x515cbb; continue; }
            pc = 0x515c9f; continue;
            }
            0x515c9f => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,0xfau32);
            c.push(m,0x1u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x32u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x515cbb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x5891c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x515cbb; continue;
            }
            0x515cbb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x515cc4 => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.r[2]; wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x515cee; continue; }
            pc = 0x515cd5; continue;
            }
            0x515cd5 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x515cee; continue; }
            pc = 0x515ce3; continue;
            }
            0x515ce3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x515cee);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x515cee; continue;
            }
            0x515cee => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515f70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515f70;
    loop {
        match pc {
            0x515f70 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x663181u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x516153; continue; }
            pc = 0x515f9f; continue;
            }
            0x515f9f => {
            c.set(1,16,rm(m,0x672a2eu32,16));
            { let a = rm(m,c.r[6].wrapping_add(0x16),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x515fb1);
            { let args = []; c.r[0] = host.call(m, 0x515da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x515fba);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x515fcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x515fd8);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.and(a,0xfu32,8); c.set(0,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x51605a; continue; }
            pc = 0x515fe1; continue;
            }
            0x515fe1 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc),16));
            c.set(7,16,rm(m,0x6dd894u32,16));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x673658),8));
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,c.get(1,8),8); c.set(2,8,r); }
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x51602b; continue; }
            pc = 0x516003; continue;
            }
            0x516003 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x516009; continue;
            }
            0x516009 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x15d),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x516022; continue; }
            pc = 0x51601e; continue;
            }
            0x51601e => {
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            if c.cond(4) { pc = 0x51605a; continue; }
            pc = 0x516022; continue;
            }
            0x516022 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x516009; continue; }
            pc = 0x51602b; continue;
            }
            0x51602b => {
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x51605a; continue; }
            pc = 0x51602f; continue;
            }
            0x51602f => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x516038);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_mul(8).wrapping_add(0x32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x516053);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x516170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x51605a; continue;
            }
            0x51605a => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x516063);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.and(a,0x1fu32,8); c.set(0,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x516142; continue; }
            pc = 0x516070; continue;
            }
            0x516070 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            { let a = rm(m,c.r[7].wrapping_add(0x4),16); c.sub(a,c.get(5,16),16); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            if c.cond(3) { pc = 0x5160b9; continue; }
            pc = 0x51608f; continue;
            }
            0x51608f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            pc = 0x516093; continue;
            }
            0x516093 => {
            { let a = c.get(2,16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x51609e; continue; }
            pc = 0x516099; continue;
            }
            0x516099 => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x10); wm(m,a,8,0x1u32); }
            pc = 0x51609e; continue;
            }
            0x51609e => {
            { let a = rm(m,c.r[1].wrapping_add(0x3),8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x51610b; continue; }
            pc = 0x5160a4; continue;
            }
            0x5160a4 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16); c.sub(a,c.get(5,16),16); }
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            if c.cond(2) { pc = 0x516093; continue; }
            pc = 0x5160b9; continue;
            }
            0x5160b9 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x5160e4; continue; }
            pc = 0x5160c2; continue;
            }
            0x5160c2 => {
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x5160d5; continue; }
            pc = 0x5160c8; continue;
            }
            0x5160c8 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(c.r[1]).wrapping_add(0x10); wm(m,a,8,0x1u32); }
            pc = 0x5160d5; continue;
            }
            0x5160d5 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x5160c2; continue; }
            pc = 0x5160e4; continue;
            }
            0x5160e4 => {
            c.set(1,32,0x1u32);
            pc = 0x5160e9; continue;
            }
            0x5160e9 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x10),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x516102; continue; }
            pc = 0x5160f6; continue;
            }
            0x5160f6 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x516118; continue; }
            pc = 0x5160fe; continue;
            }
            0x5160fe => {
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x516118; continue; }
            pc = 0x516102; continue;
            }
            0x516102 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x5160e9; continue; }
            pc = 0x516109; continue;
            }
            0x516109 => {
            pc = 0x516142; continue;
            }
            0x51610b => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x3); wm(m,a,8,0x31u32); }
            pc = 0x516142; continue;
            }
            0x516118 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x3); wm(m,a,8,0x30u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6); wm(m,a,8,0x64u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,16,c.get(7,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            pc = 0x516142; continue;
            }
            0x516142 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x516153);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x516153; continue;
            }
            0x516153 => {
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

pub fn f_005171e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5171e0;
    loop {
        match pc {
            0x5171e0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x517238; continue; }
            pc = 0x5171ed; continue;
            }
            0x5171ed => {
            { let a = rm(m,c.r[6].wrapping_add(0x36),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x517210; continue; }
            pc = 0x5171f2; continue;
            }
            0x5171f2 => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x517201; continue; }
            pc = 0x5171f8; continue;
            }
            0x5171f8 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5171fe);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x517201; continue;
            }
            0x517201 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x51720a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,c.get(3,8)); }
            pc = 0x517210; continue;
            }
            0x517210 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,rm(m,0x6dfc14u32,32),32); }
            if c.cond(5) { pc = 0x517238; continue; }
            pc = 0x517222; continue;
            }
            0x517222 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x517238);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x517238; continue;
            }
            0x517238 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0xa8u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051a3c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51a3c0;
    loop {
        match pc {
            0x51a3c0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.push(m,c.get(6,32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x51a43f; continue; }
            pc = 0x51a3ce; continue;
            }
            0x51a3ce => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x51a43f; continue; }
            pc = 0x51a3f4; continue;
            }
            0x51a3f4 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[1].wrapping_add(0x54));
            pc = 0x51a3f9; continue;
            }
            0x51a3f9 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.get(3,32));
            c.set(7,32,0x18u32);
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x51a416; continue;
            }
            0x51a416 => {
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x51a427; continue; }
            pc = 0x51a41c; continue;
            }
            0x51a41c => {
            { let a = rm(m,c.r[2],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x51a427; continue; }
            pc = 0x51a421; continue;
            }
            0x51a421 => {
            { let a = c.r[2]; wm(m,a,32,0x1u32); }
            pc = 0x51a427; continue;
            }
            0x51a427 => {
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x51a416; continue; }
            pc = 0x51a42d; continue;
            }
            0x51a42d => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x51a3f9; continue; }
            pc = 0x51a43d; continue;
            }
            0x51a43d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x51a43f; continue;
            }
            0x51a43f => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051a450(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51a450;
    loop {
        match pc {
            0x51a450 => {
            c.set(0,32,c.r[1].wrapping_add(0x54));
            c.set(1,32,0x18u32);
            pc = 0x51a458; continue;
            }
            0x51a458 => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x51a463; continue; }
            pc = 0x51a45d; continue;
            }
            0x51a45d => {
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            pc = 0x51a463; continue;
            }
            0x51a463 => {
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x51a458; continue; }
            pc = 0x51a469; continue;
            }
            0x51a469 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051f050(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51f050;
    loop {
        match pc {
            0x51f050 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x776),16));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x51f0ed; continue; }
            pc = 0x51f068; continue;
            }
            0x51f068 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x760),16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x51f0ed; continue; }
            pc = 0x51f075; continue;
            }
            0x51f075 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x51f095; continue; }
            pc = 0x51f07b; continue;
            }
            0x51f07b => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            pc = 0x51f084; continue;
            }
            0x51f084 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6],16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x51f0ed; continue; }
            pc = 0x51f08d; continue;
            }
            0x51f08d => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x51f084; continue; }
            pc = 0x51f095; continue;
            }
            0x51f095 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            c.set(2,8,0xffu32);
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x760),16));
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4); wm(m,a,16,c.get(6,16)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x6); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x7); wm(m,a,8,0x0u32); }
            { let a = c.r[1].wrapping_add(0x760); wm(m,a,16,c.get(0,16)); }
            pc = 0x51f0ed; continue;
            }
            0x51f0ed => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00521240(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x521240;
    loop {
        match pc {
            0x521240 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x5212fd; continue; }
            pc = 0x521254; continue;
            }
            0x521254 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(3,8,0xffu32);
            pc = 0x521259; continue;
            }
            0x521259 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(6,32,c.get(1,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x760),16));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[7].wrapping_add(0x760); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x7),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[0],8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m, 0x5212ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4facd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x3); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x6); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x7); wm(m,a,8,0x0u32); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x521259; continue; }
            pc = 0x5212fa; continue;
            }
            0x5212fa => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5212fd; continue;
            }
            0x5212fd => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524d60(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524d60;
    loop {
        match pc {
            0x524d60 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x524d72);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x768),16));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x524db6; continue; }
            pc = 0x524d88; continue;
            }
            0x524d88 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x524d96; continue;
            }
            0x524d96 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),8));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x524da9; continue; }
            pc = 0x524d9f; continue;
            }
            0x524d9f => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1),8));
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x524dc2; continue; }
            pc = 0x524da9; continue;
            }
            0x524da9 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x524d96; continue; }
            pc = 0x524db6; continue;
            }
            0x524db6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x524dc2 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8); c.sub(a,0x36u32,8); }
            if c.cond(5) { pc = 0x524db6; continue; }
            pc = 0x524dc9; continue;
            }
            0x524dc9 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x6f1),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x22u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1),8));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x32u32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,0x0u32); }
            c.push(m, 0x524e0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005256f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5256f0;
    loop {
        match pc {
            0x5256f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x525715; continue; }
            pc = 0x52570a; continue;
            }
            0x52570a => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(3,32,c.r[1].wrapping_add(c.r[5].wrapping_mul(8)));
            pc = 0x525717; continue;
            }
            0x525715 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x525717; continue;
            }
            0x525717 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x525754; continue; }
            pc = 0x52572c; continue;
            }
            0x52572c => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52573c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4b8),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(0x4b8); wm(m,a,32,c.get(2,32)); }
            pc = 0x525754; continue;
            }
            0x525754 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,16,c.get(0,8));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(6,16)); }
            if c.cond(3) { pc = 0x525792; continue; }
            pc = 0x52576a; continue;
            }
            0x52576a => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52577a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4bc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x4bc); wm(m,a,32,c.get(2,32)); }
            pc = 0x525792; continue;
            }
            0x525792 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5259b8; continue; }
            pc = 0x5257a1; continue;
            }
            0x5257a1 => {
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5257e8; continue; }
            pc = 0x5257ae; continue;
            }
            0x5257ae => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5257c4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x5259b8; continue; }
            pc = 0x5257d2; continue;
            }
            0x5257d2 => {
            { let a = rm(m,c.r[3].wrapping_add(0x3),8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x5257e0; continue; }
            pc = 0x5257d8; continue;
            }
            0x5257d8 => {
            { let a = c.get(1,32); let r = c.add(a,0x2ceu32,32); c.set(1,32,r); }
            pc = 0x5257f6; continue;
            }
            0x5257e0 => {
            { let a = c.get(1,32); let r = c.add(a,0x2d0u32,32); c.set(1,32,r); }
            pc = 0x5257f6; continue;
            }
            0x5257e8 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[7].wrapping_add(0x764));
            pc = 0x5257f6; continue;
            }
            0x5257f6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x52580b; continue; }
            pc = 0x5257ff; continue;
            }
            0x5257ff => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            pc = 0x52588f; continue;
            }
            0x52580b => {
            c.set(0,16,rm(m,c.r[1],16));
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x52588f; continue; }
            pc = 0x52581a; continue;
            }
            0x52581a => {
            c.set(6,32,c.get(5,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            pc = 0x525822; continue;
            }
            0x525822 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x525836; continue; }
            pc = 0x52582b; continue;
            }
            0x52582b => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x525838; continue;
            }
            0x525836 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x525838; continue;
            }
            0x525838 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x525869; continue; }
            pc = 0x525846; continue;
            }
            0x525846 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x525855; continue; }
            pc = 0x52584a; continue;
            }
            0x52584a => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x525857; continue;
            }
            0x525855 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x525857; continue;
            }
            0x525857 => {
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x525822; continue; }
            pc = 0x525863; continue;
            }
            0x525863 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x52588b; continue;
            }
            0x525869 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x52587d; continue; }
            pc = 0x525872; continue;
            }
            0x525872 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x52587f; continue;
            }
            0x52587d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x52587f; continue;
            }
            0x52587f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            pc = 0x52588b; continue;
            }
            0x52588b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x52588f; continue;
            }
            0x52588f => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.r[3].wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5258ee; continue; }
            pc = 0x52589f; continue;
            }
            0x52589f => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5258b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,c.get(0,32));
            { let a = rm(m,c.r[3].wrapping_add(0x3),8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x5258d5; continue; }
            pc = 0x5258c1; continue;
            }
            0x5258c1 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2ce),16));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x2ce); wm(m,a,16,c.get(5,16)); }
            pc = 0x5258f4; continue;
            }
            0x5258d5 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2d0),16));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x2d0); wm(m,a,16,c.get(0,16)); }
            pc = 0x5258f4; continue;
            }
            0x5258ee => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x5258f4; continue;
            }
            0x5258f4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x525901; continue; }
            pc = 0x5258fc; continue;
            }
            0x5258fc => {
            c.push(m, 0x525901);
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x525901; continue;
            }
            0x525901 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x52590c; continue; }
            pc = 0x525905; continue;
            }
            0x525905 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x52590c);
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x52590c; continue;
            }
            0x52590c => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = rm(m,c.r[2].wrapping_add(0xc324),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5259b8; continue; }
            pc = 0x525926; continue;
            }
            0x525926 => {
            { let a = rm(m,c.r[3].wrapping_add(0x3),8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x525943; continue; }
            pc = 0x52592c; continue;
            }
            0x52592c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2ce),16));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x525943);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x521310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x525943; continue;
            }
            0x525943 => {
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x52594d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x521310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5259b8; continue; }
            pc = 0x525954; continue;
            }
            0x525954 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x76a),16));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x525989; continue; }
            pc = 0x52596a; continue;
            }
            0x52596a => {
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5259c2; continue; }
            pc = 0x52596e; continue;
            }
            0x52596e => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x52597d; continue; }
            pc = 0x525972; continue;
            }
            0x525972 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x52597f; continue;
            }
            0x52597d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x52597f; continue;
            }
            0x52597f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x52596a; continue; }
            pc = 0x525989; continue;
            }
            0x525989 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x3),8));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672bb4),8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x5259a2; continue;
            }
            0x5259a2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[3],8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5259b8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x5891c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x5259b8; continue;
            }
            0x5259b8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5259c2 => {
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,0x32u32);
            c.push(m,c.get(1,32));
            pc = 0x5259a2; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052d0b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52d0b0;
    loop {
        match pc {
            0x52d0b0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x663394u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x4),8); c.sub(a,0x7fu32,8); }
            if c.cond(3) { pc = 0x52d2f3; continue; }
            pc = 0x52d0d8; continue;
            }
            0x52d0d8 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x52d0e1);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x52d2e2; continue; }
            pc = 0x52d0fa; continue;
            }
            0x52d0fa => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x52d2e2; continue; }
            pc = 0x52d10c; continue;
            }
            0x52d10c => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x52d2e2; continue; }
            pc = 0x52d131; continue;
            }
            0x52d131 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x52d2e2; continue; }
            pc = 0x52d155; continue;
            }
            0x52d155 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.set(5,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x52d183);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x52d18a);
            { let args = []; c.r[0] = host.call(m, 0x4f8bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1cu32); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x13a),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x19); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(2,32));
            c.push(m,0xffffffffu32);
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x52d1ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(0,32));
            c.push(m,0xfffffffeu32);
            c.push(m, 0x52d1fd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x52d2cd; continue; }
            pc = 0x52d211; continue;
            }
            0x52d211 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52d221);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x52d2cd; continue; }
            pc = 0x52d22c; continue;
            }
            0x52d22c => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(0,32));
            c.push(m,0x27u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x52d242);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(1,16,rm(m,c.r[7],8));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x52d25b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x52d271; continue; }
            pc = 0x52d268; continue;
            }
            0x52d268 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x52d2cd; continue; }
            pc = 0x52d271; continue;
            }
            0x52d271 => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x52d27e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5),8));
            c.push(m,c.get(1,32));
            { let a = c.get(0,8); let r = c.sub(a,0x2u32,8); c.set(0,8,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(0,8); let r = c.and(a,0xf0u32,8); c.set(0,8,r); }
            c.push(m,0x6de3bcu32);
            { let a = c.get(0,32); let r = c.add(a,0x18u32,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.push(m, 0x52d29f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6de3bdu32);
            c.push(m, 0x52d2b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[7],8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x0u32); }
            c.push(m, 0x52d2cd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x52d2cd; continue;
            }
            0x52d2cd => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x52d2de);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            pc = 0x52d2f5; continue;
            }
            0x52d2e2 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x52d2f3);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x52d2f3; continue;
            }
            0x52d2f3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x52d2f5; continue;
            }
            0x52d2f5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052dff0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52dff0;
    loop {
        match pc {
            0x52dff0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(2) { pc = 0x52e005; continue; }
            pc = 0x52dffe; continue;
            }
            0x52dffe => {
            { let a = c.get(0,32); let r = c.sub(a,0xfdu32,32); c.set(0,32,r); }
            pc = 0x52e008; continue;
            }
            0x52e005 => {
            { let a = c.get(0,32); let r = c.add(a,0x7u32,32); c.set(0,32,r); }
            pc = 0x52e008; continue;
            }
            0x52e008 => {
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x52e020; continue; }
            pc = 0x52e018; continue;
            }
            0x52e018 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,c.r[2].wrapping_add(0xffffffff));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x52e020 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005306b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5306b0;
    loop {
        match pc {
            0x5306b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = rm(m,c.r[6].wrapping_add(0x7c),16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x53075a; continue; }
            pc = 0x5306c1; continue;
            }
            0x5306c1 => {
            { let a = c.get(0,16); c.sub(a,0x14u32,16); }
            { let a = c.r[6].wrapping_add(0x7c); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x5306d3; continue; }
            pc = 0x5306cb; continue;
            }
            0x5306cb => {
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,0x80u32); }
            pc = 0x5306da; continue;
            }
            0x5306d3 => {
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            pc = 0x5306da; continue;
            }
            0x5306da => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5306ed);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xffu32);
            c.push(m,c.get(7,32));
            c.push(m, 0x530707);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x7c),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x70),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x53071e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x70),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x530731; continue; }
            pc = 0x530728; continue;
            }
            0x530728 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x53072e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x530731; continue;
            }
            0x530731 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x7c),16));
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(3) { pc = 0x53075a; continue; }
            pc = 0x53073f; continue;
            }
            0x53073f => {
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x70),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(2) { pc = 0x53073f; continue; }
            pc = 0x53075a; continue;
            }
            0x53075a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x7c),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x70),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.r[6].wrapping_add(0x7c); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00530780(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x530780;
    loop {
        match pc {
            0x530780 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x7c),16));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x5307b4; continue; }
            pc = 0x530797; continue;
            }
            0x530797 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x530894; continue; }
            pc = 0x5307a4; continue;
            }
            0x5307a4 => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x70),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x530797; continue; }
            pc = 0x5307b4; continue;
            }
            0x5307b4 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x70),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[1].wrapping_add(0x7c); wm(m,a,16,c.get(6,16)); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(6,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x530894; continue; }
            pc = 0x5307d4; continue;
            }
            0x5307d4 => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x7),8));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x530894; continue; }
            pc = 0x5307e4; continue;
            }
            0x5307e4 => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,16,c.get(6,16)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2e0),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2d8));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x53081d; continue; }
            pc = 0x53080f; continue;
            }
            0x53080f => {
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,16); let r = c.sub(a,c.get(3,16),16); c.set(6,16,r); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            pc = 0x530822; continue;
            }
            0x53081d => {
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x530822; continue;
            }
            0x530822 => {
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x530834; continue; }
            pc = 0x53082f; continue;
            }
            0x53082f => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            pc = 0x530839; continue;
            }
            0x530834 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            pc = 0x530839; continue;
            }
            0x530839 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x7),8));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x2d4); wm(m,a,32,c.get(5,32)); }
            pc = 0x530858; continue;
            }
            0x530858 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x7),8));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2d4));
            c.set(0,32,c.r[5].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2d8),32));
            c.set(3,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x530858; continue; }
            pc = 0x530894; continue;
            }
            0x530894 => {
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

pub fn f_005319d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5319d0;
    loop {
        match pc {
            0x5319d0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x663486u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x89),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x531b5c; continue; }
            pc = 0x5319fa; continue;
            }
            0x5319fa => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x531a9b; continue; }
            pc = 0x531a12; continue;
            }
            0x531a12 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6de518u32,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x19),8));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x531a48);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x531a55);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6df274u32);
            c.push(m, 0x531a66);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6df275u32);
            c.push(m, 0x531a7f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[6],8));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x7u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x531a9b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x531a9b; continue;
            }
            0x531a9b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6de518u32,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m, 0x531ab7);
            { let args = []; c.r[0] = host.call(m, 0x4f8b50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[7].wrapping_add(0x89); wm(m,a,8,0x1u32); }
            c.push(m, 0x531ac7);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,c.get(2,8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x6de518u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x56u32); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x531b06);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(6,32,rm(m,0x6de518u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xfffffffeu32);
            c.set(1,32,0x6dd730u32);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x531b4b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x531b5c);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x531b5c; continue;
            }
            0x531b5c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00533ea0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x533ea0;
    loop {
        match pc {
            0x533ea0 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xc),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x8),16));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            pc = 0x533eb0; continue;
            }
            0x533eb0 => {
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x533ec9; continue; }
            pc = 0x533eb5; continue;
            }
            0x533eb5 => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xc),8));
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x533ec9; continue; }
            pc = 0x533ebf; continue;
            }
            0x533ebf => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x533ed6; continue; }
            pc = 0x533ec9; continue;
            }
            0x533ec9 => {
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x533eb0; continue; }
            pc = 0x533ecf; continue;
            }
            0x533ecf => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x533ed6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00534600(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534600;
    loop {
        match pc {
            0x534600 => {
            c.set(1,32,rm(m,0x6de584u32,32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x534635; continue; }
            pc = 0x53460c; continue;
            }
            0x53460c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            pc = 0x534618; continue;
            }
            0x534618 => {
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x53462f; continue; }
            pc = 0x53461d; continue;
            }
            0x53461d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xc),8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x53462f; continue; }
            pc = 0x534626; continue;
            }
            0x534626 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x534638; continue; }
            pc = 0x53462f; continue;
            }
            0x53462f => {
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x534618; continue; }
            pc = 0x534635; continue;
            }
            0x534635 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x534638 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x534330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00543d10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543d10;
    loop {
        match pc {
            0x543d10 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m, 0x543d24);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x543d31);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x543d5b; continue; }
            pc = 0x543d3e; continue;
            }
            0x543d3e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff),8));
            { let a = c.r[0].wrapping_add(c.r[3]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x543d3e; continue; }
            pc = 0x543d5b; continue;
            }
            0x543d5b => {
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x543d76; continue; }
            pc = 0x543d68; continue;
            }
            0x543d68 => {
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x543d68; continue; }
            pc = 0x543d76; continue;
            }
            0x543d76 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x543d7f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x543d88);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00544590(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544590;
    loop {
        match pc {
            0x544590 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663723u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4]);
            c.push(m, 0x5445b1);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbdc8u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5445fd; continue; }
            pc = 0x5445c2; continue;
            }
            0x5445c2 => {
            c.push(m,0x2cu32);
            c.push(m, 0x5445c9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x5445e2; continue; }
            pc = 0x5445d9; continue;
            }
            0x5445d9 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5445e0);
            { let args = []; c.r[0] = host.call(m, 0x444ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5445e4; continue;
            }
            0x5445e2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5445e4; continue;
            }
            0x5445e4 => {
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x5445fd; continue; }
            pc = 0x5445f6; continue;
            }
            0x5445f6 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5445fd);
            { let args = []; c.r[0] = host.call(m, 0x444d30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5445fd; continue;
            }
            0x5445fd => {
            c.set(1,32,rm(m,0x6df304u32,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x544661; continue; }
            pc = 0x54460a; continue;
            }
            0x54460a => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x54461a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x549f40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,0x14u32);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x5446fe; continue; }
            pc = 0x54462c; continue;
            }
            0x54462c => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x544637);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x54464f; continue; }
            pc = 0x54463e; continue;
            }
            0x54463e => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x544649);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0x8001u32); }
            pc = 0x54464f; continue;
            }
            0x54464f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x54462c; continue; }
            pc = 0x54465c; continue;
            }
            0x54465c => {
            pc = 0x5446fe; continue;
            }
            0x544661 => {
            { let a = c.get(0,32); let r = c.and(a,0x3fffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x54466c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x544698; continue; }
            pc = 0x544675; continue;
            }
            0x544675 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x544686);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x544698 => {
            c.set(1,32,rm(m,0x6df304u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x5446a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54c480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x5446b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x549f40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.push(m, 0x5446bc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,0x14u32);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x5446fe; continue; }
            pc = 0x5446cd; continue;
            }
            0x5446cd => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5446d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x5446f0; continue; }
            pc = 0x5446df; continue;
            }
            0x5446df => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5446ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0x8001u32); }
            pc = 0x5446f0; continue;
            }
            0x5446f0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5446cd; continue; }
            pc = 0x5446fe; continue;
            }
            0x5446fe => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x54470f);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00544730(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544730;
    loop {
        match pc {
            0x544730 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663735u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4]);
            c.push(m, 0x54474f);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6df310u32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x54476c; continue; }
            pc = 0x544760; continue;
            }
            0x544760 => {
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x54476a);
            { let args = []; c.r[0] = host.call(m, 0x54ddf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x544786; continue;
            }
            0x54476c => {
            c.set(1,32,rm(m,0x6df308u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x5447a6; continue; }
            pc = 0x544778; continue;
            }
            0x544778 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x544786);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54da60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x544786; continue;
            }
            0x544786 => {
            c.set(1,32,c.r[4]);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x544797);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5447a6 => {
            { let a = c.get(0,32); let r = c.and(a,0x3fffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5447b2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5447c7; continue; }
            pc = 0x5447bb; continue;
            }
            0x5447bb => {
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x5447c5);
            { let args = []; c.r[0] = host.call(m, 0x54ddf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5447eb; continue;
            }
            0x5447c7 => {
            c.set(0,32,rm(m,0x6df308u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x5447d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54c480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x5447e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54da60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.push(m, 0x5447e8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5447eb; continue;
            }
            0x5447eb => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5447fc);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
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

pub fn f_004f3ce0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3ce0;
    loop {
        match pc {
            0x4f3ce0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662e92u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = rm(m,c.r[6].wrapping_add(0x5),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4f3d72; continue; }
            pc = 0x4f3d10; continue;
            }
            0x4f3d10 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4f3d6b; continue; }
            pc = 0x4f3d17; continue;
            }
            0x4f3d17 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f3d38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b1360u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4f3d49);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f3d58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4f3d69);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f3d72; continue;
            }
            0x4f3d6b => {
            if c.cond(3) { pc = 0x4f3d72; continue; }
            pc = 0x4f3d6d; continue;
            }
            0x4f3d6d => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x1u32); }
            pc = 0x4f3d72; continue;
            }
            0x4f3d72 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4f3d88; continue; }
            pc = 0x4f3d78; continue;
            }
            0x4f3d78 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4f3d84);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4f3ce0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,8,c.get(0,8));
            pc = 0x4f3d8a; continue;
            }
            0x4f3d88 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4f3d8a; continue;
            }
            0x4f3d8a => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),8); c.sub(a,0x3u32,8); }
            if c.cond(3) { pc = 0x4f3e90; continue; }
            pc = 0x4f3d94; continue;
            }
            0x4f3d94 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4f3de0; continue; }
            pc = 0x4f3d98; continue;
            }
            0x4f3d98 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4f3da7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f3daf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f3dc9; continue; }
            pc = 0x4f3db8; continue;
            }
            0x4f3db8 => {
            c.set(2,32,rm(m,0x6b136cu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f3dc4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f3e90; continue;
            }
            0x4f3dc9 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b1364),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f3ddb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f3e90; continue;
            }
            0x4f3de0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f3e37; continue; }
            pc = 0x4f3de8; continue;
            }
            0x4f3de8 => {
            c.set(1,32,rm(m,0x6b135cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f3df6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f3e05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f3e0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b136c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f3e21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4f3e37 => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.push(m, 0x4f3e58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6b136c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f3e70);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f3e7f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4f3e90);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f3e90; continue;
            }
            0x4f3e90 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00534960(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534960;
    loop {
        match pc {
            0x534960 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x60),32));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6e5558u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x534985; continue; }
            pc = 0x53497e; continue;
            }
            0x53497e => {
            { let a = c.r[0].wrapping_add(0x12f8); wm(m,a,8,0x1u32); }
            pc = 0x534985; continue;
            }
            0x534985 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8b50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8b50;
    loop {
        match pc {
            0x4f8b50 => {
            c.push(m,c.get(1,32));
            c.set(0,16,rm(m,0x672dd2u32,16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x14),16));
            { let a = c.r[4]; wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            if c.cond(14) { pc = 0x4f8b8f; continue; }
            pc = 0x4f8b69; continue;
            }
            0x4f8b69 => {
            c.set(6,32,0xc800u32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x4f8b87; continue; }
            pc = 0x4f8b7b; continue;
            }
            0x4f8b7b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f8baf; continue;
            }
            0x4f8b87 => {
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,16,0xc800u32); }
            pc = 0x4f8baf; continue;
            }
            0x4f8b8f => {
            c.set(6,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4f8ba9; continue; }
            pc = 0x4f8b9d; continue;
            }
            0x4f8b9d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f8baf; continue;
            }
            0x4f8ba9 => {
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,16,0x0u32); }
            pc = 0x4f8baf; continue;
            }
            0x4f8baf => {
            c.set(0,32,c.r[1].wrapping_add(0x120));
            c.set(2,32,0x28u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4f8bbb; continue;
            }
            0x4f8bbb => {
            c.fld(c.ld(m,c.r[0],4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x464),4); c.fop('+',v); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.fstp32(m,c.r[0].wrapping_add(0xfffffff4));
            if c.cond(5) { pc = 0x4f8bbb; continue; }
            pc = 0x4f8bcc; continue;
            }
            0x4f8bcc => {
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fdce0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fdce0;
    loop {
        match pc {
            0x4fdce0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.push(m,c.get(7,32));
            c.set(2,8,0xdu32);
            if c.cond(5) { pc = 0x4fdd8f; continue; }
            pc = 0x4fdcf9; continue;
            }
            0x4fdcf9 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd19; continue; }
            pc = 0x4fdcff; continue;
            }
            0x4fdcff => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xfffffffe),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd19; continue; }
            pc = 0x4fdd0c; continue;
            }
            0x4fdd0c => {
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0x8),32),32); c.set(6,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fdd19; continue;
            }
            0x4fdd19 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,c.r[7].wrapping_add(c.r[6]));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd3c; continue; }
            pc = 0x4fdd27; continue;
            }
            0x4fdd27 => {
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0xfffffffd),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd3c; continue; }
            pc = 0x4fdd31; continue;
            }
            0x4fdd31 => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fdd3c; continue;
            }
            0x4fdd3c => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd5e; continue; }
            pc = 0x4fdd4c; continue;
            }
            0x4fdd4c => {
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xfffffffe),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fdd5e; continue; }
            pc = 0x4fdd54; continue;
            }
            0x4fdd54 => {
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fdd5e; continue;
            }
            0x4fdd5e => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[0]),8));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(3,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fde10; continue; }
            pc = 0x4fdd75; continue;
            }
            0x4fdd75 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffd),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fde10; continue; }
            pc = 0x4fdd7f; continue;
            }
            0x4fdd7f => {
            { let a = c.r[7].wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4fdd8f => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,c.r[7].wrapping_add(c.r[6]));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddaf; continue; }
            pc = 0x4fdd9b; continue;
            }
            0x4fdd9b => {
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0xfffffffe),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddaf; continue; }
            pc = 0x4fdda5; continue;
            }
            0x4fdda5 => {
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fddaf; continue;
            }
            0x4fddaf => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7],8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddcd; continue; }
            pc = 0x4fddbd; continue;
            }
            0x4fddbd => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffd),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddcd; continue; }
            pc = 0x4fddc3; continue;
            }
            0x4fddc3 => {
            { let a = c.r[7].wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fddcd; continue;
            }
            0x4fddcd => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddee; continue; }
            pc = 0x4fddd6; continue;
            }
            0x4fddd6 => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xfffffffe),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fddee; continue; }
            pc = 0x4fdde1; continue;
            }
            0x4fdde1 => {
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x8),32),32); c.set(6,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fddee; continue;
            }
            0x4fddee => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fde10; continue; }
            pc = 0x4fddfd; continue;
            }
            0x4fddfd => {
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xfffffffd),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4fde10; continue; }
            pc = 0x4fde05; continue;
            }
            0x4fde05 => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffffe); wm(m,a,8,c.get(2,8)); }
            pc = 0x4fde10; continue;
            }
            0x4fde10 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00534330(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534330;
    loop {
        match pc {
            0x534330 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5345f0; continue; }
            pc = 0x53434d; continue;
            }
            0x53434d => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5345f0; continue; }
            pc = 0x53435b; continue;
            }
            0x53435b => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53436b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53437d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x53438f; continue; }
            pc = 0x534386; continue;
            }
            0x534386 => {
            { let a = rm(m,c.r[5],32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5345f0; continue; }
            pc = 0x53438f; continue;
            }
            0x53438f => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x534573; continue; }
            pc = 0x5343af; continue;
            }
            0x5343af => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x534419; continue; }
            pc = 0x5343b2; continue;
            }
            0x5343b2 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5345e3; continue; }
            pc = 0x5343b9; continue;
            }
            0x5343b9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(7,32,rm(m,c.r[5],32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4b8),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x4b8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4bc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x4bc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4bc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0x4bc); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4b8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x4b8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x2u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x534419 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x5345e3; continue; }
            pc = 0x53442a; continue;
            }
            0x53442a => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            if c.cond(3) { pc = 0x534489; continue; }
            pc = 0x534446; continue;
            }
            0x534446 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x534489; continue; }
            pc = 0x534455; continue;
            }
            0x534455 => {
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x134); wm(m,a,16,0x0u32); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c); wm(m,a,8,r); } }
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(2,16,rm(m,0x6dd888u32,16));
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = 0x6dd888u32; wm(m,a,16,c.get(1,16)); }
            pc = 0x534489; continue;
            }
            0x534489 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xe),16));
            c.set(1,32,c.r[3].wrapping_add(0xe));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x5345e3; continue; }
            pc = 0x53449d; continue;
            }
            0x53449d => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x10),16));
            pc = 0x5344a1; continue;
            }
            0x5344a1 => {
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x5344d4; continue; }
            pc = 0x5344a6; continue;
            }
            0x5344a6 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x5344a1; continue; }
            pc = 0x5344c3; continue;
            }
            0x5344c3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x2u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5344d4 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xe),16));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[7],8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x534508);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53450f);
            { let args = []; c.r[0] = host.call(m, 0x4f5900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = rm(m,c.r[7].wrapping_add(0x3d),8); let r = c.or(a,0x4u32,8); { let a = c.r[7].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x47c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x478),16); let r = c.inc(a,16); { let a = c.r[5].wrapping_add(0x478); wm(m,a,16,r); } }
            { let a = c.r[5].wrapping_add(0x47c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x47c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(0x478),16); let r = c.dec(a,16); { let a = c.r[3].wrapping_add(0x478); wm(m,a,16,r); } }
            { let a = c.r[3].wrapping_add(0x47c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4bc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0x4bc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4b8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x4b8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x2u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x534573 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53458b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5345e3; continue; }
            pc = 0x53458f; continue;
            }
            0x53458f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5345b4; continue; }
            pc = 0x5345a9; continue;
            }
            0x5345a9 => {
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x5345c0; continue;
            }
            0x5345b4 => {
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x5345c0; continue;
            }
            0x5345c0 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5345c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5007a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4a0),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0x4a0); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = rm(m,c.r[3].wrapping_add(0x49c),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[3].wrapping_add(0x49c); wm(m,a,32,r); } }
            pc = 0x5345e3; continue;
            }
            0x5345e3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x2u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x5345f0; continue;
            }
            0x5345f0 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
